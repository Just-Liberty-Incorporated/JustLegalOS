use super::types::{RecordsCustodian, RecordsLaw};

pub fn find_records_law(jurisdiction: &str, record_type: &str) -> Result<RecordsLaw, String> {
    let j_upper = jurisdiction.trim().to_ascii_uppercase();
    let r_lower = record_type.trim().to_ascii_lowercase();

    match j_upper.as_str() {
        "CO" | "COLORADO" => {
            if r_lower.contains("own") || r_lower.contains("claim-file") || r_lower.contains("my-ui") {
                Ok(RecordsLaw {
                    name: "Colorado Division of Unemployment Insurance Claimant Access (20 CFR § 603.5(d))".into(),
                    citation: "20 CFR § 603.5(d); C.R.S. § 8-72-107; CDLE Form B-652".into(),
                    jurisdiction: "CO".into(),
                    response_working_days: 10,
                    extension_working_days: 10,
                    fee_structure: "Free for standard claimant file requests. Do NOT use CORA (UI claim files are confidential to third parties under C.R.S. § 8-72-107).".into(),
                    exemptions_of_note: vec![
                        "C.R.S. § 8-72-107 (Unemployment records are confidential and exempt from third-party CORA inspection).".into(),
                        "MyUI+ online portal does NOT hold call recordings, agent internal notes, or interstate ICON/IB-4 logs; must request via Form B-652.".into(),
                    ],
                    custodians: vec![
                        RecordsCustodian {
                            agency: "Colorado Department of Labor and Employment (CDLE) - UI Division".into(),
                            email: None,
                            portal_url: Some("https://myui.coworkforce.com".into()),
                            mailing_address: Some("CDLE Unemployment Insurance Division, Records Unit, PO Box 400, Denver, CO 80201-0400".into()),
                            fee_rate: Some("Free for claimant; requires notarized Form B-652 with full SSN".into()),
                            notes: Some("Requires notarized Form B-652 with full Social Security Number. Specific items must be itemized; CDLE rejects broad 'all records' requests.".into()),
                        }
                    ],
                    own_file_route: Some("MyUI+ portal or notarized Form B-652 with full SSN sent via mail. Records returned by mail only.".into()),
                    source: "cdle.colorado.gov; 20 CFR § 603.5(d); C.R.S. § 8-72-107".into(),
                    verified_on: "2026-10-19".into(),
                })
            } else {
                Ok(RecordsLaw {
                    name: "Colorado Open Records Act (CORA)".into(),
                    citation: "C.R.S. § 24-72-201 et seq.".into(),
                    jurisdiction: "CO".into(),
                    response_working_days: 3,
                    extension_working_days: 7,
                    fee_structure: "First hour of research and retrieval free; maximum $41.37/hour thereafter. Request fee estimate before retrieval begins.".into(),
                    exemptions_of_note: vec![
                        "C.R.S. § 8-72-107 (Individual UI claim records confidential).".into(),
                        "C.R.S. § 2-3-110.5 (State Auditor workpapers confidential).".into(),
                        "C.R.S. § 24-72-204(3)(a)(IV) (Trade secrets / confidential commercial data).".into(),
                    ],
                    custodians: vec![
                        RecordsCustodian {
                            agency: "Colorado Department of Labor and Employment (CDLE)".into(),
                            email: Some("cdle_pr@state.co.us".into()),
                            portal_url: Some("https://cdle.colorado.gov/open-records-requests".into()),
                            mailing_address: Some("CDLE CORA Custodian, 633 17th Street, Suite 1200, Denver, CO 80202".into()),
                            fee_rate: Some("≤ $41.37/hr after 1 free hr; electronic copies free".into()),
                            notes: Some("Submit requests to cdle_pr@state.co.us. Statutory deadline is 3 working days; agency may invoke 7-working-day extenuating circumstances extension.".into()),
                        }
                    ],
                    own_file_route: None,
                    source: "colorado.public.law/statutes/crs_24-72-201; cdle.colorado.gov".into(),
                    verified_on: "2026-10-19".into(),
                })
            }
        }

        "US" | "FEDERAL" | "FED" => {
            if r_lower.contains("oig") || r_lower.contains("privacy") || r_lower.contains("own") {
                Ok(RecordsLaw {
                    name: "Privacy Act of 1974 (Individual Records Access)".into(),
                    citation: "5 U.S.C. § 552a".into(),
                    jurisdiction: "US".into(),
                    response_working_days: 20,
                    extension_working_days: 10,
                    fee_structure: "No search or review fees; standard duplication fees only.".into(),
                    exemptions_of_note: vec![
                        "5 U.S.C. § 552a(j)(2) / (k)(2) (Investigatory material compiled for law enforcement purposes in active OIG files).".into(),
                    ],
                    custodians: vec![
                        RecordsCustodian {
                            agency: "U.S. Department of Labor, Office of Inspector General (DOL-OIG)".into(),
                            email: Some("foiarequests@dol.gov".into()),
                            portal_url: Some("https://www.dol.gov/general/foia".into()),
                            mailing_address: Some("Office of Inspector General, Disclosure Unit, 200 Constitution Ave NW, Room S-5508, Washington, DC 20210".into()),
                            fee_rate: Some("Free for electronic delivery".into()),
                            notes: Some("Must include sworn declaration of identity under penalty of perjury pursuant to 28 U.S.C. § 1746.".into()),
                        }
                    ],
                    own_file_route: Some("Requires signed verification of identity under 28 U.S.C. § 1746.".into()),
                    source: "dol.gov/general/foia; 5 U.S.C. § 552a".into(),
                    verified_on: "2026-10-19".into(),
                })
            } else {
                Ok(RecordsLaw {
                    name: "Freedom of Information Act (FOIA)".into(),
                    citation: "5 U.S.C. § 552".into(),
                    jurisdiction: "US".into(),
                    response_working_days: 20,
                    extension_working_days: 10,
                    fee_structure: "Fee waiver available under 5 U.S.C. § 552(a)(4)(A)(iii) for nonprofit public interest organizations contributing to public understanding of government operations.".into(),
                    exemptions_of_note: vec![
                        "Exemption 4 (Trade secrets / commercial info).".into(),
                        "Exemption 5 (Deliberative process / inter-agency privileged memos).".into(),
                        "Exemption 7 (Law enforcement investigatory records).".into(),
                    ],
                    custodians: vec![
                        RecordsCustodian {
                            agency: "U.S. Department of Labor, Employment and Training Administration (ETA)".into(),
                            email: Some("foiarequests@dol.gov".into()),
                            portal_url: Some("https://www.foia.gov".into()),
                            mailing_address: Some("U.S. Department of Labor - ETA FOIA Officer, 200 Constitution Ave NW, Room N-5653, Washington, DC 20210".into()),
                            fee_rate: Some("Nonprofit fee waiver applicable".into()),
                            notes: Some("Submit via foiarequests@dol.gov or online PAL portal. 20 working day statutory limit.".into()),
                        }
                    ],
                    own_file_route: None,
                    source: "law.cornell.edu/uscode/text/5/552; dol.gov".into(),
                    verified_on: "2026-10-19".into(),
                })
            }
        }

        "NC" | "NORTH CAROLINA" => {
            Ok(RecordsLaw {
                name: "North Carolina Division of Employment Security Records Request".into(),
                citation: "N.C. Gen. Stat. § 96-4; NC DES Form DESLEGAL 100".into(),
                jurisdiction: "NC".into(),
                response_working_days: 15,
                extension_working_days: 15,
                fee_structure: "$15.00 minimum statutory fee. Must be paid strictly via certified check or cashier's check/money order payable to 'NC Division of Employment Security'.".into(),
                exemptions_of_note: vec![
                    "Must submit DESLEGAL 100 plus formal request letter as one complete package.".into(),
                    "Records returned strictly by postal mail (no email delivery of wage records).".into(),
                    "Witness signature line is for in-person filings only; leave blank for mailed submissions.".into(),
                ],
                custodians: vec![
                    RecordsCustodian {
                        agency: "North Carolina Division of Employment Security (NC DES)".into(),
                        email: None,
                        portal_url: Some("https://des.nc.gov".into()),
                        mailing_address: Some("NC Division of Employment Security, Attn: Legal Dept - Records, PO Box 25903, Raleigh, NC 27611-5903".into()),
                        fee_rate: Some("$15.00 money order or certified check required".into()),
                        notes: Some("Do NOT send cash or personal checks. Send via USPS with tracking.".into()),
                    }
                ],
                own_file_route: Some("Form DESLEGAL 100 + $15 money order to PO Box 25903, Raleigh NC 27611-5903. Delivered via mail only.".into()),
                source: "des.nc.gov; N.C. Gen. Stat. § 96-4".into(),
                verified_on: "2026-10-19".into(),
            })
        }

        _ => Err(format!(
            "Unsupported records jurisdiction '{}'. Supported jurisdictions: CO (Colorado), US (Federal), NC (North Carolina).",
            jurisdiction
        )),
    }
}
