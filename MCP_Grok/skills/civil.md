# Skill: Civil Court Navigation & Document Assembly

## Purpose
Guides self-represented litigants through organizing, fact-checking, and formatting their own drafts for civil court pleadings, appeals, administrative orders, and everyday legal documents (wills, leases, powers of attorney).

---

## 1. Core Operating Rules

1. **Never Invent Arguments, Facts, or Citations:**
   - Only format the user's supplied facts, dates, party names, docket numbers, and statutes.
   - If a citation or deadline is missing, report it as `MISSING`. Never guess or hallucinate citations.
2. **Name the place, then load what that case needs:**
   - Each case folder keeps its own state, county, and court. Confirm them from the papers. Do not copy them from another case, and do not treat a blank case as finished.
   - A fraud or records matter is a case too. Create it with `case create --kind fraud_investigation` or `--kind records_discovery`. Attach states with `jurisdiction add <dir> --state OH --role program`. Roles are `observer`, `program`, `target`, `enforcement`, `court`, and `reference`. `--state ALL` attaches every state as reference on that case only.
   - Call `jurisdiction find <dir>`. It names the state, the court, and the state court pages to open first, with court locations and local rules at the top. Open those pages and look for the court named on the case. If the state site does not link to that court, search for that court's own page. If the search does not find it, stop and tell the person, and ask them to find it or give the link. When the court's own page is found, save it with `jurisdiction lock-rule <dir> --slot local_court_rules --cite "<name on the page>" --url "<page>"`. A law-firm page is not the court's page. A deadline is written down only when that saved page states it. If the court name is blank, the command says so and the search waits.
   - **Where sourced deadline math already exists:** Four appellate and procedural workflows have a clock:
     - New Jersey Appellate Division (`nj-app-div`)
    - Colorado Court of Appeals (`co-app-ct`)
    - Colorado District Court Mandamus Rule 106 (`co-mandamus`)
    - Federal Appellate Rules (`fed-app-ct`)
   - **Trial-Level Civil Matters (Evictions, Debt/Collections, Family Court, Small Claims):**
     - The software does **NOT** compute statutory filing or answer deadlines for county or municipal trial courts.
     - Never guess or calculate an unverified deadline.
     - Hearing dates and compliance dates can only be extracted from an order or notice the user already received (`order decode`).
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
3. **No Automatic Filing or Service (FRCP 11(a) Precision):**
   - Under Federal Rule of Civil Procedure 11(a) and state parallels, every pleading, written motion, and other paper must be signed by the attorney of record or personally by the self-represented party. JustLegal OS never signs, e-files, or serves court pleadings (0% filing/serving).
   - It formats the user's own words and checks only the sourced packs listed above. If the pack's court is not the case's court, deadline math stops. The local rule for any other court is the page saved on that case. A deadline that page does not state is left blank. Every export is the user's document to personally review, sign, and submit.

---

## 1b. Workstation-setup and more than one case

Before creating a new court-paper matter, call `skills/setup.md` (Windows checks included). Read existing folders under Documents\JustLegal\Cases first.

If more than one case is present:
1. For each case, open the **newest order** (or newest file in `orders/`).
2. From that paper only, write state, county, court, and docket number. If the paper does not state a field, write `MISSING`. Never guess a court (including Colorado Court of Appeals).
3. Show the list, then ask which case to start with.

---

## 2. Supported Civil Workflows

- **Everyday Legal Documents:** Scaffolding for residential leases, simple wills, general powers of attorney (POAs), and healthcare directives.
- **Court Order Decoding:** Ingests judicial orders and extracts hearing dates, mandatory appearances, and compliance deadlines into an offline calendar (.ics).
- **Docket & Record Reconciliation:** Reconciles state/federal dockets against local files using `record inventory` and `record reconcile`, flagging missing transcripts or orders.
- **Procedural Safeguards:**
  - 30-Day Extension Safeguard (`case create --kind enlargement`) to prevent default during pro se transitions.
  - ADA Title II Reasonable Accommodation Petitions (`ada petition`) for remote hearings, CART transcription, or reading time.
- **1-Page Attorney Consultation Dossier:** Assembles a structured procedural briefing summary to enable licensed attorneys to provide cost-effective limited-scope representation under ABA Model Rule 1.2(c).
