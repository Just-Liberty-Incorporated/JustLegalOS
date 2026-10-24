#!/usr/bin/env python3
import json
import re
import os

def clean_text(s):
    if not isinstance(s, str):
        return s
    # Remove markdown link wrappers [text](url) -> url or text
    s = re.sub(r'\[(https?://[^\]]+)\]\((https?://[^\)]+)\)', r'\1', s)
    s = re.sub(r'\[([^\]]+)\]\((https?://[^\)]+)\)', r'\2', s)
    # Clean parenthetical notes from research prompts
    s = re.sub(r'\(NJDHS form;\s*check[^)]+\)', '(NJDHS DMAHS Form)', s, flags=re.IGNORECASE)
    s = re.sub(r'\([^)]*check\s+DMAHS[^)]*\)', '(DMAHS Division Form)', s, flags=re.IGNORECASE)
    s = s.replace('(Check DMAHS)', '(DMAHS Division Form)')
    return s.strip()

def deep_clean(obj):
    if isinstance(obj, str):
        return clean_text(obj)
    elif isinstance(obj, list):
        return [deep_clean(x) for x in obj]
    elif isinstance(obj, dict):
        return {k: deep_clean(v) for k, v in obj.items()}
    return obj

def main():
    batch9_path = 'C:/Users/Deus/Desktop/JustLegal OS GrokBot/DataForFraud/batch 9 dc and final.json'
    old_states_path = 'src-tauri/data/states_50.json'
    output_path = 'src-tauri/data/states_50.json'
    
    with open(batch9_path, encoding='utf-8') as f:
        grok_data = json.load(f)
        
    with open(old_states_path, encoding='utf-8') as f:
        old_data = json.load(f)
        
    # Old data might be a dict with 'states' or a list
    if isinstance(old_data, list):
        old_by_code = {s['state_code']: s for s in old_data if 'state_code' in s}
    elif isinstance(old_data, dict):
        old_by_code = {s['state_code']: s for s in old_data.get('states', []) if 'state_code' in s}
    else:
        old_by_code = {}
        
    grok_states = grok_data.get('states', [])
    
    merged_states = []
    
    for s in grok_states:
        code = s.get('state_code')
        old = old_by_code.get(code, {})
        
        # 1. Clean entire state object
        s = deep_clean(s)
        
        s1 = s.setdefault('stage_1_personal_records_discovery', {})
        
        # --- Medicaid Refinements ---
        med = s1.setdefault('medicaid_claims_and_capitation', {})
        if not med.get('mailing_address') and old.get('medicaid_and_hipaa', {}).get('mailing_address'):
            med['mailing_address'] = old['medicaid_and_hipaa']['mailing_address']
        if not med.get('custodian_office'):
            med['custodian_office'] = old.get('medicaid_and_hipaa', {}).get('hipaa_records_custodian', f"{s.get('state_name')} Medicaid Privacy Officer")
        if code == 'NJ':
            med['mailing_address'] = "P.O. Box 712, Trenton, NJ 08625-0712"
            med['custodian_office'] = "DMAHS Office of Legal and Regulatory Affairs — HIPAA Privacy Officer"
            med['phone'] = "609-588-2600"
            med['fax'] = "609-588-3583"
            
        # --- Unemployment Refinements ---
        ui = s1.setdefault('unemployment_claim_file', {})
        old_ui = old.get('unemployment_workforce', {})
        
        if not ui.get('agency_name') and old_ui.get('agency_name'):
            ui['agency_name'] = old_ui['agency_name']
        if not ui.get('mailing_address') and old_ui.get('custodian_contact'):
            ui['mailing_address'] = old_ui['custodian_contact']
        if not ui.get('custodian_office'):
            ui['custodian_office'] = f"{ui.get('agency_name', 'State Workforce Agency')} — UI Custodian of Records"
            
        if not ui.get('governing_law'):
            ui['governing_law'] = "20 C.F.R. § 603.5(c) (Individual Claimant Disclosure Mandate)"
            
        if not ui.get('accepted_delivery_channels'):
            ui['accepted_delivery_channels'] = ["online_portal", "postal_mail"]
            
        # Rejection traps for UI
        if not ui.get('rejection_traps_and_requirements'):
            traps = [
                "Third-party open-records/FOIA requests for claimant files are rejected as confidential; request must cite 20 C.F.R. § 603.5(c) as the individual claimant.",
                "Must include full legal name, SSN, current address, and signature verifying identity under penalty of perjury (28 U.S.C. § 1746).",
                "Explicitly itemize: (1) Monetary/non-monetary determinations, (2) Adjudicator fact-finding and hold notes, (3) Call logs and recorded hearing audio, and (4) Interstate wage-transfer (IB-4/ICON) records."
            ]
            if code == 'CO':
                traps.insert(0, "Colorado CDLE strictly requires Form B-652 with a notary seal; blanket 'all records' requests are rejected.")
            elif code == 'NC':
                traps.insert(0, "North Carolina DES requires Form DESLEGAL 100 with a $15 certified check or money order; leave staff witness line blank.")
            elif code == 'AL':
                traps.insert(0, "Alabama Department of Labor requires Form 480 (Individual Confidential Information Request).")
            ui['rejection_traps_and_requirements'] = traps
            
        # Forms for UI
        if not ui.get('official_request_form') or not ui['official_request_form'].get('form_number_or_name'):
            if code == 'CO':
                ui['official_request_form'] = {
                    "has_specific_form": True,
                    "form_number_or_name": "Form B-652 (Claimant's Waiver and Authorization to Release Information)",
                    "form_url_or_portal": "https://cdle.colorado.gov/unemployment"
                }
            elif code == 'NC':
                ui['official_request_form'] = {
                    "has_specific_form": True,
                    "form_number_or_name": "Form DESLEGAL 100 (Request for Claimant Records)",
                    "form_url_or_portal": "https://www.des.nc.gov/"
                }
            elif code == 'AL':
                ui['official_request_form'] = {
                    "has_specific_form": True,
                    "form_number_or_name": "Form 480 (Individual Confidential Information Request)",
                    "form_url_or_portal": "https://labor.alabama.gov/"
                }
            elif code == 'NJ':
                ui['official_request_form'] = {
                    "has_specific_form": True,
                    "form_number_or_name": "NJDOL Claimant Records Request",
                    "form_url_or_portal": "https://www.nj.gov/labor/myunemployment/"
                }
            else:
                ui['official_request_form'] = {
                    "has_specific_form": False,
                    "form_number_or_name": f"{ui.get('agency_name')} Claimant Records Request Form or Written 20 CFR § 603.5(c) Demand",
                    "form_url_or_portal": old_ui.get('public_records_url', "https://www.dol.gov/")
                }
                
        # --- Stage 2 Public Verification Databases Refinements ---
        s2_db = s.setdefault('stage_2_public_verification_databases', {})
        if code == 'TX':
            s2_db['childcare_daycare_licensing_lookup_url'] = "https://www.dfps.texas.gov/Child_Care/Search_Colorado_Child_Care/"
        elif code == 'NJ':
            s2_db['health_facility_hospice_homehealth_licensing_url'] = "https://healthapps.state.nj.us/facilities/fsSearch.aspx"
            s2_db['childcare_daycare_licensing_lookup_url'] = "https://childcareexplorer.njccis.com/"
            s2_db['state_checkbook_and_contracts_url'] = "https://www.yourmoney.nj.gov/"
            
        # --- Stage 4 Enforcement Receivers Refinements ---
        s4 = s.setdefault('stage_4_fraud_enforcement_receivers', {})
        s4['timing_firewall_rule'] = "LOCKED UNTIL STAGE 1 RECORDS ARE RECEIVED AND STAGE 3 CONTRADICTION MATRIX IS VERIFIED."
        s4['federal_whip_portal'] = "https://www.america.gov/fraud"
        
        # Backfill auditor / comptroller fraud hotline if missing or empty
        old_recv = old.get('fraud_receivers', {})
        old_auditor = old_recv.get('state_auditor_or_ig_hotline', {})
        
        if not s4.get('state_comptroller_or_auditor_fraud_hotline') and old_auditor:
            s4['state_comptroller_or_auditor_fraud_hotline'] = {
                "name": old_auditor.get('name', f"{s.get('state_name')} State Auditor Fraud Hotline"),
                "phone": old_auditor.get('phone', "N/A"),
                "web_portal": old_auditor.get('url', "https://www.oversight.gov/")
            }
            
        # Explicit NJ Comptroller fix
        if code == 'NJ':
            s4['state_comptroller_or_auditor_fraud_hotline'] = {
                "name": "New Jersey Office of the State Comptroller (OSC) — Medicaid Fraud Division",
                "email": "MedicaidFraud@osc.nj.gov",
                "phone": "1-855-OSC-TIPS (1-855-672-8477)",
                "web_portal": "https://www.nj.gov/comptroller/report/"
            }
            s4['state_medicaid_fraud_control_unit_mfcu'] = {
                "name": "New Jersey Attorney General — Medicaid Fraud Control Unit (OIFP)",
                "phone": "609-376-2484",
                "url": "https://www.njoag.gov/about/divisions-and-offices/office-of-the-insurance-fraud-prosecutor-home/medicaid-fraud-control-unit/"
            }

        # --- Dual Mapping for Full Backwards Compatibility ---
        s_traps = s.get('stage_2_state_traps_and_litigation_rules', {})
        s['recording_and_observation_traps'] = {
            "consent_type": s_traps.get('recording_consent_type', 'one_party'),
            "statute_citation": s_traps.get('recording_statute', f"{code} Wiretap Act"),
            "exceptions_or_nuances": s_traps.get('recording_nuances', "Standard statutory rules apply.")
        }
        
        s['sovereign_immunity_and_tort_notice'] = {
            "tort_claims_act_cite": s_traps.get('tort_claims_act_cite', f"{code} Tort Claims Act"),
            "notice_of_claim_deadline_days": s_traps.get('tort_notice_deadline_days', 180),
            "unsworn_declaration_rule": s_traps.get('court_unsworn_certification_rule', "28 U.S.C. § 1746 declaration in lieu of oath")
        }
        
        sunshine = s.get('stage_2_open_records_sunshine_law', {})
        s['open_records_law'] = {
            "statute_name": sunshine.get('statute_name', f"{code} Open Records Law"),
            "citation": sunshine.get('citation', f"{code} Code"),
            "response_days": sunshine.get('response_days', 10),
            "day_type": sunshine.get('day_type', "working_days"),
            "appeal_body": sunshine.get('appeal_body', "State Court"),
            "official_url": s.get('stage_2_public_verification_databases', {}).get('state_checkbook_and_contracts_url', "")
        }
        
        s['medicaid_and_hipaa'] = {
            "agency_name": med.get('agency_name', f"{s.get('state_name')} Medicaid"),
            "program_brand_name": med.get('program_brand_name'),
            "hipaa_records_custodian": med.get('custodian_office'),
            "mailing_address": med.get('mailing_address'),
            "portal_or_contact_url": med.get('official_request_form', {}).get('form_url_or_portal'),
            "federal_floor_cites": ["45 C.F.R. § 164.524", "42 C.F.R. § 431.60"]
        }
        
        s['unemployment_workforce'] = {
            "agency_name": ui.get('agency_name', f"{s.get('state_name')} Unemployment Agency"),
            "own_claim_file_route": "20 C.F.R. § 603.5(c) Individual Claimant Disclosure Mandate",
            "public_records_url": ui.get('official_request_form', {}).get('form_url_or_portal', ""),
            "custodian_contact": ui.get('mailing_address', "")
        }
        
        fca = s.get('stage_3_sovereign_immunity_and_fca_viability', {}).get('state_false_claims_act', {})
        s['state_false_claims_act'] = {
            "coverage_type": fca.get('coverage_type', "none"),
            "statute_name": fca.get('statute_name'),
            "citation": fca.get('citation'),
            "relator_share_intervened": fca.get('relator_share_intervened'),
            "relator_share_declined": fca.get('relator_share_declined'),
            "notes": None
        }
        
        s['fraud_receivers'] = {
            "medicaid_fraud_control_unit_mfcu": {
                "name": s4.get('state_medicaid_fraud_control_unit_mfcu', {}).get('name'),
                "url": s4.get('state_medicaid_fraud_control_unit_mfcu', {}).get('url'),
                "phone": s4.get('state_medicaid_fraud_control_unit_mfcu', {}).get('phone')
            },
            "state_auditor_or_ig_hotline": {
                "name": s4.get('state_comptroller_or_auditor_fraud_hotline', {}).get('name'),
                "url": s4.get('state_comptroller_or_auditor_fraud_hotline', {}).get('web_portal'),
                "phone": s4.get('state_comptroller_or_auditor_fraud_hotline', {}).get('phone')
            }
        }
        
        # Operational enrichment with zero fraud emails in verified_emails
        form_name = med.get('official_request_form', {}).get('form_number_or_name')
        verified_forms = [form_name] if form_name else []
        ui_form = ui.get('official_request_form', {}).get('form_number_or_name')
        if ui_form:
            verified_forms.append(ui_form)
            
        s['operational_enrichment'] = {
            "narrative_summary": f"Standardized 6-stage discovery and verification dataset for {s.get('state_name')}.",
            "verified_emails": [],  # Strictly empty so finder never grabs a fraud email for Stage 1!
            "verified_forms": verified_forms,
            "verified_faxes": [med['fax']] if med.get('fax') else []
        }
            
        merged_states.append(s)
        
    # Sort alphabetically by state_code
    merged_states.sort(key=lambda x: x.get('state_code', ''))
    
    # Save output as both list and wrapped dict support
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(merged_states, f, indent=2, ensure_ascii=False)
        
    print(f"Successfully merged {len(merged_states)} jurisdictions into {output_path} (dual compatibility enabled)")

if __name__ == '__main__':
    main()
