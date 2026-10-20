//! Case Bootstrap and Rescue Engine.
//!
//! Reconstructs a structured case workspace from an unorganized directory
//! of PDFs, Word documents, or text notes without moving or altering original files.

use std::fs;
use std::path::Path;
use uuid::Uuid;

use super::types::BootstrapResult;
use crate::case_profile::{create_case_folder, load_case, save_case, CaseMode, CaseProfile};
use crate::record::inventory::scan_case_record;

/// Bootstraps an unorganized folder of legal documents into a full JustLegal OS case.
pub fn bootstrap_case_from_folder(
    source_folder: &Path,
    destination_parent: &Path,
    case_title: Option<&str>,
) -> Result<BootstrapResult, String> {
    if !source_folder.is_dir() {
        return Err(format!("Source folder does not exist or is not a directory: {}", source_folder.display()));
    }

    let resolved_title = case_title.unwrap_or_else(|| {
        source_folder
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Rescued Case Matter")
    });

    let case_id = format!("case-{}", Uuid::new_v4().simple());
    let target_dir = destination_parent.join(&case_id);

    // Initialize clean layout
    let (_created_path, profile) = create_case_folder(&target_dir, resolved_title, CaseMode::Standard, "motion")?;

    let mut files_scanned = 0usize;
    let mut orders_imported = 0usize;
    let mut filings_imported = 0usize;
    let mut evidence_imported = 0usize;
    let mut drafts_imported = 0usize;
    let mut detected_docket: Option<String> = None;
    let mut detected_court: Option<String> = None;

    // Scan files recursively
    for entry in walkdir::WalkDir::new(source_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        files_scanned += 1;
        let file_path = entry.path();
        let fname = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
        let lower = fname.to_ascii_lowercase();

        let is_party_filing = lower.contains("motion")
            || lower.contains("brief")
            || lower.contains("complaint")
            || lower.contains("petition")
            || lower.contains("application")
            || lower.contains("response")
            || lower.contains("reply")
            || lower.contains("notice")
            || lower.contains("appeal");

        let subfolder = if is_party_filing {
            filings_imported += 1;
            "filings"
        } else if lower.contains("order")
            || lower.contains("judgment")
            || lower.contains("ruling")
            || lower.contains("dismiss")
            || lower.contains("decree")
            || lower.contains("decision")
        {
            orders_imported += 1;
            "orders"
        } else if lower.contains("transcript") || lower.contains("exhibit") || lower.contains("audio") || lower.contains("recording") || lower.contains("photo") {
            evidence_imported += 1;
            "evidence"
        } else {
            drafts_imported += 1;
            "drafts"
        };

        let dest_file = target_dir.join(subfolder).join(fname);
        // Copy original file (never move or delete)
        let _ = fs::copy(file_path, &dest_file);

        // Accurate heuristic detection of docket or court in filenames
        if detected_docket.is_none() {
            // 1. Check federal format: e.g. 1:24-cv-03390, 24-cv-03390
            let fed_regex = regex::Regex::new(r"(\b\d{1,2}:\d{2}-[a-zA-Z]{2,4}-\d{3,6}\b|\b\d{2}-[a-zA-Z]{2,4}-\d{3,6}\b)").unwrap();
            let nj_regex = regex::Regex::new(r"(\bA-\d{4,6}-\d{2}\b)").unwrap();
            let state_alphanumeric = regex::Regex::new(r"(\b\d{2,4}[A-Z]{2,3}\d{3,6}\b)").unwrap();
            let standard_docket = regex::Regex::new(r"(\b\d{2}-\d{4,5}\b)").unwrap();
            let is_date = regex::Regex::new(r"(\d{2,4}-\d{2}-\d{2,4})").unwrap().is_match(fname);

            if let Some(cap) = fed_regex.captures(fname) {
                detected_docket = Some(cap[1].to_string());
            } else if let Some(cap) = nj_regex.captures(fname) {
                detected_docket = Some(cap[1].to_string());
            } else if let Some(cap) = state_alphanumeric.captures(fname) {
                detected_docket = Some(cap[1].to_string());
            } else if !is_date {
                if let Some(cap) = standard_docket.captures(fname) {
                    detected_docket = Some(cap[1].to_string());
                }
            }
        }
        if detected_court.is_none() {
            if lower.contains("d. colo") || lower.contains("district of colorado") || lower.contains("d.c. colo") {
                detected_court = Some("United States District Court for the District of Colorado".into());
            } else if lower.contains("10th cir") || lower.contains("tenth circuit") {
                detected_court = Some("United States Court of Appeals for the Tenth Circuit".into());
            } else if lower.contains("d.n.j") || lower.contains("district of new jersey") {
                detected_court = Some("United States District Court for the District of New Jersey".into());
            } else if lower.contains("3rd cir") || lower.contains("third circuit") {
                detected_court = Some("United States Court of Appeals for the Third Circuit".into());
            } else if lower.contains("colorado court of appeals") || lower.contains("co court of appeals") {
                detected_court = Some("Colorado Court of Appeals".into());
            } else if lower.contains("colorado supreme") {
                detected_court = Some("Colorado Supreme Court".into());
            } else if lower.contains("appellate division") || lower.contains("nj-app-div") {
                detected_court = Some("New Jersey Superior Court, Appellate Division".into());
            } else if lower.contains("colorado") || lower.contains("co-") {
                detected_court = Some("Colorado Court of Appeals".into());
            } else if lower.contains("new jersey") || lower.contains("nj-") {
                detected_court = Some("New Jersey Superior Court, Appellate Division".into());
            }
        }
    }

    // Update profile with detected attributes
    let mut updated_profile = load_case(&target_dir)?;
    if let Some(ref d) = detected_docket {
        updated_profile.docket_number = d.clone();
    }
    if let Some(ref c) = detected_court {
        updated_profile.court.name = c.clone();
    }
    save_case(&target_dir, &updated_profile)?;

    // Scan initial inventory
    let _ = scan_case_record(&target_dir, &updated_profile);

    let mut pending_obligations = Vec::new();
    if orders_imported > 0 {
        pending_obligations.push(format!("{orders_imported} court order(s) imported: decode orders to verify candidate deadlines."));
    }
    if filings_imported > 0 {
        pending_obligations.push(format!("{filings_imported} filings imported: reconcile against official court docket sheet."));
    }

    Ok(BootstrapResult {
        case_id: profile.id,
        case_path: target_dir.to_string_lossy().to_string(),
        title: resolved_title.to_string(),
        files_scanned,
        orders_imported,
        filings_imported,
        evidence_imported,
        drafts_imported,
        detected_docket_number: detected_docket,
        detected_court,
        pending_obligations,
        summary: format!(
            "Successfully bootstrapped case with {} files ({} orders, {} filings, {} exhibits, {} drafts).",
            files_scanned, orders_imported, filings_imported, evidence_imported, drafts_imported
        ),
    })
}
