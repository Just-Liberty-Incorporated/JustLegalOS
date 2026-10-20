use serde_json::{json, Value};

use crate::case_profile;
use crate::check;
use crate::court_md;
use crate::documents;
use crate::rule_pack::RulePack;

pub fn validate_mcp_path(path_str: &str, safe_mode: bool) -> Result<std::path::PathBuf, String> {
    let trimmed = path_str.trim();
    if trimmed.is_empty() {
        return Err("A case path is required.".to_string());
    }

    let p = std::path::Path::new(trimmed);

    // Reject path traversal components
    for comp in p.components() {
        if matches!(comp, std::path::Component::ParentDir) {
            return Err(format!(
                "Access denied: relative path traversal ('..') is strictly prohibited in MCP tools: '{trimmed}'"
            ));
        }
    }

    // Resolve: if relative, resolve under default_cases_root()
    let resolved = if p.is_relative() {
        case_profile::default_cases_root().join(p)
    } else {
        p.to_path_buf()
    };

    let canon = crate::sandbox::canonicalize_existing_prefix(&resolved);

    // Block raw system roots
    let canon_str = canon.to_string_lossy();
    if canon_str == "/" || canon_str.ends_with(":\\") || canon_str.ends_with(":/") {
        return Err(format!("Access denied: cannot operate on system root: '{trimmed}'"));
    }

    #[cfg(windows)]
    {
        if let Ok(win_dir) = std::env::var("SystemRoot") {
            let canon_win = crate::sandbox::canonicalize_existing_prefix(std::path::Path::new(&win_dir));
            if canon.starts_with(&canon_win) {
                return Err(format!("Access denied: access to Windows system directory is forbidden: '{trimmed}'"));
            }
        }
        if let Ok(prog_files) = std::env::var("ProgramFiles") {
            let canon_prog = crate::sandbox::canonicalize_existing_prefix(std::path::Path::new(&prog_files));
            if canon.starts_with(&canon_prog) {
                return Err(format!("Access denied: access to Program Files directory is forbidden: '{trimmed}'"));
            }
        }
    }

    // If safe_mode is active, enforce confinement to allowed locations:
    if safe_mode {
        let cases_root = crate::sandbox::canonicalize_existing_prefix(&case_profile::default_cases_root());
        let is_under_cases = canon == cases_root || canon.starts_with(&cases_root);

        let is_under_cwd = if let Ok(cwd) = std::env::current_dir() {
            let canon_cwd = crate::sandbox::canonicalize_existing_prefix(&cwd);
            if let Some(home) = dirs::home_dir() {
                let canon_home = crate::sandbox::canonicalize_existing_prefix(&home);
                if canon_cwd != canon_home {
                    canon == canon_cwd || canon.starts_with(&canon_cwd)
                } else {
                    false
                }
            } else {
                canon == canon_cwd || canon.starts_with(&canon_cwd)
            }
        } else {
            false
        };

        let is_under_temp = {
            let temp = crate::sandbox::canonicalize_existing_prefix(&std::env::temp_dir());
            canon == temp || canon.starts_with(&temp)
        };

        if !is_under_cases && !is_under_cwd && !is_under_temp {
            return Err(format!(
                "Access denied: path '{}' is outside permitted JustLegal OS case directories ({}). MCP tools are strictly confined to protect system privacy.",
                trimmed,
                cases_root.display()
            ));
        }

        if crate::seal::is_dir_sealed(&canon) || crate::seal::read_marker_sealed_at(&canon).is_some() {
            return Err("This matter is sealed/confidential under airgap protection (EO 14395). External bot access over MCP is strictly prohibited. Access locally in JustLegal OS.".to_string());
        }
    }

    Ok(canon)
}

pub fn call_tool(name: &str, args: &Value) -> Result<Value, String> {
    call_tool_with_opts(name, args, true)
}

pub fn call_tool_with_opts(name: &str, args: &Value, safe_mode: bool) -> Result<Value, String> {
    let s = |k: &str| args.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();

    if safe_mode && ["cloud_pull", "cloud_push", "sign_pdf", "packs_remove"].contains(&name) {
        return Err(format!("Tool '{name}' is restricted in bot-safe mode. High-risk cloud sync and signature operations must be executed directly by the user in JustLegal OS."));
    }

    let path_arg = s("path");
    let valid_path = if !path_arg.is_empty() {
        Some(validate_mcp_path(&path_arg, safe_mode)?)
    } else {
        None
    };

    match name {
        "case_list" => {
            let root = if s("root").is_empty() {
                case_profile::default_cases_root()
            } else {
                validate_mcp_path(&s("root"), safe_mode)?
            };
            let profiles = case_profile::list_cases_in(&root)?;
            let mut cases_out = Vec::new();
            for p in profiles {
                let case_dir = root.join(&p.id);
                let dir = if case_dir.exists() { case_dir } else { root.join(&p.id) };
                let is_sealed = crate::seal::is_dir_sealed(&dir) || p.mode == case_profile::CaseMode::Confidential || p.sealed_at.is_some();
                if is_sealed && safe_mode {
                    cases_out.push(json!({
                        "id": p.id,
                        "title": "[Confidential / Sealed Case — Protected from Bot Access]",
                        "is_sealed": true,
                        "is_confidential": true,
                        "court": p.court.name,
                        "governing_state": p.governing_state
                    }));
                } else {
                    cases_out.push(json!({
                        "id": p.id,
                        "title": p.title,
                        "path": dir.display().to_string(),
                        "kind": p.document_kind,
                        "court": p.court.name,
                        "governing_state": p.governing_state,
                        "docket_number": p.docket_number,
                        "is_sealed": is_sealed,
                        "is_confidential": p.mode == case_profile::CaseMode::Confidential,
                        "updated_at": p.updated_at
                    }));
                }
            }
            Ok(json!({
                "root": root.display().to_string(),
                "count": cases_out.len(),
                "cases": cases_out,
                "never_files": true
            }))
        }
        "case_create" => {
            let path = valid_path.clone().unwrap_or_else(|| std::path::PathBuf::from(s("path")));
            let kind = if s("kind").is_empty() { "motion".into() } else { s("kind") };
            let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
            if dry_run {
                return Ok(json!({
                    "dry_run": true,
                    "would_create": path.display().to_string(),
                    "title": s("title"),
                    "kind": kind,
                    "pack": s("pack"),
                    "never_files": true
                }));
            }
            let (dir, mut profile) = case_profile::create_case_folder(
                &path,
                &s("title"),
                case_profile::CaseMode::Standard,
                &kind,
            )?;
            if !s("pack").is_empty() {
                profile.rule_pack_id = s("pack");
                profile.court.id = s("pack");
                case_profile::save_case(&dir, &profile)?;
            }
            Ok(json!({"id": profile.id, "folder": dir, "never_files": true}))
        }
        "draft_import" => {
            let p = valid_path.as_ref().ok_or_else(|| "A case path is required".to_string())?;
            let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
            if dry_run {
                let _profile = case_profile::load_case(p)?;
                let doc = court_md::parse_markdown(&s("markdown"));
                return Ok(json!({
                    "dry_run": true,
                    "would_import_to": p.display().to_string(),
                    "lines": doc.lines.len(),
                    "case_no": doc.meta.case_no,
                    "never_files": true
                }));
            }
            let doc = court_md::import_markdown(p, &s("markdown"))?;
            Ok(json!({"lines": doc.lines.len(), "case_no": doc.meta.case_no, "never_files": true}))
        }
        "draft_preview" => {
            let profile = if s("path").is_empty() {
                case_profile::CaseProfile::new_standard("Preview", "motion")
            } else {
                let p = valid_path.as_ref().unwrap();
                case_profile::load_case(p)?
            };
            let preview = court_md::preview_markdown(&profile, &s("markdown"));
            Ok(serde_json::to_value(preview).unwrap())
        }
        "evidence_add" => {
            let p = valid_path.as_ref().ok_or_else(|| "A case path is required".to_string())?;
            let file = s("file");
            if file.is_empty() {
                return Err("Exhibit file path is required".to_string());
            }
            let file_p = std::path::Path::new(&file);
            let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
            let file_name = file_p.file_name().and_then(|n| n.to_str()).unwrap_or("exhibit.bin");
            if dry_run {
                if !file_p.exists() {
                    return Err(format!("Source exhibit file does not exist: '{file}'"));
                }
                return Ok(json!({
                    "dry_run": true,
                    "would_add_exhibit": file_name,
                    "target_case": p.display().to_string(),
                    "description": s("description"),
                    "never_files": true
                }));
            }
            let bytes = std::fs::read(&file).map_err(|e| e.to_string())?;
            let dest = check::add_exhibit(
                p,
                file_name,
                &bytes,
                &s("description"),
            )?;
            Ok(json!({"path": dest, "never_files": true}))
        }
        "check_fact" => {
            let p = valid_path.as_ref().ok_or_else(|| "A case path is required".to_string())?;
            let draft = check::draft_text_for_case(p)?;
            let report = check::run_fact_check(p, &draft)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "check_authority" => {
            let p = valid_path.as_ref().ok_or_else(|| "A case path is required".to_string())?;
            let draft = check::draft_text_for_case(p)?;
            let report = check::run_authority_check(p, &draft)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "check_deadline" => {
            let report = check::run_deadline(
                std::path::Path::new(&s("path")),
                &s("pack"),
                &s("rule"),
                &s("trigger_date"),
            )?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "check_all" => {
            let pack = s("pack");
            let rule = s("rule");
            let trigger = s("trigger_date");
            let report = check::run_all(
                std::path::Path::new(&s("path")),
                if pack.is_empty() { None } else { Some(pack.as_str()) },
                if rule.is_empty() { None } else { Some(rule.as_str()) },
                if trigger.is_empty() { None } else { Some(trigger.as_str()) },
            )?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "export_court" => {
            let path = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&path)?;
            let kind = if s("kind").is_empty() { "court_md".into() } else { s("kind") };
            let exported = documents::export_pleading(&path, &profile, &kind, None)?;
            Ok(json!({"docx": exported.docx, "pdf": exported.pdf, "never_files": true}))
        }
        "playbook" => Ok(json!({"playbook": crate::help::playbook()})),
        "case_seed" => {
            let (dir, profile) = crate::seed::seed_lucille(std::path::Path::new(&s("path")))?;
            Ok(json!({"folder": dir, "id": profile.id, "title": profile.title, "never_files": true}))
        }
        "draft_sections" => {
            let profile = case_profile::load_case(std::path::Path::new(&s("path")))?;
            let draft = crate::guided::load_or_init(std::path::Path::new(&s("path")), &profile.document_kind)?;
            Ok(serde_json::to_value(draft).unwrap())
        }
        "draft_fill" => {
            let draft = crate::guided::set_section(std::path::Path::new(&s("path")), &s("id"), &s("text"))?;
            Ok(serde_json::to_value(draft).unwrap())
        }
        "evidence_index" => {
            Ok(serde_json::to_value(crate::exhibits::rebuild(std::path::Path::new(&s("path")))?).unwrap())
        }
        "evidence_link" => {
            let fact_id = args.get("fact_id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            Ok(serde_json::to_value(crate::exhibits::link_fact(
                std::path::Path::new(&s("path")),
                fact_id,
                &s("letter"),
            )?)
            .unwrap())
        }
        "export_packet" => {
            Ok(serde_json::to_value(crate::packet::assemble_for_case(std::path::Path::new(&s("path")))?).unwrap())
        }
        "export_ics" => {
            let dir = std::path::PathBuf::from(s("path"));
            let result = check::run_deadline(&dir, &s("pack"), &s("rule"), &s("trigger_date"))?;
            let ics = crate::calendar::write_ics(&dir, &result)?;
            Ok(json!({"ics": ics, "due": result.due, "never_files": true}))
        }
        "packs_list" => {
            let catalog = if s("catalog").is_empty() {
                None
            } else {
                Some(std::path::PathBuf::from(s("catalog")))
            };
            Ok(serde_json::to_value(crate::forms::list_packs(catalog.as_deref(), None)?).unwrap())
        }
        "packs_validate" => {
            Ok(serde_json::to_value(RulePack::validate_file(std::path::Path::new(&s("file")))?).unwrap())
        }
        "packs_install" => {
            let file = if s("file").is_empty() {
                None
            } else {
                Some(std::path::PathBuf::from(s("file")))
            };
            let catalog = if s("catalog").is_empty() {
                None
            } else {
                Some(std::path::PathBuf::from(s("catalog")))
            };
            let situation = if s("situation").is_empty() {
                None
            } else {
                Some(s("situation"))
            };
            Ok(serde_json::to_value(crate::forms::install(crate::forms::InstallOpts {
                state: s("state"),
                situation,
                catalog,
                file,
                root: None,
                allow_network: false,
            })?)
            .unwrap())
        }
        "packs_remove" => Ok(json!({"removed": crate::forms::remove_pack(&s("state"), None)?})),
        "packs_verify" => {
            let state = if s("state").is_empty() {
                None
            } else {
                Some(s("state"))
            };
            Ok(serde_json::to_value(crate::forms::verify_pack(state.as_deref(), None)?).unwrap())
        }
        "guide_questions" => Ok(serde_json::to_value(crate::guide::questions()?).unwrap()),
        "guide_find" => {
            let court = if s("court").is_empty() {
                None
            } else {
                Some(s("court"))
            };
            let trigger = if s("trigger_date").is_empty() {
                None
            } else {
                Some(s("trigger_date"))
            };
            Ok(serde_json::to_value(crate::guide::find(&s("state"), court.as_deref(), &s("situation"), trigger.as_deref())?).unwrap())
        }
        "help_links" => Ok(serde_json::to_value(crate::help_links::HelpLinks::load()?).unwrap()),
        "gaps_add" => Ok(serde_json::to_value(crate::gaps::add(
            std::path::Path::new(&s("path")),
            &s("title"),
            &s("court"),
            &s("rule"),
            &s("happened"),
            &s("hard"),
        )?)
        .unwrap()),
        "check_preflight" => {
            let p_str = s("path");
            let report = check::preflight_audit(std::path::Path::new(&p_str))?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "check_transcripts" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let draft = profile.notice_of_appeal.user_text.as_str();
            let report = check::check_transcripts(dir, draft)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "export_exhibit_packet" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let p_prefix = s("prefix");
            let prefix = if p_prefix.is_empty() { None } else { Some(p_prefix.as_str()) };
            let start = args.get("start").and_then(|v| v.as_u64()).map(|n| n as u32);
            let report = crate::packet::assemble_exhibit_packet(dir, prefix, start)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "form_inspect" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let case = if s("path").is_empty() { None } else { Some(std::path::PathBuf::from(s("path"))) };
            let report = crate::forms::inspect_court_form(&pdf, case.as_deref())?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "form_fill" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let case = valid_path.as_ref().ok_or_else(|| "A case path is required".to_string())?;
            let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
            if dry_run {
                let profile = case_profile::load_case(case)?;
                return Ok(json!({
                    "dry_run": true,
                    "pdf_source": pdf.display().to_string(),
                    "case_path": case.display().to_string(),
                    "case_title": profile.title,
                    "never_files": true
                }));
            }
            let report = crate::forms::fill_court_form(&pdf, case, None, None)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "cloud_pull" => {
            let session = crate::sync::load_cloud_session().ok_or_else(|| "Not logged in to JustLegal cloud. Run cloud login first.".to_string())?;
            let root = if s("root").is_empty() { case_profile::default_cases_root() } else { std::path::PathBuf::from(s("root")) };
            let case_filter = if s("case").is_empty() { None } else { Some(s("case")) };
            let report = crate::sync::cloud_pull_all(&session, &root, case_filter.as_deref())?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "cloud_push" => {
            let session = crate::sync::load_cloud_session().ok_or_else(|| "Not logged in to JustLegal cloud. Run cloud login first.".to_string())?;
            let dir = std::path::PathBuf::from(s("path"));
            let report = crate::sync::cloud_push_case(&dir, &session)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "motion_list" => {
            let p_str = s("path");
            let profile = case_profile::load_case(std::path::Path::new(&p_str))?;
            Ok(serde_json::to_value(profile.filings).unwrap())
        }
        "motion_create" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let mut profile = case_profile::load_case(dir)?;
            let now = chrono::Local::now().to_rfc3339();
            let filing = case_profile::CaseFilingItem {
                id: uuid::Uuid::new_v4().to_string(),
                title: s("title"),
                kind: if s("kind").is_empty() { "motion".into() } else { s("kind") },
                rule: s("rule"),
                status: "draft".into(),
                user_text: s("text"),
                relief_sought: String::new(),
                created_at: now.clone(),
                updated_at: now,
            };
            profile.filings.push(filing.clone());
            case_profile::save_case(dir, &profile)?;
            Ok(serde_json::to_value(filing).unwrap())
        }
        "motion_export" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let id = s("id");
            let filing = profile.filings.iter().find(|f| f.id == id)
                .ok_or_else(|| format!("Filing ID '{id}' not found in case."))?;
            let paths = crate::documents::export_custom_filing(dir, &profile, &filing.title, &filing.user_text, None)?;
            Ok(serde_json::to_value(paths).unwrap())
        }
        "service_cert" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let recipients: Vec<crate::service::ServiceRecipient> = profile
                .caption
                .appellees()
                .into_iter()
                .map(|p| crate::service::ServiceRecipient {
                    name: p.name.clone(),
                    role: p.role.clone(),
                    address: p.address.clone(),
                    email: p.email.clone(),
                })
                .collect();
            let filing_t = if s("filing").is_empty() { "the accompanying papers".into() } else { s("filing") };
            let method = if s("method").is_empty() { "eCourts / Electronic Filing System".into() } else { s("method") };
            let params = crate::service::ServiceCertParams {
                filing_title: filing_t,
                service_date: s("date"),
                method,
                recipients,
                signature_name: profile.filer.signature_name.clone(),
            };
            let result = crate::service::generate_certificate_of_service(dir, &profile, &params)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "service_order" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let title = if s("title").is_empty() { "RELIEF REQUESTED".into() } else { s("title") };
            let result = crate::service::generate_proposed_order(dir, &profile, &title, &s("relief"))?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "pdf_merge" => {
            let files_arr = args.get("files").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let paths: Vec<std::path::PathBuf> = files_arr.iter().filter_map(|v| v.as_str()).map(std::path::PathBuf::from).collect();
            let out = std::path::PathBuf::from(s("out"));
            let result = crate::pdf_tools::merge_pdfs(&paths, &out)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "pdf_split" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let out = std::path::PathBuf::from(s("out"));
            let pages_str = s("pages");
            let mut page_list = Vec::new();
            for part in pages_str.split(&[',', ' '][..]) {
                let trimmed = part.trim();
                if trimmed.is_empty() { continue; }
                if let Some((s_str, e_str)) = trimmed.split_once('-') {
                    if let (Ok(start), Ok(end)) = (s_str.trim().parse::<u32>(), e_str.trim().parse::<u32>()) {
                        for p in start..=end { page_list.push(p); }
                    }
                } else if let Ok(num) = trimmed.parse::<u32>() {
                    page_list.push(num);
                }
            }
            let result = crate::pdf_tools::split_pdf(&pdf, &page_list, &out)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "pdf_rotate" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let out = std::path::PathBuf::from(s("out"));
            let degrees = args.get("degrees").and_then(|v| v.as_i64()).unwrap_or(90);
            let result = crate::pdf_tools::rotate_pdf(&pdf, degrees, None, &out)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "pdf_redact" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let out = std::path::PathBuf::from(s("out"));
            let words_arr = args.get("words").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let words: Vec<String> = words_arr.iter().filter_map(|v| v.as_str().map(String::from)).collect();
            let result = crate::pdf_tools::redact_pdf_text(&pdf, &words, &out)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "sign_pdf" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let name = s("name");
            let page = args.get("page").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let x = args.get("x").and_then(|v| v.as_f64()).unwrap_or(72.0) as f32;
            let y = args.get("y").and_then(|v| v.as_f64()).unwrap_or(100.0) as f32;
            let out_str = s("out");
            let out_p = if out_str.is_empty() {
                pdf.with_file_name(format!("signed-{}", pdf.file_name().unwrap().to_string_lossy()))
            } else {
                std::path::PathBuf::from(out_str)
            };
            let result = crate::signature::stamp_pdf(&pdf, &name, page, x, y, &out_p)?;
            Ok(serde_json::to_value(result).unwrap())
        }
        "case_context" => {
            let dir = std::path::PathBuf::from(s("path"));
            Ok(serde_json::to_value(crate::intake::case_context(&dir)?).unwrap())
        }
        "case_next_steps" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let report = crate::legal_os::predict_next_steps(&profile, Some(&dir));
            Ok(serde_json::to_value(report).unwrap())
        }
        "case_escalate" => {
            let dir = std::path::PathBuf::from(s("path"));
            let report = crate::portfolio::escalate_case(&dir, &s("target"), None)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "intake_status" => {
            let dir = std::path::PathBuf::from(s("path"));
            let report = crate::intake::intake_status(&dir)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "intake_answer" => {
            let dir = std::path::PathBuf::from(s("path"));
            let report = crate::intake::intake_answer(&dir, &s("question_id"), &s("answer"))?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "intake_scan" => {
            let dir = std::path::PathBuf::from(s("path"));
            Ok(serde_json::to_value(crate::intake::intake_scan(&dir)).unwrap())
        }
        "check_venue" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            Ok(serde_json::to_value(crate::legal_os::check_venue_competence(&profile)).unwrap())
        }
        "portfolio_status" => {
            let root = if s("root").is_empty() {
                case_profile::default_cases_root()
            } else {
                std::path::PathBuf::from(s("root"))
            };
            Ok(serde_json::to_value(crate::portfolio::portfolio_status(&root)?).unwrap())
        }
        "record_inventory" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let report = crate::record::inventory::scan_case_record(&dir, &profile)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "record_reconcile" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let entries = crate::record::reconciliation::parse_text_docket(&s("docket"));
            let report = crate::record::reconciliation::reconcile_docket(&dir, &profile, &entries)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "record_missing" => {
            let dir = std::path::PathBuf::from(s("path"));
            let inv = crate::record::inventory::load_record_inventory(&dir);
            Ok(serde_json::to_value(inv.map(|r| r.missing_records).unwrap_or_default()).unwrap())
        }
        "record_request_form" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let state = if s("state").is_empty() { None } else { Some(s("state")) };
            let packet = crate::record::requests::generate_records_request(&dir, &profile, state.as_deref())?;
            Ok(serde_json::to_value(packet).unwrap())
        }
        "accountability_audit" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let report = crate::accountability::audit::audit_record_discrepancies(&dir, &profile)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "accountability_routes" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let report = crate::accountability::audit::audit_record_discrepancies(&dir, &profile)?;
            let target_id = s("issue_id");
            if !target_id.is_empty() {
                if let Some(issue) = report.issues.into_iter().find(|i| i.id == target_id) {
                    return Ok(serde_json::to_value(issue.available_routes).unwrap());
                }
            }
            Ok(serde_json::to_value(crate::accountability::routes::determine_routes_for_issue(
                &profile,
                crate::accountability::types::RecordSupportStatus::UnsupportedInRecord,
                false,
            )).unwrap())
        }
        "accountability_resources" => {
            let state = if s("state").is_empty() { None } else { Some(s("state")) };
            let cat = if s("category").is_empty() { None } else { Some(s("category")) };
            let res = crate::accountability::resources::list_accountability_resources(state.as_deref(), cat.as_deref());
            Ok(serde_json::to_value(res).unwrap())
        }
        "accountability_packet" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let report = crate::accountability::audit::audit_record_discrepancies(&dir, &profile)?;
            let issue_id = s("issue_id");
            let issue = report.issues.into_iter().find(|i| i.id == issue_id)
                .ok_or_else(|| format!("Issue '{issue_id}' not found in case."))?;
            let packet = crate::accountability::packets::assemble_factual_packet(&dir, &profile, &issue)?;
            Ok(serde_json::to_value(packet).unwrap())
        }
        "ada_petition" => {
            let dir = std::path::PathBuf::from(s("path"));
            let profile = case_profile::load_case(&dir)?;
            let accomm_arr = args.get("accommodations").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let mut accommodations = Vec::new();
            for item in accomm_arr {
                if let Some(str_val) = item.as_str() {
                    let cat = match str_val {
                        "remote" | "remote_telephonic_appearance" => crate::ada::AccommodationCategory::RemoteTelephonicAppearance,
                        "mobility" | "physical_mobility_access" => crate::ada::AccommodationCategory::PhysicalMobilityAccess,
                        "listening" | "visual_hearing_assistance" => crate::ada::AccommodationCategory::VisualHearingAssistance,
                        "reading_time" | "extended_reading_time" => crate::ada::AccommodationCategory::ExtendedReadingTime,
                        "support_person" | "support_person_accompaniment" => crate::ada::AccommodationCategory::SupportPersonAccompaniment,
                        "magnification" | "document_magnification_accessible_pdf" => crate::ada::AccommodationCategory::DocumentMagnificationAccessiblePdf,
                        "interpreter" | "communication_interpreter" => crate::ada::AccommodationCategory::CommunicationInterpreter,
                        _ => continue,
                    };
                    accommodations.push(cat);
                }
            }
            if accommodations.is_empty() {
                accommodations.push(crate::ada::AccommodationCategory::RemoteTelephonicAppearance);
            }
            let payload = crate::ada::AdaRequestPayload {
                case_id: profile.id.clone(),
                requester_name: s("name"),
                requester_role: if s("role").is_empty() { "Self-Represented Litigant".into() } else { s("role") },
                contact_phone: s("phone"),
                contact_email: s("email"),
                accommodations,
                specific_needs_narrative: s("needs"),
                upcoming_date: if s("date").is_empty() { None } else { Some(s("date")) },
            };
            let report = crate::ada::generator::generate_ada_petition(&profile, &payload)?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "pdf_extract_text" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            Ok(serde_json::to_value(crate::pdf_tools::extract_pdf_text(&pdf)?).unwrap())
        }
        "agent_manifest" => Ok(crate::help::agent_manifest()),
        "import_folder" => {
            let dest = s("path");
            let from = s("from");
            let title = s("title");
            let report = crate::folder_import::import_folder(
                std::path::Path::new(&dest),
                std::path::Path::new(&from),
                if title.is_empty() { None } else { Some(title.as_str()) },
            )?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "order_decode" => {
            let pdf = std::path::PathBuf::from(s("pdf"));
            let p_str = s("path");
            if !p_str.is_empty() {
                let report = crate::order::decoder::ingest_order_into_case(std::path::Path::new(&p_str), &pdf)?;
                Ok(serde_json::to_value(report).unwrap())
            } else {
                let report = crate::order::decoder::decode_order_pdf(&pdf)?;
                Ok(serde_json::to_value(report).unwrap())
            }
        }
        "timeline_audit_mail" => {
            let p_str = s("path");
            let profile = case_profile::load_case(std::path::Path::new(&p_str))?;
            let state_str = s("state");
            let state = if state_str.is_empty() {
                if !profile.governing_state.is_empty() {
                    Some(profile.governing_state.as_str())
                } else {
                    Some("CO")
                }
            } else {
                Some(state_str.as_str())
            };
            let p_mark = s("postmark");
            let p_del = s("delivery");
            let p_clk = s("clerk_date");
            let report = crate::order::mail_audit::audit_mail_gap(
                &profile.id,
                &s("order_date"),
                if p_mark.is_empty() { None } else { Some(&p_mark) },
                if p_del.is_empty() { None } else { Some(&p_del) },
                if p_clk.is_empty() { None } else { Some(&p_clk) },
                state,
            );
            Ok(serde_json::to_value(report).unwrap())
        }
        "case_bootstrap" => {
            let from = std::path::PathBuf::from(s("from"));
            let to = std::path::PathBuf::from(s("to"));
            let title = s("title");
            let report = crate::order::bootstrap::bootstrap_case_from_folder(
                &from,
                &to,
                if title.is_empty() { None } else { Some(&title) },
            )?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "court_access_directory" => {
            let profiles = crate::court_access::get_all_court_profiles();
            Ok(serde_json::to_value(profiles).unwrap())
        }
        "court_access_profile" => {
            let profile = crate::court_access::find_court_access_profile(&s("query"));
            Ok(serde_json::to_value(profile).unwrap())
        }
        "record_retrieval_plan" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let plan = crate::court_access::plan_record_retrieval(dir, &profile);
            Ok(serde_json::to_value(plan).unwrap())
        }
        "record_retrieval_execute" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let mut profile = case_profile::load_case(dir)?;
            let state_str = s("state");
            let state = if state_str.is_empty() { None } else { Some(state_str.as_str()) };
            let packet = crate::court_access::execute_record_retrieval_request(dir, &mut profile, state)?;
            Ok(serde_json::to_value(packet).unwrap())
        }
        "workflow_plan" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let plan = crate::workflow::plan_workflow(dir, &profile, &s("objective"));
            crate::workflow::save_workflow_plan(dir, &plan)?;
            Ok(serde_json::to_value(plan).unwrap())
        }
        "workflow_status" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let plan = crate::workflow::load_workflow_plan(dir);
            Ok(serde_json::to_value(plan).unwrap())
        }
        "workflow_advance" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let mut profile = case_profile::load_case(dir)?;
            let mut plan = crate::workflow::load_workflow_plan(dir)
                .ok_or_else(|| "No active workflow plan found for this case. Formulate a plan first.".to_string())?;
            let approve = args.get("approve").and_then(|v| v.as_bool()).unwrap_or(false);
            let res = crate::workflow::advance_workflow_step(dir, &mut profile, &mut plan, approve)?;
            Ok(serde_json::to_value(res).unwrap())
        }
        "issues_stress_test" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let issue_id = s("issue_id");
            let results = if !issue_id.is_empty() {
                if let Some(target_issue) = profile.issues.iter().find(|i| i.id == issue_id) {
                    vec![crate::consultation::stress_test_issue(&profile, target_issue)]
                } else {
                    return Err(format!("Issue with id '{issue_id}' not found in case."));
                }
            } else {
                crate::consultation::stress_test_all_issues(&profile)
            };
            Ok(serde_json::to_value(results).unwrap())
        }
        "consultation_prepare" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let questions: Vec<String> = args.get("questions")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|s| s.as_str().map(|str| str.to_string())).collect())
                .unwrap_or_default();
            let dossier = crate::consultation::generate_consultation_dossier(dir, &profile, &questions)?;
            Ok(serde_json::to_value(dossier).unwrap())
        }
        "extension_evaluate" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let target = s("target");
            let days = args.get("days").and_then(|v| v.as_u64()).unwrap_or(30) as u32;
            let consent = args.get("consent").and_then(|v| v.as_bool()).unwrap_or(false);
            let prop = crate::consultation::evaluate_extension_request(&profile, &target, days, consent);
            Ok(serde_json::to_value(prop).unwrap())
        }
        "extension_suggest" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let assessment = crate::consultation::assess_baseline_extension(&profile, Some(dir));
            Ok(serde_json::to_value(assessment).unwrap())
        }
        "qui_tam_verify_entity" => {
            let query = s("query");
            let state = if s("state").is_empty() { None } else { Some(s("state")) };
            let report = crate::qui_tam::verify_entity(&query, state.as_deref())?;
            Ok(serde_json::to_value(report).unwrap())
        }
        "qui_tam_audit_9b" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let audit = crate::qui_tam::audit_case_rule9b(dir, &profile)?;
            Ok(serde_json::to_value(audit).unwrap())
        }
        "qui_tam_ingest_observation" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let file_str = s("file");
            let file = std::path::Path::new(&file_str);
            let notes = s("notes");
            let observer = if s("observer").is_empty() { None } else { Some(s("observer")) };
            let entry = crate::qui_tam::ingest_observation(dir, file, &notes, observer.as_deref())?;
            Ok(serde_json::to_value(entry).unwrap())
        }
        "qui_tam_records_pack" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let state = if s("state").is_empty() { None } else { Some(s("state")) };
            let pack = crate::qui_tam::generate_records_pack(dir, &profile, state.as_deref())?;
            Ok(serde_json::to_value(pack).unwrap())
        }
        "qui_tam_export_disclosure" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let category = if s("category").is_empty() { None } else { Some(s("category")) };
            let attestation = if s("attestation").is_empty() { None } else { Some(s("attestation")) };
            let pkg = crate::qui_tam::export_doj_disclosure(dir, &profile, category.as_deref(), attestation.as_deref())?;
            Ok(serde_json::to_value(pkg).unwrap())
        }
        "qui_tam_format_portal" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let agency = if s("agency").is_empty() { "HHS".into() } else { s("agency") };
            let packet = crate::qui_tam::export_doj_disclosure(dir, &profile, None, None)?;
            let block = crate::qui_tam::generate_portal_submission_block(&packet, &agency);
            Ok(serde_json::to_value(block).unwrap())
        }
        "qui_tam_format_inquiry" => {
            let p_str = s("path");
            let dir = std::path::Path::new(&p_str);
            let profile = case_profile::load_case(dir)?;
            let packet = crate::qui_tam::export_doj_disclosure(dir, &profile, None, None)?;
            let pkg = crate::qui_tam::generate_attorney_inquiry_package(&packet, &profile);
            Ok(serde_json::to_value(pkg).unwrap())
        }
        "case_profile" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let mut profile = case_profile::load_case(&dir)?;
            let mut modified = false;
            let tt = s("target_type");
            if !tt.is_empty() {
                profile.target_type = tt.to_lowercase();
                modified = true;
                if profile.target_type == "gov-agency" {
                    profile.fca_defendant_viable = false;
                    profile.fca_notes = "Under Vermont ANR v. Stevens, 529 U.S. 765 (2000), a state agency is not a 'person' under 31 U.S.C. § 3729(a) and cannot be sued in a qui tam action. Direct FCA action is barred as a matter of law. Route to contractors under 31 U.S.C. § 3729(b)(2)(A)(ii) or systemic oversight routes.".to_string();
                } else if profile.target_type == "gov-contractor" || profile.target_type == "private-provider" {
                    profile.fca_defendant_viable = true;
                    profile.fca_notes = "Target is a private contractor/provider subject to 31 U.S.C. § 3729(a) liability for presenting false claims or causing false records to be made under federal grant funds (31 U.S.C. § 3729(b)(2)(A)(ii)).".to_string();
                } else if profile.target_type == "official-individual" {
                    profile.fca_defendant_viable = false;
                    profile.fca_notes = "Official capacity claims against state officials for damages are barred by Eleventh Amendment / Will v. Michigan. Individual capacity claims under § 1983 require qualified immunity analysis.".to_string();
                }
            }
            let prog = s("program");
            if !prog.is_empty() {
                profile.target_program = prog;
                modified = true;
            }
            let aln = s("aln");
            if !aln.is_empty() {
                profile.target_aln = aln;
                modified = true;
            }
            let cid = s("contract_id");
            if !cid.is_empty() {
                if !profile.target_contract_ids.contains(&cid) {
                    profile.target_contract_ids.push(cid);
                }
                modified = true;
            }
            let cat = s("category");
            if !cat.is_empty() {
                profile.eo_category = cat;
                modified = true;
            }
            let tr = s("tier");
            if !tr.is_empty() {
                profile.matter_tier = tr;
                modified = true;
            }
            if modified {
                case_profile::save_case(&dir, &profile)?;
            }
            Ok(json!({
                "case_id": profile.id,
                "title": profile.title,
                "target_type": profile.target_type,
                "target_program": profile.target_program,
                "target_aln": profile.target_aln,
                "target_contract_ids": profile.target_contract_ids,
                "eo_category": profile.eo_category,
                "fca_defendant_viable": profile.fca_defendant_viable,
                "fca_notes": profile.fca_notes,
                "matter_tier": profile.matter_tier,
                "effective_tier": profile.effective_tier(),
            }))
        }
        "accountability_map" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let state = if s("state").is_empty() { None } else { Some(s("state")) };
            let program = if s("program").is_empty() { None } else { Some(s("program")) };
            let target_type = if s("target_type").is_empty() { None } else { Some(s("target_type")) };
            let role = if s("role").is_empty() { None } else { Some(s("role")) };
            let key_date = if s("key_date").is_empty() { None } else { Some(s("key_date")) };
            let rep = crate::accountability::generate_accountability_map(
                &dir,
                state.as_deref(),
                program.as_deref(),
                target_type.as_deref(),
                role.as_deref(),
                key_date.as_deref(),
            )?;
            Ok(serde_json::to_value(rep).unwrap())
        }
        "records_find" => {
            let jurisdiction = if s("jurisdiction").is_empty() { "CO".to_string() } else { s("jurisdiction") };
            let record_type = if s("type").is_empty() { "contract".to_string() } else { s("type") };
            let law = crate::records::finder::find_records_law(&jurisdiction, &record_type)?;
            Ok(serde_json::to_value(law).unwrap())
        }
        "records_draft" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let jurisdiction = if s("jurisdiction").is_empty() { "CO".to_string() } else { s("jurisdiction") };
            let record_type = if s("type").is_empty() { "contract".to_string() } else { s("type") };
            let fee_waiver = args.get("fee_waiver").and_then(|v| v.as_bool()).unwrap_or(false);
            let req_name = if s("requester").is_empty() { None } else { Some(s("requester")) };
            let draft = crate::records::drafter::draft_records_request(
                &dir,
                &jurisdiction,
                &record_type,
                None,
                fee_waiver,
                req_name.as_deref(),
            )?;
            Ok(serde_json::to_value(draft).unwrap())
        }
        "records_track" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let id = s("id");
            let law = if s("law").is_empty() { "CORA".to_string() } else { s("law") };
            let agency = s("agency");
            let sent = s("sent");
            let days = args.get("days").and_then(|v| v.as_u64()).unwrap_or(3) as u32;
            let ext = args.get("ext").and_then(|v| v.as_u64()).unwrap_or(7) as u32;
            let tracking = if s("tracking").is_empty() { None } else { Some(s("tracking")) };
            let payment = if s("payment").is_empty() { None } else { Some(s("payment")) };
            let cost = args.get("cost").and_then(|v| v.as_f64());
            let entry = crate::records::tracker::track_records_request(
                &dir,
                &id,
                &law,
                &agency,
                &sent,
                days,
                ext,
                tracking.as_deref(),
                payment.as_deref(),
                cost,
            )?;
            Ok(serde_json::to_value(entry).unwrap())
        }
        "records_followup" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let id = s("id");
            let law = if s("law").is_empty() { "CORA".to_string() } else { s("law") };
            let agency = s("agency");
            let kind = if s("kind").is_empty() { "late".to_string() } else { s("kind") };
            let text = crate::records::followup::draft_followup_letter(
                &dir,
                &id,
                &law,
                &agency,
                &kind,
            )?;
            Ok(json!({
                "request_id": id,
                "law": law,
                "agency": agency,
                "kind": kind,
                "content": text
            }))
        }
        "records_org" => {
            let org = crate::records::org::just_liberty_profile();
            Ok(serde_json::to_value(org).unwrap())
        }
        "export_receiver" => {
            let p_str = s("path");
            let dir = validate_mcp_path(&p_str, safe_mode)?;
            let to = s("to");
            let anon = if s("anonymity").is_empty() { None } else { Some(s("anonymity")) };
            let block = crate::receiver::format_for_receiver(
                &dir,
                &to,
                anon.as_deref(),
            )?;
            Ok(serde_json::to_value(block).unwrap())
        }
        "claim_translate" => {
            let vernacular = s("vernacular");
            let translation = crate::claim_translate::translate_vernacular(&vernacular);
            Ok(serde_json::to_value(translation).unwrap())
        }
        "sovereign_immunity_audit" => {
            let state = if s("state").is_empty() { "CO".to_string() } else { s("state") };
            let target = if s("target").is_empty() { "gov-agency".to_string() } else { s("target") };
            let claims: Vec<String> = args
                .get("claims")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            let federal_funds = args.get("federal_funds").and_then(|v| v.as_bool()).unwrap_or(true);
            let report = crate::sovereign_immunity::audit_sovereign_immunity(&state, &target, &claims, federal_funds);
            Ok(serde_json::to_value(report).unwrap())
        }
        "collective_action_build" => {
            let mode = if s("mode").is_empty() { "rule20-joinder".to_string() } else { s("mode") };
            let title = if s("title").is_empty() { "Systemic Reform Action".to_string() } else { s("title") };
            let defendant = if s("defendant").is_empty() { "Agency / Corporate Vendor".to_string() } else { s("defendant") };
            let p_str = s("path");
            let dir = if !p_str.is_empty() {
                Some(validate_mcp_path(&p_str, safe_mode)?)
            } else {
                None
            };
            let subject = if s("subject").is_empty() { None } else { Some(s("subject")) };
            let mut co_plaintiffs = Vec::new();
            if let Some(ref d) = dir {
                if let Ok(profile) = case_profile::load_case(d) {
                    for pl in &profile.caption.plaintiffs {
                        co_plaintiffs.push(crate::collective_action::CoPlaintiffEntry {
                            name: pl.name.clone(),
                            city_state: if pl.address.is_empty() { "Denver, CO".into() } else { pl.address.clone() },
                            injury_summary: "Deprived of timely administrative determination and held without due process hearing.".into(),
                            date_of_injury_or_hold: "2024-Present".into(),
                            evidence_file: None,
                            sha256_hash: None,
                        });
                    }
                }
            }
            if co_plaintiffs.is_empty() {
                co_plaintiffs.push(crate::collective_action::CoPlaintiffEntry {
                    name: "Lead Filer".into(),
                    city_state: "Denver, CO".into(),
                    injury_summary: "Subjected to automated hold without due process notice or hearing.".into(),
                    date_of_injury_or_hold: "2024-Present".into(),
                    evidence_file: None,
                    sha256_hash: None,
                });
                co_plaintiffs.push(crate::collective_action::CoPlaintiffEntry {
                    name: "Co-Plaintiff 2 (Active Hold)".into(),
                    city_state: "Aurora, CO".into(),
                    injury_summary: "Currently subjected to ongoing unresolved account freeze; cures mootness.".into(),
                    date_of_injury_or_hold: "Ongoing".into(),
                    evidence_file: None,
                    sha256_hash: None,
                });
            }
            let output = crate::collective_action::build_collective_action(
                dir.as_deref(),
                &mode,
                &title,
                &defendant,
                &co_plaintiffs,
                subject.as_deref(),
            )?;
            Ok(serde_json::to_value(output).unwrap())
        }
        other => Err(format!(
            "Unknown tool '{other}'. Call playbook or packs_list. Known tools are listed by tools/list."
        )),
    }
}

