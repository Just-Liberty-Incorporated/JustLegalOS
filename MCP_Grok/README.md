# JustLegal Assistant: GrokBot Integration & Modular Skills
Developed by **Just Liberty Incorporated** (501(c)(3) Nonprofit Public Charity | EIN: `93-3704483`)

This directory contains the public recipe, system prompt, and 10 modular on-disk skills for running **JustLegal Assistant** on x.ai GrokBot and connecting it to **JustLegal OS** (`JustLegalOS.exe` / `justlegalos-cli`).

---

## Operating Philosophy: "The Valuable Object is a Refusal, Not a Grade"

JustLegal OS replaces artificial 0–100 scores, letter grades, and synthetic legal certifications with an honest, unembellished **4-State Verification Receipt**:
- `VERIFIED`: Confirmed against authoritative public registries (CMS NPPES NPI Registry, USAspending API, SAM.gov).
- `USER_STATED`: Stated directly by the reporter (e.g., observation notes, dates, target program nexus).
- `MISSING`: Field is required or relevant but has not yet been supplied.
- `LOOKUP_FAILED`: Public registry query failed due to network unavailability or non-match.

When required facts, workstation terms acceptance, or insider gate answers are missing, the system **fails closed** and refuses export.

---

## The 10 Modular Skills

The assistant's capabilities are split into 10 focused, single-purpose on-disk skills:

| Skill | Path | Description |
|---|---|---|
| **Start** | `skills/start.md` | Conversational intake and phone-safe mode. Classifies matter (Civil vs. Storefront Fraud vs. State Agency/Contractor). |
| **Civil** | `skills/civil.md` | Civil court motions, appeals, docket reconciliation, and 1-page Attorney Consultation Dossiers. |
| **Lookup** | `skills/lookup.md` | Live public registry queries (CMS NPPES, USAspending, SAM.gov). Never uploads case files. |
| **Observe** | `skills/observe.md` | Lawful evidence & recording rules (safe public sidewalk observations vs. 1-party wiretap consent under 18 U.S.C. § 2511(2)(d)). |
| **Receipt** | `skills/receipt.md` | 4-state verification receipt (`whip receipt`), fail-closed gap audit (`whip gaps`), and Independent Analysis correlation block (DOJ Directive 26-12). |
| **Records** | `skills/records.md` | The Open Records Lifecycle (FOIA, Privacy Act, CORA, CPRA, OPRA, CO Form B-652, CWC IB-4/ICON logs). |
| **Accountability** | `skills/accountability.md` | 14-Pathway Accountability Map (*Stevens* bar, *Ex parte Young*, ADA Title II, Rehab Act § 504, Civil RICO, Rule 20 Joinder). |
| **Route** | `skills/route.md` | 4-Track statutory routing (DOJ WHIP on america.gov/fraud vs. Sealed Qui Tam vs. OIG Oversight) and mandatory insider gatekeeper fork (`whip route-hold`). |
| **Counsel** | `skills/counsel.md` | Retained counsel package and zero-narrative conflict-check roster (`whip conflict-roster`) under ABA Model Rules 1.7 / 1.18. |
| **Export** | `skills/export.md` | Fail-closed turnkey export (`whip prepare`), 1,950-character portal limit, EXIF GPS stripping, and packet linter (`packet lint`). |

---

## The 3 Operating Environments

| Environment | Mode | Capabilities & Safety Boundary |
|---|---|---|
| **Local Desktop / CLI** | Full Local Mode | Local workspace access via `JustLegalOS-CLI.exe` or desktop app. Cryptographic SHA-256 evidence hashing, EXIF stripping, and encrypted vault storage. Hard airgap blocks network sync for confidential matters. |
| **Headless Linux Container** | Container CLI Mode | Runs in an automated container with `justlegalos-cli` installed. Executes live public data cross-checks (`whip receipt`, `jurisdiction status`). Never ingests non-public insider records into cloud containers. |
| **Mobile / Phone Chat** | Phone-Safe Mode | Bot runs in standard chat without terminal access. Remains strictly conversational (`skills/start.md`, `skills/lookup.md`). Never asks phone users to run curl commands or compute manual hashes. |

---

## Primary Verification Links

- **DOJ WHIP Primary Tip Portal:** [https://www.america.gov/fraud](https://www.america.gov/fraud)
- **DOJ WHIP Terms & FAQs:** [https://www.justice.gov/fraud/whistleblower](https://www.justice.gov/fraud/whistleblower)
- **Federal Fraud Clearinghouse:** [https://www.fraud.gov](https://www.fraud.gov)
- **CMS NPPES Provider Registry:** [https://npiregistry.cms.hhs.gov](https://npiregistry.cms.hhs.gov)
- **USAspending Federal Awards:** [https://www.usaspending.gov](https://www.usaspending.gov)
- **SAM.gov Entity Management:** [https://sam.gov](https://sam.gov)
- **JustLegal OS Verification Tools:** [https://justlegal.me/qui-tam](https://justlegal.me/qui-tam)

---

## Launch Recipe

- **Recipe File**: `grok_recipe.json`
- **System Prompt**: `system_prompt.md`
- **Skills Directory**: `skills/` (10 modular skills)
