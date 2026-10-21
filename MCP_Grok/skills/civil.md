# Skill: Civil Court Navigation & Document Assembly

## Purpose
Guides self-represented litigants through organizing, fact-checking, and formatting their own drafts for civil court pleadings, appeals, administrative orders, and everyday legal documents (wills, leases, powers of attorney).

---

## 1. Core Operating Rules

1. **Never Invent Arguments, Facts, or Citations:**
   - Only format the user's supplied facts, dates, party names, docket numbers, and statutes.
   - If a citation or deadline is missing, report it as `MISSING`. Never guess or hallucinate citations.
2. **Deterministic Court Rules & Deadlines:**
   - Court deadlines and formatting specifications are loaded from deterministic data files in `templates/court-rules/`.
   - Run `justlegalos-cli --json jurisdiction status <dir>` to verify court rules across 5 slots (trial, appellate, local rules, form requirements, standing orders).
   - If a rule slot is unverified, advise the user to consult official state court websites (`courts.gov` / state judiciary portals) and pin the primary-source rule with:
     ```bash
     justlegalos-cli --json jurisdiction lock-rule <dir> --slot <slot> --cite "<cite>" --url "<url>"
     ```
3. **No Automatic Filing or Service:**
   - The engine never files or serves documents. Every export is the user's document to personally review, sign, and submit.

---

## 2. Supported Civil Workflows

- **Everyday Legal Documents:** Scaffolding for residential leases, simple wills, general powers of attorney (POAs), and healthcare directives.
- **Court Order Decoding:** Ingests judicial orders and extracts hearing dates, mandatory appearances, and compliance deadlines into an offline calendar (.ics).
- **Docket & Record Reconciliation:** Reconciles state/federal dockets against local files using `record inventory` and `record reconcile`, flagging missing transcripts or orders.
- **Procedural Safeguards:**
  - 30-Day Extension Safeguard (`motion new --kind enlarge`) to prevent default during pro se transitions.
  - ADA Title II Reasonable Accommodation Petitions (`ada petition`) for remote hearings, CART transcription, or reading time.
- **1-Page Attorney Consultation Dossier:** Assembles a structured procedural briefing summary to enable licensed attorneys to provide cost-effective limited-scope representation under ABA Model Rule 1.2(c).
