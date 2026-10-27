# JustLegal Assistant: GrokBot Integration & Modular Skills
Developed by **Just Liberty Incorporated** (501(c)(3) Nonprofit Public Charity | EIN: `93-3704483`)

This directory contains the public recipe, system prompt, and 15 modular on-disk skills for running **JustLegal Assistant** (v9) on x.ai GrokBot and connecting it to **JustLegal OS** (`JustLegalOS.exe` / `justlegalos-cli`).

---

## Operating Philosophy: "The Valuable Object is a Refusal, Not a Grade"

JustLegal OS replaces artificial 0–100 scores, letter grades, and synthetic legal certifications with an honest, unembellished **4-State Verification Receipt**:
- `VERIFIED`: Confirmed against authoritative public registries (CMS NPPES NPI Registry, USAspending API, SAM.gov).
- `USER_STATED`: Stated directly by the reporter (e.g., observation notes, dates, target program nexus).
- `MISSING`: Field is required or relevant but has not yet been supplied.
- `LOOKUP_FAILED`: Public registry query failed due to network unavailability or non-match.

When required facts, workstation terms acceptance, or insider gate answers are missing, the system **fails closed** and refuses export.

---

## The 15 Modular Skills

The assistant's capabilities are split into focused, single-purpose on-disk skills:

| Skill | Path | Description |
|---|---|---|
| **Start** | `skills/start.md` | Conversational intake and phone-safe mode. Classifies matter (Civil vs. Storefront Fraud vs. State Agency/Contractor). |
| **Setup** | `skills/setup.md` | Workstation checks for Linux workspaces and Windows (AppData, CLI, Documents\\JustLegal\\Cases). Read existing matters first. |
| **Civil** | `skills/civil.md` | Civil court motions, appeals, docket reconciliation, and 1-page Attorney Consultation Dossiers. |
| **Lookup** | `skills/lookup.md` | Live public registry queries (CMS NPPES, USAspending, SAM.gov). Never uploads case files. |
| **Observe** | `skills/observe.md` | Lawful evidence & recording rules (safe public sidewalk observations vs. 1-party wiretap consent under 18 U.S.C. § 2511(2)(d)). |
| **Receipt** | `skills/receipt.md` | 4-state verification receipt (`whip receipt`), fail-closed gap audit (`whip gaps`), and Independent Analysis correlation block (28 U.S.C. § 530C(b)(1)(L) / DOJ WHIP Guidelines). |
| **Records** | `skills/records.md` | The Open Records Lifecycle (FOIA, Privacy Act, CORA, CPRA, OPRA, CO Form B-652, CWC IB-4/ICON logs). |
| **Accountability** | `skills/accountability.md` | 14-Pathway Accountability Map (*Stevens* bar, *Ex parte Young*, ADA Title II, Rehab Act § 504, Civil RICO, Rule 20 Joinder). |
| **Route** | `skills/route.md` | 4-Track statutory routing (DOJ WHIP on america.gov/fraud vs. Sealed Qui Tam vs. OIG Oversight) and mandatory insider gatekeeper fork (`whip route-hold`). |
| **Counsel** | `skills/counsel.md` | Retained counsel package and zero-narrative conflict-check roster (`whip conflict-roster`) under ABA Model Rules 1.7 / 1.18. |
| **Export** | `skills/export.md` | Fail-closed turnkey export (`whip prepare`), 1,950-character portal limit, EXIF GPS stripping, and packet linter (`packet lint`). Gate 5 is person-only. |
| **Unmask** | `skills/unmask.md` | Passive OSINT unmasking of surface clues into verification receipts. |
| **Polish** | `skills/polish.md` | Packet polish without inventing facts or cites. |
| **States** | `skills/states.md` | 50-state integrity checks from bundled data. |
| **Desktop** | `skills/desktop.md` | Windows AppData / Documents\\JustLegal\\Cases workstation path. |

---

## The 3 Operating Environments

| Environment | Mode | Capabilities & Safety Boundary |
|---|---|---|
| **GrokBot Linux Workspace (In Chat)** | In-Session CLI Mode (Option A) | Runs directly in LawBot's built-in Linux computer environment on x.ai. With user consent, installs `justlegalos-cli` to run deterministic checks (`whip receipt`, `qui-tam audit-9b`, `jurisdiction status`), format court packets, and query public APIs live in chat. Never ingests non-public insider records into cloud containers. |
| **Local Desktop / CLI (Your Computer)** | 100% Offline Airgapped Mode (Option B) | Standalone local application (`JustLegal OS` / `JustLegalOS-CLI.exe`) running locally on Windows or macOS. Cryptographic SHA-256 evidence hashing, EXIF GPS stripping, and client-side encrypted vault storage. Hard airgap permanently blocks network sync for sealed matters with zero telemetry. |
| **Mobile / Phone Chat** | Phone-Safe Mode | Bot runs in standard mobile chat without workspace tools. Remains strictly conversational (`skills/start.md`, `skills/lookup.md`). Walks users through legal concepts and explains court procedures without demanding terminal commands or manual hash calculation. |

---

## Primary Verification Links

- **DOJ WHIP Primary Tip Portal:** [https://www.america.gov/fraud](https://www.america.gov/fraud)
- **DOJ WHIP Terms & FAQs:** [https://www.justice.gov/fraud/whistleblower](https://www.justice.gov/fraud/whistleblower)
- **CMS NPPES Provider Registry:** [https://npiregistry.cms.hhs.gov](https://npiregistry.cms.hhs.gov)
- **USAspending Federal Awards:** [https://www.usaspending.gov](https://www.usaspending.gov)
- **SAM.gov Entity Management:** [https://sam.gov](https://sam.gov)
- **JustLegal OS Verification Tools:** [https://justlegal.me/qui-tam](https://justlegal.me/qui-tam)

---

## Launch Recipe

- **Recipe File**: `grok_recipe.json`
- **System Prompt**: `system_prompt.md`
- **Skills Directory**: `skills/` (10 modular skills)
