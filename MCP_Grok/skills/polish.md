# Skill: Pre-Submission Proofreading, Independent Analysis & Submitter Claim Polish

## Purpose
Governs the final collaborative proofreading and forensic analysis polish workflow between the citizen relator and Grok before Gate 5 submission. Ensures the dossier satisfies DOJ WHIP (28 U.S.C. § 530C(b)(1)(L)) independent analysis criteria, verifies whistleblower award claim contact fields, and formats evidence within the strict 4-file upload ceiling of `america.gov/fraud`.

---

## 1. Whistleblower Contact Block & Award Claim Requirements

A submitter cannot receive a whistleblower award if they submit anonymously without retained legal counsel. Grok must verify the submitter's award claim posture before Gate 5:

1. **Direct Whistleblower Award Claim (`28 U.S.C. § 530C(b)(1)(L)`):**
   - Requires full legal name, current telephone, email, and mailing address.
   - These details are drawn from `case.json` (`profile.filer`) and embedded directly into **Exhibit 1: Master Disclosure & Relator Contact Information**.
   - Awards are **not** paid immediately or deposited into personal accounts upfront; they are disbursed via Treasury Electronic Funds Transfer (Form SF-3881) following criminal forfeiture or civil judgment.
2. **Anonymous Tip via Retained Legal Counsel:**
   - Permitted only when a licensed attorney signs and certifies the whistleblower disclosure on behalf of the relator.
   - Relator identity remains shielded in the attorney's vault until settlement or court unsealing.
3. **Anonymous Citizen Tip (Unrepresented):**
   - If the submitter refuses to provide contact details and does not have retained counsel, the tip is filed as an informational citizen lead. Grok must confirm that no statutory whistleblower award can be claimed.

---

## 2. Section IV: Independent Forensic & Investigative Analysis

To survive triage and satisfy DOJ whistleblower guidelines, the submission must provide **independent analysis** rather than raw accusations or cut-and-paste complaint text.

Grok and the user work together to draft and refine **Section IV**, addressing four mandatory analytical prongs:

### Prong A: What Was Discovered (Factual Observations)
- Specify the exact records returned (e.g., Medicaid MMIS claims printout, SNAP transaction ledger, state corporate filings).
- Itemize target provider entities, verified 10-digit NPIs, and physical operating addresses unmasked via NPPES registry lookup.
- Include date ranges, claim counts, and specific CPT/HCPCS procedure codes.

### Prong B: Why It Constitutes Statutory Fraud
- Ground the allegations in governing statutes:
  - **False Claims Act (31 U.S.C. § 3729(a)(1)(A))**: Knowingly presenting false or fraudulent claims for payment.
  - **DOJ WHIP Guidelines & 28 U.S.C. § 530C(b)(1)(L)**: Material healthcare and procurement fraud leading to asset forfeiture.
  - State False Claims equivalents (e.g., Colorado Medicaid False Claims Act C.R.S. § 25.5-4-303.5).
- Articulate the specific theory: billing for services not rendered, services rendered during patient absence, or systematic billing without required licensed supervisory personnel.

### Prong C: How It Was Proven (Forensic Contradiction)
- Detail the empirical methodology proving the fraud:
  - **Chronological / Geographic Impossibility**: Comparing documented interstate relocation or hospitalization dates against continued in-state home health billings.
  - **EVV Override Ratio**: Disproportionate manual override rates in Electronic Visit Verification records indicating ghost visits.
  - **Staffing / Ratio Impossibility**: Licensing board surveys (CMS-2567) showing 0 registered nurses on staff while billing intensive skilled nursing codes.

### Prong D: Compliance with Independent Analysis Doctrine
- Explicitly certify that the conclusion derives from the submitter's independent investigation, cross-referencing disparate public datasets (open records, NPPES registries, state corporate filings) with primary personal records (residence leases, moving bills, claims histories).
- Satisfies the "independent analysis" prong under federal whistleblower jurisprudence, demonstrating that the critical insight was not previously disclosed in news reports or public hearings.

---

## 3. The 4-File Auto-Binder Protocol (`america.gov/fraud`)

Federal fraud intake portals enforce a strict **4 to 5 file upload ceiling**. To prevent upload failures, JustLegal OS automatically merges multi-page exhibits into at most four standardized, numbered PDF exhibits:

1. **Exhibit 1: Master Disclosure, Relator Contact & Investigative Summary** (`Exhibit_1_Master_Disclosure_and_Contact.pdf`)
   - Submitter contact info & award claim status.
   - Section IV Independent Analysis narrative.
   - Complete Provenance Log & SHA-256 integrity digests.
2. **Exhibit 2: Forensic Contradiction Report & Claims Discrepancy Ledger** (`Exhibit_2_Forensic_Contradiction_Report.pdf`)
   - Detailed ledger of fraudulent/impossible claim dates and amounts.
3. **Exhibit 3: Primary Agency & Residency Records** (`Exhibit_3_Primary_Agency_and_Residency_Records.pdf`)
   - Official records requests returned from state agencies and relocation proof.
4. **Exhibit 4: Visual Observations & Regulatory Registry Snapshots** (`Exhibit_4_Visual_and_Registry_Snapshots.pdf`)
   - EXIF-scrubbed photo evidence, NPPES verification records, and state licensing inspection reports.

---

## 4. Collaborative Proofreading Checklist (Pre-Gate 5)

Before triggering Gate 5 and asking the user for their final "GO", Grok and the user review:
- [ ] No unsworn perjury lines (`executed pursuant to 28 U.S.C. § 1746`).
- [ ] No speculative dollar guarantees or bounty percentages.
- [ ] Character count of portal text is $\le 1,950$ characters.
- [ ] All SHA-256 hashes are complete 64-character hex strings.
- [ ] Submitter contact info matches user's preferred claim status.
- [ ] Independent analysis section addresses all four prongs.
