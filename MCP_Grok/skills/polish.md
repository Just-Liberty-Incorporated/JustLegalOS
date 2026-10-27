# Skill: Pre-Submission Proofreading, Independent Analysis & Submitter Claim Polish (`skills/polish.md`)

## Purpose
Governs the final collaborative proofreading, independent investigative analysis, and submitter claim polish workflow before Gate 5 submission. Structures the dossier for submission to the DOJ Whistleblower Incentive Program (28 U.S.C. § 530C(b)(1)(L)), verifies whistleblower award claim contact fields, protects submitter privacy under Gate 2, and formats exhibits into a structured 4-exhibit evidentiary binder for `america.gov/fraud`.

---

## 1. Whistleblower Contact Block, Posture & Gate 2 Privacy Scrub

Award eligibility rules for anonymous or counsel-assisted tips are set by DOJ. Check the current DOJ program terms and FAQ at justice.gov before telling a submitter anything about awards. The agent confirms the submitter's chosen contact posture before Gate 5:

1. **Direct Whistleblower Award Claim (28 U.S.C. § 530C(b)(1)(L)):**
   - The submitter enters their full legal name, contact telephone, email, and mailing address directly into the secure portal fields of `america.gov/fraud` (and stores them locally in `case.json`).
   - **Gate 2 Privacy Scrub Boundary:** Third-party PII (patient SSNs, third-party phone numbers, unrelated medical IDs) is strictly scrubbed. The front page of Exhibit 1 identifies the submitter's award claim posture (e.g., *Direct Submitter: Contact info designated for portal submission | Award Claim under 28 U.S.C. § 530C(b)(1)(L)*) without publicizing personal home addresses across the face of the exported PDF.
   - **Award Determination:** Under DOJ Whistleblower Incentive Program (WHIP) terms for america.gov/fraud, award determinations under 28 U.S.C. § 530C(b)(1)(L) are evaluated at or around the time criminal charges are filed. Awards are discretionary and never paid upfront or pre-guaranteed.
2. **Anonymous Tip via Retained Legal Counsel:**
   - A submitter may work through their own attorney. Check DOJ's current terms for how represented tips are handled.
   - Submitter identity remains shielded in the attorney's client-confidential vault in accordance with DOJ whistleblower intake procedures.
3. **Anonymous Citizen Tip (Unrepresented):**
   - If the submitter chooses not to provide contact details and does not have retained counsel, the tip is filed as an informational citizen lead. Do not tell the submitter whether an award is available; point them to DOJ's current terms.

---

## 2. Section IV: Independent Forensic & Investigative Analysis

To survive initial DOJ triage, the submission must provide **independent analysis** and original information rather than bare accusations, unverified rumors, or cut-and-paste complaint text.

The agent and user collaborate to draft and refine **Section IV**, addressing four mandatory analytical prongs:

### Prong A: What Was Discovered (Original Factual Observations)
- Specify the exact primary records returned (e.g., Medicaid MMIS claims printout, SNAP transaction ledger, state corporate filings).
- Itemize target provider entities, verified 10-digit NPIs, and physical operating addresses confirmed via public registry lookup (NPPES).
- Include date ranges, claim counts, and specific CPT/HCPCS procedure codes.

### Prong B: Why It Constitutes Statutory Fraud
- Ground the allegations in substantive fraud statutes:
  - **Health Care Fraud (18 U.S.C. § 1347)** & **False Statements Relating to Health Care (18 U.S.C. § 1035)**: Knowingly executing a scheme to defraud a health care benefit program or making materially false statements in claims.
  - **Civil False Claims Act (31 U.S.C. § 3729(a)(1)(A))**: Knowingly presenting or causing to be presented a false or fraudulent claim for payment.
  - **Statutory Award Authority (28 U.S.C. § 530C(b)(1)(L))**: Authorizes discretionary awards for original information leading to criminal prosecution under the DOJ Whistleblower Incentive Program (america.gov/fraud).
  - State False Claims equivalents where applicable (e.g., Colorado Medicaid False Claims Act, C.R.S. § 25.5-4-303.5).
- Articulate the specific mechanism: billing for services not rendered, services billed during documented recipient absence, or systematic billing without required licensed supervisory personnel.

### Prong C: How the Mismatch Was Found (Records Comparison)
- Describe, neutrally, how the records were compared. DOJ decides whether fraud occurred:
  - **Date and Location Mismatch**: Comparing documented interstate relocation or hospitalization dates against continued in-state home health billings.
  - **EVV Override Ratio**: Disproportionate manual override rates in Electronic Visit Verification records indicating ghost visits.
  - **Staffing Records Mismatch**: Licensing board surveys (CMS-2567) showing 0 registered nurses on staff while billing intensive skilled nursing codes.

### Prong D: Factual Basis for Independent Analysis
- Document how the factual conclusions derive from the submitter's independent investigation, cross-referencing public datasets (open records, NPPES registries, state corporate filings) with primary personal records (residence leases, moving bills, claims histories).
- Detail that the critical insight was developed from primary factual records rather than derived from prior news reports or public hearings.

---

## 3. The 4-File Auto-Binder Protocol (`america.gov/fraud`)

Federal fraud intake portals accept focused exhibit attachments. JustLegal OS structures submissions into a standardized 4-exhibit PDF binder: JustLegal OS automatically organizes multi-page exhibits into four standardized, numbered PDF exhibits:

1. **Exhibit 1: Master Disclosure & Factual Summary** (`Exhibit_1_Master_Disclosure_and_Summary.pdf`)
   - Standardized diagnostic header and calling card.
   - Complete factual narrative and Section IV Independent Analysis (rendered in full across pages without truncation).
   - Complete Provenance Log & SHA-256 integrity digests.
   *(Note: The portal narrative paste block in web forms has a character ceiling of ~1,950 characters; the multi-page Exhibit 1 PDF contains the full, complete text without truncation).*
2. **Exhibit 2: Forensic Contradiction Report & Claims Discrepancy Ledger** (`Exhibit_2_Forensic_Contradiction_Report.pdf`)
   - Detailed ledger of claim dates that do not match and amounts cross-referenced with absence records.
3. **Exhibit 3: Primary Agency & Residency Records** (`Exhibit_3_Primary_Agency_and_Residency_Records.pdf`)
   - Official records requests returned from state agencies and relocation proof.
4. **Exhibit 4: Visual Observations & Regulatory Registry Snapshots** (`Exhibit_4_Visual_and_Registry_Snapshots.pdf`)
   - EXIF-scrubbed photo evidence, NPPES organizational verification records, and state licensing inspection reports.

---

## 4. Collaborative Proofreading Checklist (Pre-Gate 5)

Before triggering Gate 5 and requesting the user's final "GO":
- [ ] No unsworn perjury lines (`executed pursuant to 28 U.S.C. § 1746`).
- [ ] No speculative dollar guarantees or bounty percentages.
- [ ] Substantive fraud statutes (18 U.S.C. § 1347 / 31 U.S.C. § 3729) cited for violations; 28 U.S.C. § 530C cited strictly as award authority.
- [ ] Character count of portal narrative is $\le 1,950$ characters.
- [ ] All SHA-256 hashes are complete 64-character hex strings.
- [ ] Submitter contact info is entered in portal form fields; third-party PII is scrubbed.
- [ ] Independent analysis addresses all four prongs.
