# Security Policy & Cryptographic Verification
**JustLegal OS** — Developed by **Just Liberty Incorporated** (Colorado 501(c)(3) Nonprofit)

---

## 1. Zero-Telemetry & Local Airgap Guarantee

JustLegal OS is engineered specifically for pro se civil litigants, vulnerable citizens, and False Claims Act / Qui Tam whistleblowers. The architecture adheres to a strict, non-negotiable principle:

- **100% Offline by Default:** The desktop client (`JustLegalOS.exe`) and command-line engine (`justlegalos-cli`) operate entirely on your local machine. No account creation is required to use the local software.
- **Strict Network Airgap on Sealed Matters:** Under the False Claims Act (**31 U.S.C. § 3730(b)(2)**), qui tam complaints and relator disclosures must remain strictly confidential under in camera court seal. If a case folder contains a `.sealed` marker file or has `mode: confidential`, the Rust core engine permanently refuses all network sync, cloud backup, publish, or telemetry operations.
- **Keyless Public Registry Lookups Only:** When performing entity and fraud triage checks, the tool queries public federal databases directly (CMS NPPES NPI Registry at `npiregistry.cms.hhs.gov` and USAspending at `api.usaspending.gov`). It transmits only the query string typed by the user (target entity name or 10-digit NPI); it never uploads case folders, exhibits, or local notes.
- **No Remote Telemetry:** JustLegal OS contains zero analytics, tracking scripts, or remote logging.

---

## 2. Release Binary Cryptographic Verification

Every official release published on `https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases` is cryptographically hashed at build time.

A `SHA256SUMS.txt` manifest is published alongside every release. Users and administrators can independently verify binary integrity before execution:

### Linux / macOS
```bash
# Download binary and SHA256SUMS.txt
curl -sSL -O https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases/latest/download/justlegalos-cli-linux-x86_64
curl -sSL -O https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases/latest/download/SHA256SUMS.txt

# Verify checksum
sha256sum --check --ignore-missing SHA256SUMS.txt
```

### Windows (PowerShell)
```powershell
# Verify Windows installer checksum
Get-FileHash .\JustLegalOS_0.1.0_x64-setup.exe -Algorithm SHA256
```

Compare the resulting hash with the published value in `SHA256SUMS.txt`.

---

## 3. Core Engine Architecture & Anti-Adversarial Gaming Policy

JustLegal OS utilizes a split distribution model:

1. **Public Specification & Distribution Surface (This Repository):**
   - Official cross-platform installers and binaries (`.exe`, `.msi`, `.dmg`, Linux CLI).
   - Machine-readable **`[JUSTLEGAL OS | JUSTLEGAL STANDARDIZED FRAUD DISCLOSURE SCHEMA v1.0 | EO-14395]`** specifications.
   - Clean, auditable one-line installer scripts (`scripts/install-linux.sh`).
   - GrokBot recipes and autonomous agent briefings (`MCP_Grok/`).

2. **Core Deterministic Engine (Private Repository):**
   - **Anti-Adversarial Gaming:** To prevent fraudulent healthcare providers and corrupt entities from reverse-engineering and gaming our FRCP Rule 9(b) fraud heuristics, and to prevent commercial exploitation of our 501(c)(3) nonprofit public infrastructure, core engine source code is maintained in a private repository.
   - **Institutional Source-Code Audits:** Full source code and cryptographic build-pipeline audits are available under standard institutional NDA to:
     - Government oversight agencies (DOJ Commercial Litigation Branch Fraud Section, HHS-OIG, USDA-OIG, SBA-OIG, State AG Medicaid Fraud Control Units).
     - Accredited foundation grantmakers.
     - Charter Panel qui tam plaintiff law firms.

To request an institutional source audit, please contact:
**`josh@truejust.org`**.

---

## 4. Reporting a Security Vulnerability

If you discover a potential security vulnerability within JustLegal OS, the CLI, or our public distribution infrastructure, please report it responsibly:

- **Email:** `josh@truejust.org` (Subject: `SECURITY VULNERABILITY REPORT`)
- **Response Commitment:** We review all reports within 24 hours and will coordinate a validated fix prior to public disclosure.
