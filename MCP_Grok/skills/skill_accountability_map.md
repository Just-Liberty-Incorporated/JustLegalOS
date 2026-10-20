# Skill: 14-Pathway Accountability Map & Viability Triage

## Purpose
Map any grievance against a government agency or its contractors across all 14 legal and administrative accountability routes, computing viability grades and delivering the "Best Next 3 Moves".

---

## 1. When to Run
* Immediately after case creation or bootstrap to show the citizen their full landscape of options.
* Whenever an individual appeal deadline has passed, to prevent the citizen from giving up.
* Before drafting any administrative complaint or motion.

---

## 2. Execution Command

```bash
justlegalos-cli --json accountability map ./my-case \
  --state CO \
  --program ui \
  --target-type gov-contractor \
  --role claimant \
  --key-date "2026-01-15"
```

---

## 3. The 4 Viability Grades

* **`Solid`**: Strong statutory authority, active timeline, meets all procedural prerequisites.
  * *Example:* CORA request for public contract/SOW (`crs_24-72-201`), DOL-OIG fraud hotline report (`dol-oig`).
* **`Fixable`**: Viable theory, but currently blocked by missing factual records.
  * *Example:* FCA Qui Tam against tech contractor is `Fixable` if no invoice is yet in `evidence/` (Run `records draft` to cure).
* **`Long shot`**: Overcoming high immunity or discretionary review.
  * *Example:* Discretionary audit request to State Auditor; § 1983 damage suit against individual supervisor.
* **`Dead end`**: Absolute statutory or jurisdictional bar.
  * *Example:* FCA qui tam against the State agency under *Stevens*, 529 U.S. 765; direct UI benefit appeal filed on Day 25 when the hard cutoff is 20 days.

---

## 4. The "Best Next 3 Moves" Output
The map synthesizes the viability matrix into 3 sequenced, non-conflicting actions:
1. **Immediate Discovery:** Issue public records requests for prime contract and billing invoices (`records draft`).
2. **Firsthand Perfection:** Obtain claimant's own file under 20 CFR § 603.5(d) with fraud-flag metadata.
3. **Dual Submission:** Deliver systemic waste findings to regulatory receivers (`export receiver`) while perfecting contractor qui tam claims under seal.
