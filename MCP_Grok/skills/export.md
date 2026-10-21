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
