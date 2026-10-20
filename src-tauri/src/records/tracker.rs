use chrono::{Datelike, Duration, NaiveDate, Utc};
use std::fs;
use std::path::Path;

use super::types::{RecordsTrackerManifest, TrackedRequestEntry};

/// Computes a working-day due date skipping Saturdays, Sundays, and major holidays.
pub fn add_working_days(start_date: NaiveDate, days: u32) -> NaiveDate {
    let mut current = start_date;
    let mut added = 0;
    while added < days {
        current += Duration::days(1);
        let weekday = current.weekday();
        // Skip Saturday and Sunday
        if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun && !is_federal_holiday(current) {
            added += 1;
        }
    }
    current
}

fn is_federal_holiday(date: NaiveDate) -> bool {
    let m = date.month();
    let d = date.day();
    // New Year's Day (Jan 1)
    if m == 1 && d == 1 { return true; }
    // Independence Day (Jul 4)
    if m == 7 && d == 4 { return true; }
    // Veterans Day (Nov 11)
    if m == 11 && d == 11 { return true; }
    // Christmas (Dec 25)
    if m == 12 && d == 25 { return true; }
    false
}

pub fn track_records_request(
    case_dir: &Path,
    req_id: &str,
    law_name: &str,
    agency: &str,
    sent_date_str: &str,
    working_days: u32,
    ext_working_days: u32,
    tracking_number: Option<&str>,
    payment_ref: Option<&str>,
    cost_dollars: Option<f64>,
) -> Result<TrackedRequestEntry, String> {
    let sent_date = NaiveDate::parse_from_str(sent_date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format '{}': {}", sent_date_str, e))?;

    let due_date = add_working_days(sent_date, working_days);
    let ext_date = if ext_working_days > 0 {
        Some(add_working_days(due_date, ext_working_days).format("%Y-%m-%d").to_string())
    } else {
        None
    };

    let entry = TrackedRequestEntry {
        id: req_id.to_string(),
        law_name: law_name.to_string(),
        agency: agency.to_string(),
        sent_date: sent_date.format("%Y-%m-%d").to_string(),
        due_date: due_date.format("%Y-%m-%d").to_string(),
        extension_date: ext_date,
        tracking_number: tracking_number.map(|s| s.to_string()),
        payment_reference: payment_ref.map(|s| s.to_string()),
        cost_dollars,
        status: "Sent (Pending Statutory Response)".to_string(),
        response_text_summary: None,
        next_step: format!("Await response by statutory deadline {}. If agency invokes extension, track until {}.", due_date.format("%Y-%m-%d"), due_date.format("%Y-%m-%d")),
    };

    let records_dir = case_dir.join("records");
    let _ = fs::create_dir_all(&records_dir);
    let manifest_path = records_dir.join("tracker.json");

    let mut manifest = if manifest_path.exists() {
        fs::read_to_string(&manifest_path)
            .ok()
            .and_then(|s| serde_json::from_str::<RecordsTrackerManifest>(&s).ok())
            .unwrap_or_else(|| RecordsTrackerManifest {
                case_id: case_dir.file_name().and_then(|n| n.to_str()).unwrap_or("case").to_string(),
                requests: Vec::new(),
                last_updated: Utc::now().to_rfc3339(),
            })
    } else {
        RecordsTrackerManifest {
            case_id: case_dir.file_name().and_then(|n| n.to_str()).unwrap_or("case").to_string(),
            requests: Vec::new(),
            last_updated: Utc::now().to_rfc3339(),
        }
    };

    // Update existing or add new
    if let Some(pos) = manifest.requests.iter().position(|r| r.id == req_id) {
        manifest.requests[pos] = entry.clone();
    } else {
        manifest.requests.push(entry.clone());
    }
    manifest.last_updated = Utc::now().to_rfc3339();

    if let Ok(serialized) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write(&manifest_path, serialized);
    }

    Ok(entry)
}
