# Skill: 3/4-State Verification Receipt & Gap Analysis

## Purpose
Replaces artificial 0–100 scores and vanity grades with an honest, factual verification receipt and fail-closed gap audit.

## Philosophy: "The valuable object is a refusal, not a grade"
A submission that fails closed when missing required facts protects the citizen from criminal false-statement exposure (18 U.S.C. § 1001) and protects the court/agency from baseless filings.

## 4 Verification States
Every row in the JustLegal OS verification receipt is classified into one of four deterministic states:
1. `VERIFIED`: Confirmed against authoritative public records (CMS NPPES NPI Registry, USAspending API).
2. `USER_STATED`: Stated directly by the reporter (e.g. observation notes, dates, target program nexus).
3. `MISSING`: Field is required or relevant but has not been supplied.
4. `LOOKUP_FAILED`: Public registry lookup failed due to network unavailability or non-match.

## CLI Commands
- `justlegalos-cli whip receipt <dir>`:
  Outputs the unembellished tabular receipt of all fields, registry sources, full SHA-256 hashes, and dates.
- `justlegalos-cli whip gaps <dir>`:
  Evaluates blocking gaps vs. informational items. Exits code 1 if blocking gaps exist.

### Blocking Gaps (Export Disabled)
- Terms acceptance (`terms_accepted.json`) missing on workstation.
- Route hold question unanswered.
- User factual narrative missing or exceeding 1,950 characters.
- Missing target entity name.
- Packet linter detecting banned legal assertions or vanity phrases.

### Informational Items (Not Blocking)
- Missing NPI (may be a non-healthcare vendor or contractor).
- $0.00 direct federal awards (may be state-administered Medicaid or sub-awarded grant).
- Exhibits spanning a single calendar date.
