# Skill: 4-State Verification Receipt & Independent Analysis Correlation

## Purpose
Replaces artificial 0–100 scores and vanity grades with an honest, factual verification receipt, fail-closed gap audit, and the **Independent Analysis Correlation Block** pursuant to DOJ Directive 26-12 and 28 U.S.C. § 530C(b)(1)(L).

---

## 1. Operating Philosophy: "The Valuable Object is a Refusal, Not a Grade"

A submission that fails closed when missing required facts protects the citizen from criminal false-statement exposure (18 U.S.C. § 1001) and protects government investigators from unverified noise.

Every row in the JustLegal OS verification receipt is classified into one of four deterministic states:
1. `VERIFIED`: Confirmed against authoritative public records (CMS NPPES NPI Registry, USAspending API, SAM.gov).
2. `USER_STATED`: Stated directly by the reporter (e.g. observation notes, dates, target program nexus).
3. `MISSING`: Field is required or relevant but has not been supplied.
4. `LOOKUP_FAILED`: Public registry lookup failed due to network unavailability or non-match.

---

## 2. The Independent Analysis Correlation Block (DOJ Directive 26-12)

Under the DOJ Criminal Tip Rewards Pilot Program (Directive 26-12; 28 U.S.C. § 530C(b)(1)(L)), qualifying original information includes **"Independent Analysis"**:
> *"The individual's own analysis, whether done alone or in combination with others. Analysis means the individual's examination and evaluation of information that may be publicly available, but which reveals information that is not generally known or available to the public."*

When public data and on-the-ground observations are correlated, the engine generates the following structured correlation block:

```text
======================================================================
INDEPENDENT ANALYSIS CORRELATION BLOCK (DOJ DIRECTIVE 26-12)
Authority: 28 U.S.C. § 530C(b)(1)(L) | Evaluated via JustLegal OS
======================================================================
1. REGISTERED PUBLIC ATTRIBUTES:
   • Legal Business Name: [VERIFIED: CMS NPPES / SAM.gov]
   • Enumerated NPI: [VERIFIED: 10-Digit Type 1 or Type 2]
   • Primary Taxonomy: [VERIFIED: Provider Classification Description]
   • Practice Address: [VERIFIED: Physical Location on Registry File]
   • Federal Award Obligations: [VERIFIED: USAspending / State Medicaid Nexus]

2. CORRELATED ON-THE-GROUND REALITY:
   • Physical Facility Status: [USER_STATED: Doors locked during posted hours]
   • Observed Signage / DBA: [USER_STATED: Mismatched or absent signage]
   • Distinct Observation Dates: [2+ calendar dates verified by file byte timestamps]

3. EXPOSED ANOMALY / SCHEME SUMMARY:
   • Public registry records active healthcare/daycare billing taxonomy and federal awards,
     which directly conflicts with observed physical non-operation at registered practice site.
======================================================================
```

---

## 3. CLI Commands & Fail-Closed Audit

- `justlegalos-cli --json whip receipt <dir>`:
  Outputs the unembellished tabular receipt of all fields, registry sources, full 64-char SHA-256 hashes, and dates.
- `justlegalos-cli --json whip gaps <dir>`:
  Evaluates blocking gaps vs. informational items. Exits code 1 if blocking gaps exist.

### Blocking Gaps (Export Refused)
- Workstation terms acceptance (`terms_accepted.json`) missing on disk.
- Route hold question unanswered (insider vs. outsider).
- User factual narrative missing or exceeding 1,950 characters.
- Missing target entity legal name.
- Pre-export linter detecting banned phrases or fabricated vanity grades.

### Informational Items (Not Blocking)
- Missing NPI (may be a non-healthcare contractor or vendor).
- $0.00 direct federal awards (may be state-administered Medicaid or sub-awarded grant).
- Observations spanning a single calendar date.
