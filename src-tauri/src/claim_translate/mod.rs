pub mod types;
pub use types::*;

pub fn translate_vernacular(raw_query: &str) -> ClaimTranslation {
    let q = raw_query.to_lowercase();

    if q.contains("treason") || q.contains("sedition") || q.contains("traitor") || q.contains("betray") {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Allegation of Betrayal of Public Trust / Misuse of Public Office".into(),
            why_vernacular_fails_in_court: "Under U.S. Const. Art. III, § 3 and 18 U.S.C. § 2381, Treason strictly requires levying open war against the United States or adhering to an officially declared foreign enemy during wartime. Further, private citizens cannot prosecute federal criminal statutes (Linda R.S. v. Richard D., 410 U.S. 614 (1973)). Calling corrupt officials 'traitors' in a civil complaint triggers immediate Rule 12(f) strike or Rule 11 sanctions.".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "Theft or Bribery in Programs Receiving Federal Funds".into(),
                    statutory_citation: "18 U.S.C. § 666".into(),
                    target_entity_type: "Individual Officials & Bribery Payors".into(),
                    relief_available: "Criminal referral to DOJ Public Integrity Section (PIN)".into(),
                    governing_standard: "Theft, embezzlement, or corruptly soliciting/giving anything of value ($5,000+) in an entity receiving $10,000+ in federal assistance.".into(),
                },
                LegalCauseOfAction {
                    title: "Honest Services Wire Fraud".into(),
                    statutory_citation: "18 U.S.C. §§ 1343, 1346; Skilling v. United States, 561 U.S. 358".into(),
                    target_entity_type: "Public Officials & Contractors in Bribery/Kickback Schemes".into(),
                    relief_available: "Criminal referral & Predicate Act for Civil RICO".into(),
                    governing_standard: "Depriving the public of the intangible right of honest services through classic bribery or kickbacks.".into(),
                },
                LegalCauseOfAction {
                    title: "Civil Rights Conspiracy to Deter Witnesses or Obstruct Justice".into(),
                    statutory_citation: "42 U.S.C. § 1985(2)–(3)".into(),
                    target_entity_type: "Conspiring Officials in Individual Capacity".into(),
                    relief_available: "Civil damages against individuals (No sovereign immunity)".into(),
                    governing_standard: "Two or more persons conspiring to deter a party or witness from attending court, or depriving equal protection.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Direct economic or physical injury to your property, employment, or rights caused by the corrupt action (not general citizen grievance).".into(),
                element_y_enterprise_or_hook: "Federal assistance funding nexus ($10,000+ annual grant ALN) or interstate wire communications facilitating the scheme.".into(),
                element_z_mountain_of_evidence: vec![
                    "CORA/FOIA personnel files, conflict of interest disclosures, and state ethics commission reports.".into(),
                    "Electronic communications, calendar appointments, or transaction ledgers showing the exchange of value for official acts.".into(),
                    "Specific program grant funding agreements connecting federal funds to the agency unit involved.".into(),
                ],
                required_records_requests: vec![
                    "Public records request to agency for travel vouchers, outside employment approvals, and conflict statements.".into(),
                    "FOIA request to federal grantor agency (e.g. DOL-ETA) for federal grant compliance audits.".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO CRIMINAL REFERRAL: Package verified financial records into an 18 U.S.C. § 666 / § 1346 sworn evidentiary packet to DOJ Public Integrity Section (PIN) and FBI Public Corruption squad.".into(),
                "PIVOT TO CIVIL RICO: Use the bribery or honest services wire fraud as predicate racketeering acts under 18 U.S.C. § 1962(c) against private contractors and corrupt individuals.".into(),
                "PIVOT TO SECTION 1983: Sue the individual officials under 42 U.S.C. § 1983 for Fourteenth Amendment Due Process violations in individual capacity.".into(),
            ],
        }
    } else if q.contains("rico") || q.contains("racket") || q.contains("revolving") || q.contains("kickback") || q.contains("bribe") {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Allegation of Contractor-Agency Racket, Kickbacks, or Revolving-Door Corruption".into(),
            why_vernacular_fails_in_court: "A municipal or state government agency itself cannot form criminal mens rea to be a RICO defendant (Lancaster Cmty. Hosp. v. Antelope Valley Hosp. Dist., 940 F.2d 397 (9th Cir. 1991)). If you sue the agency under RICO, it will be dismissed on Day 1. However, the agency CAN serve as the 'passive enterprise' through which private contractors and corrupt individuals conduct racketeering (Cedric Kushner Promotions, Ltd. v. King, 533 U.S. 158 (2001)).".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "Civil RICO Treble Damages Against Private Vendor & Corrupt Officials".into(),
                    statutory_citation: "18 U.S.C. §§ 1962(c), 1964(c)".into(),
                    target_entity_type: "Private Contractor (e.g. Deloitte) + Named Corrupt Officials".into(),
                    relief_available: "Treble (3x) compensatory damages + mandatory attorney fees".into(),
                    governing_standard: "Conduct of an enterprise through a pattern (2+ predicate acts within 10 years) of racketeering activity proximately injuring business or property.".into(),
                },
                LegalCauseOfAction {
                    title: "Anti-Kickback Act of 1986 (FCA Integration)".into(),
                    statutory_citation: "41 U.S.C. §§ 8701–8707; 31 U.S.C. § 3729(a)(1)(B)".into(),
                    target_entity_type: "Prime Contractor & Subcontractors".into(),
                    relief_available: "FCA treble damages, civil penalties, and relator bounty (15–30%)".into(),
                    governing_standard: "Providing, attempting to provide, or soliciting anything of value to prime contractor or public official to reward favorable procurement treatment.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Concrete financial injury to your specific business, employment, or property proximately caused by the predicate acts (Holmes v. SIPC, 503 U.S. 258). Personal distress or general tax loss is insufficient.".into(),
                element_y_enterprise_or_hook: "Distinct legal existence between the 'person' conducting racketeering (Deloitte / corrupt actors) and the 'enterprise' (the state agency or association-in-fact) under Cedric Kushner.".into(),
                element_z_mountain_of_evidence: vec![
                    "Documented pattern of at least 2 predicate acts of mail/wire fraud (18 U.S.C. §§ 1341/1343) or federal bribery (18 U.S.C. § 201 / state equivalents).".into(),
                    "Revolving-door employment records (LinkedIn profiles, state procurement officer departures, vendor hiring notices, subcontractor awards).".into(),
                    "Authentication of specific invoices, inflated change orders, or non-delivery certificates approved by compromised officials.".into(),
                ],
                required_records_requests: vec![
                    "CORA/FOIA request for all task orders, contract amendments, and vendor subcontractor disclosures.".into(),
                    "State ethics filings and post-employment revolving-door cooling-off period compliance records.".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO VENDOR AS RICO DEFENDANT: Name the private IT/procurement corporation as the primary defendant, designating the state agency as the passive enterprise under Cedric Kushner.".into(),
                "PIVOT TO FALSE CLAIMS ACT: Link the kickback/revolving-door arrangement to false compliance certifications under 31 U.S.C. § 3729(b)(2)(A)(ii), qualifying for a 15–30% whistleblower bounty under seal.".into(),
            ],
        }
    } else if q.contains("reform") || q.contains("unconstitutional") || q.contains("abuse") || q.contains("delay") || q.contains("hold") || q.contains("due process") {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Systemic Agency Practice Reform / Procedural Due Process / Disability Access".into(),
            why_vernacular_fails_in_court: "Seeking money damages directly from the State agency for unconstitutional practices is barred by the Eleventh Amendment (Will v. Michigan, 491 U.S. 58). Furthermore, if your individual benefit hold was resolved or terminated, the State will move to dismiss your lawsuit as moot under City of Los Angeles v. Lyons, 461 U.S. 95 (1983).".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "Prospective Injunctive & Declaratory Relief (Ex parte Young Doctrine)".into(),
                    statutory_citation: "Ex parte Young, 209 U.S. 123 (1908); 42 U.S.C. § 1983".into(),
                    target_entity_type: "Agency Executive Director in Official Capacity".into(),
                    relief_available: "Binding federal injunction ordering agency to cease practice and overhaul holds".into(),
                    governing_standard: "Ongoing violation of federal constitutional due process or statutory 'when due' standards (SSA § 303(a)(1); Cal. Dep't of Human Res. Dev. v. Java, 402 U.S. 121 (1971)).".into(),
                },
                LegalCauseOfAction {
                    title: "ADA Title II Sovereign Immunity Abrogation (Money Damages + Injunction)".into(),
                    statutory_citation: "42 U.S.C. § 12132; United States v. Georgia, 546 U.S. 151 (2006)".into(),
                    target_entity_type: "State Agency Directly".into(),
                    relief_available: "Compensatory money damages AND affirmative injunctive accommodation".into(),
                    governing_standard: "Congress validly abrogated Eleventh Amendment immunity for Title II violations that independently violate the Fourteenth Amendment (e.g. denial of due process hearing or equal court access).".into(),
                },
                LegalCauseOfAction {
                    title: "Rehabilitation Act § 504 Spending Clause Waiver".into(),
                    statutory_citation: "29 U.S.C. § 794; 42 U.S.C. § 2000d-7".into(),
                    target_entity_type: "State Agency Receiving Federal Assistance (e.g. ALN 17.225)".into(),
                    relief_available: "Compensatory damages + injunction".into(),
                    governing_standard: "Receipt of federal assistance acts as a non-discretionary waiver of state sovereign immunity against disability discrimination claims.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Active or ongoing hold, or proof that the policy creates a credible threat of future injury; or joinder of currently held co-plaintiffs under FRCP Rule 20 to defeat mootness.".into(),
                element_y_enterprise_or_hook: "Fourteenth Amendment Procedural Due Process under Mathews v. Eldridge, 424 U.S. 319 (private interest + risk of erroneous deprivation + minimal state burden to provide timely hearing).".into(),
                element_z_mountain_of_evidence: vec![
                    "State Auditor (OSA) reports showing aggregate hold volumes (e.g. 196,066 unresolved holds) proving custom/policy, not rogue clerk.".into(),
                    "Personal timeline detailing submission dates, notice deficiencies, unfulfilled hearing requests, and lack of pre-deprivation process.".into(),
                    "Disability accommodation requests submitted, ignored, or denied by agency technology/staff.".into(),
                ],
                required_records_requests: vec![
                    "Claimant complete claim file under 20 CFR § 603.5(d) (Form B-652) including internal examiner notes and audit logs.".into(),
                    "CORA/FOIA request for agency hold policy memos, fraud-score algorithm documentation, and appeal backlog statistics.".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO EX PARTE YOUNG: Sue the Executive Director in official capacity for forward-looking injunctive relief to reform the unconstitutional hold process.".into(),
                "PIVOT TO ADA TITLE II: If disability accommodations were denied, sue the State agency directly for money damages under United States v. Georgia.".into(),
                "PIVOT TO RULE 20 JOINDER: Join 1–3 currently frozen co-claimants to eradicate any mootness defense by the State.".into(),
            ],
        }
    } else if q.contains("class action") || q.contains("neighbor") || q.contains("sick") || q.contains("everyone") || q.contains("group") || q.contains("contamination") {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Collective Redress / Multi-Plaintiff Group Harm / Class Action".into(),
            why_vernacular_fails_in_court: "Under FRCP Rule 23(a)(4), federal courts strictly prohibit pro se litigants from acting as class representatives or representing absent class members (Oxendine v. Williams, 509 F.2d 1405 (4th Cir. 1975)). Writing 'Class Action' on a pro se complaint causes the class claims to be stricken immediately.".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "Multi-Plaintiff Permissive Joinder (Pro Se Lawful Group Action)".into(),
                    statutory_citation: "FRCP Rule 20(a)(1)".into(),
                    target_entity_type: "Common Defendant (Agency / Vendor / Polluter)".into(),
                    relief_available: "Individual judgments for all named co-plaintiffs on a single docket".into(),
                    governing_standard: "Multiple persons may join in one action as plaintiffs if they assert rights arising out of the same transaction or occurrence, and common questions of law or fact will arise.".into(),
                },
                LegalCauseOfAction {
                    title: "Turnkey Rule 23 Class Action Pitch Binder (For Retained Contingency Counsel)".into(),
                    statutory_citation: "FRCP Rule 23(a), (b)(3)".into(),
                    target_entity_type: "Corporate Vendor or State Defendant (Injunctive)".into(),
                    relief_available: "Class-wide damages or injunctive relief handled by appointed Class Counsel".into(),
                    governing_standard: "Numerosity, commonality, typicality, and adequacy of representation satisfied by qualified retained class counsel.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Individual proof of injury for every named co-plaintiff (each co-plaintiff must sign the complaint and prove their own damages).".into(),
                element_y_enterprise_or_hook: "Single common transaction or series of occurrences (e.g. the same IT software deployment, the same toxic discharge event, the same automated hold script).".into(),
                element_z_mountain_of_evidence: vec![
                    "Roster of affected co-plaintiffs with cryptographic SHA-256 intake logs of their individual claim/injury documents.".into(),
                    "Technical root-cause evidence showing the common defect or contamination source that impacted all plaintiffs identically.".into(),
                    "Common legal question brief demonstrating why joint trial conserves judicial resources under Rule 20.".into(),
                ],
                required_records_requests: vec![
                    "Environmental testing reports, discharge permits, or agency-wide IT bug logs obtained via open records.".into(),
                    "Aggregated statistical disclosures proving the widespread scope of the incident.".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO RULE 20 MULTI-PLAINTIFF JOINDER: File together pro se as Named Co-Plaintiffs. Each person signs their own signature block; you split a single $405 court fee and defeat mootness together without violating Rule 23.".into(),
                "PIVOT TO CLASS COUNSEL BINDER: Package 20+ verified claimant intakes into a pre-vetted Turnkey Rule 23 Pitch Binder to hand to a contingency class-action firm.".into(),
            ],
        }
    } else if q.contains("constitution") || q.contains("amend") || q.contains("politician") || q.contains("petition") || q.contains("ballot") {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Direct Legislative Reform / APA Rulemaking Petition / Ballot Initiative".into(),
            why_vernacular_fails_in_court: "Judges cannot amend the Constitution or pass new legislation from the bench. Alleging in a court brief that 'the law should be changed' results in immediate dismissal for lack of justiciable case or controversy under Article III.".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "Federal APA Petition for Rulemaking (Enforceable Administrative Challenge)".into(),
                    statutory_citation: "5 U.S.C. § 553(e); 5 U.S.C. § 706".into(),
                    target_entity_type: "Federal Regulatory Agency (e.g. DOL, HHS, EPA)".into(),
                    relief_available: "Mandatory response on the record; judicial review to overturn arbitrary denials".into(),
                    governing_standard: "Every agency shall give an interested person the right to petition for the issuance, amendment, or repeal of a rule. Denial is reviewable under arbitrary & capricious standard (Mass. v. EPA, 549 U.S. 497).".into(),
                },
                LegalCauseOfAction {
                    title: "State Citizen Ballot Initiative / Constitutional Referendum".into(),
                    statutory_citation: "State Constitution (e.g. Colo. Const. Art. V, § 1)".into(),
                    target_entity_type: "Direct Citizen Electorate".into(),
                    relief_available: "Enactment of binding statutory or state constitutional law bypassing legislature".into(),
                    governing_standard: "Drafting initiative text, Title Board review, signature gathering from registered voters, and statewide ballot vote.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Interested person status under 5 U.S.C. § 553(e) or registered elector status in your state.".into(),
                element_y_enterprise_or_hook: "Statutory gap or agency rule that contradicts underlying organic statute or constitutional due process.".into(),
                element_z_mountain_of_evidence: vec![
                    "Empirical study and open records audit demonstrating regulatory failure and public harm under the current rule.".into(),
                    "Drafted replacement rule or statutory language with legislative drafting compliance.".into(),
                    "Economic impact analysis showing feasibility and reduced taxpayer waste.".into(),
                ],
                required_records_requests: vec![
                    "Agency regulatory impact statements and prior notice-and-comment rulemaking dockets.".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO APA RULEMAKING PETITION: File a formal 5 U.S.C. § 553(e) petition with the agency. If they refuse or ignore it, haul them into federal court under 5 U.S.C. § 706.".into(),
                "PIVOT TO STATE BALLOT INITIATIVE: Use state constitutional initiative mechanics to put binding reform directly on the ballot for the people to vote on.".into(),
            ],
        }
    } else {
        ClaimTranslation {
            raw_query: raw_query.to_string(),
            primary_match: "Direct False Claims / Government Mismanagement".into(),
            why_vernacular_fails_in_court: "Suing a government agency directly for fraud or damages hits sovereign immunity (Vermont ANR v. Stevens; Will v. Michigan). Pro se filings often fail because they sue the agency rather than the private contractor or responsible official.".into(),
            legal_causes_of_action: vec![
                LegalCauseOfAction {
                    title: "False Claims Act Against Private Contractor".into(),
                    statutory_citation: "31 U.S.C. § 3729(b)(2)(A)(ii)".into(),
                    target_entity_type: "Private IT/Services Contractor".into(),
                    relief_available: "Treble damages + 15–30% whistleblower bounty under seal".into(),
                    governing_standard: "Causing false records or claims to be presented under federal grant or program funds.".into(),
                },
                LegalCauseOfAction {
                    title: "Ex parte Young Prospective Injunction Against Agency Head".into(),
                    statutory_citation: "Ex parte Young, 209 U.S. 123 (1908)".into(),
                    target_entity_type: "State Executive Official in Official Capacity".into(),
                    relief_available: "Prospective injunction to halt unconstitutional policy or practice".into(),
                    governing_standard: "Ongoing federal constitutional or statutory violation by state officer.".into(),
                },
            ],
            mountain_of_proof: MountainOfProof {
                element_x_standing: "Direct individualized concrete injury caused by the challenged practice.".into(),
                element_y_enterprise_or_hook: "Federal program funding nexus (ALN) or Fourteenth Amendment Due Process clause.".into(),
                element_z_mountain_of_evidence: vec![
                    "Contractor invoices, deliverables, or audit findings showing non-performance or falsity.".into(),
                    "Personal claimant notices showing lack of timely due process.".into(),
                ],
                required_records_requests: vec![
                    "CORA/FOIA request for prime contracts, SOWs, and billing vouchers.".into(),
                    "Claimant own-file request under 20 CFR § 603.5(d).".into(),
                ],
            },
            legal_pivots: vec![
                "PIVOT TO CONTRACTOR: Aim monetary qui tam claims at the private vendor, not the immune state agency.".into(),
                "PIVOT TO PROSPECTIVE INJUNCTION: Aim systemic reform claims at the Agency Director in official capacity under Ex parte Young.".into(),
            ],
        }
    }
}
