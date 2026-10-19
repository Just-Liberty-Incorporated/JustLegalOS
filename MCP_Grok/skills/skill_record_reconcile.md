# SKILL: COURT RECORD RECONSTRUCTION & EVIDENTIARY RECONCILIATION

Use this skill when a self-represented filer or appellant needs to audit the court docket, determine if any orders or filings are missing from their possession, or reconstruct the complete record below for an appeal or motion.

---

## 1. Why Docket Reconciliation Matters
In appellate litigation and post-judgment motions, **if it is not in the record, it did not happen**.
Pro se litigants frequently discover too late that:
- An adverse order was entered without formal notice or service.
- An exhibit filed at trial was never transmitted by the clerk to the appellate court.
- Critical hearing transcripts were never requested or paid for.
- Opposing counsel filed an ex-parte letter or brief that never reached the litigant.

Reconciling the official court docket history against local case documents is the only reliable way to guarantee complete filings and avoid dismissal for incomplete appendix/record.

---

## 2. The 3-Tier Evidentiary Inventory
Classify all documents in the matter across three standard tiers:

1. **Tier 1: Court Core Instruments (Clerk Record)**
   - Official summons, verified complaints, amended pleadings.
   - All written orders, bench rulings, minute entries, and judgments entered by the clerk.
   - Proofs of service / certificates of mailing.
2. **Tier 2: Party Filings & Exhibits**
   - Notices of motion, supporting affidavits/certifications, briefs, and responses.
   - Marked trial or motion exhibits (contracts, emails, medical records, invoices).
3. **Tier 3: Verbatim Transcripts & Audio Recordings**
   - Official certified transcripts of hearings, oral arguments, depositions, or trial proceedings.
   - Court audio recordings where court reporters were not present.

---

## 3. Step-by-Step Reconciliation Protocol

### Step 1: Ingesting Docket Entries
Ask the user to paste the docket sheet from the court's online portal (e.g. PACER, Odyssey, eCourts, CCAP).
Each entry typically has:
- **Index / Number:** (e.g. Doc # 14)
- **Filing Date:** (e.g. 01/15/2026)
- **Filer:** (e.g. Defendant, Clerk, Judge)
- **Title / Description:** (e.g. "ORDER GRANTING MOTION TO COMPEL")

### Step 2: Gap & Discrepancy Detection
Compare the docket entries with the user's local folder:
- **Missing Orders:** Any docket entry titled "ORDER", "JUDGMENT", or "MINUTE ENTRY" where the user lacks the PDF/copy.
- **Unserved Filings:** Any pleading or motion filed by the opposing party where the user never received service.
- **Unfiled Submissions:** Litigant documents sent to the court that never appeared on the docket sheet.

### Step 3: Formatting Formal Clerk Requests
If entries are missing, generate a standardized, polite record request to the Clerk of Court:
- Identifying the exact docket index number and filing date.
- Requesting certified copies if required for appellate review.
- Inquiring regarding fee waivers (IFP / In Forma Pauperis) for record duplication.

---

## 4. JustLegal OS Desktop & CLI Commands

JustLegal OS includes built-in record reconstruction tools:
```bash
# Ingest docket entries and reconcile against local case folder
justlegalos-cli --json record reconcile ./my-case --docket "<pasted docket text>"

# View current inventory of reconciled vs missing documents
justlegalos-cli --json record inventory ./my-case

# Output list of missing items that require clerk requests
justlegalos-cli --json record missing ./my-case

# Generate state-specific formal record request letter/form
justlegalos-cli --json record request-form ./my-case --state <STATE>
```
