# Skill: Open Records Pre-Filing Discovery Lifecycle

## Purpose
Guides self-represented litigants, citizen researchers, and prospective whistleblowers through the lawful acquisition of public contracts, IT vendor amendments, and agency administrative records prior to litigation or disclosure.

---

## 1. Statutory Transparency Regimes

The engine categorizes records requests by jurisdiction and legal authority:

| Regime | Governing Statute | Scope & Best Use | Statutory Baseline Response Window |
|---|---|---|---|
| **Federal FOIA** | 5 U.S.C. § 552 | Prime federal contracts, grant applications, federal audit reports (HHS, DOL, SBA). | 20 working days (subject to +10-day extension for unusual circumstances). |
| **Federal Privacy Act** | 5 U.S.C. § 552a | First-party records held by federal agencies (own benefit claims, wage logs). | 30 calendar days baseline. |
| **Medicaid Claims & Capitation** | 45 C.F.R. § 164.524 & 42 C.F.R. § 431.60 | Complete MMIS paid/denied claims ledger, 10-digit NPIs, and monthly MCO capitation payments. | 30 calendar days mandatory federal baseline ($0 for electronic delivery). |
| **Claimant UI Own-File Access** | 20 C.F.R. § 603.5(c) / State Forms | Unemployment insurance files, complete employer wage records, internal examiner notes, IB-4 logs. | Administrative schedule (free to claimant; check state forms like CO B-652, NC DESLEGAL 100). |
| **SNAP / TANF Casefile Access** | 7 C.F.R. § 273.15(p)(1) | Complete household casefile examination prior to or during fair hearings. | Prior to administrative proceedings. |
| **State Open Records (CORA, OPRA, etc.)** | State Sunshine Acts | Public vendor contracts, IT amendments, facility health/fire inspection records. | State statutory windows (e.g. CO 3 days, NJ 7 days, CA 10 days). |

> **Critical Stage 1 vs. Stage 4 Firewall:** Personal records demands (Medicaid, UI, SNAP) must be directed exclusively to administrative records custodians / HIPAA Privacy Officers. NEVER submit a personal records request to an OIG or MFCU fraud hotline (such as MedicaidFraud@osc.nj.gov), as premature reporting damages legal standing and blows whistleblower confidentiality prior to receiving the claims ledger.

> **Statutory Clock Notice:** Statutory deadlines are initial baseline requirements under state and federal law. In practice, agencies frequently assert statutory extensions, tolling for scope clarification, or backlog delays. The engine logs baseline dates without certifying judicial enforcement.

---

## 2. The 4 CLI Open Records Commands

Workstation users operate the lifecycle through deterministic CLI handlers:

```bash
# 1. Discover relevant public records categories and statutory citations
justlegalos-cli --json records find --jurisdiction CO --type contract

# 2. Draft formal records request with 501(c)(3) public interest fee waiver
justlegalos-cli --json records draft ./my-case --jurisdiction CO --type contract --fee-waiver

# 3. Track request submission, custodian contact, and calculate statutory calendar window
justlegalos-cli --json records track ./my-case --id CORA-01 --agency "CDLE Custodian" --sent 2026-03-01 --days 3

# 4. Generate follow-up inquiry when baseline window expires without response
justlegalos-cli --json records followup ./my-case --id CORA-01 --agency "CDLE Custodian" --kind late
```

---

## 3. Public Interest Fee Waiver Protocol (501(c)(3))

Under FOIA (5 U.S.C. § 552(a)(4)(A)(iii)) and state sunshine laws, documents must be furnished without charge or at a reduced rate if disclosure of the information is in the public interest:
1. **Public Understanding:** Disclosure significantly contributes to public understanding of the operations or activities of the government.
2. **Non-Commercial:** The request is not primarily in the commercial interest of the requester.
3. **Nonprofit Mission:** Requesters may invoke Just Liberty Incorporated's 501(c)(3) public charity status (EIN: `93-3704483`) when conducting objective, non-commercial public interest research.

---

## 4. 4-State Verification Receipt Integration

All tracked records requests are integrated into the case verification receipt:
- `VERIFIED`: Official agency tracking/acknowledgment number received, or responsive records produced.
- `USER_STATED`: Date sent, targeted custodian, and request description asserted by the user.
- `MISSING`: Responsive records not yet produced, or custodian contact unconfirmed.
- `LOOKUP_FAILED`: Agency formally denied request or asserted statutory exemption (e.g. deliberative process, trade secret).

*Zero vanity viability scores. All response windows tracked with calendar math that accounts for weekends and official federal/state legal holidays.*
