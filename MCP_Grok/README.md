# JustLegal Assistant: GrokBot Integration & Modular Skills
Developed by **Just Liberty Incorporated** (501(c)(3) Nonprofit Public Charity)

This directory contains the public recipe, system prompt, and 8 modular on-disk skills for running **JustLegal Assistant** on x.ai GrokBot and connecting it to **JustLegal OS** (`JustLegalOS.exe` / `justlegalos-cli`).

---

## Operating Philosophy: "The Valuable Object is a Refusal, Not a Grade"

JustLegal OS replaces artificial 0–100 scores, letter grades, and synthetic legal certifications with an honest, unembellished **4-State Verification Receipt**:
- `VERIFIED`: Confirmed against authoritative public registries (CMS NPPES NPI Registry, USAspending API).
- `USER_STATED`: Stated directly by the reporter (e.g., observation notes, dates, target program nexus).
- `MISSING`: Field is required or relevant but has not yet been supplied.
- `LOOKUP_FAILED`: Public registry query failed due to network unavailability or non-match.

When required facts, workstation terms acceptance, or insider gate answers are missing, the system **fails closed** and refuses export.

---

## The 8 Modular Skills

The assistant's capabilities are split into 8 focused, single-purpose on-disk skills:

| Skill | Path | Description |
|---|---|---|
| **Start** | `skills/start.md` | Conversational intake and phone-safe mode. Classifies matter (Civil vs. Fraud) and explains terms gate. |
| **Civil** | `skills/civil.md` | Civil court motions, appeals, and court rule verification without inventing citations or deadlines. |
| **Lookup** | `skills/lookup.md` | Live public registry queries (CMS NPPES & USAspending). Never uploads case files. |
| **Observe** | `skills/observe.md` | 6 universal observation rules (sidewalk only, no trespass, no audio recording, no social media, DOJ Endnote 2). |
| **Receipt** | `skills/receipt.md` | 4-state verification receipt (`whip receipt`) and fail-closed gap audit (`whip gaps`). |
| **Route** | `skills/route.md` | Multi-track route comparison (DOJ WHIP vs. Civil Qui Tam vs. OIG) and mandatory insider gatekeeper fork (`whip route-hold`). |
| **Counsel** | `skills/counsel.md` | Retained counsel consultation package and conflict-check roster (`whip conflict-roster`) containing party names only. |
| **Export** | `skills/export.md` | Fail-closed turnkey export (`whip prepare`), 1,950-character portal limit, EXIF GPS stripping, and packet linter (`packet lint`). |

---

## The 3 Operating Environments

| Environment | Mode | Capabilities & Safety Boundary |
|---|---|---|
| **Local Desktop / CLI** | Full Local Mode | Local workspace access via `JustLegalOS-CLI.exe` or desktop app. Cryptographic SHA-256 evidence hashing, EXIF stripping, and encrypted vault storage. Hard airgap blocks network sync for confidential matters. |
| **Headless Linux Container** | Container CLI Mode | Runs in an automated container with `justlegalos-cli` installed. Executes live public data cross-checks (`whip receipt`, `jurisdiction status`). Never ingests non-public insider records into cloud containers. |
| **Mobile / Phone Chat** | Phone-Safe Mode | Bot runs in standard chat without terminal access. Remains strictly conversational (`skills/start.md`, `skills/lookup.md`). Never asks phone users to run curl commands or compute manual hashes. |

---

## Launch Recipe

- **Recipe File**: `grok_recipe.json`
- **System Prompt**: `system_prompt.md`
- **Skills Directory**: `skills/`
