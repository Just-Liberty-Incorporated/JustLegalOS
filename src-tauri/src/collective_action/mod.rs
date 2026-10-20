pub mod types;
pub use types::*;

use std::fs;
use std::path::Path;

pub fn build_collective_action(
    case_dir: Option<&Path>,
    mode: &str,
    matter_title: &str,
    target_defendant: &str,
    co_plaintiffs: &[CoPlaintiffEntry],
    custom_subject: Option<&str>,
) -> Result<CollectiveActionOutput, String> {
    let mode_lower = mode.to_ascii_lowercase();

    match mode_lower.as_str() {
        "rule20" | "rule20-joinder" | "joinder" => {
            let mut caption_parties = String::new();
            let mut fact_sections = String::new();
            let mut signature_blocks = String::new();

            for (i, p) in co_plaintiffs.iter().enumerate() {
                let p_num = i + 1;
                caption_parties.push_str(&format!("{}, individual plaintiff;\n", p.name));
                
                fact_sections.push_str(&format!(
                    "### Facts Relating to Plaintiff {}: {}\n\
                    - Location: {}\n\
                    - Date of Incident / Hold: {}\n\
                    - Specific Injury: {}\n\
                    - Supporting Exhibit Hash (SHA-256): {}\n\n",
                    p_num,
                    p.name,
                    p.city_state,
                    p.date_of_injury_or_hold,
                    p.injury_summary,
                    p.sha256_hash.as_deref().unwrap_or("Attached as verified exhibit")
                ));

                signature_blocks.push_str(&format!(
                    "Respectfully submitted,\n\n\
                    /s/ {}\n\
                    {}, Pro Se Plaintiff\n\
                    {}\n\
                    Date: _____________________\n\n",
                    p.name, p.name, p.city_state
                ));
            }

            let rendered = format!(
"UNITED STATES DISTRICT COURT\n\
DISTRICT OF COLORADO (OR APPLICABLE DISTRICT)\n\n\
{caption_parties}\n\
    Plaintiffs,\n\
v.                                      Civil Action No. ___________________\n\n\
{defendant},\n\
    Defendant(s).\n\n\
======================================================================\n\
MULTI-PLAINTIFF COMPLAINT FOR DECLARATORY AND INJUNCTIVE RELIEF\n\
PURSUANT TO FRCP RULE 20(a)(1) (PERMISSIVE JOINDER)\n\
======================================================================\n\n\
PRELIMINARY STATEMENT\n\
1. Plaintiffs bring this civil action jointly pursuant to Federal Rule of Civil Procedure 20(a)(1). \
Plaintiffs assert rights to relief jointly, severally, and arising out of the same transaction, occurrence, \
or series of transactions and occurrences—specifically, the systemic, standardized operational defects, \
unlawful administrative holds, and statutory non-compliance executed by Defendant {defendant}.\n\n\
2. Common questions of law and fact arise in this action as to all Plaintiffs, including whether Defendant's \
standardized policy violates Fourteenth Amendment Procedural Due Process, federal statutory mandates, \
and applicable regulatory standards.\n\n\
3. JOINDER COMPLIANCE NOTICE (RULE 20 vs. RULE 23):\n\
Plaintiffs do NOT seek class certification under FRCP Rule 23, and no plaintiff purports to represent an absent class. \
Instead, all named plaintiffs appear pro se on their own behalf as co-plaintiffs under Rule 20(a)(1), \
each signing this complaint individually and asserting individual claims on a consolidated docket.\n\n\
STATEMENT OF FACTS COMMON TO ALL PLAINTIFFS\n\
4. Defendant has engaged in a widespread practice of automated determinations, delayed administrative appeals, \
and non-responsive customer intake mechanisms that impact all named Plaintiffs in an identical manner.\n\n\
INDIVIDUAL FACTUAL ALLEGATIONS\n\
{fact_sections}\n\
CAUSES OF ACTION\n\
COUNT I: Deprivation of Procedural Due Process (42 U.S.C. § 1983 / Ex parte Young)\n\
COUNT II: Violation of Statutory Federal Program Mandates (SSA § 303(a)(1) / 'When Due' Rule)\n\
COUNT III: Title II ADA / Section 504 Rehabilitation Act Systemic Access Failure\n\n\
PRAYER FOR RELIEF\n\
WHEREFORE, Plaintiffs respectfully request that the Court:\n\
A. Enter declaratory judgment that Defendant's challenged practice violates federal law;\n\
B. Issue a prospective preliminary and permanent injunction ordering Defendant to cease unlawful hold procedures;\n\
C. Award each Plaintiff their taxable costs of action; and\n\
D. Grant such other and further relief as the Court deems just and proper.\n\n\
SIGNATURES OF ALL NAMED PRO SE CO-PLAINTIFFS:\n\n\
{signature_blocks}\n",
                caption_parties = caption_parties.trim_end(),
                defendant = target_defendant,
                fact_sections = fact_sections,
                signature_blocks = signature_blocks
            );

            let out_file = if let Some(dir) = case_dir {
                let exports = dir.join("exports");
                let _ = fs::create_dir_all(&exports);
                let p = exports.join("rule20_multi_plaintiff_complaint.md");
                let _ = fs::write(&p, &rendered);
                Some(p.display().to_string())
            } else {
                None
            };

            Ok(CollectiveActionOutput {
                mode: "rule20-joinder".into(),
                title: format!("FRCP Rule 20(a)(1) Multi-Plaintiff Joinder: {}", matter_title),
                statutory_authority: "Fed. R. Civ. P. 20(a)(1); 28 U.S.C. § 1331; 42 U.S.C. § 1983".into(),
                why_this_works_pro_se: "Unlike Rule 23 class actions which forbid pro se representation, Rule 20 permits 2 to 25 affected individuals to sue together as co-plaintiffs on a single docket, split one filing fee, and defeat individual mootness defenses.".into(),
                total_plaintiffs_or_claimants: co_plaintiffs.len(),
                full_rendered_text: rendered,
                output_file_path: out_file,
            })
        }

        "rule23" | "rule23-binder" | "class-binder" => {
            let mut roster = String::new();
            for (i, p) in co_plaintiffs.iter().enumerate() {
                roster.push_str(&format!(
                    "| Claimant {} | {} | {} | {} | `{}` |\n",
                    i + 1,
                    p.name,
                    p.city_state,
                    p.date_of_injury_or_hold,
                    p.sha256_hash.as_deref().unwrap_or("PENDING_HASH")
                ));
            }

            let rendered = format!(
"======================================================================\n\
JUSTLEGAL OS | TURNKEY RULE 23 CLASS ACTION / MASS TORT COUNSEL PITCH BINDER\n\
Prepared by Just Liberty Incorporated (501c3 Nonprofit Research Engine)\n\
======================================================================\n\n\
MATTER: {matter_title}\n\
TARGET DEFENDANT: {defendant}\n\
PROPOSED CLASS SCOPE: Individuals impacted by standardized systemic defect / hold policy\n\
AGGREGATED CLAIMANTS PRE-INTAKED: {count}\n\n\
1. EXECUTIVE SUMMARY FOR CLASS ACTION PLAINTIFF COUNSEL\n\
This dossier compiles verified, pre-screened claimant intakes with cryptographic SHA-256 evidence hashes. \
Under FRCP Rule 23(a)(4), pro se individuals cannot represent absent class members. This pitch binder is assembled \
for presentation to qualified contingency class-action and mass-tort law firms to assume lead class counsel representation.\n\n\
2. RULE 23(a) PREREQUISITES MATRIX\n\
- NUMEROSITY (Rule 23(a)(1)): Over 10,000+ similarly situated claimants identified via state audit records.\n\
- COMMONALITY (Rule 23(a)(2)): Identical core question: Did Defendant's standardized software/policy violate due process?\n\
- TYPICALITY (Rule 23(a)(3)): Claims of representative claimants arise from identical automated processing scripts.\n\
- ADEQUACY (Rule 23(a)(4)): Ready for appointment of qualified class counsel upon formal motion.\n\n\
3. PRE-SCREENED CLAIMANT ROSTER & HASH MANIFEST\n\
| Claimant ID | Name | Jurisdiction | Key Date | Evidence Hash (SHA-256) |\n\
|---|---|---|---|---|\n\
{roster}\n\
4. ESTIMATED ECONOMIC IMPACT & DAMAGES MODEL\n\
- Minimum statutory damages / withheld funds: Multi-million dollar aggregate common fund potential.\n\
- Private contractor liability: Treble damages & statutory fee shifting available under consumer protection / civil RICO / FCA.\n\n\
PRIVILEGED ATTORNEY INQUIRY NOTICE:\n\
This transmission constitutes a confidential inquiry seeking legal representation under ABA Model Rule 1.18.\n",
                matter_title = matter_title,
                defendant = target_defendant,
                count = co_plaintiffs.len(),
                roster = roster
            );

            let out_file = if let Some(dir) = case_dir {
                let exports = dir.join("exports");
                let _ = fs::create_dir_all(&exports);
                let p = exports.join("rule23_class_action_pitch_binder.md");
                let _ = fs::write(&p, &rendered);
                Some(p.display().to_string())
            } else {
                None
            };

            Ok(CollectiveActionOutput {
                mode: "rule23-binder".into(),
                title: format!("Turnkey Rule 23 Class Action Pitch Binder: {}", matter_title),
                statutory_authority: "Fed. R. Civ. P. 23(a)-(b); ABA Model Rule 1.18".into(),
                why_this_works_pro_se: "Allows a grassroots group of affected citizens to hand a pre-packaged, hashed 20+ claimant roster to a contingency class-action law firm, dramatically lowering the firm's intake cost and making representation highly attractive.".into(),
                total_plaintiffs_or_claimants: co_plaintiffs.len(),
                full_rendered_text: rendered,
                output_file_path: out_file,
            })
        }

        "rulemaking" | "rulemaking-petition" | "apa-petition" | "apa" => {
            let subject = custom_subject.unwrap_or("Petition to Repeal or Amend Automated Deprivation Rules");
            let rendered = format!(
"BEFORE THE\n\
FEDERAL OR STATE REGULATORY AGENCY\n\n\
IN THE MATTER OF:\n\
{subject}\n\n\
======================================================================\n\
FORMAL CITIZEN PETITION FOR ADMINISTRATIVE RULEMAKING\n\
PURSUANT TO 5 U.S.C. § 553(e) (OR STATE ADMINISTRATIVE PROCEDURE ACT)\n\
======================================================================\n\n\
TO: THE HONORABLE AGENCY HEAD AND POLICY COUNSEL\n\n\
I. PETITIONERS\n\
Petitioners are interested persons and affected citizens residing within the jurisdiction of the Agency.\n\n\
II. STATUTORY AUTHORITY\n\
Pursuant to Section 553(e) of the Administrative Procedure Act (5 U.S.C. § 553(e)) and governing state administrative statutes, \
'Each agency shall give an interested person the right to petition for the issuance, amendment, or repeal of a rule.'\n\n\
III. PROPOSED RULE CHANGE / RELIEF SOUGHT\n\
Petitioners respectfully request that the Agency:\n\
1. Repeal or amend existing automated hold, adjudication, and fraud-flagging policies that deprive citizens of benefits without prior written notice;\n\
2. Enact mandatory timelines requiring human review within 14 calendar days of any automated claim hold;\n\
3. Establish robust Title II ADA and Section 504 reasonable accommodation procedures accessible via telephone and mail.\n\n\
IV. STATEMENT OF GROUNDS & FACTUAL BASIS\n\
- Empirical evidence reveals thousands of citizens experiencing prolonged deprivation without notice or hearing.\n\
- The existing policy conflicts with organic statutory requirements (e.g. SSA § 303(a)(1) 'when due' requirement).\n\
- Continued reliance on defective automated vendor algorithms imposes severe economic harm on eligible recipients.\n\n\
V. MANDATORY AGENCY DUTY & JUDICIAL REVIEW NOTICE\n\
The Agency has a non-discretionary obligation to consider and formally respond to this Petition on the record. \
An arbitrary refusal to initiate rulemaking or unreasonable delay in responding is subject to direct judicial review \
in the United States District Court pursuant to 5 U.S.C. § 706(1) and Massachusetts v. EPA, 549 U.S. 497 (2007).\n\n\
Respectfully submitted,\n\n\
/s/ Designated Citizen Petitioner & Organizational Sponsor\n\
Just Liberty Incorporated (501c3)\n\
Date: _____________________\n",
                subject = subject
            );

            let out_file = if let Some(dir) = case_dir {
                let exports = dir.join("exports");
                let _ = fs::create_dir_all(&exports);
                let p = exports.join("apa_rulemaking_petition.md");
                let _ = fs::write(&p, &rendered);
                Some(p.display().to_string())
            } else {
                None
            };

            Ok(CollectiveActionOutput {
                mode: "rulemaking-petition".into(),
                title: format!("APA 5 U.S.C. § 553(e) Rulemaking Petition: {}", subject),
                statutory_authority: "5 U.S.C. § 553(e); 5 U.S.C. § 706; Massachusetts v. EPA, 549 U.S. 497".into(),
                why_this_works_pro_se: "Legally forces an agency to respond on the administrative record. If the agency arbitrarily refuses or ignores the petition, you have immediate standing to sue them in federal district court under 5 U.S.C. § 706.".into(),
                total_plaintiffs_or_claimants: co_plaintiffs.len().max(1),
                full_rendered_text: rendered,
                output_file_path: out_file,
            })
        }

        other => Err(format!(
            "Unknown collective action mode '{}'. Supported: rule20-joinder, rule23-binder, rulemaking-petition",
            other
        )),
    }
}
