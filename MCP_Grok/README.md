# JustLegal Assistant: GrokBot Blueprints & Connectors
Developed by **Just Liberty Incorporated** (501(c)(3) Nonprofit)

This directory contains the public templates, system prompts, skills, and bridge configurations for running **JustLegal Assistant** on x.ai GrokBot and connecting it to **JustLegal OS** (`JustLegalOS.exe` / `justlegalos-cli`).

---

## The Two GrokBot Personas

### 1. JustLegal Navigator (General Civil Self-Help & Transactional Dossiers)
- **Target Audience:** Pro se litigants, tenants, consumers, and families navigating civil court cases, appeals, and routine legal instruments.
- **Recipe & Prompt:** `bot_civil_recipe.json` & `system_prompt_civil.md`
- **Skills (`MCP_Grok/skills/`):**
  - `skill_order_decoder.md`: Parsing minute entries, orders, and identifying tolling rules.
  - `skill_record_reconcile.md`: Comparing docket entries against local files to find gaps.
  - `skill_deadline_extension.md`: Structuring motions/stipulations for enlargement of time under court rules.
- **Capabilities:**
  - Plain-language intake and time-sensitive obligation detector.
  - Sourced forms discovery across 22,000+ official court forms on [justlegal.me](https://justlegal.me).
  - Pasted court order analysis and rule-based deadline calculation.
  - Generates 1-2 page structured Consultation Dossiers (`export consultation-dossier`) for unbundled attorney consultations and legal aid clinics.
  - Working document outlines, certificates of service, and attorney referral guides.

### 2. JustLegal Fraud & Whistleblower Navigator (EO 14395 & Qui Tam)
- **Target Audience:** Citizen investigators, healthcare whistleblowers, and relators investigating government program fraud under **Executive Order 14395** (*Executive Order 14395 — Establishing the Task Force to Eliminate Fraud*) and the **False Claims Act (31 U.S.C. §§ 3729–3733)**.
- **Recipe & Prompt:** `bot_fraud_recipe.json` & `system_prompt_fraud.md`
- **Skills (`MCP_Grok/skills/`):**
  - `skill_entity_crosscheck.md`: CMS NPPES NPI and USAspending public verification.
  - `skill_fraud_triage.md`: FRCP Rule 9(b) 6-element scoring rubric (0–100 scale).
  - `skill_observation_protocol.md`: Lawful photographic/field logging protocols.
  - `skill_submission_router.md`: Track A (Qui Tam Counsel First) vs Track B (Direct OIG).
- **Capabilities:**
  - Instant NPI & USAspending entity verification (identifying ghost clinics and shell companies).
  - Generates the machine-readable calling card header:
    `JUSTLEGAL OS | JUSTLEGAL STANDARDIZED FRAUD DISCLOSURE [SCHEMA v1.0 | EO-14395]`
  - FRCP Rule 9(b) 6-element viability scoring.
  - Strict statutory seal warnings (31 U.S.C. § 3730(b)(2)) prohibiting premature social media publication.
  - HIPAA Whistleblower Safe Harbor guidance (45 C.F.R. § 164.502(j)).
  - Direct routing to The Anti-Fraud Coalition (TAF), State Medicaid Fraud Control Units (MFCUs), and DOJ Fraud Section.

---

## The 3-Mode Operating Paradigm

GrokBot can operate across three environments depending on client capabilities:

| Environment | Mode | Capabilities & Safety Boundary |
|---|---|---|
| **Local Desktop** | Mode 1: Local MCP / CLI | Full local workspace access via `JustLegalOS-CLI.exe` or desktop MCP. Cryptographic evidence hashing and local encrypted vault storage. Hard airgap blocks network sync for `.sealed` matters. |
| **Cloud Linux Sandbox** | Mode 2: Headless CLI Container | Bot runs in an automated container with `justlegalos-cli` installed. Executes live public data cross-checks (`verify-entity`, `records-pack`, `audit-9b`). Strict rule: never ingest HIPAA PHI or unfiled qui tam complaints into a cloud sandbox. |
| **Mobile / Pure Web Chat** | Mode 3: Zero-Fabrication Fallback | Bot runs in standard chat without terminal access. **Zero fabrication rule:** never hallucinate SHA-256 hashes, Merkle roots, or fake numeric scores from memory. Directs user to [justlegal.me/qui-tam](https://justlegal.me/qui-tam) for live browser verification. |

---

## How to Create and Share on x.ai GrokBot

1. Log into your x.ai account and navigate to **Grok Bot** (`https://x.ai/bot/create`).
2. Copy the name, description, and instructions from `MCP_Grok/system_prompt_civil.md` (or `system_prompt_fraud.md`).
3. Under Bot Settings, select **Share as Template**. This creates an unpublished draft.
4. Review the draft to ensure no private credentials or internal URLs are included.
5. Click **Publish as Public Link** and copy the resulting `https://x.ai/bot/<id>` link.
6. Share this link on X, GitHub, or embed it on [justlegal.me/fraud-triage](https://justlegal.me/fraud-triage).

---

## How GrokBot Interacts with JustLegal OS

### Option A: Local Desktop (Windows / macOS)
When a user has **JustLegal OS** (`JustLegalOS.exe` / `JustLegalOS-CLI.exe`) installed locally:
1. GrokBot can request authorization to execute read-only diagnostic and document commands:
   ```bash
   .\JustLegalOS-CLI.exe --json doctor
   .\JustLegalOS-CLI.exe --json case context <path>
   .\JustLegalOS-CLI.exe --json check venue <path>
   .\JustLegalOS-CLI.exe --json qui-tam verify-entity --name "<target>"
   .\JustLegalOS-CLI.exe --json qui-tam audit-9b <path>
   ```
2. **Explicit User Approval:** Grok requires the user to approve local command execution on their machine.
3. **Airgap Enforcement:** The engine strictly forbids inspection of `.sealed` or confidential cases, protecting sensitive whistleblower dossiers.

### Option B: Headless Linux / Cloud Container (One-Line Installer)
For cloud-hosted bots or headless Linux containers, install the standalone CLI with zero desktop dependencies:
```bash
curl -sSL https://raw.githubusercontent.com/Just-Liberty-Incorporated/JustLegalOS/main/scripts/install-linux.sh | bash
```
Then start the MCP stdio server (runs in bot-safe mode by default):
```bash
justlegalos-cli mcp
```
By default, high-risk tools like `cloud_pull`, `cloud_push`, `sign_pdf`, and `packs_remove` are completely hidden and blocked, and all file operations are strictly confined to the cases directory. To allow cloud and signature tools, the user must explicitly pass `--unrestricted`.

---

## ⚠️ Important Data Boundary: Cloud Bot vs. Local Desktop

1. **Cloud Bot Environment:** When conversing with a cloud-hosted GrokBot (or any remote AI assistant), the conversation text is processed on external cloud servers. Any cases created via a cloud container live on that container, not on the user's personal computer. Never paste unsealed qui tam allegations, patient medical records, or confidential trade secrets into a cloud chat.
2. **Local Desktop App (`JustLegalOS.exe`):** For confidential, sealed, or sensitive litigation matters (especially False Claims Act / Qui Tam dossiers under 31 U.S.C. § 3730(b)(2)), users must run JustLegal OS locally on their own computer, where files are stored in an encrypted local vault with zero telemetry.
