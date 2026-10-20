use std::fs;
use std::path::Path;
use uuid::Uuid;

use super::finder::find_records_law;
use super::org::just_liberty_profile;
use super::types::RecordsDraftRequest;

pub fn draft_records_request(
    case_dir: &Path,
    jurisdiction: &str,
    record_type: &str,
    custom_items: Option<Vec<String>>,
    use_fee_waiver: bool,
    custom_requester_name: Option<&str>,
) -> Result<RecordsDraftRequest, String> {
    let law = find_records_law(jurisdiction, record_type)?;
    let org = just_liberty_profile();

    let req_id = format!("rec-{}", &Uuid::new_v4().to_string()[..8]);
    let date_str = chrono::Local::now().format("%B %d, %Y").to_string();

    let custodian = law.custodians.first().ok_or_else(|| "No custodian listed for law".to_string())?;

    let is_own_claim = law.own_file_route.is_some() || record_type.to_lowercase().contains("own");

    let requester_header = if is_own_claim {
        let name = custom_requester_name.unwrap_or("Claimant / Relator");
        format!(
            "{}\n[Claimant Residential Address]\n[Claimant Phone & Email]\nSocial Security Number: [Required on Form B-652 / Leave Blank for Public Transmission]",
            name
        )
    } else {
        format!(
            "{}\n{}\nEIN: {}\nColorado SOS ID: {}\n{}\nPhone: {} | Email: {}",
            org.name, org.legal_structure, org.ein, org.sos_id, org.address, org.phone, org.email
        )
    };

    let items = custom_items.unwrap_or_else(|| {
        if is_own_claim {
            vec![
                "Complete Claimant File including initial determination, redeterminations, and notices of decision".into(),
                "Internal agency notes, examiner notes, and phone call logs/recordings concerning claim holds".into(),
                "Interstate wage transfer logs, ICON records, and IB-4 data exchanges if an interstate claim".into(),
                "Identity verification audit logs, fraud-flag metadata, and LexisNexis / ID.me transaction receipts".into(),
            ]
        } else {
            vec![
                "All prime contracts, task orders, and contract amendments between Agency and primary IT/software vendor".into(),
                "Monthly and milestone invoices, payment vouchers, and drawdown requests submitted by vendor".into(),
                "Vendor Statements of Work (SOW), Service Level Agreement (SLA) performance reports, and defect logs".into(),
                "Annual Statement on Standards for Attestation Engagements (SSAE-18) SOC 1 Type II compliance reports".into(),
            ]
        }
    });

    let fee_waiver_text = if !use_fee_waiver {
        "Requester agrees to pay reasonable statutory duplication fees up to $25.00. Notify requester in advance if estimated fees exceed this amount.".to_string()
    } else if law.jurisdiction == "US" {
        format!(
            "FEE WAIVER REQUEST PURSUANT TO 5 U.S.C. § 552(a)(4)(A)(iii):\n\
            Requester {} is a 501(c)(3) nonprofit public interest research and civil liberties organization (EIN {}). \
            Disclosure of the requested information is in the public interest because it is likely to contribute significantly to public understanding \
            of the operations or activities of the government (specifically, the administration of federal assistance funds and contractor oversight) \
            and is not primarily in the commercial interest of the requester. In the event fees are assessed despite this waiver, notify requester in advance if fees exceed $0.00.",
            org.name, org.ein
        )
    } else if law.jurisdiction == "CO" {
        format!(
            "FEE WAIVER & COST NOTIFICATION PURSUANT TO C.R.S. § 24-72-205(6):\n\
            Requester {} is a Colorado nonprofit public interest organization (EIN {}). \
            The records are sought for public interest accountability and non-commercial public dissemination. \
            Under CORA, the first hour of research and retrieval is provided free of charge. \
            Please provide records electronically to avoid duplication fees. \
            MANDATORY: Notify requester in advance in writing if total research and retrieval fees are estimated to exceed $0.00.",
            org.name, org.ein
        )
    } else {
        "Please notify requester in advance if total retrieval fees are estimated to exceed $0.00.".to_string()
    };

    let estimated_cost_notice = if law.jurisdiction == "NC" {
        "Statutory $15.00 minimum certified check or money order required with submission. Do NOT send cash or personal checks.".to_string()
    } else if law.jurisdiction == "CO" {
        "First hour free; maximum statutory rate thereafter is $41.37/hour under C.R.S. § 24-72-205(6)(b). Electronic transmission is free.".to_string()
    } else {
        "Standard electronic delivery is free; statutory public interest fee waiver requested.".to_string()
    };

    let mailing_instructions = if law.jurisdiction == "NC" {
        Some("Must be mailed via USPS with certified check / money order payable to 'NC Division of Employment Security' to PO Box 25903, Raleigh NC 27611-5903. Leave witness line blank.".into())
    } else if is_own_claim && law.jurisdiction == "CO" {
        Some("Mail notarized Form B-652 with full SSN to CDLE Records Unit, PO Box 400, Denver CO 80201-0400. Records are returned by mail only.".into())
    } else {
        None
    };

    let mut items_formatted = String::new();
    for (i, it) in items.iter().enumerate() {
        items_formatted.push_str(&format!("{}. {}\n", i + 1, it));
    }

    let subject = format!("Formal Public Records Request: {} [{}]", law.name, req_id);

    let formal_letter_text = format!(
"{date}\n\n\
TO:\n\
Records Custodian\n\
{agency}\n\
{email_or_mail}\n\n\
FROM:\n\
{requester}\n\n\
RE: {subject}\n\n\
Dear Records Custodian,\n\n\
Pursuant to {statute}, I hereby formally request inspection and copies of the following public records within the statutory response window ({working_days} working days):\n\n\
SPECIFIC RECORDS REQUESTED:\n\
{items}\n\
{fee_waiver}\n\n\
PREFERRED FORMAT:\n\
Please provide all records in electronic format (PDF / CSV) delivered via email to avoid paper duplication costs. If any record exists in electronic database form, please export in native format.\n\n\
EXEMPTION NOTICE & SEGREGATION:\n\
If you determine that any portion of the requested records is exempt from disclosure under statutory provisions, please segregate the exempt material and release all reasonably segregable non-exempt portions. For any withheld record or redaction, please state the specific statutory exemption relied upon.\n\n\
Thank you for your prompt assistance with this public records request.\n\n\
Respectfully submitted,\n\n\
/s/ Authorized Requester\n\
{requester_sig}\n",
        date = date_str,
        agency = custodian.agency,
        email_or_mail = custodian.email.clone().unwrap_or_else(|| custodian.mailing_address.clone().unwrap_or_default()),
        requester = requester_header,
        subject = subject,
        statute = law.citation,
        working_days = law.response_working_days,
        items = items_formatted,
        fee_waiver = fee_waiver_text,
        requester_sig = if is_own_claim { custom_requester_name.unwrap_or("Claimant") } else { "Just Liberty Incorporated" }
    );

    let drafts_dir = case_dir.join("drafts");
    let _ = fs::create_dir_all(&drafts_dir);
    let draft_file = drafts_dir.join(format!("records_request_{}_{}.md", law.jurisdiction.to_lowercase(), req_id));
    let _ = fs::write(&draft_file, &formal_letter_text);

    Ok(RecordsDraftRequest {
        request_id: req_id,
        jurisdiction: law.jurisdiction,
        record_type: record_type.to_string(),
        custodian_agency: custodian.agency.clone(),
        custodian_email: custodian.email.clone(),
        subject,
        items_requested: items,
        formal_letter_text,
        fee_waiver_requested: use_fee_waiver,
        fee_waiver_text,
        estimated_cost_notice,
        mailing_instructions,
    })
}
