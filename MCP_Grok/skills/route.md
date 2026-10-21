# Skill: Multi-Track Statutory Routing & Insider Gatekeeper

## Purpose
Evaluates legal routing pathways under the DOJ Criminal Tip Rewards Pilot Program (Directive 26-12 / 28 U.S.C. § 530C(b)(1)(L)), the Civil False Claims Act (31 U.S.C. §§ 3729–3733), and the Inspector General Act.
Enforces the mandatory Insider vs. Outsider Gatekeeper fork before export.

---

## 1. The Mandatory Insider vs. Outsider Fork (`whip route-hold`)

Before any public portal submission or export can proceed, the user must state whether they possess non-public internal corporate or agency records:

### Outsider Track (`insider=false`)
- **Profile:** Citizen observer, community member, or independent researcher relying on public records (CMS NPPES, USAspending) and public exterior observations.
- **Allowed Output:** Turnkey public portal paste block for `america.gov/fraud` / `fraud.gov` and `WHIP_Evidence_Packet.pdf`.
- **Portal Status:** Public submission unblocked.

### Insider Track (`insider=true`)
- **Profile:** Current or former employee, contractor, or insider holding non-public billing logs, patient lists, internal emails, or confidential audits.
- **PUBLIC WEB PORTAL EXPORT IS STRICTLY BLOCKED (`portal_export_blocked = true`).**
- **Why:** Filing non-public workplace documents into an open web portal compromises the mandatory federal court seal under 31 U.S.C. § 3730(b)(2), forfeits First-to-File relator standing, and alerts the target.
- **Allowed Output:** 
  1. A zero-narrative Conflict-Check Roster (`whip conflict-roster`) containing entity names, NPIs, and jurisdiction only (to allow law firms to run conflict checks under ABA Model Rule 1.18).
  2. An encrypted factual evidence index for licensed contingency relator counsel.

### Culpable Participant Disqualification Warning
- **Legal Bar:** Under DOJ WHIP program terms and federal whistleblower principles, individuals who planned, initiated, or actively directed the fraudulent scheme are disqualified from receiving whistleblower awards.
- **Protocol:** If the user was an active participant in the fraud, they must **NOT** submit an unsealed tip or portal filing. Instruct them to seek private criminal defense or relator counsel immediately under attorney-client privilege.

---

## 2. The 4-Track Statutory Decision Matrix

| Track | Forum & Portal | Legal Authority | Typical Requirements | Financial Recovery |
|---|---|---|---|---|
| **Track 1: DOJ WHIP Fast-Track** | `america.gov/fraud`<br>`justice.gov/fraud/whistleblower`<br>`fraud.gov` | 28 U.S.C. § 530C(b)(1)(L);<br>DOJ Directive 26-12 | Direct tip or Independent Analysis. No lawyer needed ($0 upfront). | Discretionary **$10,000 to $30,000** early award following criminal charges. Preserves eligibility for larger stacked awards upon asset forfeiture. |
| **Track 2: Sealed Civil Qui Tam** | U.S. District Court<br>(In Camera Seal) | 31 U.S.C. §§ 3729–3733 | $1M+ fraud; insider records; requires licensed relator counsel ($0 upfront contingency). | **15% to 25%** if DOJ intervenes; **25% to 30%** if DOJ declines (treble damages). Evaluated under DOJ's 60–120 day benefits-fraud review clock. |
| **Track 3: Systemic Agency & State Oversight** | DOL-OIG, HHS-OIG, GAO FraudNet, State Auditor, State AG MFCU | Inspector General Act (5 U.S.C. §§ 401–424); State Sunshine Laws | State agency systemic failure (*Stevens* bar prevents qui tam); regulatory non-compliance. | Administrative reform, cost disallowance, or state restitution (no relator share). |
| **Track 4: 100% Anonymous Watchdog** | Agency OIG Hotlines | Inspector General Act (5 U.S.C. §§ 401–424) | Citizen seeks zero public exposure and zero reward. | PII and device metadata scrubbed; zero financial reward. |

---

## 3. The 8 Covered Departments & 4 Priority Areas (DOJ WHIP)

The DOJ WHIP program (backed by a $25 Million Treasury commitment) offers early-stage rewards for fraud impacting:
- **8 Covered Federal Departments:** Health and Human Services (HHS), Department of Labor (DOL / Unemployment Insurance), Department of Agriculture (USDA / SNAP), Housing and Urban Development (HUD), Department of Education (ED), Veterans Affairs (VA), Department of War / Defense (DOD), and the Small Business Administration (SBA).
- **4 Priority Healthcare Sectors:** Hospice, home health care, day care, and special services fraud (including billing for non-existent patients, ghost storefronts, and kickbacks).

---

## 4. Ethical Boundaries & 501(c)(3) Public Charity Model

All JustLegal OS software, GrokBot skills, and CLI tools are developed and provided **100% free of charge** by **Just Liberty Incorporated**, a Colorado 501(c)(3) nonprofit public charity (EIN: `93-3704483`):
- We never charge user fees.
- We never split legal fees or take a percentage of any whistleblower reward or settlement.
- We never sell or monetize case data.
