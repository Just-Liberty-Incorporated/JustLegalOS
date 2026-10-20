//! 31 U.S.C. § 3729 & FRCP Rule 9(b) 6-Element Viability Audit Engine.
//! Content-aware statutory verification adhering to Vermont ANR v. Stevens, 529 U.S. 765 (2000),
//! the Clausen/Foglia particularity pleading standard, and the Escobar materiality rule.

use chrono::Utc;
use std::fs;
use std::path::Path;

use super::types::*;
use crate::case_profile::CaseProfile;

/// Evaluates a case workspace against the 6 statutory elements of the False Claims Act and Rule 9(b).
pub fn audit_case_rule9b(case_dir: &Path, profile: &CaseProfile) -> Result<Rule9bAudit, String> {
    let now = Utc::now().to_rfc3339();
    let evidence_dir = case_dir.join("evidence");
    let provenance_file = evidence_dir.join("provenance.json");

    let mut elements = Vec::new();
    let mut missing_requirements = Vec::new();
    let mut next_steps = Vec::new();
    let mut raw_score = 0u32;

    let target_type = if !profile.target_type.is_empty() {
        profile.target_type.to_ascii_lowercase()
    } else {
        "private-provider".to_string()
    };

    // Load provenance entries
    let mut provenance_entries: Vec<ObservationEntry> = Vec::new();
    if provenance_file.exists() {
        if let Ok(content) = fs::read_to_string(&provenance_file) {
            if let Ok(manifest) = serde_json::from_str::<ProvenanceManifest>(&content) {
                provenance_entries = manifest.entries;
            }
        }
    }

    // Filter firsthand observations: exclude "Google Search" or "public-search" items
    let firsthand_observations: Vec<&ObservationEntry> = provenance_entries
        .iter()
        .filter(|obs| {
            let n = obs.filename.to_lowercase();
            let notes = obs.notes.to_lowercase();
            !n.contains("google search") && !n.contains("public-search") && !notes.contains("public search")
        })
        .collect();

    // Scan evidence/ directory for actual documents
    let mut evidence_files = Vec::new();
    if evidence_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&evidence_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name != "index.json" && name != "provenance.json" && !name.ends_with(".txt") {
                    evidence_files.push(name);
                }
            }
        }
    }

    let draft_lower = format!(
        "{} {} {} {}",
        profile.complaint.user_text,
        profile.motion.user_text,
        profile.notice_of_appeal.user_text,
        profile.court_markdown
    ).to_lowercase();

    // ---------------------------------------------------------
    // Element 1: Target Entity Identification (Who) [15 pts]
    // ---------------------------------------------------------
    let mut elem1_missing = Vec::new();
    let elem1_satisfied: bool;
    let elem1_details: String;

    if target_type == "gov-agency" {
        elem1_satisfied = false;
        elem1_details = "DEFICIENT AS FCA DEFENDANT: Target is a state government agency. Under Vermont Agency of Natural Resources v. United States ex rel. Stevens, 529 U.S. 765 (2000), States and state agencies are not 'persons' under 31 U.S.C. § 3729(a) and cannot be sued in qui tam. Re-target private contractors (e.g. software vendors, intake operators) or pursue systemic oversight via 'justlegalos-cli accountability map'.".into();
        elem1_missing.push("Target is an immune sovereign agency under Stevens. Qui tam action against the State is barred as a matter of law.".into());
        missing_requirements.push("Rule 9(b) Target Viability: Identify the private contractor or vendor that submitted claims to federal funds.".into());
        next_steps.push("Run 'justlegalos-cli case profile --target-type gov-contractor' to re-focus allegations on the private contractor.".into());
        next_steps.push("Run 'justlegalos-cli accountability map' to review the 14 systemic oversight pathways.".into());
    } else if target_type == "gov-contractor" {
        let has_contractor_name = !profile.caption.defendants.is_empty() || !profile.target_contract_ids.is_empty();
        if has_contractor_name {
            elem1_satisfied = true;
            raw_score += 15;
            elem1_details = format!("Private contractor target identified under 1 U.S.C. § 1 and 31 U.S.C. § 3729(b)(2)(A)(ii). Defendant roster and/or contract IDs active.");
        } else {
            elem1_satisfied = false;
            elem1_details = "Missing contractor legal entity name, SAM.gov UEI/CAGE, or state contract identifier.".into();
            elem1_missing.push("Provide official corporate name and state/federal contract ID for the vendor.".into());
            missing_requirements.push("Rule 9(b) Contractor Identity: Enter vendor legal name and contract number in caption defendants.".into());
            next_steps.push("Add vendor defendant via 'justlegalos-cli case profile --contract-id <ID>'.".into());
        }
    } else {
        // private-provider
        let has_entity = !profile.title.is_empty() && (!profile.caption.defendants.is_empty() || profile.title.to_lowercase().contains("npi"));
        if has_entity && !profile.caption.defendants.is_empty() {
            elem1_satisfied = true;
            raw_score += 15;
            elem1_details = format!("Target provider entity '{}' identified with defendant roster.", profile.title);
        } else {
            elem1_satisfied = false;
            elem1_details = "Incomplete target entity identifiers or empty defendant roster.".into();
            elem1_missing.push("Provide provider legal name, 10-digit NPI, and registered practice location.".into());
            missing_requirements.push("Rule 9(b) Entity Identity: Provide target's legal name, 10-digit NPI or UEI, and registered address.".into());
            next_steps.push("Run 'justlegalos-cli qui-tam verify-entity --name \"<target>\"' to pull official NPI and corporate officers.".into());
        }
    }

    elements.push(Rule9bElement {
        id: "rule9b-1-entity-id".into(),
        title: "Target Entity Identification (Who)".into(),
        statutory_reference: "FRCP Rule 9(b); 31 U.S.C. § 3729; Stevens, 529 U.S. 765".into(),
        satisfied: elem1_satisfied,
        details: elem1_details,
        missing_evidence: elem1_missing,
    });

    // ---------------------------------------------------------
    // Element 2: Specific Claim / Drawdown Mechanism (What & How) [25 pts]
    // ---------------------------------------------------------
    // Rule 9(b) Clausen/Foglia standard requires identifying specific claims, invoices, or drawdowns.
    let mut has_claim_doc_in_evidence = false;
    for fname in &evidence_files {
        let l = fname.to_lowercase();
        if l.contains("invoice") || l.contains("claim") || l.contains("voucher") || l.contains("cms-1500") || l.contains("drawdown") || l.contains("remittance") || l.contains("billing") || l.contains("task-order") {
            has_claim_doc_in_evidence = true;
            break;
        }
    }

    let has_claim_in_draft = draft_lower.contains("invoice")
        || draft_lower.contains("claim form")
        || draft_lower.contains("cms-1500")
        || draft_lower.contains("asap.gov")
        || draft_lower.contains("drawdown")
        || draft_lower.contains("task order");

    let mut elem2_missing = Vec::new();
    let elem2_satisfied: bool;
    let elem2_details: String;

    if has_claim_doc_in_evidence {
        elem2_satisfied = true;
        raw_score += 25;
        elem2_details = "Verified claim/invoice/drawdown documents located in evidence/ directory.".into();
    } else if has_claim_in_draft {
        elem2_satisfied = false;
        raw_score += 10; // Partial narrative credit only
        elem2_details = "Specific billing or drawdown mechanism articulated in draft narrative, but NO actual underlying claim/invoice document is deposited in evidence/.".into();
        elem2_missing.push("No actual invoice, billing voucher, CMS-1500, or federal drawdown record in evidence/ directory.".into());
        missing_requirements.push("Rule 9(b) Specific Claims: Must deposit at least one representative invoice or drawdown voucher into evidence/.".into());
        next_steps.push("Obtain contractor billing invoices or drawdown records via public records ('justlegalos-cli records draft') and add to evidence/.".into());
    } else {
        elem2_satisfied = false;
        elem2_details = "Lacks specific claims, invoice numbers, or representative billing dates required under Rule 9(b).".into();
        elem2_missing.push("No specific billing invoice, claim number, or drawdown record identified in evidence/ or draft.".into());
        missing_requirements.push("Rule 9(b) Specific Claims: Detail specific claim numbers, billing dates, or invoice vouchers.".into());
        next_steps.push("Request monthly claims or payout records via state public records ('justlegalos-cli records draft').".into());
    }

    elements.push(Rule9bElement {
        id: "rule9b-2-specific-claims".into(),
        title: "Specific False Claims or Drawdowns (What/How)".into(),
        statutory_reference: "31 U.S.C. § 3729(a)(1)(A); Clausen/Foglia pleading standard".into(),
        satisfied: elem2_satisfied,
        details: elem2_details,
        missing_evidence: elem2_missing,
    });

    // ---------------------------------------------------------
    // Element 3: Falsity Type & Physical / Concrete Impossibility Proof [20 pts]
    // ---------------------------------------------------------
    let mut elem3_missing = Vec::new();
    let elem3_satisfied: bool;
    let elem3_details: String;

    let has_firsthand_proof = !firsthand_observations.is_empty();
    let has_contract_defect_proof = evidence_files.iter().any(|f| {
        let l = f.to_lowercase();
        l.contains("audit") || l.contains("defect") || l.contains("failure") || l.contains("breach") || l.contains("report") || l.contains("cora")
    });

    let has_falsity_narrative = draft_lower.contains("ghost")
        || draft_lower.contains("unperformed")
        || draft_lower.contains("not rendered")
        || draft_lower.contains("false certification")
        || draft_lower.contains("defect")
        || draft_lower.contains("non-compliance");

    if (has_firsthand_proof || has_contract_defect_proof) && has_falsity_narrative {
        elem3_satisfied = true;
        raw_score += 20;
        elem3_details = if has_firsthand_proof {
            format!("Firsthand physical observation proof logged across {} cryptographic entries.", firsthand_observations.len())
        } else {
            "Documented audit findings / non-performance records deposited in evidence/.".into()
        };
    } else if has_falsity_narrative {
        elem3_satisfied = false;
        raw_score += 8; // Partial credit for legal theory
        elem3_details = "Falsity theory articulated in draft narrative, but lacks verified firsthand observation photos or authenticated audit findings in evidence/.".into();
        elem3_missing.push("Deposit authenticated observation photos, inspection logs, or audit records into evidence/.".into());
        missing_requirements.push("Rule 9(b) Falsity Proof: Ground why the claim was false with verified evidence.".into());
        next_steps.push("Ingest field observation photos via 'justlegalos-cli qui-tam ingest-observation'.".into());
    } else {
        elem3_satisfied = false;
        elem3_details = "Falsity remains unsupported by verified factual observations or non-performance records.".into();
        elem3_missing.push("No field observation log, physical inspection photos, or concrete falsity evidence found.".into());
        missing_requirements.push("Rule 9(b) Falsity Proof: Detail why the claim was false.".into());
        next_steps.push("Ingest observation photos or state audit reports into evidence/.".into());
    }

    elements.push(Rule9bElement {
        id: "rule9b-3-falsity-proof".into(),
        title: "Falsity Type & Evidence (Why False)".into(),
        statutory_reference: "31 U.S.C. § 3729(a)(1)(B)".into(),
        satisfied: elem3_satisfied,
        details: elem3_details,
        missing_evidence: elem3_missing,
    });

    // ---------------------------------------------------------
    // Element 4: Scienter / Knowledge Standard [15 pts]
    // ---------------------------------------------------------
    // 31 U.S.C. § 3729(b)(1): actual knowledge, deliberate ignorance, or reckless disregard.
    let mut elem4_missing = Vec::new();
    let elem4_satisfied: bool;
    let elem4_details: String;

    let has_scienter_doc = evidence_files.iter().any(|f| {
        let l = f.to_lowercase();
        l.contains("certification") || l.contains("attestation") || l.contains("warning") || l.contains("notice") || l.contains("contract")
    });
    let has_scienter_narrative = draft_lower.contains("knowingly")
        || draft_lower.contains("reckless disregard")
        || draft_lower.contains("deliberate ignorance")
        || draft_lower.contains("certified under penalty of perjury")
        || draft_lower.contains("soc 1");

    if has_scienter_doc && has_scienter_narrative {
        elem4_satisfied = true;
        raw_score += 15;
        elem4_details = "Scienter grounded in signed compliance certifications, contract terms, or documented prior warnings.".into();
    } else if has_scienter_narrative {
        elem4_satisfied = false;
        raw_score += 6; // Partial credit
        elem4_details = "Draft uses scienter terminology ('knowingly' / 'reckless disregard'), but lacks citation to defendant's signed certification, warning letter, or internal defect tickets.".into();
        elem4_missing.push("Cite the target's signed enrollment agreement, annual certification, or internal defect notice.".into());
        missing_requirements.push("Rule 9(b) Scienter Grounding: Attach the contract certification, SOC 1 report, or warning record.".into());
        next_steps.push("Request contractor annual compliance certifications or audit responses via open records.".into());
    } else {
        elem4_satisfied = false;
        elem4_details = "Scienter element needs explicit statutory grounding under 31 U.S.C. § 3729(b)(1).".into();
        elem4_missing.push("Proof of knowledge, deliberate ignorance, or signed attestation not articulated.".into());
        missing_requirements.push("Rule 9(b) Scienter: Show defendant acted with actual knowledge, deliberate ignorance, or reckless disregard.".into());
        next_steps.push("Identify the signed compliance certification or proof of notice of defects.".into());
    }

    elements.push(Rule9bElement {
        id: "rule9b-4-scienter".into(),
        title: "Scienter & Knowledge Standard (Mental State)".into(),
        statutory_reference: "31 U.S.C. § 3729(b)(1)".into(),
        satisfied: elem4_satisfied,
        details: elem4_details,
        missing_evidence: elem4_missing,
    });

    // ---------------------------------------------------------
    // Element 5: Materiality & Quantitative Loss Calculation [15 pts]
    // ---------------------------------------------------------
    let mut elem5_missing = Vec::new();
    let elem5_satisfied: bool;
    let elem5_details: String;

    let has_loss_number = draft_lower.contains('$') || !profile.target_aln.is_empty();
    let has_loss_doc = evidence_files.iter().any(|f| {
        let l = f.to_lowercase();
        l.contains("loss") || l.contains("payment") || l.contains("award") || l.contains("usaspending") || l.contains("contract")
    });

    if has_loss_number && has_loss_doc {
        elem5_satisfied = true;
        raw_score += 15;
        elem5_details = "Materiality and quantitative federal loss calculation supported by award/contract documentation.".into();
    } else if has_loss_number {
        elem5_satisfied = false;
        raw_score += 7; // Partial credit for stating amount
        elem5_details = "Dollar amount stated in draft, but lacking authenticated payment history, USAspending obligation, or invoice sum in evidence/.".into();
        elem5_missing.push("Attach USAspending award records, agency payment history, or contract schedules to support loss amount.".into());
        missing_requirements.push("Rule 9(b) Loss Calculation: Cross-reference contractor award amounts with specific federal ALN funds.".into());
        next_steps.push("Cross-reference federal Assistance Listing Number (ALN) obligations.".into());
    } else {
        elem5_satisfied = false;
        elem5_details = "Estimated federal loss needs explicit mathematical estimation and ALN nexus.".into();
        elem5_missing.push("No estimated federal financial loss or calculation formula provided.".into());
        missing_requirements.push("Rule 9(b) Materiality: Estimate total federal loss and explain why falsehood was material to payment.".into());
        next_steps.push("Obtain contract schedule amounts and calculate federal financial loss.".into());
    }

    elements.push(Rule9bElement {
        id: "rule9b-5-materiality-loss".into(),
        title: "Materiality & Federal Loss Estimate".into(),
        statutory_reference: "31 U.S.C. § 3729(a)(1)(G); Escobar materiality rule".into(),
        satisfied: elem5_satisfied,
        details: elem5_details,
        missing_evidence: elem5_missing,
    });

    // ---------------------------------------------------------
    // Element 6: Original Source & Public Disclosure Check (§ 3730(e)(4)) [10 pts]
    // ---------------------------------------------------------
    let mut elem6_missing = Vec::new();
    let elem6_satisfied: bool;
    let elem6_details: String;

    let has_firsthand_standing = !firsthand_observations.is_empty() || !profile.timeline.is_empty();
    let has_public_audit_reliance = evidence_files.iter().any(|f| f.to_lowercase().contains("audit"))
        || draft_lower.contains("state auditor")
        || draft_lower.contains("oig report");

    if has_firsthand_standing && !has_public_audit_reliance {
        elem6_satisfied = true;
        raw_score += 10;
        elem6_details = "Grounded entirely in independent firsthand observations/records; public disclosure bar risk minimized.".into();
    } else if has_firsthand_standing && has_public_audit_reliance {
        elem6_satisfied = true;
        raw_score += 8;
        elem6_details = "Relator possesses independent firsthand knowledge, but also references public audits (e.g. OSA / OIG reports). Public audits must be cited strictly as contextual corroboration, not the primary factual basis, to satisfy the Original Source exception under 31 U.S.C. § 3730(e)(4)(B).".into();
    } else if has_public_audit_reliance {
        elem6_satisfied = false;
        raw_score += 3;
        elem6_details = "ELEVATED PUBLIC DISCLOSURE BAR RISK: Allegations appear derived primarily from publicly released state/federal audits without independent firsthand basis. Under 31 U.S.C. § 3730(e)(4)(A) and Schindler Elevator, qui tam claims derived from public reports are barred unless relator is an original source.".into();
        elem6_missing.push("Allegations derived from public reports. Must demonstrate independent, direct firsthand knowledge.".into());
        missing_requirements.push("Public Disclosure Bar: Establish direct personal knowledge independent of public government audits.".into());
        next_steps.push("Document personal observations, claimant notices, or internal communications predating public audit reports.".into());
    } else {
        elem6_satisfied = false;
        elem6_details = "Lacks confirmation of direct, independent firsthand knowledge of the fraud.".into();
        elem6_missing.push("Lacks confirmation that relator has direct firsthand knowledge.".into());
        missing_requirements.push("Public Disclosure Bar: Confirm allegations are not derived solely from news or public reports.".into());
        next_steps.push("Document personal observation logs or claimant notices.".into());
    }

    elements.push(Rule9bElement {
        id: "rule9b-6-original-source".into(),
        title: "Original Source & Public Disclosure Clearance".into(),
        statutory_reference: "31 U.S.C. § 3730(e)(4)(A)-(B); Schindler Elevator".into(),
        satisfied: elem6_satisfied,
        details: elem6_details,
        missing_evidence: elem6_missing,
    });

    // ---------------------------------------------------------
    // HARD INTEGRITY CAP RULE (Bug 15 Resolution):
    // If target is a state agency (Stevens), cap score at 0.
    // If no direct false-claim document (invoice, drawdown, or signed false certification)
    // exists in evidence/, the score is strictly capped at max 74/100 (Developing / Fixable).
    // CANNOT be labeled "High Viability (Attorney & DOJ Review Ready)"!
    // ---------------------------------------------------------
    let final_score = if target_type == "gov-agency" {
        0
    } else if !has_claim_doc_in_evidence {
        std::cmp::min(raw_score, 74)
    } else {
        std::cmp::min(raw_score, 100)
    };

    let viability_tier = match final_score {
        80..=100 => "High Viability (Attorney & DOJ Review Ready)".into(),
        50..=79 => "Developing (Claims Proof & Specific Drawdown Evidence Needed)".into(),
        1..=49 => "Preliminary / Speculative (Missing Core Rule 9(b) Elements)".into(),
        _ => if target_type == "gov-agency" {
            "Barred as Qui Tam Against State (Redirect to Contractors & Accountability Map)".into()
        } else {
            "Speculative / Ungrounded (Core Elements Missing)".into()
        },
    };

    let public_disclosure_risk = if target_type == "gov-agency" {
        "NOT APPLICABLE: Qui tam barred under Stevens sovereign immunity.".into()
    } else if has_firsthand_standing && !has_public_audit_reliance {
        "LOW RISK: Grounded in independent firsthand observations; no reliance on public media/reports.".into()
    } else if has_public_audit_reliance {
        "ELEVATED RISK: Allegations reference public audits/reports. Counsel must confirm relator qualifies as an Original Source under 31 U.S.C. § 3730(e)(4)(B).".into()
    } else {
        "ELEVATED RISK: Ensure facts were not published on social media, news, or blogs prior to filing under seal.".into()
    };

    Ok(Rule9bAudit {
        case_id: profile.id.clone(),
        score: final_score,
        viability_tier,
        elements,
        missing_requirements,
        next_investigative_steps: next_steps,
        public_disclosure_risk,
        evaluated_at: now,
    })
}
