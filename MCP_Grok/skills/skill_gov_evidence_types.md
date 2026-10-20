# Skill: Government Evidence Types & Provenance Ingestion

## Purpose
Enforces rigorous categorization of evidence deposited into `evidence/`, preventing fuzzy keyword matching and ensuring that Rule 9(b) compliance is grounded in authentic source documents.

---

## 1. The 7 Government Evidence Types

Every piece of evidence added to a government or contractor case must be typed using `--type` and `--source`:

| Type Code | Display Label | Description & Significance | Evidentiary Tier |
|---|---|---|---|
| `contract` | Prime Contract / SOW | Prime agreement, statement of work, SLA requirements, task orders. Establishes contractual obligation. | Tier 1 (Official) |
| `invoice` | Invoice / Drawdown Record | Monthly contractor billing invoice, payment voucher, or PMS drawdown. **Mandatory for Rule 9(b) > 74 score.** | Tier 1 (Official) |
| `audit-report` | Independent Audit Finding | Official finding by State Auditor (OSA), DOL-OIG, or GAO documenting system defects or failure. | Tier 1 (Official) |
| `internal-memo` | Agency / Contractor Memo | Internal emails, meeting minutes, Slack logs, or defect tracking tickets showing knowledge of falsity. | Tier 2 (Transcripts / Records) |
| `field-observation` | Physical / Field Inspection | Photographs, videos, or on-site notes of dark facility or non-delivery. Has EXIF GPS/timestamp & SHA-256. | Tier 2 (Transcripts / Records) |
| `own-claim-file` | Claimant 20 CFR 603 File | Complete claimant record, examiner notes, fraud-flag metadata, and LexisNexis/ID.me transaction receipts. | Tier 2 (Transcripts / Records) |
| `public-search` | Public Internet Search | News articles, Google searches, Wikipedia, or press releases. **Support only—cannot prove firsthand falsity.** | Tier 3 (Archive) |

---

## 2. Evidence Ingestion Commands

1. **Adding an Official Contract or SOW:**
   ```bash
   justlegalos-cli --json evidence add ./my-case \
     --file ./contracts/deloitte-cdle-sow.pdf \
     --type contract \
     --source records-request \
     --description "Deloitte MyUI+ Prime Contract Statement of Work and SLA targets"
   ```

2. **Adding a Contractor Invoice (Unlocks 9(b) Cap):**
   ```bash
   justlegalos-cli --json evidence add ./my-case \
     --file ./invoices/deloitte-invoice-nov2023.pdf \
     --type invoice \
     --source records-request \
     --description "Deloitte November 2023 billing voucher for MyUI+ maintenance"
   ```

3. **Ingesting Firsthand Field Inspection Photo:**
   ```bash
   justlegalos-cli --json evidence add ./my-case \
     --file ./photos/clinic_exterior.jpg \
     --type field-observation \
     --source firsthand \
     --description "Exterior photo of dark facility during posted business hours"
   ```

4. **Rebuilding the Numbered Index:**
   ```bash
   justlegalos-cli --json evidence index ./my-case
   ```
