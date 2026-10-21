# Government Target Intake & The Dual-Track Prism

When a user complains about a state agency, benefit freezes, or government contractors, apply the **Investigative Prism** to split the matter into viable legal pathways.

## 1. The Legal Reality: State Sovereign Immunity
When a citizen says: *"The State Unemployment Agency / Health Department froze my account and stole billions! I want to sue the State for fraud!"*

The bot must explain the statutory barriers in plain English:
1. **The State Agency Cannot Be Sued for Qui Tam Money:** Under *Vermont Agency of Natural Resources v. United States ex rel. Stevens*, 529 U.S. 765 (2000), a state or state agency is not a "person" under the False Claims Act and cannot be sued for qui tam treble damages.
2. **No § 1983 Damages Against State Agencies:** Under *Will v. Michigan Dept. of State Police*, 491 U.S. 58 (1989), state agencies have Eleventh Amendment sovereign immunity from money damages in federal court.

## 2. The Two-Weapon Solution
Split the user's grievance into two actionable tracks:

```text
       USER: "The state agency locked my account and wasted billions!"
                                     │
                 ┌───────────────────┴───────────────────┐
                 ▼                                       ▼
  WEAPON 1: THE PRIVATE CONTRACTOR         WEAPON 2: THE STATE AGENCY
    (Qui Tam / Treble Damages Track)         (Systemic Oversight Track)
  • Target: Tech or billing vendor          • Target: State agency leadership
  • Law: 31 U.S.C. § 3729                   • Law: Ex parte Young (§ 1983 inj.)
  • Claims: False certifications of          • Remedy: Stop the hold; open files
    system performance or billing fraud       under 20 CFR § 603.5(d); OIG tip
```

## 3. The 74-Point Rule 9(b) Cap
For Qui Tam claims against private contractors:
- Under FRCP Rule 9(b), claims must plead the "who, what, when, where, and how" of false billing.
- If the user has observed system bugs or account freezes but does **not** have physical copies of contractor invoices, vouchers, or statements of work deposited in `evidence/`, the Qui Tam readiness score is **strictly capped at 74/100 (Developing)**.
- Guide the user to acquire public contracts, invoices, and SLA defect logs via statutory public records requests (`justlegalos-cli records draft`).
