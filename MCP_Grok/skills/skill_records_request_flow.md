# Skill: Statutory Open Records Suite & Date Math Flow

## Purpose
Automate discovery, drafting, tracking, and follow-up for public records (CORA, FOIA) and confidential claimant records (20 CFR § 603.5(d)).

---

## 1. Flow Overview

```text
[Citizen Observation]
        │
        ▼
1. records find ───► Discover governing law, custodian email/portal, statutory deadline
        │
        ▼
2. records draft ──► Generate formal letter with 501(c)(3) fee waiver justification
        │
        ▼
3. records track ──► Working-day calendar calculation (skips weekends & federal holidays)
        │
   ┌────┴───────────────────────────┐
   ▼                                ▼
[On-Time Production]         [Overdue / Denied / Wrong-Route]
   │                                │
   ▼                                ▼
evidence add                 4. records followup ──► Late Notice, Admin Appeal,
                                                     or Own-Claim Correction
```

---

## 2. Execution Commands

1. **Discover Regime:**
   ```bash
   # For contractor contracts or audits in Colorado:
   justlegalos-cli --json records find --jurisdiction CO --type contract

   # For individual UI claimant files:
   justlegalos-cli --json records find --jurisdiction CO --type ui-claim-file

   # For federal Department of Labor or OIG:
   justlegalos-cli --json records find --jurisdiction US --type foia
   ```

2. **Draft Statutory Request Letter:**
   ```bash
   justlegalos-cli --json records draft ./my-case \
     --jurisdiction CO \
     --type contract \
     --fee-waiver
   ```

3. **Track Request with Working-Day Math:**
   ```bash
   # Automatically computes statutory due date (3 working days for CORA) and extension date (7 working days)
   justlegalos-cli --json records track ./my-case \
     --id "CORA-2026-001" \
     --law "CORA" \
     --agency "CDLE" \
     --sent "2026-10-20" \
     --days 3 \
     --ext 7 \
     --tracking "USPS-9400111899562"
   ```

4. **Draft Statutory Follow-up / Appeal:**
   ```bash
   # If deadline passed without response:
   justlegalos-cli --json records followup ./my-case \
     --id "CORA-2026-001" \
     --law "CORA" \
     --agency "CDLE" \
     --kind late

   # If agency improperly refused citing unemployment confidentiality:
   justlegalos-cli --json records followup ./my-case \
     --id "CORA-2026-001" \
     --law "CORA" \
     --agency "CDLE" \
     --kind wrong-route
   ```
