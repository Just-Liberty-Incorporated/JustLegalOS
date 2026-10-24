# Documentation Index & System Map

**JustLegal OS** (powered by the **OpenSeason** pure-Rust formatting and triage core) is a local, privacy-first **Legal Operating System & Court Document Compiler** developed by **Just Liberty Incorporated** (Colorado 501(c)(3) Nonprofit).

It supports everyday legal instruments (**Tier 1 Transactional**: Wills, Leases, Contractor Agreements, Powers of Attorney), pre-suit administrative demands (**Tier 2 Administrative**: Cure Notices, Security Deposit Demands, CORA/FOIA Requests), full court litigation & appeals (**Tier 3 Litigation**: Motions, Pre-Filing Complaints, Reconsideration, Notices of Appeal, Appellate Briefs, Mandamus, and SCOTUS Certiorari), an airgapped **Confidential Vault** (encrypted False Claims / whistleblower vault), and the **Two-Tier Whistleblower Engine** (DOJ WHIP statutory whistleblower awards on `fraud.gov` + sealed Qui Tam).

Both the desktop window (`JustLegalOS.exe` / `OpenSeason.exe`) and the headless CLI/MCP server (`JustLegalOS-CLI.exe` / `justlegalos-cli`) call the exact same deterministic Rust engine (`src-tauri/src/`). Neither ever invents facts/citations, practices law, or files/serves documents.

---

## Documentation Map

| Document | Purpose & Audience |
|---|---|
| [EXECUTIVE-SUMMARY.md](EXECUTIVE-SUMMARY.md) | **Start Here:** Full architectural executive summary, 6 Legal OS subsystems, 4 Workbench Pillars, and module inventory |
| [AGENT-INTERFACE.md](AGENT-INTERFACE.md) | **AI Agents & CLI Users:** Complete `--json` CLI reference (37 top-level commands) and stdio JSON-RPC 2.0 MCP tool catalog |
| [architecture.md](architecture.md) | **System Architects:** Module-by-module map (<800 lines per file rule), 25 active Rust modules, Multi-Venue Matrix, and Tolling Engine |
| [QUI-TAM-EO14395.md](QUI-TAM-EO14395.md) | **Whistleblower & Fraud Whitepaper:** Technical specification for EO 14395, Two-Tier Whistleblower Economy (DOJ WHIP statutory awards vs. Qui Tam 15%–30%), and FRCP Rule 9(b) 6-element scoring |
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
| [../MCP_Grok/README.md](../MCP_Grok/README.md) | **GrokBot AI Connectors:** x.ai GrokBot blueprints, conversational triage prompts, and MCP agent connectors |
| [../MCP_Grok/skills/skill_whip_triage.md](../MCP_Grok/skills/skill_whip_triage.md) | **WHIP Strategy Skill:** Evaluating Tier 1 WHIP (america.gov/fraud) vs. Tier 2 Sealed Qui Tam (15%–30%) |
| [skills/openseason-motion/SKILL.md](skills/openseason-motion/SKILL.md) | **Agent Skill File:** Fast onboarding flow (`case context` -> `intake` -> `next-steps` -> `check` -> `export`) |
