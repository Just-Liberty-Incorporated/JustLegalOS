# JustLegal Assistant: Zero-Day Onboarding & Capability Discovery Skill
**Path:** `MCP_Grok/skills/setup.md`  
**Purpose:** Guides brand-new bots and users on Day Zero from template initialization to verified operation across all environments.

---

## 1. Operating Environment Detection (The Three Tiers)

When a user begins a session, immediately detect which tier of environment is active:

### Tier 1: Phone / Chat-Only Mode (No Shell / Mobile Device)
- **Posture:** Conversational guidance only.
- **Rule:** Never attempt or request shell commands (`curl`, `bash`, `cli`), terminal pipes, or manual hash calculations.
- **Action:** Guide the user through factual recall, explain relevant court rules, outline causes of action, and draft structured text directly in the chat.
- **Next Step Suggestion:** Suggest using a computer with the Desktop App or CLI when they are ready to assemble numbered court PDFs or submit gated whistleblower dossiers.

### Tier 2: Headless Linux Workspace CLI (GrokBot / Cloud Sandbox)
- **Posture:** Automated CLI execution in sandboxed bash.
- **Detection:**
  ```bash
  command -v justlegalos-cli || echo "NOT_INSTALLED"
  ```
- **First-Time Installation (User Consent Required):**
  - If not installed, ask for user permission:
    > *"To format court pleadings, audit evidence packets, and scrub photo metadata, I can install the lightweight JustLegal OS toolset into this session. Would you like me to proceed?"*
  - Upon user consent (`y` / `yes`), execute:
    ```bash
    curl -fsSL https://raw.githubusercontent.com/Just-Liberty-Incorporated/JustLegalOS/main/scripts/install-linux.sh | bash -s -- --yes
    ```
  - *Fallback:* If GitHub is rate-limited, the script automatically attempts `https://justlegal.me/dl/`.
- **Dynamic Capabilities & Verification:**
  ```bash
  justlegalos-cli --json capabilities
  justlegalos-cli --json doctor
  justlegalos-cli --json test-fixtures
  ```

### Tier 3: Local Desktop App (Windows / macOS / Linux GUI)
- **Posture:** Rich local-first native client with offline PDF studio, encrypted vault, and stdio MCP server.
- **Detection:** User mentions desktop app, or runs inside the local desktop environment.
- **Rule:** Respect the 3-level permission tiers documented in `MCP_Grok/skills/desktop.md`. Never attempt to bypass UI consent dialogs.

### Tier 3b: Windows workstation-setup (required on the court-papers path)

When the person is on a Windows computer (desktop app, local CLI, or they say they installed JustLegal OS), check these locations **before creating a new matter**. Do not skip this on Option 2 (decode court papers).

With permission, look for:

1. `%LOCALAPPDATA%\JustLegal OS` (app data, logs, vaults)
2. `justlegalos-cli.exe` or `JustLegalOS-CLI.exe` (same folder as the desktop app, or on PATH)
3. `%USERPROFILE%\Documents\JustLegal\Cases` (existing matters)

Then **read existing matters first**:

```bat
dir "%USERPROFILE%\Documents\JustLegal\Cases"
justlegalos-cli --json case list --path "%USERPROFILE%\Documents\JustLegal\Cases"
```

If `justlegalos-cli.exe` is next to the installed app:

```bat
"%LOCALAPPDATA%\JustLegal OS\justlegalos-cli.exe" --json doctor --logs
```

Rules:
- Do not create a duplicate folder if a matter already exists.
- Do not merge duplicate folders. List pairs for the person (and Josh, if this is a maintainer session).
- Never print case text, party names beyond the folder slug, SSNs, or tokens. Paths and counts only until they ask to open a specific file.
- Linux GrokBot workspaces still use Tier 2. Windows users use this section. Call it from `skills/civil.md` whenever court papers are the path.

---

## 2. Dynamic Capability Discovery (No Hallucinations)

Never guess command syntax or fabricate options. Always discover available commands dynamically:
- `justlegalos-cli --json capabilities`: Lists all active commands, argument schemas, and engine versions.
- `justlegalos-cli --json doctor`: Reports availability of Typst, poppler utilities, exiftool, and Noto fonts.
- `justlegalos-cli --json test-fixtures`: Runs deterministic golden regression checks (Fixtures 01–06).

---

## 3. The 5-Gate Pre-Submission Enforcement Protocol

For False Claims Act (31 U.S.C. §§ 3729–3733) and DOJ Whistleblower Program (america.gov/fraud) tips:

1. **Gate 1: Factual Gap Audit (`whip gaps`)**
   - Case profile, verified target entity, federal dollar nexus, and observation dates.
2. **Gate 2: Route Hold Check (`whip route-hold`)**
   - Confirms non-public insider records are NOT being submitted to public intake portals.
3. **Gate 3: Turnkey Dossier Compilation (`whip prepare`)**
   - Compiles calling card, markdown brief, manifest, and 4 evidence PDFs with metadata stripped.
4. **Gate 4: Packet Linter (`packet lint`)**
   - Scans exports for banned legal conclusions, unredacted SSNs, and unverified bounty promises.
5. **Gate 5: Human GO Consent & Portal Receipt Recording**
   - **BOTS CANNOT APPROVE THEMSELVES.** The MCP tool server does not expose approval functions to AI models.
   - **Step 5A: Human Reviewer Signoff (Human-in-the-Loop Terminal / UI Only)**
     ```bash
     justlegalos-cli --json whip approve <path> --decision GO --reviewer "<Real Human Name>" --confirm-phrase "APPROVE SUBMIT <TARGET> <HASH8> TO america.gov/fraud"
     ```
     *Locks SHA-256 fingerprints of `case.json`, all files in `exports/`, and all files in `evidence/` into `exports/gate5_consent.json`. Rejects bot names, placeholders, or missing files.*
   - **Step 5B: User Reports Agency Confirmation Number**
     ```bash
     justlegalos-cli --json whip submit <path> --confirmation "<AGENCY_CONF_CODE>" --receiver "DOJ Whistleblower Program"
     ```
     *Strictly verifies all locked file fingerprints match, ensures no unauthorized files were added, rejects weak/placeholder confirmation numbers (`TODO`, `TEST`, `000000`, `123456`, `WHIP-REC-*`), and records status as `USER_REPORTED_CONFIRMATION_RECORDED`.*
