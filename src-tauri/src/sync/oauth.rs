use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use reqwest::blocking::Client;
use serde_json::{json, Value};

use super::cloud::urlencoding;
use super::local::save_cloud_session;
use super::types::{firebase_api_key, firebase_project_id, CloudSession};

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Verify a Firebase ID token against IdentityToolkit `accounts:lookup` and return `(uid, email)`.
pub fn lookup_firebase_token(id_token: &str) -> Result<(String, String), String> {
    let api_key = firebase_api_key();
    if api_key.trim().is_empty() {
        return Err("Firebase API key is not configured.".to_string());
    }
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
        api_key
    );
    let resp = client
        .post(&url)
        .json(&json!({ "idToken": id_token.trim() }))
        .send()
        .map_err(|e| format!("Token verification request failed: {e}"))?;
    let status = resp.status();
    let body: Value = resp
        .json()
        .map_err(|e| format!("Invalid token lookup response: {e}"))?;
    if !status.is_success() {
        let msg = body
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .unwrap_or("INVALID_ID_TOKEN");
        return Err(format!("Firebase rejected the token ({msg})."));
    }
    let user = body
        .pointer("/users/0")
        .ok_or_else(|| "No Firebase user found for this token.".to_string())?;
    let uid = user
        .get("localId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing localId in Firebase user profile.".to_string())?
        .to_string();
    let email = user
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("authenticated-user@justlegal.me")
        .to_string();
    Ok((uid, email))
}

fn decode_query_param(query: &str, key: &str) -> String {
    let prefix = format!("{key}=");
    for part in query.split('&') {
        if let Some(raw) = part.strip_prefix(&prefix) {
            return percent_decode(raw);
        }
    }
    String::new()
}

fn percent_decode(input: &str) -> String {
    let mut bytes = Vec::with_capacity(input.len());
    let raw = input.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if raw[i] == b'%' && i + 2 < raw.len() {
            if let Ok(hex) = std::str::from_utf8(&raw[i + 1..i + 3]) {
                if let Ok(val) = u8::from_str_radix(hex, 16) {
                    bytes.push(val);
                    i += 3;
                    continue;
                }
            }
            bytes.push(raw[i]);
            i += 1;
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn open_system_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .spawn()
            .map_err(|e| format!("Could not open browser: {e}"))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Could not open browser: {e}"))?;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Could not open browser: {e}"))?;
    }
    Ok(())
}

/// Launch a loopback browser window on `http://localhost:{port}/auth` to sign in with Google via Firebase Auth.
pub fn cloud_sign_in_google_browser() -> Result<CloudSession, String> {
    let api_key = firebase_api_key();
    let project_id = firebase_project_id();
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Could not start local OAuth listener: {e}"))?;
    listener
        .set_nonblocking(true)
        .map_err(|e| format!("Could not configure local OAuth listener: {e}"))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("Could not read local listener port: {e}"))?
        .port();

    let auth_url = format!("http://localhost:{port}/auth");
    open_system_browser(&auth_url)?;

    let auth_html = build_google_bridge_html(&api_key, &project_id, port);
    let deadline = Instant::now() + Duration::from_secs(180);

    while Instant::now() < deadline {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                let mut buf = [0u8; 16384];
                let n = stream.read(&mut buf).unwrap_or(0);
                if n == 0 {
                    continue;
                }
                let req = String::from_utf8_lossy(&buf[..n]);
                let first_line = req.lines().next().unwrap_or("");

                if first_line.starts_with("GET /auth") || first_line.starts_with("GET / ") || first_line.starts_with("GET /?") {
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        auth_html.len(),
                        auth_html
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    let _ = stream.flush();
                } else if first_line.starts_with("GET /callback?") {
                    let path_part = first_line.split_whitespace().nth(1).unwrap_or("");
                    let query = path_part.strip_prefix("/callback?").unwrap_or("");
                    let err_param = decode_query_param(query, "error");
                    if !err_param.is_empty() {
                        let html = format!(
                            "<html><body style='font-family:system-ui;padding:40px;text-align:center'><h2>Google Sign-In Cancelled</h2><p>{}</p><p>You can close this tab and return to JustLegal OS.</p></body></html>",
                            urlencoding::encode(&err_param)
                        );
                        let resp = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            html.len(),
                            html
                        );
                        let _ = stream.write_all(resp.as_bytes());
                        return Err(format!("Google sign-in failed: {err_param}"));
                    }

                    let id_token = decode_query_param(query, "idToken");
                    let refresh_token = decode_query_param(query, "refreshToken");
                    if id_token.trim().is_empty() {
                        return Err("Google sign-in callback did not include an idToken.".to_string());
                    }

                    let (uid, email) = lookup_firebase_token(&id_token)?;
                    let session = CloudSession {
                        id_token,
                        refresh_token,
                        uid,
                        email: email.clone(),
                        expires_at: now_secs() + 3600,
                    };
                    save_cloud_session(&session)?;

                    let done_html = format!(
                        "<!doctype html><html><head><meta charset='utf-8'><title>JustLegal OS — Signed In</title></head><body style='font-family:system-ui,-apple-system,sans-serif;background:#f6f8fb;color:#142033;display:grid;place-items:center;min-height:90vh;margin:0'><div style='background:#fff;border:1px solid #dde3ec;border-radius:12px;padding:32px;max-width:440px;text-align:center;box-shadow:0 4px 16px rgba(15,23,42,.06)'><h2 style='margin:0 0 10px;color:#1d4ed8'>Connected to JustLegal OS</h2><p style='margin:0 0 14px;font-size:14px;color:#475569'>Signed in as <strong>{}</strong>.</p><p style='margin:0;font-size:13px;color:#64748b'>You can close this browser tab and return to JustLegal OS.</p><script>setTimeout(()=>window.close(),1500);</script></div></body></html>",
                        email
                    );
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        done_html.len(),
                        done_html
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    let _ = stream.flush();
                    return Ok(session);
                } else {
                    let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    let _ = stream.write_all(resp.as_bytes());
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(format!("Local OAuth listener error: {e}"));
            }
        }
    }

    Err("Google sign-in timed out after 3 minutes. Please try again.".to_string())
}

fn build_google_bridge_html(api_key: &str, project_id: &str, port: u16) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Sign in to JustLegal OS</title>
  <script src="https://www.gstatic.com/firebasejs/10.14.1/firebase-app-compat.js"></script>
  <script src="https://www.gstatic.com/firebasejs/10.14.1/firebase-auth-compat.js"></script>
</head>
<body style="font-family:system-ui,-apple-system,sans-serif;background:#f6f8fb;color:#142033;display:grid;place-items:center;min-height:92vh;margin:0">
  <div style="background:#fff;border:1px solid #dde3ec;border-radius:14px;padding:34px;max-width:440px;width:90%;text-align:center;box-shadow:0 8px 24px rgba(15,23,42,.06)">
    <div style="font-size:12px;font-weight:700;letter-spacing:.08em;text-transform:uppercase;color:#1d4ed8;margin-bottom:8px">JustLegal OS · Local Desktop Bridge</div>
    <h1 style="font-size:22px;margin:0 0 10px">Sign in with Google</h1>
    <p id="status" style="font-size:14px;color:#5b6678;line-height:1.6;margin:0 0 22px">Connect your JustLegal.me account to sync standard matters with JustLegal OS on this computer.</p>
    <button id="btn" onclick="startGoogle()" style="width:100%;padding:12px 18px;border-radius:10px;border:1px solid #cbd5e1;background:#1d4ed8;color:#fff;font-weight:600;font-size:14px;cursor:pointer">Continue with Google</button>
    <p style="font-size:11px;color:#64748b;margin-top:18px">Protected fraud / Qui Tam matters remain strictly local and never sync to the cloud.</p>
  </div>
  <script>
    firebase.initializeApp({{
      apiKey: "{api_key}",
      authDomain: "{project_id}.firebaseapp.com",
      projectId: "{project_id}"
    }});
    const auth = firebase.auth();
    const provider = new firebase.auth.GoogleAuthProvider();
    provider.setCustomParameters({{ prompt: "select_account" }});

    async function finish(user) {{
      if (!user) return;
      document.getElementById("status").textContent = "Verifying session with JustLegal OS...";
      document.getElementById("btn").disabled = true;
      const idToken = await user.getIdToken(true);
      const refreshToken = user.refreshToken || "";
      const q = new URLSearchParams({{ idToken, refreshToken }});
      window.location.href = "http://localhost:{port}/callback?" + q.toString();
    }}

    auth.getRedirectResult().then((res) => {{
      if (res && res.user) finish(res.user);
    }}).catch((err) => {{
      document.getElementById("status").textContent = err.message || String(err);
    }});

    async function startGoogle() {{
      const statusEl = document.getElementById("status");
      statusEl.textContent = "Opening Google sign-in...";
      try {{
        const res = await auth.signInWithPopup(provider);
        if (res && res.user) await finish(res.user);
      }} catch (err) {{
        if (err && (err.code === "auth/popup-blocked" || err.code === "auth/popup-closed-by-user")) {{
          statusEl.textContent = "Redirecting to Google sign-in...";
          await auth.signInWithRedirect(provider);
        }} else {{
          statusEl.textContent = (err && err.message) ? err.message : String(err);
        }}
      }}
    }}
  </script>
</body>
</html>"#
    )
}
