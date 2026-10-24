# Skill: Civil Court Navigation & Document Assembly

## Purpose
Guides self-represented litigants through organizing, fact-checking, and formatting their own drafts for civil court pleadings, appeals, administrative orders, and everyday legal documents (wills, leases, powers of attorney).

---

## 1. Core Operating Rules

1. **Never Invent Arguments, Facts, or Citations:**
   - Only format the user's supplied facts, dates, party names, docket numbers, and statutes.
   - If a citation or deadline is missing, report it as `MISSING`. Never guess or hallucinate citations.
2. **Deterministic Court Rules & Deadline Boundaries:**
   - **Where Sourced Deadline Math Exists:** Verified statutory deadline packs exist ONLY for 4 specific appellate and procedural workflows:
     - New Jersey Appellate Division (`nj-app-div`)
     - Colorado Court of Appeals (`co-app-div`)
     - Colorado District Court Mandamus Rule 106 (`co-mandamus`)
     - Federal Appellate Rules (`fed-app`)
   - **Trial-Level Civil Matters (Evictions, Debt/Collections, Family Court, Small Claims):**
     - The software does **NOT** compute statutory filing or answer deadlines for county or municipal trial courts.
     - Never guess or calculate an unverified deadline.
     - Hearing dates and compliance dates can only be extracted from an order or notice the user already received (`order ingest`).
   - **Jurisdiction Status Command:**
     `justlegalos-cli --json jurisdiction status <dir>` reports on 5 specific jurisdictional slots:
     1. Wiretap & Recording Consent Rules (`state_traps`)
     2. Official Government Statutes & Rules Portals (`primary_sources`)
     3. False Claims Act Scope & WHIP Alignment (`fraud_fca_scope`)
     4. Open Records & Pre-Filing Discovery Clocks (`open_records`)
     5. Local Court Rules & Forms Pack Availability (`local_court_rules`)
   - If a slot reports `UNVERIFIED_GAP`, advise the user to consult their local court clerk or state judiciary website and pin the rule with:
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
