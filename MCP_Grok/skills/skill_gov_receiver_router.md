# Skill: Regulatory Receiver Routing & Auto-Fill Formatting

## Purpose
Convert case evidence and findings into the exact intake requirements and character limits published by federal and state regulatory receivers.

---

## 1. Supported Receivers (P13)

| Receiver ID | Agency Title | Intake Limits & Portal | Best Fit For |
|---|---|---|---|
| `dol-oig` | U.S. DOL Office of Inspector General | eCasePortal (https://www.oig.dol.gov/hotline.htm); 3,000 char narrative | Federal grant fraud, UI contractor non-performance (ALN 17.225) |
| `co-osa` | Colorado Office of the State Auditor | Fraud Hotline (C.R.S. § 2-3-110.5); 2,500 char narrative | Systemic waste, state agency mismanagement, contractor defect cover-ups |
| `gao-fraudnet` | U.S. Government Accountability Office | FraudNet Portal (https://www.gao.gov/fraudnet); 2,000 char narrative | Multi-state systemic tech failures across federal assistance funds |
| `counsel` | Prospective Qui Tam Whistleblower Counsel | Privileged intake block under ABA Model Rule 1.18 | Treble damages qui tam lawsuits against private government contractors |

---

## 2. Execution Command

```bash
# Format for DOL-OIG:
justlegalos-cli --json export receiver ./my-case --to dol-oig --anonymity Confidential

# Format for Colorado State Auditor:
justlegalos-cli --json export receiver ./my-case --to co-osa

# Format for Whistleblower Counsel:
justlegalos-cli --json export receiver ./my-case --to counsel
```

---

## 3. Output Payload Structure
The formatter returns:
* `receiver_title`: Formal title of recipient office.
* `portal_or_email`: Direct web portal URL or email address.
* `fields`: Structured key-value fields respecting character limits.
* `complete_paste_text`: Monolithic pre-formatted paste block ready for copy-pasting.
* `attachment_manifest`: List of cleaned, indexed exhibit files to upload with submission.
