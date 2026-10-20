use std::fs;
use std::path::Path;

pub fn draft_followup_letter(
    case_dir: &Path,
    req_id: &str,
    law_name: &str,
    agency: &str,
    followup_kind: &str, // "late", "denied", "wrong-route"
) -> Result<String, String> {
    let date_str = chrono::Local::now().format("%B %d, %Y").to_string();

    let letter_content = match followup_kind.to_ascii_lowercase().as_str() {
        "late" => format!(
"{date}\n\n\
TO: Records Custodian, {agency}\n\
RE: STATUTORY DEADLINE EXCEEDED: Formal Public Records Request [{req_id}]\n\n\
Dear Records Custodian,\n\n\
On a prior date, requester submitted a formal public records request under {law_name} referenced above. \
The mandatory statutory response window has elapsed without production of the requested records or written notice of extenuating circumstances.\n\n\
Under governing law, timely compliance is non-discretionary. Please provide an immediate status update and produce the non-exempt responsive records electronically without further delay.\n\n\
Respectfully submitted,\n\
/s/ Authorized Requester\n",
            date = date_str, agency = agency, req_id = req_id, law_name = law_name
        ),

        "denied" => format!(
"{date}\n\n\
TO: Agency Head / Appeals Officer, {agency}\n\
RE: FORMAL ADMINISTRATIVE APPEAL: Denial of Public Records Request [{req_id}]\n\n\
Dear Appeals Officer,\n\n\
Requester hereby files this formal administrative appeal regarding the denial/redaction of records requested under {law_name} (Request ID: {req_id}).\n\n\
STATEMENT OF GROUNDS:\n\
1. The agency has not met its burden to demonstrate that the withheld records are exempt from mandatory disclosure.\n\
2. Governing precedent requires narrow construction of exemptions and mandatory segregation of all non-exempt portions.\n\
3. Disclosure is in the overwhelming public interest to ensure public integrity of government program operations.\n\n\
Requester requests immediate administrative reversal of the withholding determination and full production of responsive records.\n\n\
Respectfully submitted,\n\
/s/ Authorized Requester\n",
            date = date_str, agency = agency, req_id = req_id, law_name = law_name
        ),

        "wrong-route" => format!(
"{date}\n\n\
TO: Records Unit, {agency}\n\
RE: CORRECTION TO OWN-CLAIMANT RECORDS ROUTE: Claim File Submission [{req_id}]\n\n\
Dear Records Officer,\n\n\
In response to your notification regarding claimant privacy restrictions under unemployment confidentiality statutes (e.g. C.R.S. § 8-72-107), \
requester clarifies that this request is submitted on behalf of the individual claimant pursuant to 20 CFR § 603.5(d) and state own-file procedures.\n\n\
Enclosed please find the completed, notarized claimant authorization form (e.g. Form B-652 / DESLEGAL 100) bearing the claimant's verified identity. \
Please produce the claimant's file, examiner notes, and transaction audit logs pursuant to federal claimant access standards.\n\n\
Respectfully submitted,\n\
/s/ Authorized Requester\n",
            date = date_str, agency = agency, req_id = req_id
        ),

        _ => return Err(format!("Unknown followup kind '{}'. Supported: late, denied, wrong-route", followup_kind)),
    };

    let drafts_dir = case_dir.join("drafts");
    let _ = fs::create_dir_all(&drafts_dir);
    let path = drafts_dir.join(format!("records_followup_{}_{}.md", followup_kind, req_id));
    let _ = fs::write(&path, &letter_content);

    Ok(letter_content)
}
