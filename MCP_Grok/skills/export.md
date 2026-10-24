# Skill: Fail-Closed Turnkey Export & Pre-Filing Packet Linter

## Purpose
Governs the assembly of turnkey export packets (`exports/WHIP_Evidence_Packet.pdf`, `exports/whip_submission.json`), enforces the 1,950-character limit, strips EXIF GPS metadata, and runs the pre-export packet linter.

---

## 1. Hard Export Requirements

1. **Character Limit Enforcement**:
   - `america.gov/fraud` and federal web forms limit narrative text fields to ~2,000 characters.
   - JustLegal OS enforces a strict **$\le 1,950$-character limit** on portal paste narratives.
   - If user-authored text exceeds 1,950 characters, export is **REFUSED** with an error. The software NEVER silently truncates user text.
2. **Cryptographic Hashes**:
   - SHA-256 digests are calculated over the exact byte streams of exhibit files.
   - Hashes are always emitted as complete **64-character lowercase hexadecimal strings**.
   - Truncated hashes or zero-byte placeholders for non-existent files are rejected.
3. **EXIF GPS Privacy Scrubbing**:
   - Photographic exhibits (JPEG, PNG) are inspected and stripped of EXIF GPS coordinates, camera serial numbers, and device tags prior to hashing.
   - Preserves citizen reporter anonymity and location privacy.
4. **Dual Standard Footers**:
   - **Outsider Public Portal Export:**
     > "This disclosure was assembled by the reporter using JustLegal OS offline verification software. Just Liberty Incorporated does not practice law and does not represent the reporter. Verified against CMS NPPES & USAspending public APIs. Not legal advice."
   - **Insider Attorney Consultation Package:**
     > "This factual record index was compiled offline by the client for legal consultation. Just Liberty Incorporated is not legal counsel. This document is provided to retained counsel to facilitate legal evaluation."

---

## 2. Pre-Export Packet Linter (`justlegalos-cli packet lint <dir>`)

Scans all generated files in `exports/` before user distribution.
Fails closed (exits code 1) if any forbidden phrase is found:
- `attorney-client privileged evaluation` (claims unverified privilege on non-counsel drafts)
- `public disclosure status: clear` (hardcodes legal bar clearance without judicial inquiry)
- `executed pursuant to 28 u.s.c. § 1746` / `executed under 28 u.s.c. § 1746` (unsworn perjury lines)
- `high viability` (emits subjective vanity viability grades)
- `shuttered facility` (invents factual finding of shuttered status)
- `zero observed` / `zero patient traffic` (invents factual findings)
- `estimated recovery` / `recovery potential` (conflates spending with relator bounty)
- `guaranteed` (promises guaranteed outcome or reward)
- `fbi raid` (sensationalizes law enforcement procedures)
- `independent analysis satisfied` (certifies legal standard without agency authority)
- `proves objective` (certifies factual closure rather than logging observations)
- `qualifies for four-door` (emits legacy vanity gating claim)

---

## 3. The 5-Gate Pre-Submission Lock (Browser Co-Pilot & Portal Protocol)

LawBot may use its built-in browser and workspace tools to navigate to official portals (`america.gov/fraud`, `fraud.gov`, agency OIGs, GAO FraudNet, state auditor portals, or FOIA custodians) and auto-fill verified data **IF AND ONLY IF** all 5 gates pass:

1. **Gate 1 — Verification Receipt Complete (`receipt.md`):**
   Target entity credentials (CMS NPPES NPI, USAspending awards, SAM.gov UEI) are confirmed, and all claims are cleanly partitioned into `VERIFIED` vs. `USER_STATED` with 0 vanity scores.
2. **Gate 2 — Lawful Evidence & Privacy Scrubbing (`observe.md`):**
   Evidence respects state recording/wiretap laws (18 U.S.C. § 2511(2)(d) or all-party rules) and public-space observation boundaries. Third-party PII/PHI is scrubbed, and EXIF GPS tags are stripped.
3. **Gate 3 — Route & Insider Clearance (`route.md`):**
   If the user holds non-public insider records for a sealed False Claims Act qui tam case, public portal submission is **strictly blocked** (`portal_export_blocked = true`) to prevent triggering the Public Disclosure Bar (31 U.S.C. § 3730(e)(4)). Instead, route to counsel outreach (`counsel.md`).
4. **Gate 4 — Pre-Export Linter Pass (`packet lint`):**
   Narrative is strictly $\le 1,950$ characters, exhibit hashes are full 64-char SHA-256 strings, and the packet linter reports 0 errors and 0 banned phrases.
5. **Gate 5 — Explicit Final Human Preview & "GO" Confirmation:**
   LawBot displays the complete final text/fields in chat (or displays the filled portal form in the shared browser) and prompts:
   > *"Your packet has passed all verification, state-law, and formatting checks. Here is the exact text ready for [Portal Name]. Would you like me to open the portal in my browser, fill in these fields, and submit it for you now (or pause on the final review screen so you can click Submit yourself)?"*
   Submission proceeds **only** after receiving the user's explicit affirmative instruction.
