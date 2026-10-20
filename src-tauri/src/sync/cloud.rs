use std::fs;
use std::path::{Path, PathBuf};

use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::case_profile::{self, CaseMode, TimelineEvent};
use crate::seal;
use super::local::{calculate_file_sha256, load_case_sync_metadata, route_doctype_to_subfolder, save_case_sync_metadata, save_cloud_session};
use super::types::{firebase_api_key, firebase_project_id, firebase_storage_bucket, CloudSession, PullReport, PushReport, SyncError, SyncFileRecord};

pub fn cloud_sign_in_password(email: &str, password: &str) -> Result<CloudSession, String> {
    let api_key = firebase_api_key();
    if api_key.trim().is_empty() {
        return Err("Firebase API key is empty. Unset or fix JUSTLEGAL_FIREBASE_API_KEY / FIREBASE_API_KEY.".to_string());
    }

    let client = Client::builder().build().map_err(|e| e.to_string())?;
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );
    let payload = json!({
        "email": email.trim(),
        "password": password,
        "returnSecureToken": true
    });

    let resp = client
        .post(&url)
        .json(&payload)
        .send()
        .map_err(|e| format!("Network request failed: {e}"))?;

    let status = resp.status();
    let body: Value = resp.json().map_err(|e| format!("Invalid JSON response: {e}"))?;

    if !status.is_success() {
        let msg = body
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .unwrap_or("Sign-in failed");
        return Err(format!("Firebase Authentication error: {msg}"));
    }

    let id_token = body
        .get("idToken")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing idToken".to_string())?
        .to_string();
    let refresh_token = body
        .get("refreshToken")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let uid = body
        .get("localId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing uid".to_string())?
        .to_string();
    let email_resp = body
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or(email)
        .to_string();
    let expires_in: u64 = body
        .get("expiresIn")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600);

    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + expires_in;

    let session = CloudSession {
        id_token,
        refresh_token,
        uid,
        email: email_resp,
        expires_at,
    };

    save_cloud_session(&session)?;
    Ok(session)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Refresh the ID token 5 minutes before it expires.
const REFRESH_MARGIN_SECS: u64 = 300;

/// Exchange the refresh token for a new ID token and persist the session.
/// Never logs token values.
pub fn cloud_refresh_session(session: &mut CloudSession) -> Result<(), String> {
    if session.refresh_token.trim().is_empty() {
        return Err("Cloud session expired and has no refresh token. Please sign in again.".to_string());
    }
    let api_key = firebase_api_key();
    let client = Client::builder().build().map_err(|e| e.to_string())?;
    let url = format!("https://securetoken.googleapis.com/v1/token?key={}", api_key);
    let resp = client
        .post(&url)
        .form(&[("grant_type", "refresh_token"), ("refresh_token", session.refresh_token.as_str())])
        .send()
        .map_err(|e| format!("Token refresh request failed: {e}"))?;
    let status = resp.status();
    let body: Value = resp.json().map_err(|e| format!("Invalid token refresh response: {e}"))?;
    if !status.is_success() {
        let msg = body
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .unwrap_or("refresh failed");
        return Err(format!("Cloud session refresh failed ({msg}). Please sign in again."));
    }
    let id_token = body
        .get("id_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Token refresh response missing id_token".to_string())?;
    session.id_token = id_token.to_string();
    if let Some(rt) = body.get("refresh_token").and_then(|v| v.as_str()) {
        if !rt.is_empty() {
            session.refresh_token = rt.to_string();
        }
    }
    if let Some(uid) = body.get("user_id").and_then(|v| v.as_str()) {
        if !uid.is_empty() {
            session.uid = uid.to_string();
        }
    }
    let expires_in: u64 = body
        .get("expires_in")
        .and_then(|v| v.as_str().and_then(|s| s.parse().ok()).or_else(|| v.as_u64()))
        .unwrap_or(3600);
    session.expires_at = now_secs() + expires_in;
    save_cloud_session(session)?;
    Ok(())
}

/// Refresh the session if the ID token is expired, unknown, or within the margin.
pub fn ensure_fresh_session(session: &mut CloudSession) -> Result<(), String> {
    if session.expires_at <= now_secs() + REFRESH_MARGIN_SECS {
        if session.refresh_token.trim().is_empty() {
            // Nothing we can do; let the request try and surface a 401 if needed.
            return Ok(());
        }
        cloud_refresh_session(session)?;
    }
    Ok(())
}

/// Send an authenticated request; on HTTP 401, refresh the token once and retry.
fn send_auth<F>(session: &mut CloudSession, build: F) -> Result<Response, String>
where
    F: Fn(&str) -> RequestBuilder,
{
    let resp = build(&session.id_token)
        .send()
        .map_err(|e| format!("Network request failed: {e}"))?;
    if resp.status() == StatusCode::UNAUTHORIZED && !session.refresh_token.trim().is_empty() {
        cloud_refresh_session(session)?;
        return build(&session.id_token)
            .send()
            .map_err(|e| format!("Network request failed: {e}"));
    }
    Ok(resp)
}

const MAX_UPLOAD_BYTES: u64 = 100 * 1024 * 1024;

/// Content type for an upload, or None if Storage rules would reject it.
fn upload_content_type(fname: &str) -> Option<&'static str> {
    let ext = Path::new(fname)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "pdf" => Some("application/pdf"),
        "doc" => Some("application/msword"),
        "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "tif" | "tiff" => Some("image/tiff"),
        "txt" => Some("text/plain"),
        _ => None,
    }
}

fn extract_string_field(doc: &Value, name: &str) -> String {
    doc.pointer(&format!("/fields/{}/stringValue", name))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn cloud_pull_all(
    session: &CloudSession,
    target_root: &Path,
    filter_case_id: Option<&str>,
) -> Result<PullReport, String> {
    let mut sess = session.clone();
    ensure_fresh_session(&mut sess)?;
    let session_uid = sess.uid.clone();
    let client = Client::builder().build().map_err(|e| e.to_string())?;
    let project_id = firebase_project_id();
    let storage_bucket = firebase_storage_bucket();

    let run_query_url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:runQuery",
        project_id
    );

    let query_payload = json!({
        "structuredQuery": {
            "from": [{ "collectionId": "cases" }],
            "where": {
                "fieldFilter": {
                    "field": { "fieldPath": "userId" },
                    "op": "EQUAL",
                    "value": { "stringValue": session_uid }
                }
            }
        }
    });

    let resp = send_auth(&mut sess, |t| client.post(&run_query_url).bearer_auth(t).json(&query_payload))
        .map_err(|e| format!("Firestore query failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Firestore returned HTTP {}", resp.status()));
    }

    let items: Vec<Value> = resp.json().map_err(|e| format!("Query response parse error: {e}"))?;
    let mut report = PullReport::default();
    report.ok = true;

    // Load existing local cases to check matches
    let local_cases = case_profile::list_cases_in(target_root).unwrap_or_default();

    for item in items {
        let doc = match item.get("document") {
            Some(d) => d,
            None => continue,
        };

        let full_name = doc.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let cloud_case_id = full_name.rsplit('/').next().unwrap_or("").to_string();
        if cloud_case_id.is_empty() {
            continue;
        }

        if let Some(target_id) = filter_case_id {
            if cloud_case_id != target_id {
                continue;
            }
        }

        let title = extract_string_field(doc, "title");
        let case_number = extract_string_field(doc, "caseNumber");
        // Optional fields: older docs may lack them (extract_string_field returns "").
        let mut venue = extract_string_field(doc, "venue");
        if venue.is_empty() {
            venue = extract_string_field(doc, "court");
        }
        if venue.is_empty() {
            venue = extract_string_field(doc, "jurisdiction");
        }
        let state = extract_string_field(doc, "state");

        // Match existing local case: either by cloud_matter_id OR (if empty) by docket_number
        let mut matched_dir: Option<PathBuf> = None;
        for c in &local_cases {
            if c.cloud_matter_id.as_deref() == Some(&cloud_case_id) {
                matched_dir = Some(target_root.join(&c.id));
                break;
            }
            if !case_number.is_empty() && c.docket_number.trim() == case_number.trim() {
                matched_dir = Some(target_root.join(&c.id));
                break;
            }
        }

        let case_dir = match matched_dir {
            Some(dir) => {
                report.cases_updated += 1;
                dir
            }
            None => {
                // Generate a clean slug or UUID
                let slug = if !title.is_empty() {
                    let s: String = title
                        .to_ascii_lowercase()
                        .chars()
                        .map(|c| if c.is_alphanumeric() { c } else { '-' })
                        .collect();
                    s.split('-').filter(|x| !x.is_empty()).collect::<Vec<&str>>().join("-")
                } else {
                    cloud_case_id.clone()
                };
                let new_dir = target_root.join(&slug);
                let (dir, _) = case_profile::create_case_folder(
                    &new_dir,
                    if !title.is_empty() { &title } else { "Untitled Case" },
                    CaseMode::Standard,
                    "motion",
                )?;
                report.cases_pulled += 1;
                dir
            }
        };

        // Strict guard: skip if sealed or confidential
        if seal::is_dir_sealed(&case_dir) {
            report.issues.push(format!("Skipping sealed case folder: {}", case_dir.display()));
            continue;
        }

        let mut profile = case_profile::load_case(&case_dir)?;
        if profile.is_sync_forbidden() {
            report.issues.push(format!("Skipping confidential case: {}", profile.title));
            continue;
        }

        // Update profile metadata
        profile.cloud_matter_id = Some(cloud_case_id.clone());
        profile.cloud_synced_at = Some(chrono::Utc::now().to_rfc3339());
        if profile.docket_number.is_empty() && !case_number.is_empty() {
            profile.docket_number = case_number;
        }
        if profile.court.name.is_empty() && !venue.is_empty() {
            profile.court.name = venue;
        }
        if profile.rule_pack_id == "generic-us-appellate" && state.to_ascii_lowercase().contains("new jersey") {
            profile.rule_pack_id = "nj-app-div".into();
        }

        // Pull Events (Timeline)
        let events_url = format!(
            "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/cases/{}/events?pageSize=100",
            project_id, cloud_case_id
        );
        if let Ok(ev_resp) = send_auth(&mut sess, |t| client.get(&events_url).bearer_auth(t)) {
            if ev_resp.status().is_success() {
                if let Ok(ev_body) = ev_resp.json::<Value>() {
                    if let Some(docs) = ev_body.get("documents").and_then(|v| v.as_array()) {
                        for edoc in docs {
                            let etitle = extract_string_field(edoc, "title");
                            let edate = extract_string_field(edoc, "dateText");
                            let edesc = extract_string_field(edoc, "description");
                            let ename = edoc.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let eid = format!("cloud:{}", ename.rsplit('/').next().unwrap_or(""));

                            if !profile.timeline.iter().any(|x| x.id == eid || (x.date == edate && x.title == etitle)) {
                                profile.timeline.push(TimelineEvent {
                                    id: eid,
                                    title: etitle,
                                    date: edate,
                                    notes: edesc,
                                });
                            }
                        }
                    }
                }
            }
        }

        case_profile::save_case(&case_dir, &profile)?;

        // Pull Documents & Download Attached Storage Files
        let mut sync_meta = load_case_sync_metadata(&case_dir);
        sync_meta.cloud_matter_id = cloud_case_id.clone();
        sync_meta.last_synced_at = chrono::Utc::now().to_rfc3339();

        let docs_url = format!(
            "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/cases/{}/documents?pageSize=100",
            project_id, cloud_case_id
        );
        if let Ok(doc_resp) = send_auth(&mut sess, |t| client.get(&docs_url).bearer_auth(t)) {
            if doc_resp.status().is_success() {
                if let Ok(doc_body) = doc_resp.json::<Value>() {
                    if let Some(docs) = doc_body.get("documents").and_then(|v| v.as_array()) {
                        for ddoc in docs {
                            let dtitle = extract_string_field(ddoc, "title");
                            let doc_type = extract_string_field(ddoc, "docType");
                            let mut orig_name = extract_string_field(ddoc, "originalFilename");
                            if orig_name.is_empty() {
                                orig_name = if dtitle.is_empty() {
                                    "unnamed-record.pdf".into()
                                } else if dtitle.ends_with(".pdf") {
                                    dtitle.clone()
                                } else {
                                    format!("{}.pdf", dtitle)
                                };
                            }

                            let dname = ddoc.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let doc_id = dname.rsplit('/').next().unwrap_or("").to_string();
                            let subfolder = route_doctype_to_subfolder(&doc_type);
                            let target_folder = case_dir.join(subfolder);
                            let _ = fs::create_dir_all(&target_folder);
                            let dest_file = target_folder.join(&orig_name);

                            // Check if already synced with same doc_id and exists
                            let already_synced = sync_meta.files.iter().any(|f| f.doc_id == doc_id && dest_file.exists());
                            if already_synced {
                                continue;
                            }

                            // Download from Firebase Storage
                            // Primary path: cases/{caseId}/documents/{docId}/{orig_name}
                            let storage_path = format!("cases/{}/documents/{}/{}", cloud_case_id, doc_id, orig_name);
                            let encoded_path = urlencoding::encode(&storage_path);
                            let download_url = format!(
                                "https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media",
                                storage_bucket, encoded_path
                            );

                            let mut file_downloaded = false;
                            if let Ok(file_resp) = send_auth(&mut sess, |t| client.get(&download_url).bearer_auth(t)) {
                                if file_resp.status().is_success() {
                                    if let Ok(bytes) = file_resp.bytes() {
                                        if fs::write(&dest_file, &bytes).is_ok() {
                                            file_downloaded = true;
                                        }
                                    }
                                }
                            }

                            if !file_downloaded {
                                report.issues.push(format!("Could not download '{}' (doc {}) from cloud storage", orig_name, doc_id));
                            }

                            if file_downloaded {
                                report.files_downloaded += 1;
                                let hash = calculate_file_sha256(&dest_file).unwrap_or_default();
                                let size = fs::metadata(&dest_file).map(|m| m.len()).unwrap_or(0);
                                sync_meta.files.push(SyncFileRecord {
                                    doc_id,
                                    title: dtitle,
                                    filename: orig_name,
                                    subfolder: subfolder.to_string(),
                                    sha256: hash,
                                    size_bytes: size,
                                    synced_at: chrono::Utc::now().to_rfc3339(),
                                });
                            }
                        }
                    }
                }
            }
        }

        let _ = save_case_sync_metadata(&case_dir, &sync_meta);
    }

    Ok(report)
}

pub fn cloud_push_case(case_dir: &Path, session: &CloudSession) -> Result<PushReport, String> {
    if seal::is_dir_sealed(case_dir) {
        return Err(SyncError::ConfidentialForbidden.to_string());
    }
    let mut profile = case_profile::load_case(case_dir)
        .map_err(|e| format!("Could not load case profile: {e}"))?;
    if profile.is_sync_forbidden() {
        return Err(SyncError::ConfidentialForbidden.to_string());
    }

    let mut sess = session.clone();
    ensure_fresh_session(&mut sess)?;
    let session_uid = sess.uid.clone();
    let client = Client::builder().build().map_err(|e| e.to_string())?;
    let project_id = firebase_project_id();
    let storage_bucket = firebase_storage_bucket();

    // Create cloud case if missing
    let cloud_matter_id = match &profile.cloud_matter_id {
        Some(id) if !id.trim().is_empty() => id.clone(),
        _ => {
            let create_url = format!(
                "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/cases",
                project_id
            );
            let payload = json!({
                "fields": {
                    "userId": { "stringValue": session_uid },
                    "title": { "stringValue": profile.title },
                    "caseNumber": { "stringValue": profile.docket_number },
                    "venue": { "stringValue": profile.court.name },
                    "jurisdiction": { "stringValue": profile.court.name },
                    "status": { "stringValue": "Active" },
                    "caseType": { "stringValue": "Civil" },
                    "createdAt": { "timestampValue": profile.created_at },
                    "updatedAt": { "timestampValue": chrono::Utc::now().to_rfc3339() }
                }
            });

            let resp = send_auth(&mut sess, |t| client.post(&create_url).bearer_auth(t).json(&payload))
                .map_err(|e| format!("Case creation on cloud failed: {e}"))?;

            if !resp.status().is_success() {
                return Err(format!("Cloud returned HTTP {}", resp.status()));
            }

            let body: Value = resp.json().map_err(|e| format!("Invalid JSON: {e}"))?;
            let full_name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let new_id = full_name.rsplit('/').next().unwrap_or("").to_string();
            if new_id.is_empty() {
                return Err("Failed to obtain created matter ID".into());
            }

            profile.cloud_matter_id = Some(new_id.clone());
            case_profile::save_case(case_dir, &profile)?;
            new_id
        }
    };

    let mut sync_meta = load_case_sync_metadata(case_dir);
    sync_meta.cloud_matter_id = cloud_matter_id.clone();
    let mut files_uploaded = 0;
    let mut skipped: Vec<String> = Vec::new();

    // Check exports/ folder for compiled documents to push
    let exports_dir = case_dir.join("exports");
    if exports_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&exports_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
                    let size_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    let content_type = match upload_content_type(&fname) {
                        Some(ct) => ct,
                        None => {
                            skipped.push(format!("{fname} (file type not allowed by cloud storage)"));
                            continue;
                        }
                    };
                    if size_bytes > MAX_UPLOAD_BYTES {
                        skipped.push(format!("{fname} (over 100 MB limit)"));
                        continue;
                    }
                    let hash = calculate_file_sha256(&path).unwrap_or_default();

                    // If already synced with same SHA-256, skip
                    if sync_meta.files.iter().any(|f| f.filename == fname && f.sha256 == hash) {
                        continue;
                    }

                    // 1. Create document record in Firestore
                    let doc_create_url = format!(
                        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/cases/{}/documents",
                        project_id, cloud_matter_id
                    );
                    let doc_payload = json!({
                        "fields": {
                            "title": { "stringValue": fname },
                            "docType": { "stringValue": "Court Filing" },
                            "originalFilename": { "stringValue": fname },
                            "contentType": { "stringValue": content_type },
                            "sizeBytes": { "integerValue": size_bytes.to_string() },
                            "uploadedBy": { "stringValue": session_uid },
                            "createdAt": { "timestampValue": chrono::Utc::now().to_rfc3339() },
                            "updatedAt": { "timestampValue": chrono::Utc::now().to_rfc3339() }
                        }
                    });

                    // 1. The Firestore record must exist BEFORE the Storage upload (rules check it).
                    let dresp = match send_auth(&mut sess, |t| client.post(&doc_create_url).bearer_auth(t).json(&doc_payload)) {
                        Ok(r) => r,
                        Err(e) => {
                            skipped.push(format!("{fname} (could not create cloud record: {e})"));
                            continue;
                        }
                    };
                    if !dresp.status().is_success() {
                        skipped.push(format!("{fname} (cloud record creation returned HTTP {})", dresp.status()));
                        continue;
                    }
                    let doc_id = dresp
                        .json::<Value>()
                        .ok()
                        .and_then(|b| b.get("name").and_then(|v| v.as_str()).map(|s| s.rsplit('/').next().unwrap_or("").to_string()))
                        .unwrap_or_default();
                    if doc_id.is_empty() {
                        skipped.push(format!("{fname} (cloud record created but no document ID returned)"));
                        continue;
                    }

                    // 2. Upload file bytes to cases/{caseId}/documents/{docId}/{file}
                    let storage_path = format!("cases/{}/documents/{}/{}", cloud_matter_id, doc_id, fname);
                    let upload_url = format!(
                        "https://firebasestorage.googleapis.com/v0/b/{}/o?name={}",
                        storage_bucket,
                        urlencoding::encode(&storage_path)
                    );
                    let bytes = match fs::read(&path) {
                        Ok(b) => b,
                        Err(e) => {
                            skipped.push(format!("{fname} (could not read local file: {e})"));
                            continue;
                        }
                    };
                    let upload = send_auth(&mut sess, |t| {
                        client
                            .post(&upload_url)
                            .bearer_auth(t)
                            .header("Content-Type", content_type)
                            .body(bytes.clone())
                    });
                    match upload {
                        Ok(r) if r.status().is_success() => {}
                        Ok(r) => {
                            skipped.push(format!("{fname} (storage upload returned HTTP {}; cloud record {doc_id} has no file)", r.status()));
                            continue;
                        }
                        Err(e) => {
                            skipped.push(format!("{fname} (storage upload failed: {e}; cloud record {doc_id} has no file)"));
                            continue;
                        }
                    }

                    files_uploaded += 1;
                    sync_meta.files.push(SyncFileRecord {
                        doc_id,
                        title: fname.clone(),
                        filename: fname,
                        subfolder: "exports".to_string(),
                        sha256: hash,
                        size_bytes,
                        synced_at: chrono::Utc::now().to_rfc3339(),
                    });
                }
            }
        }
    }

    sync_meta.last_synced_at = chrono::Utc::now().to_rfc3339();
    let _ = save_case_sync_metadata(case_dir, &sync_meta);

    Ok(PushReport {
        ok: true,
        cloud_matter_id,
        files_uploaded,
        message: if skipped.is_empty() {
            format!("Successfully synced with cloud (uploaded {} files)", files_uploaded)
        } else {
            format!(
                "Synced with cloud (uploaded {} files). Skipped {}: {}",
                files_uploaded,
                skipped.len(),
                skipped.join("; ")
            )
        },
    })
}

// Simple URL encoder helper to avoid external crate requirement
pub mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut out = String::new();
        for b in s.bytes() {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                out.push(b as char);
            } else {
                out.push_str(&format!("%{:02X}", b));
            }
        }
        out
    }
}
