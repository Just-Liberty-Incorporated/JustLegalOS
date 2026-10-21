# Skill: Enforcement Route Comparison & Insider Gatekeeper

## Purpose
Evaluates routing pathways under 28 U.S.C. § 530C(b)(1)(L) (DOJ Pilot Program), 31 U.S.C. §§ 3729–3733 (Civil False Claims Act), and the Inspector General Act.
Enforces the mandatory Insider vs. Outsider fork.

## The Insider vs. Outsider Fork (`whip route-hold`)
Before any public portal export can proceed, the user must answer whether they possess non-public internal corporate or agency records:
- **Outsider (`insider=false`)**:
  - Reporter is an exterior citizen observer relying on public records and exterior site observations.
  - Public portal submission to `america.gov/fraud` is unblocked.
- **Insider (`insider=true`)**:
  - Reporter possesses internal billing logs, employee emails, patient lists, or non-public audits.
  - **PUBLIC PORTAL EXPORT IS STRICTLY BLOCKED.**
  - Why: Filing non-public materials into an open agency portal can forfeit statutory relator standing, violate mandatory court seal requirements under 31 U.S.C. § 3730(b)(2), and alert wrongdoers.
  - Permitted output: Factual index for licensed counsel and conflict-check roster (names only).

## Multi-Track Statutory Comparison (`whip compare-routes`)
1. **DOJ Criminal Tip Rewards Pilot (`america.gov/fraud`)**:
   - Authority: 28 U.S.C. § 530C(b)(1)(L).
   - Citizen tip portal; no retained counsel required.
   - For tips leading to qualifying criminal charges or forfeiture.
2. **Civil False Claims Act Qui Tam Action**:
   - Authority: 31 U.S.C. §§ 3729–3733.
   - Pro se litigants cannot file qui tam actions; MUST be represented by licensed relator counsel.
   - Must be filed under in camera seal in federal district court.
   - Relator share: 15% to 30% of recovered proceeds upon success.
   - Sovereign Immunity: State agencies are not "persons" subject to qui tam (Vermont ANR v. Stevens, 529 U.S. 765).
3. **Inspector General Administrative Oversight**:
   - Authority: Inspector General Act of 1978.
   - Online hotlines (HHS-OIG, DOL-OIG, GAO FraudNet).
   - Administrative remedy; no private financial recovery.
