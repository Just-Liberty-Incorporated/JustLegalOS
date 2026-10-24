# Troubleshooting & Diagnostic Runbook

Use this runbook to quickly diagnose and fix any issue across the Desktop UI (`JustLegalOS.exe` / `OpenSeason.exe`), the Agent CLI/MCP (`JustLegalOS-CLI.exe` / `justlegalos-cli`), PDF compilation, Venue Validation, or JustLegal Cloud Sync.

---

## 1. Quick Health Check Commands

Run these from PowerShell in `c:\Users\Deus\Documents\DocumentBuilder`:

```powershell
# 1. Inspect full Legal OS context for a case folder (1-call diagnostic)
.\JustLegalOS-CLI.exe --json case context "C:\Users\Deus\Documents\JustLegal\Cases\<case-slug>"

# 2. Check Guided Intake Wizard status & see what question/field is missing
.\JustLegalOS-CLI.exe --json intake status "C:\Users\Deus\Documents\JustLegal\Cases\<case-slug>"

# 3. Check Multi-Venue Competence Matrix (NJ R. 1:4-4(b), CO Rule 121 conferral, Will witnesses, etc.)
.\JustLegalOS-CLI.exe --json check venue "C:\Users\Deus\Documents\JustLegal\Cases\<case-slug>"

# 4. Run Court Pre-Flight Readiness Audit
.\JustLegalOS-CLI.exe --json check preflight "C:\Users\Deus\Documents\JustLegal\Cases\<case-slug>"

# 5. View Portfolio Radar across all cases
.\JustLegalOS-CLI.exe --json portfolio status
```

---

## 2. Common Issues & Exact Fixes

### Issue A: Export says `"Court name is required"` or `"Docket number is required"` on a Will, Lease, Demand Letter, or New Unfiled Complaint
- **Cause:** The case's `matter_tier` or `procedural_stage` was left at the default active litigation setting instead of Tier 1 (`transactional`), Tier 2 (`administrative`), or Pre-Filing (`pre_filing`).
- **Fix (UI):** Open the case, click **🧙 Guided Wizard** at the top right, and set **Matter Tier** to:
  - `Tier 1: Transactional` for Wills, Leases, Contractor Agreements, or POAs.
  - `Tier 2: Pre-Suit / Notice` for Demand Letters, Cure Notices, or OPRA/FOIA requests.
  - `Tier 3: Court Litigation` + **Stage: `Pre-Filing`** for an initial Complaint that does not have a court docket number yet (renders `Case No. __________ (To Be Assigned)`).
- **Fix (CLI):** Set `"matter_tier": "transactional"` (or `"administrative"`, or `"procedural_stage": "pre_filing"`) in `case.json` or via `intake answer`.

---

### Issue B: New Jersey Venue Check Fails with `venue.nj_r_1_4_4b`
- **Cause:** A New Jersey certification or motion draft contains hedging language like `"to the best of my knowledge"` or `"information and belief"`. Under **N.J. Ct. R. 1:4-4(b)** and **R. 1:6-6**, facts must be stated on personal knowledge only, and the certification must use the verbatim punishment clause.
- **Fix:** Remove `"to the best of my knowledge"` from your draft text. When you run `export packet`, JustLegal OS automatically injects the verbatim `R. 1:4-4(b)` certification oath:
  > *"I certify that the foregoing statements made by me are true. I am aware that if any of the foregoing statements made by me are willfully false, I am subject to punishment."*

---

### Issue C: Colorado Venue Check Fails with `venue.co_rule_121_conferral`
- **Cause:** Trial-court civil motions in Colorado often use **C.R.C.P. 121 § 1-15(8)** conferral. That label is `needs_verification`. Papers in the Colorado Court of Appeals may be governed by **C.A.R.** instead, so the checker does not assert Rule 121 there.
- **Fix:** Confirm the rule that governs the paper. For a trial-court motion, open the **Wizard** and answer the Conferral question, or run:
  ```powershell
  .\JustLegalOS-CLI.exe --json intake answer <case-dir> --id conferral_status --value "Conferred with opposing party on YYYY-MM-DD; motion is opposed."
  ```

---

### Issue D: Multiple Motions in the Same Case Overwriting Draft Sections
- **Cause:** Calling global `draft fill` writes to `drafts/sections.json`.
- **Fix:** Use the **📑 Motions & Filings** docket tab in the UI, or use filing-scoped IPC (`scoped_sections_cmd` / `scoped_fill_cmd`) which stores each motion's guided sections in `drafts/sections-{filing_id}.json`.

---

### Issue E: `JustLegalOS.exe` Fails to Copy During Build (`os error 32: process cannot access the file`)
- **Cause:** `JustLegalOS.exe` (or `OpenSeason.exe`) is currently running in the background, locking the executable on Windows.
- **Fix:** Close the application window or stop the process before copying the new binary:
  ```powershell
  Stop-Process -Name "JustLegalOS" -Force -ErrorAction SilentlyContinue
  Stop-Process -Name "OpenSeason" -Force -ErrorAction SilentlyContinue
  Copy-Item -Force "src-tauri\target\release\open-season.exe" ".\JustLegalOS.exe"
  Copy-Item -Force "src-tauri\target\release\open-season.exe" ".\OpenSeason.exe"
  ```

---

### Issue F: Offline PDF Scan (`intake scan`) Returns 0 Candidates on a Scanned Image PDF
- **Cause:** `pdf_tools::extract_pdf_text` extracts embedded PDF text streams (`Tj`/`TJ` operators) 100% offline using `lopdf`. If a court order was scanned as a flat bitmap image without an OCR text layer, it contains no text operators.
- **Fix:** Place a text-searchable PDF or a companion `.txt` / `.md` note in `orders/` or `evidence/`—`intake scan` reads both `.pdf` text streams and `.txt`/`.md` files in `orders/` and `evidence/`.

---

### Issue G: JustLegal Cloud Sync Refused (`ConfidentialForbidden`)
- **Cause:** The case folder has a `.sealed` marker file, `mode: "confidential"`, or `sealed_at` set.
- **By Design:** Confidential whistleblower vaults are strictly airgapped in `src-tauri/src/sync.rs`. Never delete `.sealed` to force cloud sync. Use local Word/PDF export or `.osb` encrypted bundle export instead.
---

### Issue H: `fraud.gov` Narrative Field Truncation (`>2,000 characters`)
- **Cause:** Official federal agency portals (e.g. `fraud.gov`, `oig.hhs.gov`, `oig.dol.gov`) strictly enforce a 2,000 character maximum limit on web submission forms. Sprawling complaint drafts get cut off mid-sentence.
- **Fix:** Use `justlegalos-cli --json whip prepare <case>` or the Split-Screen Portal Co-Pilot (`PortalCopilotModal.svelte`). The engine automatically formats high-density narrative blocks under 1,800 characters, embeds the Calling Card v1.1 header, and references the full details in the attached, hashed `WHIP_Evidence_Packet.pdf`.

---

### Issue I: Rule 9(b) Qui Tam Audit Capped at 74/100 (`Score Capped: Missing Billing Records`)
- **Cause:** Under Federal Rule of Civil Procedure 9(b), an FCA claim alleging false claims cannot survive without representative specific false billings, invoices, or drawdown vouchers. Without physical billing records in `evidence/`, the engine deterministically caps the viability grade at 74/100 (Developing / Fixable).
- **Fix:** Deposit at least one itemized bill, remittance advice, or ASAP.gov drawdown voucher into `evidence/` and run `justlegalos-cli --json qui-tam audit-9b <case>`.

---

### Issue J: Accidental Forfeiture of Relator Reward under Public Disclosure Bar (31 U.S.C. § 3730(e)(4))
- **Cause:** Litigant submits a non-public contractor fraud claim directly to `fraud.gov` or posts allegations/photos on TikTok, X, or YouTube before filing under seal.
- **Fix:** Run `justlegalos-cli --json whip compare-routes <case>` BEFORE submitting! Substantial claims against private contractors (e.g. Deloitte, Maximus) with non-public proof must be filed under seal in federal court through contingency relator counsel to preserve the relator's share (15%–25% intervened / 25%–30% declined under 31 U.S.C. § 3730(d)).

---

### Issue K: Open Records Statutory Deadline Tracking Skips Holidays Incorrectly
- **Cause:** Manual calendar calculation fails to account for statutory working-day definitions under state sunshine laws (e.g. Colorado CORA C.R.S. § 24-72-205(6) 3-working-day rule).
- **Fix:** Use `justlegalos-cli --json records track <case> --sent YYYY-MM-DD --days 3`, which uses deterministic working-day date math that automatically excludes Saturdays, Sundays, and official legal holidays.

---

### Issue L: CMS NPPES / USAspending Registry Returns `LOOKUP_FAILED_OR_OFFLINE`
- **Cause:** Local internet connection is offline or federal API endpoints are experiencing temporary rate limits.
- **By Design:** The engine sets `lookup_status = LOOKUP_FAILED_OR_OFFLINE` and marks the entity as `Unverified`—it **never** falsely accuses a provider of being unregistered due to a network glitch. Once connected, re-run with explicit NPI: `.\JustLegalOS-CLI.exe --json qui-tam verify-entity --name "<name>" --npi <10-digits>`.
