# JustLegal OS: Desktop Application & Local Engine Bridge Skill
**Path:** `MCP_Grok/skills/desktop.md`  
**Purpose:** Guides AI agents and GrokBots on how to interface with the JustLegal OS Desktop Application, understand its visual tabs, encrypted vaults, offline PDF tools, and enforce 3-level permission tiers.

---

## 1. Desktop Application Overview

**JustLegal OS** is a local-first legal operating system and court document formatting engine developed by **Just Liberty Incorporated** (501(c)(3) nonprofit public charity | https://truejust.org | https://justlegal.me).

When a user is running the **JustLegal OS Desktop App** (Windows, macOS, Linux):
- They have access to the complete native graphical interface, hardware-accelerated document previews, and an integrated safe MCP stdio server.
- The desktop app connects to the same underlying Rust engine (`justlegalos-cli`) that powers the CLI and MCP stdio protocols.
- Cases live on local disk: `Documents/JustLegal/Cases/{caseId}/`.

---

## 2. Desktop Interface Tabs & Tools

The Desktop Application organizes legal workflows into specialized workspaces:

1. **Portfolio & Matters Tab:**
   - Visual dashboard of active matters, procedural stages, governing venues, and statutory deadlines.
   - Each open case shows the court being looked for, the state court pages to open first, and either the saved court page or a plain statement that it has not been found. A saved page is the local rule for that case. It is not a due date unless the page states one.
   - 1-call case orientation snapshot (`case context`), which includes `court_doors` for that case.
   - Order Decoder: Ingests court order PDFs, extracts operative rulings, and calculates response deadlines with jurisdictional tolling math.

2. **Court Document Assembly & Pleading Studio:**
   - 28-line numbered legal pleading compiler with boxed or classic caption layouts.
   - Sourced citation and court rules verification across jurisdictional rule packs (`nj-app-div`, `co-mandamus`, `generic-us-appellate`).
   - Export to clean, court-shaped Word (`.docx`) and Typst-compiled PDFs.

3. **Whistleblower & Fraud Dossier Studio (DOJ WHIP):**
   - 5-Gate pre-submission pipeline with visual progress indicators.
   - Automated EXIF GPS stripping and SHA-256 cryptographic provenance logging.
   - Interactive Route Hold decision tree (insider records vs. citizen observer posture).
   - Turnkey portal calling card, narrative compiler (<= 1,950 characters), and Exhibit 1–4 bundle.

4. **Forms Catalog & Offline PDF Studio:**
   - On-demand forms packs from the verified JustLegal catalog (downloaded pack-by-pack on user request; once downloaded, forms operate 100% offline).
   - Local PDF AcroForm inspector and form-filler.
   - Client-side zero-leakage PDF redactor (scrubs SSNs, DOBs, phone numbers without cloud roundtrips).
   - Bates stamping, PDF merger, and page splitter.

5. **Encrypted Vault:**
   - Local vault storage for sensitive and whistleblower matters.
   - Cryptographic engine: **XChaCha20-Poly1305** authenticated symmetric encryption with **Argon2id** password hashing.
   - Strict Airgap: Any case marked `.sealed` or `mode: confidential` is locked against network sync and cloud backups.

---

## 3. Three-Level Permission Tiers for AI Agents

To ensure absolute client safety, all agent actions must respect three permission tiers:

| Tier | Category | Allowed Actions | Agent Permissions |
| :--- | :--- | :--- | :--- |
| **Tier 1** | **Read-Only / Safe Mode** | `case context`, `jurisdiction find`, `jurisdiction doors`, `whip receipt`, `whip gaps`, `packet lint`, `records find`, `check state-traps`, `capabilities`, `doctor` | **Autonomous:** Agent may call freely without prior human confirmation. |
| **Tier 2** | **Local Authoring** | `case create`, `draft import`, `whip prepare`, `records draft`, `export court-md`, `pdf redact`, `unmask` | **Permitted with User Consent:** Agent may execute after stating intended action. |
| **Tier 3** | **Protected Human Action** | `whip approve` (Gate 5 GO), `case seal`, external filing, cloud sync | **STRICTLY FORBIDDEN TO BOTS:** Must be executed directly by the human in the UI or terminal. |

---

## 4. The Golden Boundary

- **Court Filings (CM/ECF):** 100% preparation, 0% filing. Rule 11 requires human filing.
- **Fraud Submissions (america.gov/fraud):** 5-gate pipeline must be complete. The bot never approves Gate 5. A person types `--confirm-phrase` in a terminal, or uses the desktop button. `--passphrase` and `JUSTLEGAL_GATE5_PHRASE` are refused. After the person pastes the packet and receives an agency confirmation number, record it with `whip submit`.
