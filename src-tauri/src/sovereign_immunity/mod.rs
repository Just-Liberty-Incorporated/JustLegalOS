pub mod types;
pub use types::*;

pub fn audit_sovereign_immunity(
    state: &str,
    target_type: &str,
    claims: &[String],
    accepts_federal_funds: bool,
) -> SovereignImmunityReport {
    let t_lower = target_type.to_lowercase();
    let is_agency = t_lower.contains("agency") || t_lower == "gov-agency";
    let is_contractor = t_lower.contains("contractor") || t_lower == "gov-contractor";
    let is_individual = t_lower.contains("individual") || t_lower == "official-individual";
    let is_local_gov = t_lower.contains("county") || t_lower.contains("city") || t_lower == "local-gov";

    let mut pathways = Vec::new();

    // 1. Ex parte Young Pathway
    pathways.push(ImmunityBypassPathway {
        doctrine_name: "Ex parte Young Doctrine (Prospective Reform)".into(),
        citation: "Ex parte Young, 209 U.S. 123 (1908)".into(),
        relief_type: "Prospective Injunction & Declaratory Relief".into(),
        applicable_target: "State Agency Executive Director in Official Capacity".into(),
        explanation: "The Eleventh Amendment does not bar federal suits against state officers in their official capacities to enjoin ongoing violations of federal constitutional law or federal statutes (e.g. Social Security Act § 303(a)(1) 'when due' requirement).".into(),
    });

    // 2. ADA Title II Valid Abrogation
    pathways.push(ImmunityBypassPathway {
        doctrine_name: "ADA Title II Fourteenth Amendment Abrogation".into(),
        citation: "42 U.S.C. § 12132; United States v. Georgia, 546 U.S. 151 (2006); Tennessee v. Lane, 541 U.S. 509 (2004)".into(),
        relief_type: "Money Damages AND Injunctive Relief".into(),
        applicable_target: "State Agency Directly".into(),
        explanation: "Congress validly abrogated State sovereign immunity under § 5 of the Fourteenth Amendment for ADA Title II violations that independently implicate constitutional due process, equal protection, or fundamental access to public administration and courts.".into(),
    });

    // 3. Rehabilitation Act § 504 Spending Clause Waiver
    if accepts_federal_funds {
        pathways.push(ImmunityBypassPathway {
            doctrine_name: "Rehabilitation Act § 504 Spending Clause Waiver".into(),
            citation: "29 U.S.C. § 794; 42 U.S.C. § 2000d-7".into(),
            relief_type: "Compensatory Money Damages & Injunctive Relief".into(),
            applicable_target: "State Department Receiving Federal Grants (e.g. CDLE / ALN 17.225)".into(),
            explanation: "Under 42 U.S.C. § 2000d-7, a State waives its Eleventh Amendment immunity to suits in federal court for violations of § 504 of the Rehabilitation Act as an unambiguous condition of accepting federal financial assistance.".into(),
        });
    }

    // 4. County / Municipal Liability (Monell)
    if is_local_gov {
        pathways.push(ImmunityBypassPathway {
            doctrine_name: "Monell Municipal / County Non-Immunity".into(),
            citation: "Monell v. Dept. of Social Services, 436 U.S. 658 (1978); Cook County v. Chandler, 538 U.S. 119 (2003)".into(),
            relief_type: "Full Compensatory Damages & FCA Treble Damages".into(),
            applicable_target: "Counties, Cities, Municipal Entities".into(),
            explanation: "Counties and municipalities are NOT arms of the State and have ZERO Eleventh Amendment sovereign immunity. They are subject to both § 1983 liability and False Claims Act qui tam liability.".into(),
        });
    }

    // 5. Private Contractor Liability (Zero Sovereign Immunity)
    if is_contractor {
        pathways.push(ImmunityBypassPathway {
            doctrine_name: "Private Government Contractor Non-Immunity".into(),
            citation: "31 U.S.C. § 3729; Yearsley v. W.A. Ross Construction Co., 309 U.S. 18 (1940) (Derivative Immunity Strictly Limited)".into(),
            relief_type: "Treble Damages, Civil Penalties, Relator Bounty (15–30%), Common Law Tort".into(),
            applicable_target: "Private Tech/Intake Vendors (e.g. Deloitte, Accenture, CGI)".into(),
            explanation: "Private corporations performing government contracts enjoy no sovereign immunity. Derivative Yearsley immunity applies ONLY if the contractor strictly adhered to valid government specifications without defect or deceit.".into(),
        });
    }

    let general_rule = if is_agency {
        "Under Will v. Michigan Dept. of State Police, 491 U.S. 58 (1989) and Vermont ANR v. Stevens, 529 U.S. 765 (2000), a state agency is immune from retrospective treasury money damages under § 1983 and cannot be an FCA qui tam defendant. Retrospective damages must pivot to ADA Title II abrogation, Section 504 waiver, or private contractors.".to_string()
    } else if is_contractor {
        "Private contractors are FCA 'persons' (1 U.S.C. § 1; 31 U.S.C. § 3729(b)(2)(A)(ii)) with NO sovereign immunity. Full monetary treble damages and qui tam relator bounties apply.".to_string()
    } else if is_individual {
        "Officials in official capacity are shielded from damages under the Eleventh Amendment, but vulnerable to prospective injunctions under Ex parte Young. In individual capacity, damages are available under § 1983 / § 1985 subject to qualified immunity.".to_string()
    } else {
        "Local government entities (counties, cities) have no Eleventh Amendment immunity under Monell.".to_string()
    };

    let retrospective_damages_barred = is_agency;

    let mut evaluated_claims = Vec::new();
    for c in claims {
        let cl_lower = c.to_lowercase();
        if cl_lower.contains("fca") || cl_lower.contains("qui tam") {
            if is_agency {
                evaluated_claims.push(ClaimImmunityResult {
                    claim_name: c.clone(),
                    viable: false,
                    legal_basis: "Barred against State agency directly by Vermont ANR v. Stevens, 529 U.S. 765 (2000).".into(),
                    recommended_defendant: "PIVOT: Name the Private Prime Contractor (e.g. Deloitte) under 31 U.S.C. § 3729(b)(2)(A)(ii).".into(),
                });
            } else {
                evaluated_claims.push(ClaimImmunityResult {
                    claim_name: c.clone(),
                    viable: true,
                    legal_basis: "Fully viable against private contractor under 31 U.S.C. § 3729.".into(),
                    recommended_defendant: "Private Contractor Corporation".into(),
                });
            }
        } else if cl_lower.contains("ada") || cl_lower.contains("disability") || cl_lower.contains("rehab") {
            evaluated_claims.push(ClaimImmunityResult {
                claim_name: c.clone(),
                viable: true,
                legal_basis: "Viable for damages and injunction: Eleventh Amendment immunity is validly abrogated (United States v. Georgia, 546 U.S. 151) and waived via federal funding (29 U.S.C. § 794).".into(),
                recommended_defendant: "State Department or Agency Directly".into(),
            });
        } else if cl_lower.contains("injunction") || cl_lower.contains("due process") || cl_lower.contains("reform") {
            evaluated_claims.push(ClaimImmunityResult {
                claim_name: c.clone(),
                viable: true,
                legal_basis: "Viable under Ex parte Young, 209 U.S. 123 (1908) for prospective declaratory and injunctive relief.".into(),
                recommended_defendant: "Agency Executive Director in Official Capacity".into(),
            });
        } else {
            evaluated_claims.push(ClaimImmunityResult {
                claim_name: c.clone(),
                viable: !is_agency,
                legal_basis: if is_agency {
                    "Retrospective general tort/contract damages against the State treasury are barred by the Eleventh Amendment.".into()
                } else {
                    "Common law or statutory claims against private actors are not barred by sovereign immunity.".into()
                },
                recommended_defendant: if is_agency { "Pivot to Official Capacity (Injunction) or Contractor (Damages)".into() } else { "Private Entity".into() },
            });
        }
    }

    let overall_recommendation = if is_agency {
        format!(
            "To hold the State of {} accountable without being dismissed on sovereign immunity: (1) Seek systemic prospective reform by naming the Executive Director in official capacity under Ex parte Young; (2) Pursue monetary damages directly against the agency under ADA Title II (42 U.S.C. § 12132 / United States v. Georgia) or Rehab Act § 504; and (3) Seek treble monetary recovery against the private tech/billing vendor under the False Claims Act.",
            state
        )
    } else {
        "Defendant has no sovereign immunity. Proceed directly against corporate entity under False Claims Act, Civil RICO, or common law.".to_string()
    };

    SovereignImmunityReport {
        state: state.to_string(),
        target_type: target_type.to_string(),
        general_rule,
        retrospective_damages_barred,
        active_bypass_pathways: pathways,
        claims_evaluated: evaluated_claims,
        overall_recommendation,
    }
}
