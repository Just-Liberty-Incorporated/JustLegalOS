# Documentation Index & System Map

**JustLegal OS** (powered by the **OpenSeason** pure-Rust formatting and triage core) is a local, privacy-first **Legal Operating System & Court Document Compiler** developed by **Just Liberty Incorporated** (Colorado 501(c)(3) Nonprofit).

It supports everyday legal instruments (**Tier 1 Transactional**: Wills, Leases, Contractor Agreements, Powers of Attorney), pre-suit administrative demands (**Tier 2 Administrative**: Cure Notices, Security Deposit Demands, CORA/FOIA Requests), full court litigation & appeals (**Tier 3 Litigation**: Motions, Pre-Filing Complaints, Reconsideration, Notices of Appeal, Appellate Briefs, Mandamus, and SCOTUS Certiorari), an airgapped **Confidential Vault** (encrypted False Claims / whistleblower vault), and the **Two-Tier Whistleblower Engine** (DOJ WHIP statutory awards on `america.gov/fraud` & `fraud.gov` + sealed Qui Tam).

Both the desktop window (`JustLegalOS.exe` / `OpenSeason.exe`) and the headless CLI/MCP server (`JustLegalOS-CLI.exe` / `justlegalos-cli`) call the exact same deterministic Rust engine (`src-tauri/src/`). Neither ever invents facts/citations, practices law, or files/serves documents.

---

## Documentation Map

| Document | Purpose & Audience |
|---|---|
| [EXECUTIVE-SUMMARY.md](EXECUTIVE-SUMMARY.md) | **Start Here:** Full architectural executive summary, 6 Legal OS subsystems, 4 Workbench Pillars, and module inventory |
| [AGENT-INTERFACE.md](AGENT-INTERFACE.md) | **AI Agents & CLI Users:** Complete `--json` CLI reference (37 top-level commands) and stdio JSON-RPC 2.0 MCP tool catalog |
| [architecture.md](architecture.md) | **System Architects:** Module-by-module map (<800 lines per file rule), 25 active Rust modules, Multi-Venue Matrix, and Tolling Engine |
| [QUI-TAM-EO14395.md](QUI-TAM-EO14395.md) | **Whistleblower & Fraud Whitepaper:** Technical specification for EO 14395, Dual-Track Routing (WHIP tip awards vs. Qui Tam 15%–30%), and FRCP Rule 9(b) 6-element verification |
| [troubleshooting.md](troubleshooting.md) | **Troubleshooting & Operations:** Step-by-step runbook for diagnosing & fixing build, export, venue validation, PDF, WHIP limits, or cloud sync issues |
| [developer_guide.md](developer_guide.md) | **Contributors & Scaling:** How to add new state venues, document kinds, rule packs, Tauri commands, and Svelte 5 components |
| [CODE-SIGNING-AND-UPDATES.md](CODE-SIGNING-AND-UPDATES.md) | **Release & Security:** Authenticode signing, SmartScreen reputation, macOS universal binaries, and SHA-256 integrity verification |
| [SYNC-AND-CLOUD.md](SYNC-AND-CLOUD.md) | **Cloud Integration:** JustLegal Cloud (`justlegal.me`) Firebase Auth, additive-only sync, subfolder routing, and confidential airgap |
| [FORMS-PACKS.md](FORMS-PACKS.md) | **Court Forms:** Forms catalog, SHA-256 verification, AcroForm auto-filler (`JDF` / state forms) |
| [RULE-PACKS.md](RULE-PACKS.md) | **Deadline Math:** Holiday-aware deadline packs (`nj-app-div`, `co-mandamus`, `generic-us-appellate`) |
| [cases.md](cases.md) | **Case Folder Layout:** Portable `Documents/JustLegal/Cases/{caseId}/` structure and `case.json` schema |
| [api.md](api.md) | **IPC Reference:** Tauri `invoke` commands and Rust function bindings |
| [TESTING-ON-WINDOWS.md](TESTING-ON-WINDOWS.md) | **Windows Build & QA:** Visual Studio C++ toolchain, `cargo test`, and release binary checklist |
| [../AGENTS.md](../AGENTS.md) | **Mandatory AI Rules:** Hard guardrails for any AI touching a case folder |
| [../MCP_Grok/BOT_DESCRIPTIONS.md](../MCP_Grok/BOT_DESCRIPTIONS.md) | **GrokBot Bot Profiles:** System prompts and bootstrap checklists for Bot 1 (Civil), Bot 2 (WHIP Scout), Bot 3 (Qui Tam) |
| [../MCP_Grok/workspace_skills/](../MCP_Grok/workspace_skills/) | **7 Modular GrokBot Skills:** `rules_verification.md`, `whip_triage.md`, `state_traps.md`, `gov_target_intake.md`, `independent_analysis.md`, `accountability_map.md`, `civil_dossier.md` |
| [../MCP_Grok/workspace_templates/](../MCP_Grok/workspace_templates/) | **Standard Submission Templates:** 4-Door Submission Menu (`four_door_menu.md`) and Bridge Dossier (`bridge_dossier.md`) |
| [skills/openseason-motion/SKILL.md](skills/openseason-motion/SKILL.md) | **Agent Skill File:** Fast onboarding flow (`case context` -> `intake` -> `next-steps` -> `check` -> `export`) |
