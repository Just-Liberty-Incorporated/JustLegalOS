# Skill: 50-State & Federal Statutory Routing Matrix

## Purpose
Provides deterministic, primary-source routing data for all 50 U.S. states, the District of Columbia, and the Federal Government.
Eliminates guesswork regarding Open Records response times, Medicaid HIPAA record access, UI claim-file access under 20 C.F.R. § 603.5(c), state False Claims Act relator standing, state MFCU reporting contacts, and state wiretap recording consent rules.

---

## 1. Federal Floor Architecture

Whenever a self-represented citizen requests their **own** records, federal regulations establish a mandatory national standard that overrides state public records exemptions:
- **Medicaid Claims & Capitation History:** 45 C.F.R. § 164.524 (HIPAA Right of Access — 30 calendar days) & 42 C.F.R. § 431.60 (CMS Patient Access Interoperability).
- **Unemployment (UI) Claimant Files:** 20 C.F.R. § 603.5(c) (Confidential disclosure to individual to whom records pertain).
- **SNAP / TANF Case Records:** 7 C.F.R. § 273.15(p)(1) & 45 C.F.R. § 205.10(a)(13)(i) (Casefile examination rights, typically administered at the county/local level).

---

## 2. 50-State + D.C. Quick Routing Table

| State | Circuit | Open Records Law & Clock | Medicaid / HIPAA Custodian | UI Agency & Route | State FCA | Wiretap Consent |
|---|---|---|---|---|---|---|
| **AL** (Alabama) | 11th Cir. | Alabama Open Records Act / Public Records Law (15 days) | Alabama Medicaid Agency | Alabama Dept. Workforce | `none` | `one_party` |
| **AK** (Alaska) | 9th Cir. | Alaska Public Records Act (10 days) | Alaska Dept. Health | Alaska Dept. Labor and Workforce Development | `none` | `one_party` |
| **AZ** (Arizona) | 9th Cir. | Arizona Public Records Law (Prompt) | Arizona Health Care Cost Containment System (AHCCCS) | Arizona Dept. Economic Security | `none` | `one_party` |
| **AR** (Arkansas) | 8th Cir. | Arkansas Freedom of Information Act (3 days) | Arkansas Dept. Human Services | Arkansas Div. Workforce Services | `none` | `one_party` |
| **CA** (California) | 9th Cir. | California Public Records Act (CPRA) (10 days) | California Dept. Health Care Services (DHCS) | California Employment Development Department (EDD) | `general` | `all_party` |
| **CO** (Colorado) | 10th Cir. | Colorado Open Records Act (CORA) (3 days) | Colorado Dept. Health Care Policy and Financing (HCPF) | Colorado Dept. Labor and Employment | `general` | `one_party` |
| **CT** (Connecticut) | 2nd Cir. | Connecticut Freedom of Information Act (4 days) | Connecticut Dept. Social Services | Connecticut Dept. Labor | `general` | `mixed` |
| **DE** (Delaware) | 3rd Cir. | Delaware Freedom of Information Act (15 days) | Delaware Dept. Health and Social Services | Delaware Dept. Labor | `general` | `mixed` |
| **FL** (Florida) | 11th Cir. | Florida Public Records Act (Prompt) | Florida Agency for Health Care Administration (AHCA) | Florida Dept. Commerce | `general` | `all_party` |
| **GA** (Georgia) | 11th Cir. | Georgia Open Records Act (3 days) | Georgia Dept. Community Health | Georgia Dept. Labor | `general` | `one_party` |
| **HI** (Hawaii) | 9th Cir. | Uniform Information Practices Act (Modified) (10 days) | Hawaii Dept. Human Services | Hawaii Dept. Labor and Industrial Relations | `general` | `one_party` |
| **ID** (Idaho) | 9th Cir. | Idaho Public Records Act (10 days) | Idaho Dept. Health and Welfare | Idaho Dept. Labor | `none` | `one_party` |
| **IL** (Illinois) | 7th Cir. | Illinois Freedom of Information Act (5 days) | Illinois Dept. Healthcare and Family Services (HFS) | Illinois Dept. Employment Security (IDES) | `general` | `all_party` |
| **IN** (Indiana) | 7th Cir. | Indiana Access to Public Records Act (7 days) | Indiana Family and Social Services Administration | Indiana Dept. Workforce Development | `general` | `one_party` |
| **IA** (Iowa) | 8th Cir. | Iowa Open Records Law (Prompt) | Iowa Dept. Health and Human Services | Iowa Workforce Development | `general` | `one_party` |
| **KS** (Kansas) | 10th Cir. | Kansas Open Records Act (KORA) (3 days) | Kansas Dept. Health and Environment | Kansas Dept. Labor | `none` | `one_party` |
| **KY** (Kentucky) | 6th Cir. | Kentucky Open Records Act (5 days) | Kentucky Cabinet for Health and Family Services | Kentucky Education and Labor Cabinet | `none` | `one_party` |
| **LA** (Louisiana) | 5th Cir. | Louisiana Public Records Law (5 days) | Louisiana Dept. Health | Louisiana Workforce Commission | `medicaid_only` | `one_party` |
| **ME** (Maine) | 1st Cir. | Maine Freedom of Access Act (5 days) | Maine Dept. Health and Human Services | Maine Dept. Labor | `none` | `one_party` |
| **MD** (Maryland) | 4th Cir. | Maryland Public Information Act (30 days) | Maryland Dept. Health | Maryland Dept. Labor | `general` | `all_party` |
| **MA** (Massachusetts) | 1st Cir. | Massachusetts Public Records Law (10 days) | Massachusetts Executive Office of Health and Human Services | Massachusetts Dept. Unemployment Assistance | `general` | `all_party` |
| **MI** (Michigan) | 6th Cir. | Michigan Freedom of Information Act (5 days) | Michigan Dept. Health and Human Services | Michigan Unemployment Insurance Agency | `medicaid_only` | `mixed` |
| **MN** (Minnesota) | 8th Cir. | Minnesota Government Data Practices Act (Prompt) | Minnesota Dept. Human Services | Minnesota Dept. Employment and Economic Development | `general` | `one_party` |
| **MS** (Mississippi) | 5th Cir. | Mississippi Public Records Act (7 days) | Mississippi Div. Medicaid | Mississippi Dept. Employment Security | `none` | `one_party` |
| **MO** (Missouri) | 8th Cir. | Missouri Sunshine Law (3 days) | Missouri Dept. Social Services | Missouri Dept. Labor and Industrial Relations | `none` | `one_party` |
| **MT** (Montana) | 9th Cir. | Montana Constitution Right to Know / Public Records Act (Prompt) | Montana Dept. Public Health and Human Services | Montana Dept. Labor and Industry | `general` | `all_party` |
| **NE** (Nebraska) | 8th Cir. | Nebraska Public Records Law (4 days) | Nebraska Dept. Health and Human Services | Nebraska Dept. Labor | `none` | `one_party` |
| **NV** (Nevada) | 9th Cir. | Nevada Public Records Act (5 days) | Nevada Dept. Health and Human Services | Nevada Dept. Employment | `general` | `mixed` |
| **NH** (New Hampshire) | 1st Cir. | New Hampshire Right-to-Know Law (5 days) | New Hampshire Dept. Health and Human Services | New Hampshire Employment Security | `medicaid_only` | `all_party` |
| **NJ** (New Jersey) | 3rd Cir. | Open Public Records Act (OPRA) (7 days) | New Jersey Dept. Human Services | New Jersey Dept. Labor and Workforce Development (NJDOL) | `general` | `one_party` |
| **NM** (New Mexico) | 10th Cir. | New Mexico Inspection of Public Records Act (IPRA) (15 days) | New Mexico Health Care Authority | New Mexico Dept. Workforce Solutions | `general` | `one_party` |
| **NY** (New York) | 2nd Cir. | Freedom of Information Law (FOIL) (5 days) | New York State Dept. Health | New York State Dept. Labor | `general` | `one_party` |
| **NC** (North Carolina) | 4th Cir. | North Carolina Public Records Law (Prompt) | North Carolina Dept. Health and Human Services | North Carolina Div. Employment Security | `general` | `one_party` |
| **ND** (North Dakota) | 8th Cir. | North Dakota Open Records Law (Prompt) | North Dakota Health and Human Services | Job Service North Dakota | `none` | `one_party` |
| **OH** (Ohio) | 6th Cir. | Ohio Public Records Act (Prompt) | Ohio Dept. Medicaid | Ohio Dept. Job and Family Services | `none` | `one_party` |
| **OK** (Oklahoma) | 10th Cir. | Oklahoma Open Records Act (Prompt) | Oklahoma Health Care Authority | Oklahoma Employment Security Commission | `medicaid_only` | `one_party` |
| **OR** (Oregon) | 9th Cir. | Oregon Public Records Law (5 days) | Oregon Health Authority | Oregon Employment Department | `none` | `mixed` |
| **PA** (Pennsylvania) | 3rd Cir. | Pennsylvania Right-to-Know Law (5 days) | Pennsylvania Dept. Human Services | Pennsylvania Dept. Labor & Industry | `none` | `all_party` |
| **RI** (Rhode Island) | 1st Cir. | Access to Public Records Act (APRA) (10 days) | Rhode Island Executive Office of Health and Human Services | Rhode Island Dept. Labor and Training | `general` | `one_party` |
| **SC** (South Carolina) | 4th Cir. | South Carolina Freedom of Information Act (10 days) | South Carolina Dept. Health and Human Services | South Carolina Dept. Employment and Workforce | `none` | `one_party` |
| **SD** (South Dakota) | 8th Cir. | South Dakota Public Records Law (10 days) | South Dakota Dept. Social Services | South Dakota Dept. Labor and Regulation | `none` | `one_party` |
| **TN** (Tennessee) | 6th Cir. | Tennessee Public Records Act (7 days) | Tennessee Dept. Finance and Administration | Tennessee Dept. Labor and Workforce Development | `general` | `one_party` |
| **TX** (Texas) | 5th Cir. | Texas Public Information Act (Prompt) | Texas Health and Human Services Commission | Texas Workforce Commission | `medicaid_only` | `one_party` |
| **UT** (Utah) | 10th Cir. | Government Records Access and Management Act (GRAMA) (10 days) | Utah Dept. Health and Human Services | Utah Dept. Workforce Services | `none` | `one_party` |
| **VT** (Vermont) | 2nd Cir. | Vermont Public Records Act (3 days) | Vermont Agency of Human Services | Vermont Dept. Labor | `general` | `one_party` |
| **VA** (Virginia) | 4th Cir. | Virginia Freedom of Information Act (5 days) | Virginia Dept. Medical Assistance Services | Virginia Employment Commission | `general` | `one_party` |
| **WA** (Washington) | 9th Cir. | Washington Public Records Act (5 days) | Washington State Health Care Authority | Washington Employment Security Department | `medicaid_only` | `all_party` |
| **WV** (West Virginia) | 4th Cir. | West Virginia Freedom of Information Act (5 days) | West Virginia Dept. Human Services | WorkForce West Virginia | `none` | `one_party` |
| **WI** (Wisconsin) | 7th Cir. | Wisconsin Public Records Law (Prompt) | Wisconsin Dept. Health Services | Wisconsin Dept. Workforce Development | `none` | `one_party` |
| **WY** (Wyoming) | 10th Cir. | Wyoming Public Records Act (30 days) | Wyoming Dept. Health | Wyoming Dept. Workforce Services | `none` | `one_party` |
| **DC** (District of Columbia) | D.C. Cir. | District of Columbia Freedom of Information Act (15 days) | District of Columbia Dept. Health Care Finance | District of Columbia Dept. Employment Services | `general` | `one_party` |

---

## 3. Operational Routing Guidelines by Discipline

### A. Health Care & Medicaid Fraud (MMIS Claims & Encounter Ledgers)
1. **Never send a state open records (FOIA/OPRA) request for personal Medicaid claims.** They will be rejected as exempt medical records.
2. **Draft a formal 45 C.F.R. § 164.524 HIPAA Demand** to the State Medicaid Agency's Privacy Officer.
3. Demand: (1) MMIS Paid/Denied claims ledger with billing NPIs, CPT codes, and dates; (2) MCO monthly capitation payment history; (3) Interstate PARIS match notices.
4. Start a **30-calendar-day statutory clock**. If the agency fails to produce records within 30 days, draft a formal notice under 45 C.F.R. § 160.306 for an administrative complaint to the HHS Office for Civil Rights (OCR).

### B. Unemployment Insurance (UI Claim Files)
1. **Demand via 20 C.F.R. § 603.5(c).** State UI records are confidential and exempt from general public inspection.
2. Check state-specific form requirements (e.g. CO Form B-652, GA DOL-5111, CA DE 5600, NC DES Release, MI UIA 6102).
3. Request complete file: initial determinations, redeterminations, examiner notes, phone call logs, interstate wage logs (ICON/IB-4), and fraud-flag metadata.

### C. State False Claims Act (Qui Tam) Classification
- **`general` (e.g. CA, CO, NJ, DE, FL, HI, IL, MA, NV, NY, NC, RI, TN, VA):** State statute allows private relators to file suit for fraud against all state funds (including Medicaid).
- **`medicaid_only` (e.g. TX, MI, LA, OK, WA, NH):** Private-relator qui tam is restricted to Medicaid / healthcare fraud. Non-Medicaid fraud must route through federal FCA (31 U.S.C. § 3729) or state auditor channels.
- **`none` (e.g. OH, PA, AL, AK, ID, ME, MO, MS, ND, SC, SD, UT, WV, WY):** No private-relator state statute. Relators must anchor their false claims actions to federal funds under 31 U.S.C. §§ 3729–3733.

### D. Wiretap & Recording Traps
- **`one_party`:** 18 U.S.C. § 2511(2)(d) and state statute permit recording by a participant, provided it is not for criminal or tortious purpose.
- **`all_party`:** Restricted without consent of all participants (e.g., CA, FL, IL, MD, MA, PA, WA), subject to public-space and crime/extortion exceptions.
- **`mixed`:** Different rules apply to telephone vs. in-person conversations (e.g., CT, MI, NV, OR, DE). Verify communication type before logging audio exhibits.