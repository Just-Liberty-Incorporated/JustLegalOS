# Skill: Public Disclosure Bar Audit & Original Source Grounding

## Purpose
Audit case evidence against the False Claims Act Public Disclosure Bar (31 U.S.C. § 3730(e)(4)) and Supreme Court precedent (*Schindler Elevator Corp. v. U.S. ex rel. Kirk*, 563 U.S. 401 (2011)).

---

## 1. The Legal Doctrine

Under 31 U.S.C. § 3730(e)(4):
* A court must dismiss an FCA qui tam action if substantially the same allegations or transactions were publicly disclosed in a federal hearing, audit, report, or in the news media.
* **The FOIA Trap (*Schindler Elevator*, 2011):** The Supreme Court held that written agency responses to FOIA requests (and state open records like CORA) are "reports" under the Public Disclosure Bar.
* **The Original Source Exception:** A relator overcomes the bar ONLY IF they are an **"original source"**—an individual who:
  1. Prior to public disclosure, voluntarily disclosed the information to the Government; OR
  2. Possesses knowledge that is **independent of and materially adds to** the publicly disclosed allegations.

---

## 2. Evidence Categorization for Original Source Compliance

| Evidence Class | Source Code | Qualifies as Independent Knowledge? | Role in Case |
|---|---|---|---|
| Claimant File / Audit Trail | `own-claim-file` | **YES** (Personal claimant experience with fraud flag & examiner notes) | Core firsthand basis |
| Internal Whistleblower Memo | `internal-memo` | **YES** (Direct workplace observation of contractor non-performance) | Core firsthand basis |
| Physical Inspection / Photo | `field-observation` | **YES** (Direct physical observation documented under hash) | Core firsthand basis |
| CORA / FOIA Production | `records-request` | **NO** (Public disclosure under *Schindler Elevator*) | Corroborating framework / SOW |
| News / Google Searches | `public-search` | **NO** (Public media disclosure) | Background context only |

---

## 3. Auditing with `qui-tam audit-9b`

When running `audit-9b`:
```bash
justlegalos-cli --json qui-tam audit-9b ./my-case
```

The engine checks Element 6 (Original Source & Public Disclosure Check):
* If the case relies solely on public audits or CORA responses without any firsthand claimant file, employee memo, or field observation, Element 6 will flag:
  `"Lacks verified independent firsthand observation; relator risks dismissal under 31 U.S.C. § 3730(e)(4) and Schindler Elevator Corp. v. U.S. ex rel. Kirk."`
* Next Step: Deposit claimant's own file (`20 CFR § 603.5(d)`) or direct inspection evidence to establish independent relator standing.
