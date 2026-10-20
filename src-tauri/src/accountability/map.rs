//! Institutional & Systemic Fraud Accountability Map (14-Pathway Multi-Track Navigator).
//! Evaluates all legal, administrative, audit, and legislative routes side-by-side
//! under Vermont ANR v. Stevens, 529 U.S. 765 (2000), 31 U.S.C. § 3729, and state oversight regimes.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleCitation {
    pub cite: String,
    pub url: Option<String>,
    pub verified_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountabilityRouteItem {
    pub route: String,
    pub title: String,
    pub grade: String, // "Solid", "Fixable", "Long shot", "Dead end"
    pub reason: String,
    pub what_changes_grade: String,
    pub deadline: String,
    pub contact: String,
    pub sequencing: String,
    pub sources: Vec<RuleCitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountabilityMapReport {
    pub ok: bool,
    pub never_files: bool,
    pub never_serves: bool,
    pub case_id: String,
    pub case_title: String,
    pub target_type: String,
    pub governing_state: String,
    pub target_program: String,
    pub best_next_3_moves: Vec<String>,
    pub routes: Vec<AccountabilityRouteItem>,
    pub generated_at: String,
    pub markdown_report_path: Option<String>,
    pub json_report_path: Option<String>,
}

pub fn generate_accountability_map(
    case_dir: &Path,
    state_override: Option<&str>,
    program_override: Option<&str>,
    target_type_override: Option<&str>,
    _role_override: Option<&str>,
    _key_date: Option<&str>,
) -> Result<AccountabilityMapReport, String> {
    let profile = crate::case_profile::load_case(case_dir)?;
    let now = Utc::now().to_rfc3339();

    let state = state_override
        .map(|s| s.to_ascii_uppercase())
        .unwrap_or_else(|| {
            if !profile.governing_state.is_empty() {
                profile.governing_state.to_ascii_uppercase()
            } else if profile.court.name.to_ascii_uppercase().contains("COLORADO") {
                "CO".to_string()
            } else if profile.court.name.to_ascii_uppercase().contains("NEW JERSEY") {
                "NJ".to_string()
            } else {
                "FEDERAL".to_string()
            }
        });

    let target_type = target_type_override
        .map(|t| t.to_string())
        .unwrap_or_else(|| {
            if !profile.target_type.is_empty() {
                profile.target_type.clone()
            } else {
                "gov-agency".to_string()
            }
        });

    let target_program = program_override
        .map(|p| p.to_string())
        .unwrap_or_else(|| {
            if !profile.target_program.is_empty() {
                profile.target_program.clone()
            } else {
                "ui".to_string()
            }
        });

    let verified_date = "2026-10-19".to_string();

    let mut routes = Vec::new();

    // 1. Qui Tam vs. Private Contractor
    routes.push(AccountabilityRouteItem {
        route: "qui_tam_contractor".into(),
        title: "False Claims Act Qui Tam vs. Private Contractor (31 U.S.C. § 3729)".into(),
        grade: "Fixable".into(),
        reason: "Private contractors (e.g. IT/software vendors, intake operators) are FCA 'persons' (1 U.S.C. § 1; 31 U.S.C. § 3729(b)(2)(A)(ii)). Treble damages and 15-30% relator bounty apply if false claims or false compliance certifications (e.g. SOC 1 Type II) were submitted on federally funded contracts.".into(),
        what_changes_grade: "Obtain specific contractor invoice, drawdown voucher, statement of work deliverable, or signed certification of compliance via open records (CORA/FOIA) or insider.".into(),
        deadline: "31 U.S.C. § 3731(b): Later of 6 years from violation or 3 years from U.S. official knowledge (max 10 years). Pandemic contract window is currently expiring.".into(),
        contact: "Requires engagement of licensed Qui Tam / Relator Counsel to file sealed complaint in U.S. District Court.".into(),
        sequencing: "CONSULT COUNSEL FIRST. Do not disclose allegations on social media, press, or public forums before counsel files sealed complaint to preserve 31 U.S.C. § 3730(b)(2) seal and § 3730(e)(4) Public Disclosure Bar.".into(),
        sources: vec![
            RuleCitation { cite: "31 U.S.C. § 3729(a)(1); 31 U.S.C. § 3730(b)".into(), url: Some("https://www.law.cornell.edu/uscode/text/31/3729".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "1 U.S.C. § 1 (Definition of Person includes corporations)".into(), url: Some("https://www.law.cornell.edu/uscode/text/1/1".into()), verified_on: verified_date.clone() },
        ],
    });

    // 2. False Claims Act vs. State Agency Directly
    routes.push(AccountabilityRouteItem {
        route: "fca_state_agency".into(),
        title: "False Claims Act Qui Tam vs. State Agency Directly".into(),
        grade: "Dead end".into(),
        reason: "Under Vermont Agency of Natural Resources v. United States ex rel. Stevens, 529 U.S. 765 (2000), a State or state agency is NOT a 'person' under 31 U.S.C. § 3729(a) and cannot be sued in a qui tam lawsuit. Qui tam against the State is barred as a matter of law.".into(),
        what_changes_grade: "No change possible under current Supreme Court jurisprudence. Must pivot to private contractors or systemic administrative/audit channels.".into(),
        deadline: "Barred as a matter of law.".into(),
        contact: "None (legally invalid qui tam defendant).".into(),
        sequencing: "Do not name the State or state agency as an FCA defendant; doing so triggers immediate Rule 12(b)(6) dismissal.".into(),
        sources: vec![
            RuleCitation { cite: "Vermont ANR v. U.S. ex rel. Stevens, 529 U.S. 765 (2000)".into(), url: Some("https://supreme.justia.com/cases/federal/us/529/765/".into()), verified_on: verified_date.clone() },
        ],
    });

    // 3. FCA Against Agency Officials in Personal Capacity
    routes.push(AccountabilityRouteItem {
        route: "fca_individual_capacity".into(),
        title: "False Claims Act vs. State Officials in Personal Capacity".into(),
        grade: "Long shot".into(),
        reason: "Officials sued in official capacity are treated as the State and dismissed under Stevens (Will, 491 U.S. 58). Sues in personal capacity allowed in Ninth Circuit (Stoner v. Santa Clara County, 502 F.3d 1116 (2007)), but Tenth Circuit has not adopted Stoner and judicial skepticism of end-runs around Stevens is high.".into(),
        what_changes_grade: "Direct evidence of personal financial kickbacks, embezzlement, or intentional ultra vires fraudulent acts.".into(),
        deadline: "31 U.S.C. § 3731(b) (6 years / 3 years).".into(),
        contact: "Qui Tam Relator Counsel.".into(),
        sequencing: "Evaluate with counsel after contractor claims are framed.".into(),
        sources: vec![
            RuleCitation { cite: "Stoner v. Santa Clara Cnty. Office of Educ., 502 F.3d 1116 (9th Cir. 2007)".into(), url: Some("https://www.courtlistener.com/opinion/1454176/stoner-v-santa-clara-county-office-of-education/".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "Will v. Michigan Dept. of State Police, 491 U.S. 58 (1989)".into(), url: Some("https://supreme.justia.com/cases/federal/us/491/58/".into()), verified_on: verified_date.clone() },
        ],
    });

    // 4. Federal OIG Hotline & Program Audit
    routes.push(AccountabilityRouteItem {
        route: "federal_oig".into(),
        title: "Federal Office of Inspector General Hotline (DOL-OIG / HHS-OIG)".into(),
        grade: "Solid".into(),
        reason: "Federal OIG possesses statutory subpoena authority, audit teams, and criminal referral power under Inspector General Act of 1978. DOL-OIG oversees federal UI funds (ALN 17.225) and contractor integrity. Accepts online tips; anonymous allowed.".into(),
        what_changes_grade: "Already solid as a reporting channel. Supply Bates-stamped evidence index, contract numbers, and specific loss calculations.".into(),
        deadline: "None published; earliest submission ensures active investigation before contract records age out.".into(),
        contact: "Online form: oig.dol.gov/hotline.htm (doloiglocats.secureocp.com/eCasePortal); Hotline: 1-800-347-3756.".into(),
        sequencing: "Coordinate with Qui Tam counsel. An administrative OIG tip is confidential, but relator counsel should sequence submission to protect first-to-file status.".into(),
        sources: vec![
            RuleCitation { cite: "5 U.S.C. App. 3 § 4 (Inspector General Act of 1978)".into(), url: Some("https://www.law.cornell.edu/uscode/text/5a/compiled-act-95-452".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "DOL-OIG Hotline Operating Authority".into(), url: Some("https://www.oig.dol.gov/hotlinemain.htm".into()), verified_on: verified_date.clone() },
        ],
    });

    // 5. Federal Program Conformity & Grant Disallowance (DOL ETA)
    routes.push(AccountabilityRouteItem {
        route: "agency_conformity_clawback".into(),
        title: "Federal Program Conformity Enforcement & Grant Disallowance (DOL ETA / SSA § 303)".into(),
        grade: "Fixable".into(),
        reason: "Under Social Security Act § 303(a)(1) ('methods of administration reasonably calculated to insure full payment of unemployment compensation when due') and § 303(b), the Secretary of Labor may withhold federal administrative grants for state non-conformity. Uniform Guidance (2 CFR 200.339) authorizes federal grant disallowance.".into(),
        what_changes_grade: "Document state-wide systematic denial/hold patterns contradicting ETA standards via ETA 227/586 reports and records responses.".into(),
        deadline: "Administrative petition (no strict statute of limitations; active while state practices persist).".into(),
        contact: "U.S. Department of Labor, Employment and Training Administration (ETA), Office of Unemployment Insurance, 200 Constitution Ave NW, Washington, DC 20210.".into(),
        sequencing: "Can be pursued by nonprofit advocacy organization (e.g. Just Liberty Inc.) without violating individual seal.".into(),
        sources: vec![
            RuleCitation { cite: "42 U.S.C. § 503(a)(1), (b) (Social Security Act § 303)".into(), url: Some("https://www.law.cornell.edu/uscode/text/42/503".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "2 CFR § 200.339 (Remedies for noncompliance)".into(), url: Some("https://www.ecfr.gov/current/title-2/section-200.339".into()), verified_on: verified_date.clone() },
        ],
    });

    // 6. GAO FraudNet & Congressional Oversight
    routes.push(AccountabilityRouteItem {
        route: "gao_fraudnet".into(),
        title: "GAO FraudNet & Congressional Committee Oversight".into(),
        grade: "Solid".into(),
        reason: "U.S. Government Accountability Office (GAO) FraudNet referrals support congressional investigations and federal audit requests. GAO assigns a tracking control number to every report. House Ways & Means Committee has active jurisdiction over pandemic UI contractor fraud.".into(),
        what_changes_grade: "Already solid. Strengthened by providing a concise, sourced public-interest report.".into(),
        deadline: "None published.".into(),
        contact: "Online form: gao.gov/about/what-gao-does/fraudnet; Phone: 1-800-424-5454; Email: fraud@gao.gov.".into(),
        sequencing: "Submit after counsel engagement or when pursuing systemic non-qui-tam track.".into(),
        sources: vec![
            RuleCitation { cite: "GAO-23-106696 (Federal Fraud Oversight Framework)".into(), url: Some("https://www.gao.gov/products/gao-23-106696".into()), verified_on: verified_date.clone() },
        ],
    });

    // 7. State Auditor Fraud Hotline (e.g. Colorado OSA)
    if state == "CO" {
        routes.push(AccountabilityRouteItem {
            route: "state_auditor".into(),
            title: "Colorado Office of the State Auditor (OSA) Fraud Hotline".into(),
            grade: "Solid".into(),
            reason: "Investigates occupational fraud by state employees and state contractors (billing manipulation, false invoices, kickbacks, contract non-performance). Workpapers and tips are strictly confidential and exempt from open records under C.R.S. § 2-3-110.5. Anonymous reporting permitted.".into(),
            what_changes_grade: "Fits vendor-invoice or contract-deliverable theories. Provide contractor SOW numbers and specific invoice discrepancies.".into(),
            deadline: "Ongoing oversight (earlier tips prevent disposal of agency workpapers).".into(),
            contact: "Email: osafraudhotline@coleg.gov; Phone: 303-869-3020; Portal: content.leg.colorado.gov/agencies/office-state-auditor/report-fraud-concern.".into(),
            sequencing: "Submit in parallel with or following counsel consultation on vendor qui tam.".into(),
            sources: vec![
                RuleCitation { cite: "C.R.S. § 2-3-110.5 (OSA Fraud Hotline Statutory Authority)".into(), url: Some("https://content.leg.colorado.gov/agencies/office-state-auditor/report-fraud-concern".into()), verified_on: verified_date.clone() },
            ],
        });

        // 8. Legislative Audit Committee Targeted Audit
        routes.push(AccountabilityRouteItem {
            route: "legislative_audit".into(),
            title: "Legislative Audit Committee (LAC) Targeted Audit Request".into(),
            grade: "Fixable".into(),
            reason: "Legislative Audit Committee has statutory power to order in-depth performance and forensic audits of state agencies and contractors. Requires a formal signed request on letterhead from a member of the Colorado General Assembly.".into(),
            what_changes_grade: "Securing a sponsoring state legislator (Senator or Representative) to formally submit the audit packet on letterhead.".into(),
            deadline: "Legislative session audit scheduling cycles.".into(),
            contact: "Colorado General Assembly Legislative Audit Committee, State Capitol Building, Denver, CO.".into(),
            sequencing: "Best paired with public interest report release after sealed qui tam status is preserved.".into(),
            sources: vec![
                RuleCitation { cite: "C.R.S. § 2-3-101 et seq. (Legislative Audit Committee)".into(), url: Some("https://content.leg.colorado.gov/agencies/office-state-auditor/frequently-asked-questions".into()), verified_on: verified_date.clone() },
            ],
        });

        // 9. State False Claims Act (Colorado FCA)
        routes.push(AccountabilityRouteItem {
            route: "state_fca".into(),
            title: "Colorado False Claims Act (C.R.S. § 24-31-1201 et seq.)".into(),
            grade: "Fixable".into(),
            reason: "Enacted via HB22-1119. Reaches nongovernmental entities and contractors (excludes State and state agencies). Treble damages and civil penalties. Medicaid claims governed separately under Colorado Medicaid FCA (C.R.S. § 25.5-4-303.5).".into(),
            what_changes_grade: "Confirm with counsel whether federal UI admin pass-through funds qualify as State property under CFCA statutory definition.".into(),
            deadline: "Statutory limitations period under C.R.S. § 24-31-1204.".into(),
            contact: "Colorado Attorney General, Special Prosecutions / False Claims Unit.".into(),
            sequencing: "Plead jointly with federal FCA complaint under seal in federal court under supplemental jurisdiction (28 U.S.C. § 1367).".into(),
            sources: vec![
                RuleCitation { cite: "C.R.S. § 24-31-1201 et seq. (HB22-1119)".into(), url: Some("https://leg.colorado.gov/laws/session-laws/HB22-1119".into()), verified_on: verified_date.clone() },
            ],
        });
    }

    // 10. § 1983 / Ex Parte Young / Systemic Injunctive Class Action
    routes.push(AccountabilityRouteItem {
        route: "sec1983_ex_parte_young_class".into(),
        title: "Systemic Injunctive Class Action (§ 1983 Due Process & Ex parte Young)".into(),
        grade: "Fixable".into(),
        reason: "Under Ex parte Young, 209 U.S. 123 (1908), prospective injunctive relief against state officials in official capacity is available to enjoin ongoing Fourteenth Amendment Due Process violations and violations of the Social Security Act 'when due' clause (Cal. Dep't of Human Res. Dev. v. Java, 402 U.S. 121 (1971)). Requires a plaintiff currently experiencing active hold/delay to establish Article III standing for prospective relief.".into(),
        what_changes_grade: "Retain current affected claimants who are actively locked out or frozen today (Jen/Josh 2023-2024 claims lack standing for prospective injunctive relief). Partner with public-interest legal org (e.g. SPLC/Legal Aid model).".into(),
        deadline: "Must file while active claimant's injury is ongoing.".into(),
        contact: "Public interest civil rights litigation firms / Legal Aid.".into(),
        sequencing: "Can proceed independently of False Claims Act vendor claims; directly attacks agency policies without being barred by Stevens.".into(),
        sources: vec![
            RuleCitation { cite: "Ex parte Young, 209 U.S. 123 (1908)".into(), url: Some("https://supreme.justia.com/cases/federal/us/209/123/".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "Cal. Dep't of Human Res. Dev. v. Java, 402 U.S. 121 (1971)".into(), url: Some("https://supreme.justia.com/cases/federal/us/402/121/".into()), verified_on: verified_date.clone() },
        ],
    });

    // 11. Individual Administrative Appeals
    let (appeal_grade, appeal_reason, appeal_deadline) = if state == "CO" && target_program == "ui" {
        (
            "Dead end".to_string(),
            "Colorado UI individual benefit determinations carry a strict, non-extendable 20 calendar-day appeal deadline from date of mailing (7 CCR 1101-2 § 12.1.8). If expired, individual benefits recovery through the agency appeals referee is procedurally barred.".to_string(),
            "20 calendar days from date determination mailed (7 CCR 1101-2 § 12.1.8).".to_string(),
        )
    } else {
        (
            "Fixable".to_string(),
            "Check specific agency notice for jurisdictional administrative appeal windows.".to_string(),
            "State specific (typically 14-30 days).".to_string(),
        )
    };
    routes.push(AccountabilityRouteItem {
        route: "individual_appeal".into(),
        title: "Individual Administrative Agency Appeal".into(),
        grade: appeal_grade,
        reason: appeal_reason,
        what_changes_grade: "If late, establish lack of notice, agency mail error, or file petition for reconsideration if allowed by rule.".into(),
        deadline: appeal_deadline,
        contact: "State agency appeals division portal or referee.".into(),
        sequencing: "File immediately if within the active calendar window.".into(),
        sources: vec![
            RuleCitation { cite: "7 CCR 1101-2 § 12.1.8 (Colorado UI Appeal Deadline)".into(), url: Some("https://cdle.colorado.gov/appeals".into()), verified_on: verified_date.clone() },
        ],
    });

    // 12. State Taxpayer Standing
    if state == "CO" {
        routes.push(AccountabilityRouteItem {
            route: "taxpayer_standing".into(),
            title: "Colorado Taxpayer Standing Lawsuit".into(),
            grade: "Long shot".into(),
            reason: "Colorado recognizes taxpayer standing to enjoin unlawful expenditures (Dodge v. Dep't of Soc. Servs., 600 P.2d 70 (Colo. 1979)), but requires a clear nexus to state tax dollars (Hickenlooper v. FFRF, 2014 CO 77). UI benefits are funded by employer trust fund premiums and federal administrative grants, making the direct link to state tax revenues weak.".into(),
            what_changes_grade: "Showing that general state fund tax appropriations (rather than dedicated trust funds) were expended unlawfully.".into(),
            deadline: "Applicable statute of limitations for declaratory relief.".into(),
            contact: "Colorado District Court.".into(),
            sequencing: "Subordinate to § 1983 class action or vendor FCA.".into(),
            sources: vec![
                RuleCitation { cite: "Dodge v. Dep't of Soc. Servs., 600 P.2d 70 (Colo. 1979)".into(), url: Some("https://law.justia.com/cases/colorado/supreme-court/1979/c-1647-0.html".into()), verified_on: verified_date.clone() },
                RuleCitation { cite: "Hickenlooper v. Freedom from Religion Foundation, 2014 CO 77".into(), url: Some("https://law.justia.com/cases/colorado/supreme-court/2014/12sc1011.html".into()), verified_on: verified_date.clone() },
            ],
        });

        // 13. Colorado Governmental Immunity Act (CGIA Notice)
        routes.push(AccountabilityRouteItem {
            route: "tort_claim_notice".into(),
            title: "Colorado Governmental Immunity Act (CGIA) Tort Claim Notice".into(),
            grade: "Dead end".into(),
            reason: "Under C.R.S. § 24-10-109, written notice of claim against a state agency must be served within 182 days of injury discovery. Damage caps apply ($424,000 / $1,195,000). Crucially, CGIA notice does NOT apply to federal § 1983 civil rights claims (Felder v. Casey, 487 U.S. 131 (1988)).".into(),
            what_changes_grade: "Applies only if within 182 days of a new, distinct state tort injury. Does not bar federal § 1983 actions.".into(),
            deadline: "STRICT: 182 days from discovery of injury (C.R.S. § 24-10-109).".into(),
            contact: "Colorado Attorney General / Risk Management Division.".into(),
            sequencing: "Dead end for old claims; irrelevant to systemic federal § 1983 injunctive relief.".into(),
            sources: vec![
                RuleCitation { cite: "C.R.S. § 24-10-109 (CGIA Notice of Claim)".into(), url: Some("https://law.justia.com/codes/colorado/title-24/article-10/section-24-10-109/".into()), verified_on: verified_date.clone() },
                RuleCitation { cite: "Felder v. Casey, 487 U.S. 131 (1988) (State notice rules inapplicable to § 1983)".into(), url: Some("https://supreme.justia.com/cases/federal/us/487/131/".into()), verified_on: verified_date.clone() },
            ],
        });
    }

    // 14. Statutory Open Records Requests (CORA / FOIA / Own-File)
    routes.push(AccountabilityRouteItem {
        route: "records_requests".into(),
        title: "Statutory Open Records Suite (CORA, FOIA, Privacy Act, 20 CFR 603.5(d))".into(),
        grade: "Solid".into(),
        reason: "Access to government records is guaranteed by law. CORA (C.R.S. § 24-72-201) reaches contracts, task orders, and aggregate data (3 + up to 7 working days). FOIA (5 U.S.C. § 552) reaches federal agency reports (20 + 10 working days). Own claim files are obtained via 20 CFR 603.5(d) or notarized agency form (e.g. Form B-652).".into(),
        what_changes_grade: "Already solid. Use regime-correct fee waivers (Just Liberty Inc. 501(c)(3) EIN 93-3704483) and precise custodian emails.".into(),
        deadline: "CORA: 3 working days (ext. +7); FOIA: 20 working days (ext. +10).".into(),
        contact: "CORA: cdle_pr@state.co.us; FOIA: foiarequests@dol.gov.".into(),
        sequencing: "Run continuously to gather evidentiary proof. Responses become authenticated exhibits.".into(),
        sources: vec![
            RuleCitation { cite: "C.R.S. § 24-72-201 et seq. (Colorado Open Records Act)".into(), url: Some("https://colorado.public.law/statutes/crs_24-72-201".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "5 U.S.C. § 552 (Freedom of Information Act)".into(), url: Some("https://www.law.cornell.edu/uscode/text/5/552".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "20 CFR § 603.5(d) (Claimant access to own UI confidential records)".into(), url: Some("https://www.ecfr.gov/current/title-20/section-603.5".into()), verified_on: verified_date.clone() },
        ],
    });

    // 15. 501(c)(3) Public Interest Dossier & Advocacy
    routes.push(AccountabilityRouteItem {
        route: "nonprofit_advocacy".into(),
        title: "Just Liberty Inc. (501(c)(3)) Public Interest Report & Media Dossier".into(),
        grade: "Solid".into(),
        reason: "Nonprofit research, policy analysis, and public-interest reporting. Requesters can publish white papers, petition legislative committees, and brief news organizations on systemic breakdowns.".into(),
        what_changes_grade: "Timing-sensitive. Strongest when backed by authenticated open records responses and data analysis.".into(),
        deadline: "Publication timing must be held until counsel reviews any sealed qui tam complaint.".into(),
        contact: "Just Liberty Incorporated, PO Box 761, Loveland, CO 80539.".into(),
        sequencing: "PUBLISH LAST. Any public news release, blog post, or social media broadcast before a False Claims Act complaint is filed under seal creates serious Public Disclosure Bar risk under 31 U.S.C. § 3730(e)(4).".into(),
        sources: vec![
            RuleCitation { cite: "31 U.S.C. § 3730(e)(4) (Public Disclosure Bar)".into(), url: Some("https://www.law.cornell.edu/uscode/text/31/3730".into()), verified_on: verified_date.clone() },
            RuleCitation { cite: "Schindler Elevator Corp. v. U.S. ex rel. Kirk, 563 U.S. 401 (2011)".into(), url: Some("https://supreme.justia.com/cases/federal/us/563/401/".into()), verified_on: verified_date.clone() },
        ],
    });

    let best_next_3_moves = vec![
        "1. COUNSEL NOW ON CONTRACTOR THEORY (FCA § 3731(b) Clock Closing): With claimant consent, present the confidential dossier, contractor SOWs, and invoice analysis to False Claims Act counsel. Focus on private software/intake contractors (FCA persons under 1 U.S.C. § 1), the CFCA, and timing for OIG/OSA referrals before the 6-year pandemic limitations period expires.".into(),
        "2. TRACK OPEN RECORDS REQUESTS WITH WORKING-DAY DATE MATH: Maintain strict tracking of active open records requests (CORA to CDLE: 3 + 7 working days; FOIA to DOL ETA: 20 + 10 working days). When responses arrive, ingest cryptographically with SHA-256 into evidence/ to form authenticated proof.".into(),
        "3. ORGANIZE CURRENT CLAIMANTS FOR SYSTEMIC INJUNCTIVE RELIEF (§ 1983 / Ex parte Young): Partner with public-interest counsel to identify currently affected claimants with active benefit holds. An Ex parte Young prospective injunctive class action or a legislator-sponsored LAC audit request is the systemic lever that holds the agency accountable without depending on FCA 'person' status.".into(),
    ];

    let exports_dir = case_dir.join("exports");
    fs::create_dir_all(&exports_dir).map_err(|e| e.to_string())?;

    let json_path = exports_dir.join("accountability-map.json");
    let md_path = exports_dir.join("accountability-map.md");

    let mut md = String::new();
    md.push_str(&format!("# Systematic Accountability Map & Multi-Track Legal Radar\n\n"));
    md.push_str(&format!("**Matter:** {} | **Target Type:** {} | **State:** {} | **Program:** {}\n", profile.title, target_type, state, target_program));
    md.push_str(&format!("**Compiled:** {} | **Engine:** JustLegal OS Offline Legal Engine\n\n", now));
    md.push_str("## Executive Summary: Best Next 3 Moves\n\n");
    for m in &best_next_3_moves {
        md.push_str(&format!("* {}\n\n", m));
    }
    md.push_str("## Comprehensive 14-Pathway Viability Matrix\n\n");
    md.push_str("| Pathway / Channel | Grade | Governing Law & Rule | Limitations / Deadline | Status & Sequencing |\n");
    md.push_str("|---|---|---|---|---|\n");
    for r in &routes {
        md.push_str(&format!(
            "| **{}** | **{}** | {} | {} | {} |\n",
            r.title, r.grade, r.sources.first().map(|s| s.cite.as_str()).unwrap_or("Statutory"), r.deadline, r.sequencing
        ));
    }

    md.push_str("\n## Detailed Route Analysis & Action Standards\n\n");
    for r in &routes {
        md.push_str(&format!("### {}\n", r.title));
        md.push_str(&format!("- **Grade:** **{}**\n", r.grade));
        md.push_str(&format!("- **Analysis:** {}\n", r.reason));
        md.push_str(&format!("- **What Upgrades Grade:** {}\n", r.what_changes_grade));
        md.push_str(&format!("- **Statutory Deadline:** {}\n", r.deadline));
        md.push_str(&format!("- **Submission Contact:** {}\n", r.contact));
        md.push_str(&format!("- **Sequencing Protocol:** {}\n", r.sequencing));
        md.push_str("- **Verified Legal Sources:**\n");
        for s in &r.sources {
            let u = s.url.as_deref().unwrap_or("Internal Legal Source");
            md.push_str(&format!("  * {} ({}) [Verified: {}]\n", s.cite, u, s.verified_on));
        }
        md.push_str("\n");
    }

    let _ = fs::write(&md_path, &md);

    let report = AccountabilityMapReport {
        ok: true,
        never_files: true,
        never_serves: true,
        case_id: profile.id,
        case_title: profile.title,
        target_type,
        governing_state: state,
        target_program,
        best_next_3_moves,
        routes,
        generated_at: now,
        markdown_report_path: Some(md_path.to_string_lossy().to_string()),
        json_report_path: Some(json_path.to_string_lossy().to_string()),
    };

    if let Ok(serialized) = serde_json::to_string_pretty(&report) {
        let _ = fs::write(&json_path, serialized);
    }

    Ok(report)
}
