//! CMS NPPES v2.1 Provider Registry client and Entity Verification against USAspending under EO 14395.

use chrono::Utc;
use reqwest::blocking::Client;
use serde_json::Value;

use super::types::*;
use crate::usaspending;

pub const NPPES_API_URL: &str = "https://npiregistry.cms.hhs.gov/api/?version=2.1";

fn query_nppes_api(client: &Client, params: &[(&str, &str)]) -> Result<Vec<NppesEntity>, String> {
    let response = client
        .get(NPPES_API_URL)
        .query(params)
        .send()
        .map_err(|e| format!("NPPES API request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("NPPES returned HTTP status {}", response.status()));
    }

    let body_text = response
        .text()
        .map_err(|e| format!("Failed to read NPPES response: {e}"))?;

    let root: Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse NPPES JSON: {e}"))?;

    let results = root.get("results").and_then(|r| r.as_array());
    let mut entities = Vec::new();

    if let Some(items) = results {
        for item in items {
            if let Some(parsed) = parse_nppes_item(item) {
                entities.push(parsed);
            }
        }
    }

    Ok(entities)
}

/// Search CMS NPPES registry by NPI number or by organization/provider name.
pub fn lookup_nppes(query: &str, state: Option<&str>) -> Result<Vec<NppesEntity>, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

    let trimmed = query.trim();
    let is_numeric_npi = trimmed.len() == 10 && trimmed.chars().all(|c| c.is_ascii_digit());

    if is_numeric_npi {
        return query_nppes_api(&client, &[("number", trimmed)]);
    }

    let clean_state = state.map(|s| s.trim()).filter(|s| !s.is_empty());
    let st_upper = clean_state.map(|s| s.to_uppercase());

    // Strategy 1: Exact organization_name search
    let mut params = vec![("organization_name", trimmed), ("limit", "10")];
    if let Some(ref st) = st_upper {
        params.push(("state", st.as_str()));
    }
    let results = query_nppes_api(&client, &params)?;
    if !results.is_empty() {
        return Ok(results);
    }

    // Strategy 2: If no results and doesn't already contain '*', try wildcard organization search
    if !trimmed.contains('*') {
        let wildcard_query = format!("{}*", trimmed);
        let mut wc_params = vec![("organization_name", wildcard_query.as_str()), ("limit", "10")];
        if let Some(ref st) = st_upper {
            wc_params.push(("state", st.as_str()));
        }
        if let Ok(wc_results) = query_nppes_api(&client, &wc_params) {
            if !wc_results.is_empty() {
                return Ok(wc_results);
            }
        }
    }

    // Strategy 3: If still no results, try individual provider search (first_name / last_name)
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() >= 2 {
        let first_name = parts[0];
        let last_name = parts[1..].join(" ");
        let mut ind_params = vec![
            ("first_name", first_name),
            ("last_name", last_name.as_str()),
            ("limit", "10"),
        ];
        if let Some(ref st) = st_upper {
            ind_params.push(("state", st.as_str()));
        }
        if let Ok(ind_results) = query_nppes_api(&client, &ind_params) {
            if !ind_results.is_empty() {
                return Ok(ind_results);
            }
        }
    }

    Ok(results)
}

fn parse_nppes_item(item: &Value) -> Option<NppesEntity> {
    let npi = match item.get("number")? {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return None,
    };
    let entity_type = item.get("enumeration_type")?.as_str().unwrap_or("NPI-2").to_string();

    let basic = item.get("basic");
    let organization_name = basic.and_then(|b| b.get("organization_name")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let first_name = basic.and_then(|b| b.get("first_name")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let last_name = basic.and_then(|b| b.get("last_name")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let enumeration_date = basic.and_then(|b| b.get("enumeration_date")).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let last_updated = basic.and_then(|b| b.get("last_updated")).and_then(|v| v.as_str()).unwrap_or("").to_string();

    let mut practice_location = NppesAddress::default();
    let mut mailing_address = NppesAddress::default();

    if let Some(addresses) = item.get("addresses").and_then(|a| a.as_array()) {
        for addr in addresses {
            let purpose = addr.get("address_purpose").and_then(|v| v.as_str()).unwrap_or("");
            let parsed_addr = NppesAddress {
                purpose: purpose.to_string(),
                address_1: addr.get("address_1").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                address_2: addr.get("address_2").and_then(|v| v.as_str()).map(|s| s.to_string()),
                city: addr.get("city").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                state: addr.get("state").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                postal_code: addr.get("postal_code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                phone: addr.get("telephone_number").and_then(|v| v.as_str()).map(|s| s.to_string()),
            };

            if purpose == "LOCATION" {
                practice_location = parsed_addr;
            } else if purpose == "MAILING" {
                mailing_address = parsed_addr;
            }
        }
    }

    let mut taxonomies = Vec::new();
    if let Some(tax_array) = item.get("taxonomies").and_then(|t| t.as_array()) {
        for t in tax_array {
            taxonomies.push(NppesTaxonomy {
                code: t.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                desc: t.get("desc").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                primary: t.get("primary").and_then(|v| v.as_bool()).unwrap_or(false),
                state: t.get("state").and_then(|v| v.as_str()).map(|s| s.to_string()),
                license: t.get("license").and_then(|v| v.as_str()).map(|s| s.to_string()),
            });
        }
    }

    let mut authorized_official = None;
    if let Some(b) = basic {
        if let (Some(fn_val), Some(ln_val)) = (b.get("authorized_official_first_name"), b.get("authorized_official_last_name")) {
            authorized_official = Some(NppesOfficial {
                first_name: fn_val.as_str().unwrap_or("").to_string(),
                last_name: ln_val.as_str().unwrap_or("").to_string(),
                title: b.get("authorized_official_title_or_position").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                phone: b.get("authorized_official_telephone_number").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            });
        }
    }

    Some(NppesEntity {
        npi,
        entity_type,
        organization_name,
        first_name,
        last_name,
        enumeration_date,
        last_updated,
        practice_location,
        mailing_address,
        taxonomies,
        authorized_official,
    })
}

/// Verifies an entity across CMS NPPES (Tier 1) and USAspending (Tier 2), providing Tier 3 guidance.
pub fn verify_entity(query: &str, state: Option<&str>) -> Result<EntityVerificationReport, String> {
    let now = Utc::now().to_rfc3339();
    let nppes_res = lookup_nppes(query, state);
    let (nppes_results, lookup_status, mut red_flags) = match nppes_res {
        Ok(results) => {
            if results.is_empty() {
                let mut rf = Vec::new();
                rf.push("Tier 1 Registry Notice: No active CMS NPI registration found matching search criteria.".into());
                (results, "VERIFIED_NO_MATCH".to_string(), rf)
            } else {
                (results, "VERIFIED_MATCH".to_string(), Vec::new())
            }
        }
        Err(e) => {
            let mut rf = Vec::new();
            rf.push(format!("Tier 1 Registry Warning: CMS NPPES API lookup offline or unreachable ({}). Provider registration status could not be verified online.", e));
            (Vec::new(), "LOOKUP_FAILED_OR_OFFLINE".to_string(), rf)
        }
    };
    let primary_entity = nppes_results.into_iter().next();

    // Query USAspending using entity name or query
    let target_name = if let Some(ref ent) = primary_entity {
        ent.organization_name
            .clone()
            .or_else(|| {
                match (&ent.first_name, &ent.last_name) {
                    (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
                    _ => None,
                }
            })
            .unwrap_or_else(|| query.to_string())
    } else {
        query.to_string()
    };

    let awards = usaspending::check_target(&target_name).unwrap_or_default();
    let total_obligations: f64 = awards.iter().map(|a| a.total_obligation).sum();

    if let Some(ref ent) = primary_entity {
        // Red flag 1: Telehealth / dispatch taxonomy vs physical expectation
        for tax in &ent.taxonomies {
            let desc_lower = tax.desc.to_lowercase();
            if desc_lower.contains("dispatch") || desc_lower.contains("telehealth") || desc_lower.contains("transport") {
                red_flags.push(format!(
                    "Taxonomy Warning: Entity registered as '{}' (Code {}), which signifies remote/dispatch rather than dedicated physical facility.",
                    tax.desc, tax.code
                ));
            }
        }

        // Red flag 2: PO Box or Commercial Mail Receiving Agency as physical practice location
        let practice_lower = ent.practice_location.address_1.to_lowercase();
        if practice_lower.contains("po box") || practice_lower.contains("p.o. box") || practice_lower.contains("suite pmbe") {
            red_flags.push("Address Warning: Physical practice location is registered as a Post Office Box or CMRA.".into());
        }

        // Red flag 3: Discrepancy between practice location state and registered tax state
        if !ent.practice_location.state.is_empty() {
            for tax in &ent.taxonomies {
                if let Some(ref tax_state) = tax.state {
                    if !tax_state.is_empty() && tax_state != &ent.practice_location.state {
                        red_flags.push(format!(
                            "Jurisdiction Discrepancy: Physical location is in {}, but license taxonomy is in {}.",
                            ent.practice_location.state, tax_state
                        ));
                    }
                }
            }
        }
    }

    if awards.is_empty() {
        red_flags.push("Tier 2 Notice: No direct federal contracts, assistance awards, or prime loans identified in USAspending under this exact entity name.".into());
    }

    let tier_guidance = vec![
        "Tier 1 (CMS NPPES Provider Registry): Direct federal NPI enumeration and taxonomy records. Confirms whether provider/entity is an active CMS-registered healthcare organization (NPI-2) or individual (NPI-1).".to_string(),
        "Tier 2 (USAspending Direct Awards): Direct federal procurement contracts, prime grants, and direct loans under Federal Award ID / UEI. Captures direct Department of Health & Human Services (HHS), VA, and DoD funding.".to_string(),
        "Tier 3 (State-Administered Medicaid Claims Guidance): State Medicaid expenditures and managed care organization (MCO) disbursements flow through state health agencies rather than direct USAspending awards. For state Medicaid fraud verification, obtain state agency claims data via state public records/FOIA or subpoena.".to_string(),
    ];

    Ok(EntityVerificationReport {
        target_query: query.to_string(),
        npi_matched: primary_entity.as_ref().map(|e| e.npi.clone()),
        nppes_entity: primary_entity,
        lookup_status,
        total_federal_obligations: total_obligations,
        usaspending_count: awards.len(),
        awards_summary: awards,
        red_flags,
        tier_guidance,
        verified_at: now,
    })
}
