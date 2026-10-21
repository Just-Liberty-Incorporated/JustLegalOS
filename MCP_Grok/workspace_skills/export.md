# Skill: Fail-Closed Turnkey Export & Packet Linter

## Purpose
Governs the assembly of turnkey export packets (`exports/WHIP_Evidence_Packet.pdf`, `exports/whip_submission.json`), enforces the 1,950-character limit, strips EXIF GPS metadata, and runs the fail-closed packet linter.

## Hard Export Requirements
1. **Character Limit Enforcement**:
   - `america.gov/fraud` limits submissions to 2,000 characters.
   - JustLegal OS enforces a strict ≤ 1,950-character limit on portal narratives.
   - If the user's narrative exceeds 1,950 characters, export is **REFUSED** with an error. The software NEVER silently truncates text.
2. **Cryptographic Hashes**:
   - SHA-256 hashes are computed over the complete byte sequences of exhibit files.
   - Hashes are always emitted as full 64-character lowercase hexadecimal strings.
   - Never truncated to 16 characters. No zero-byte hashes emitted for non-existent files.
3. **EXIF GPS Metadata Stripping**:
   - Photographic exhibits (JPEG/PNG) are inspected and stripped of EXIF GPS coordinates and camera identifying tags prior to hashing.
   - Stripped items are recorded in the manifest.
4. **Dual Verified Footers**:
   - Outsider Export:
     > "This disclosure was assembled by the reporter using JustLegal OS offline verification software. Just Liberty Incorporated does not practice law and does not represent the reporter. Verified against CMS NPPES & USAspending public APIs. Not legal advice."
   - Insider Counsel Export:
     > "This factual record index was compiled offline by the client for legal consultation. Just Liberty Incorporated is not legal counsel. This document is provided to retained counsel to facilitate legal evaluation."

## Packet Linter (`justlegalos-cli packet lint <dir>`)
Scans all generated files in `exports/` before user distribution.
Fails closed (exits code 1) if any forbidden phrase is found:
- `attorney-client privileged evaluation`
- `public disclosure status: clear`
- `executed pursuant to 28 u.s.c. § 1746` / `executed under 28 u.s.c. § 1746`
- `high viability`
- `directive 26-12`
- `shuttered facility`
- `zero observed` / `zero patient traffic`
- `estimated recovery` / `recovery potential`
- `guaranteed`
- `fbi raid`
- `independent analysis satisfied`
- `proves objective`
- `qualifies for four-door`
