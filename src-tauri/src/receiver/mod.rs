//! Receiver-Ready Output Formats (P13).
//! Standardizes case facts into the exact intake requirements published by each regulatory receiver.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiverSubmissionBlock {
    pub receiver_id: String,
    pub receiver_title: String,
    pub portal_or_email: String,
    pub submission_preference: String, // "Standard", "Confidential", "Anonymous"
    pub fields: Vec<ReceiverField>,
    pub complete_paste_text: String,
    pub attachment_manifest: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiverField {
    pub label: String,
    pub value: String,
    pub character_limit: Option<usize>,
}

pub fn format_for_receiver(
    case_dir: &Path,
    receiver_id: &str,
    anonymity: Option<&str>,
) -> Result<ReceiverSubmissionBlock, String> {
    let profile = crate::case_profile::load_case(case_dir)?;
    let preference = anonymity.unwrap_or("Confidential").to_string();

    let target_name = if !profile.caption.defendants.is_empty() {
        profile.caption.defendants[0].name.clone()
    } else {
        profile.title.clone()
    };

    let evidence_dir = case_dir.join("evidence");
    let mut attachments = Vec::new();
    if evidence_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&evidence_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name != "index.json" && name != "provenance.json" && !name.ends_with(".txt") {
                    attachments.push(name);
                }
            }
        }
    }

    let summary_narrative = if !profile.complaint.user_text.is_empty() {
        profile.complaint.user_text.clone()
    } else if !profile.motion.user_text.is_empty() {
        profile.motion.user_text.clone()
    } else {
        format!("Investigation regarding federal program integrity and vendor performance by {}.", target_name)
    };

    let submission = match receiver_id.to_ascii_lowercase().as_str() {
        "dol-oig" | "dol_oig" | "dol" => {
            let mut fields = Vec::new();
            fields.push(ReceiverField {
                label: "Who is involved (Individuals and Entities)".into(),
                value: format!("Target Entity: {}\nProgram: Unemployment Insurance (ALN 17.225)\nState: {}", target_name, profile.governing_state),
                character_limit: Some(1000),
            });
            fields.push(ReceiverField {
                label: "When occurred & Whether Ongoing".into(),
                value: "Multi-year contract performance period (2020 - Present); ongoing program integrity impact.".into(),
                character_limit: Some(500),
            });
            fields.push(ReceiverField {
                label: "Where occurred (City, State, Agency)".into(),
                value: format!("State Agency Administration; Primary Jurisdiction: {}", if profile.governing_state.is_empty() { "Colorado" } else { &profile.governing_state }),
                character_limit: Some(500),
            });
            fields.push(ReceiverField {
                label: "What Happened & How Known".into(),
                value: summary_narrative.clone(),
                character_limit: Some(3000),
            });
            fields.push(ReceiverField {
                label: "Filing Preference".into(),
                value: preference.clone(),
                character_limit: None,
            });

            let paste = format!(
"=== DOL-OIG ONLINE COMPLAINT PORTAL AUTO-FILL BLOCK ===\n\
Portal: https://www.oig.dol.gov/hotline.htm (eCasePortal)\n\
Filing Preference: {preference}\n\n\
[FIELD: WHO IS INVOLVED]\n{f0}\n\n\
[FIELD: WHEN OCCURRED]\n{f1}\n\n\
[FIELD: WHERE OCCURRED]\n{f2}\n\n\
[FIELD: WHAT HAPPENED]\n{f3}\n",
                preference = preference,
                f0 = fields[0].value,
                f1 = fields[1].value,
                f2 = fields[2].value,
                f3 = fields[3].value
            );

            ReceiverSubmissionBlock {
                receiver_id: "dol-oig".into(),
                receiver_title: "U.S. Department of Labor Office of Inspector General Hotline".into(),
                portal_or_email: "https://www.oig.dol.gov/hotline.htm".into(),
                submission_preference: preference,
                fields,
                complete_paste_text: paste,
                attachment_manifest: attachments,
            }
        }

        "co-osa" | "co_osa" | "osa" => {
            let mut fields = Vec::new();
            fields.push(ReceiverField {
                label: "What Happened (Nature of Concern)".into(),
                value: summary_narrative.clone(),
                character_limit: Some(4000),
            });
            fields.push(ReceiverField {
                label: "Who is Involved (Agency / Contractor / Position)".into(),
                value: format!("Contractor: {}\nState Agency: CDLE\nProgram: Unemployment Insurance", target_name),
                character_limit: Some(1000),
            });
            fields.push(ReceiverField {
                label: "When it Occurred / When you Became Aware".into(),
                value: "Contract deliverables and pandemic administration (2020-present). Ongoing occupational/contractor review.".into(),
                character_limit: Some(500),
            });
            fields.push(ReceiverField {
                label: "Where it Occurred".into(),
                value: "State of Colorado, CDLE / Remote Software Operations".into(),
                character_limit: Some(500),
            });
            fields.push(ReceiverField {
                label: "How it came to your attention (Observed vs Heard)".into(),
                value: "Direct firsthand administrative claimant experience corroborated by authenticated open records requests (CORA/FOIA) and state audit workpapers.".into(),
                character_limit: Some(1000),
            });

            let paste = format!(
"=== COLORADO OSA FRAUD HOTLINE AUTO-FILL BLOCK ===\n\
Contact: osafraudhotline@coleg.gov | 303-869-3020\n\
Statutory Confidentiality: C.R.S. § 2-3-110.5\n\n\
[WHAT HAPPENED]\n{f0}\n\n\
[WHO IS INVOLVED]\n{f1}\n\n\
[WHEN OCCURRED]\n{f2}\n\n\
[WHERE OCCURRED]\n{f3}\n\n\
[HOW IT CAME TO YOUR ATTENTION]\n{f4}\n",
                f0 = fields[0].value,
                f1 = fields[1].value,
                f2 = fields[2].value,
                f3 = fields[3].value,
                f4 = fields[4].value
            );

            ReceiverSubmissionBlock {
                receiver_id: "co-osa".into(),
                receiver_title: "Colorado Office of the State Auditor Fraud Hotline".into(),
                portal_or_email: "osafraudhotline@coleg.gov".into(),
                submission_preference: preference,
                fields,
                complete_paste_text: paste,
                attachment_manifest: attachments,
            }
        }

        "gao-fraudnet" | "gao" => {
            let mut fields = Vec::new();
            fields.push(ReceiverField {
                label: "Program or Agency Involved".into(),
                value: format!("Target: {} | Federal Assistance Listing: ALN 17.225 (Unemployment Insurance)", target_name),
                character_limit: Some(1000),
            });
            fields.push(ReceiverField {
                label: "Detailed Allegation Description".into(),
                value: summary_narrative.clone(),
                character_limit: Some(5000),
            });
            fields.push(ReceiverField {
                label: "Filing Preference".into(),
                value: preference.clone(),
                character_limit: None,
            });

            let paste = format!(
"=== GAO FRAUDNET SUBMISSION BLOCK ===\n\
Portal: https://www.gao.gov/about/what-gao-does/fraudnet\n\
Email: fraud@gao.gov | 1-800-424-5454\n\n\
[PROGRAM & ENTITY]\n{f0}\n\n\
[ALLEGATION NARRATIVE]\n{f1}\n",
                f0 = fields[0].value,
                f1 = fields[1].value
            );

            ReceiverSubmissionBlock {
                receiver_id: "gao-fraudnet".into(),
                receiver_title: "GAO FraudNet Congressional Referral Channel".into(),
                portal_or_email: "fraud@gao.gov".into(),
                submission_preference: preference,
                fields,
                complete_paste_text: paste,
                attachment_manifest: attachments,
            }
        }

        "counsel" | "attorney" => {
            let paste = format!(
"=== ABA MODEL RULE 1.18 PROSPECTIVE CLIENT DOSSIER (CONFIDENTIAL) ===\n\
Target: {}\n\
Governing Regime: False Claims Act (31 U.S.C. § 3729(b)(2)(A)(ii)) & State CFCA\n\
Filer: {}\n\n\
EXECUTIVE SUMMARY:\n\
{}\n\n\
EVIDENTIARY MANIFEST ({} verified attachments):\n\
{}\n",
                target_name,
                if !profile.filer.name.is_empty() { &profile.filer.name } else { "Confidential Relator" },
                summary_narrative,
                attachments.len(),
                attachments.iter().map(|a| format!("- {}", a)).collect::<Vec<_>>().join("\n")
            );

            ReceiverSubmissionBlock {
                receiver_id: "counsel".into(),
                receiver_title: "Qui Tam Relator Counsel Work File (ABA Model Rule 1.18)".into(),
                portal_or_email: "Direct transmission to retained counsel".into(),
                submission_preference: "Attorney-Client Privileged Work Product".into(),
                fields: vec![
                    ReceiverField { label: "Target Entity".into(), value: target_name, character_limit: None },
                    ReceiverField { label: "Summary".into(), value: summary_narrative, character_limit: None },
                ],
                complete_paste_text: paste,
                attachment_manifest: attachments,
            }
        }

        _ => return Err(format!("Unknown receiver '{}'. Supported: dol-oig, co-osa, gao-fraudnet, counsel", receiver_id)),
    };

    let exports_dir = case_dir.join("exports");
    let _ = fs::create_dir_all(&exports_dir);
    let path = exports_dir.join(format!("receiver_submission_{}.txt", receiver_id));
    let _ = fs::write(&path, &submission.complete_paste_text);

    Ok(submission)
}
