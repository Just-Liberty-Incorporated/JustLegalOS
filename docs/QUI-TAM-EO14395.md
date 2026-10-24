# Executive Order 14395 & Qui Tam / False Claims Act Triage Engine

## Whitepaper & Technical Specification

**OpenSeason Legal Operating System**  
*Preserving the Integrity of Federal Programs and Protecting Public Funds (EO 14395)*  
*False Claims Act (31 U.S.C. §§ 3729–3733) & FRCP Rule 9(b) Compliance*

---

### 1. Executive Summary & Operational Mandate

On January 2025, Executive Order 14395 (*Preserving the Integrity of Federal Programs and Protecting Public Funds*) directed the Attorney General, the Office of Management and Budget (OMB), and federal Inspectors General to dramatically streamline federal fraud investigations, accelerate recoveries, and coordinate with private whistleblowers (relators).

Under the False Claims Act (**31 U.S.C. § 3730(b)(2)**), a qui tam action must be filed **under seal, in camera**, and remains sealed for an initial period of **60 days**. During this 60-day window, the Department of Justice (DOJ) Commercial Litigation Branch (Fraud Section) and United States Attorney's Offices must evaluate the complaint to decide whether to intervene.

#### The 60-Day Bottleneck
Historically, relators' disclosures arrive at government desks in sprawling, unindexed formats. Federal prosecutors and special agents spend the first 30 to 60 days on rudimentary verification:
- Running manual searches across CMS National Plan and Provider Enumeration System (NPPES) databases.
- Checking USAspending.gov grant and contract histories.
- Sifting through unauthenticated smartphone photos or spreadsheets.
- Determining whether the complaint meets the stringent pleading standards of **Federal Rule of Civil Procedure 9(b)** (particularity of fraud).

Consequently, the government is almost always forced to request repeated six-month seal extensions, clogging district court dockets and delaying recoveries.

#### The OpenSeason Solution
OpenSeason automates the first 40 hours of a federal investigation before the dossier ever leaves the relator's workstation. Operating strictly offline in an encrypted airgap, OpenSeason:
1. Cross-references entities against live, keyless federal databases (CMS NPPES v2.1 and USAspending.gov).
2. Evaluates the six core particularity elements of **FRCP Rule 9(b)** under an unembellished 4-state verification receipt without synthetic scores.
3. Cryptographically seals physical site observations (GPS EXIF extraction, SHA-256 and SHA-512 dual hashing, read-only disk lock, and an immutable `provenance.json` Merkle log).
4. Generates state-specific FOIA/public records requests (MGDPA, OPRA, CORA, CPRA) to obtain official licensing files.
5. Assembles a turnkey **DOJ 60-Day Relator Written Disclosure Packet** in both machine-readable JSON (`justlegal-disclosure-v1.json` with backward-compatible `openseason-disclosure-v1.json`) and a court-ready Typst PDF (`DOJ_Relator_Disclosure_Packet.pdf`).

---

### 2. Targeting the Three Primary Vulnerabilities of EO 14395

Executive Order 14395 specifically identifies three structural fraud patterns across HHS (Medicaid/Medicare), USDA (SNAP/Child Nutrition), SBA, HUD, Education, and DOT:

```
                      EO 14395 Primary Vulnerabilities
                                     │
     ┌───────────────────────────────┼──────────────────────────────┐
     │                               │                              │
     ▼                               ▼                              ▼
1. Provider Enrollments    2. Self-Attestations           3. Third-Party Intermediaries
   & Ghost Clinics            & Eligibility Fraud            & Billing Aggregators
   (Empty storefronts,        (Unverified income caps,       (Shell management entities,
   deactivated NPIs,          kickback schemes,              pass-through LLCs,
   UPS Store mail drops)      forged certifications)         unregistered remitters)
```

1. **Provider Enrollments & Ghost Clinics**: Fraudulent entities obtain NPIs or state vendor IDs using UPS stores, strip-mall mailboxes, or vacant suites, billing millions for services never rendered. OpenSeason detects deactivated NPIs, flags non-medical mailing addresses, and matches physical site observations.
2. **Eligibility Self-Attestations**: Providers certify compliance with federal staffing ratios, licensing prerequisites, or beneficiary eligibility thresholds without verification. OpenSeason checks state licensing status and provides pre-drafted statutory records requests.
3. **Third-Party Intermediaries**: Entities route federal receipts through billing agents, management companies, or shell LLCs. OpenSeason matches recipient names and DUNS/UEI IDs in USAspending to trace federal fund disbursement chains.

---

### 3. Architecture & Security Airgap

Federal qui tam complaints and disclosure packets must remain **strictly confidential under seal**. Premature public disclosure or inadvertent network leakage violates **31 U.S.C. § 3730(b)(2)** and can result in dismissal of the action and forfeiture of the relator's statutory share (15% to 30%).

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Local Relator Workstation                       │
│                                                                        │
│  ┌─────────────────────────┐           ┌────────────────────────────┐  │
│  │   JustLegal Desktop     │           │    JustLegalOS-CLI.exe     │  │
│  │   (Tauri GUI / Svelte)  │           │    (Pure Rust Engine)      │  │
│  └────────────┬────────────┘           └─────────────┬──────────────┘  │
│               │                                      │                 │
│               ▼                                      ▼                 │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │       Confidential Case Vault (Documents/JustLegal/Cases/...)    │  │
│  │   - .sealed Marker File & Sealed Pointer case.json               │  │
│  │   - SHA-256 / SHA-512 Immutable provenance.json                  │  │
│  │   - Read-Only Hardened Filesystem Lock (Windows/POSIX)           │  │
│  │   - XChaCha20Poly1305 + Argon2id Encrypted Archive Storage       │  │
│  └──────────────────────────────────┬───────────────────────────────┘  │
└─────────────────────────────────────┼──────────────────────────────────┘
                                      │
                         AIRGAP INTEGRITY ENFORCEMENT
                                      │
                   ┌──────────────────┴──────────────────┐
                   ▼                                     ▼
      ❌ Cloud Sync Permanently Blocked     ✅ Public Keyless Verification
         (Refuses upload on .sealed)            (CMS NPPES & USAspending)
```

- **Strict Network Airgap**: `src-tauri/src/sync.rs` checks for `.sealed` or `mode: confidential`. Any attempt to sync, push, or cloud-backup a sealed case is permanently refused.
- **No Telemetry / No Remote LLM**: All Rule 9(b) scoring, EXIF parsing, and PDF compilation execute locally in pure Rust compiled to native machine code.

---

### 4. Deterministic Rust Engine Modules (`src-tauri/src/qui_tam/`)

The qui tam subsystem consists of five modular Rust components:

| Module | Source File | Purpose |
|---|---|---|
| `types.rs` | `src-tauri/src/qui_tam/types.rs` | Serde models for NPI records, Rule 9(b) audits, observations, FOIA packs, and disclosure packets. |
| `nppes.rs` | `src-tauri/src/qui_tam/nppes.rs` | Async HTTP client for CMS NPPES API (`https://npiregistry.cms.hhs.gov/api/?version=2.1`) and USAspending awards lookup. |
| `rule9b.rs` | `src-tauri/src/qui_tam/rule9b.rs` | Deterministic 6-element FRCP Rule 9(b) particularity evaluation engine & 4-state verification receipt. |
| `observation.rs` | `src-tauri/src/qui_tam/observation.rs` | EXIF GPS/timestamp extraction, SHA-256/SHA-512 hashing, read-only file locking, and Merkle provenance logging. |
| `records_pack.rs` | `src-tauri/src/qui_tam/records_pack.rs` | State public records request generator (MGDPA, OPRA, CORA, CPRA, 5 U.S.C. § 552). |
| `disclosure.rs` | `src-tauri/src/qui_tam/disclosure.rs` | Compiles `justlegal-disclosure-v1.json` (and `openseason-disclosure-v1.json`) and Typst court-formatted `DOJ_Relator_Disclosure_Packet.pdf`. |

---

### 5. CLI Command Specification

All capabilities are available to both human operators and autonomous local AI agents via `JustLegalOS-CLI.exe --json qui-tam <subcommand>`:

#### 1. Entity Cross-Check (`qui-tam verify-entity`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam verify-entity --name "North Star Health" --state MN
```
*Queries CMS NPPES v2.1 for active/deactivated NPIs, provider taxonomy, practice location, and checks USAspending for prime award history.*

#### 2. Rule 9(b) Viability Audit (`qui-tam audit-9b`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam audit-9b ./my-case
```
*Audits `case.json` against the 6 Rule 9(b) elements (Who, What, When, Where, How, Scienter) and outputs a 4-state verification receipt with missing element warnings.*

#### 3. Field Observation Ingestion (`qui-tam ingest-observation`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam ingest-observation ./my-case \
  --file "C:\Photos\facility_exterior.jpg" \
  --notes "Building dark during posted clinical hours; no patient traffic." \
  --observer "Jordan Example"
```
*Extracts EXIF metadata, computes SHA-256 and SHA-512 hashes, marks the target file read-only, and appends to `provenance.json`.*

#### 4. Licensing & Records Request Pack (`qui-tam records-pack`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam records-pack ./my-case --state MN --agency "Department of Human Services"
```
*Generates statutory records request under Minnesota Government Data Practices Act (M.S. § 13.01 et seq.) or state equivalent.*

#### 5. DOJ Written Disclosure Packet Export (`qui-tam export-disclosure`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam export-disclosure ./my-case
```
*Compiles the formal Relator Written Disclosure Statement required by 31 U.S.C. § 3730(b)(2), producing `justlegal-disclosure-v1.json` (and `openseason-disclosure-v1.json`) and `DOJ_Relator_Disclosure_Packet.pdf` in `exports/`.*

#### 6. Agency Portal Auto-Fill Formatter (`qui-tam format-portal`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam format-portal ./my-case --agency HHS
```
*Formats an administrative submission payload pre-trimmed to character limits (<2,000 characters) for official government web forms (HHS-OIG, DOJ Civil Fraud, State MFCU, USDA, SBA) with the standardized ASCII Calling Card.*

#### 7. ABA Rule 1.18 Attorney Inquiry Package (`qui-tam format-inquiry`)
```bash
.\JustLegalOS-CLI.exe --json qui-tam format-inquiry ./my-case
```
*Generates a prospective-client privileged inquiry package under ABA Model Rule 1.18 for transmission to private qui tam counsel on 100% contingency, protecting statutory relator bounties (15%–30%) and in camera court seals.*

---

### 6. JustLegal Calling Card Standard v1.0 (`[EO-14395 COMPLIANT]`)

Every generated disclosure packet, attorney inquiry, and portal submission block carries the standardized ASCII Calling Card header:

```text
======================================================================
JUSTLEGAL OS | JUSTLEGAL STANDARDIZED FRAUD DISCLOSURE [SCHEMA v1.0 | EO-14395]
Generated via JustLegal.me / Just Liberty Inc. (501c3) Verification Engine
======================================================================
• MATTER CLASSIFICATION: HEALTHCARE_BILLING_DISPATCH_MISMATCH
• TARGET ENTITY: North Star Healthcare LLC | NPI: 1234567890 | UEI: ABCD1234EFGH
• REGISTERED TAXONOMY: 251J00000X vs. Observed Status: Empty Storefront
• VERIFIED FEDERAL OBLIGATIONS: $2,450,000.00 (USAspending Confirmed)
• FRCP RULE 9(b) PARTICULARITY STATUS: Developing (Claims Proof & Specific Drawdown Evidence Needed)
• EVIDENCE MANIFEST HASH (SHA-256): b94d27b9934d3e08a52e52d7da7dabfac484efe3...
• VERIFICATION SPECIFICATION: https://justlegal.me/qui-tam
======================================================================
```

#### Diagnostic Header Fields:
1. **Target & NPI**: Official entity name and 10-digit National Provider Identifier validated against the live CMS NPPES v2.1 registry.
2. **Taxonomy Match**: Cross-checked against CMS taxonomy codes (e.g. physical clinic vs. non-emergency transportation/dispatch).
3. **Federal Program & Verified Draw**: Identified funding pipeline and confirmed prime award obligations from USAspending.gov.
4. **Rule 9(b) Particularity Status**: 6-element statutory readiness tier evaluating whether factual allegations meet pleading requirements.
5. **Evidence Digest**: Merkle SHA-256 cryptographic root hash of all logged photographic/documentary evidence.
6. **Triage Standard URL**: Direct reference to the public triage standard (`https://justlegal.me/qui-tam`) allowing government analysts and intake attorneys to verify schema compliance in seconds.

---

### 7. Multi-Track Whistleblower Routing Engine (WHIP vs. Qui Tam vs. Agency Oversight)

To prevent catastrophic procedural errors—such as inadvertently destroying a whistleblower's 15%–30% relator bounty under the False Claims Act's public disclosure bar or submitting non-public insider records to a public web portal—JustLegal OS implements a deterministic multi-track routing engine:

```
                            Completed Case Evaluation
                                       │
        ┌──────────────────────────────┼──────────────────────────────┐
        │                              │                              │
        ▼                              ▼                              ▼
 Track 1: DOJ WHIP             Track 2: Sealed Qui Tam       Track 3: Citizen Watchdog
 (fraud.gov / Tip Rewards)     (Civil FCA Counsel First)     (Administrative Oversight)
        │                              │                              │
• Street-Level Observations    • Non-Public Insider Proof     • Localized Facility Abuse
• Independent Analysis         • 15%–25% (Intervened)         • Rapid Payment Suspension
• 28 U.S.C. § 530C(b)(1)(L)    • 25%–30% (Declined)           • State Licensure Revocation
• 28 U.S.C. § 530C(b)(1)(L) / DOJ WHIP Guidelines              • 31 U.S.C. § 3730(b)(2) Seal  • Pre-Trimmed ≤1,950 Chars
• Discretionary Tip Award      • NEVER submit to web portal   • HHS-OIG / State AG MFCU
• Zero Attorney Required       • $0 Upfront Contingency       • 501(c)(3) Public Benefit
```

#### Track 1: DOJ Whistleblower Incentive Program (WHIP on fraud.gov)
- **Authority**: 28 U.S.C. § 530C(b)(1)(L), 28 U.S.C. § 530C(b)(1)(L) / DOJ WHIP Guidelines, and Executive Order 14395.
- **Eligibility**: Street-level observations of shuttered storefronts, ghost clinics, vacant daycares, or independent analysis correlating public datasets. No attorney required ($0 upfront).
- **Statutory Reward**: Discretionary early-stage bounty following criminal charges and recoveries, with potential eligibility for stacked recoveries upon criminal forfeiture.
- **Portals**: `https://www.america.gov/fraud` and `https://www.fraud.gov`.

#### Track 2: The Relator Reward Track (Civil Qui Tam Counsel First)
- **Eligibility**: Firsthand insider evidence, non-public corporate records, documentation of fraudulent billing, and actionable claims. (While the False Claims Act contains no statutory dollar minimum, private relator counsel generally evaluate economic scale to justify contingency litigation).
- **Statutory Relator Bounty**: 15% to 25% of recovered proceeds if DOJ intervenes; 25% to 30% if the relator proceeds independently (**31 U.S.C. § 3730(d)**). Defendants face treble damages and statutory civil penalties under 31 U.S.C. § 3729(a).
- **Mandatory In Camera Court Seal**: Under **31 U.S.C. § 3730(b)(2)**, qui tam complaints must be filed in secret by retained legal counsel in federal district court. Submitting non-public records to a public web portal prior to filing under seal risks forfeiting relator status under the public disclosure bar (**31 U.S.C. § 3730(e)(4)**).
- **$0 Upfront Cost**: Legitimate qui tam plaintiff firms represent whistleblowers on 100% contingency. The relator pays $0 out of pocket.
- **ABA Model Rule 1.18 Privilege**: The attorney inquiry package generated by JustLegal OS explicitly establishes prospective-client confidentiality.
- **Nonprofit Fee-Splitting Ethics**: Under **ABA Model Rule 5.4**, Just Liberty Inc. (501(c)(3) nonprofit) never splits or accepts contingency fees.

#### Track 3: Citizen Watchdog / Administrative Enforcement Track
- **Eligibility**: Localized abuse, shell clinic storefronts, community elder/patient protection, public observations, or direct tips where the citizen seeks immediate administrative action rather than filing a sealed federal lawsuit.
- **Direct Administrative Action**: Bypasses the 60-day court seal process to enable immediate agency audits, Medicare/Medicaid payment suspensions (42 C.F.R. § 405.371), and state licensing revocations.
- **Character-Limited Web Form Blocks**: Federal and state portals (HHS-OIG, DOJ, State MFCU) strictly limit description fields to 1,500–2,000 characters. JustLegal OS formats auto-trimmed copyable blocks (≤ 1,950 characters) that fit within these constraints while retaining the standardized Calling Card header.

---

### 8. Relator Guardrails & Statutory Warnings

OpenSeason enforces three mandatory legal guardrails across all qui tam interfaces:

1. **No Pro Se Relators in Qui Tam Actions**:
   Under established federal precedent (*United States ex rel. Mergent Services v. Flaherty*, 540 F.3d 89, 93 (2d Cir. 2008)), an unrepresented individual cannot prosecute a qui tam action on behalf of the United States. OpenSeason explicitly warns relators that a licensed attorney must be retained before filing in district court.
2. **Mandatory In Camera Court Seal**:
   31 U.S.C. § 3730(b)(2) requires that the complaint be filed *in camera* and remain under seal. Public filing of the complaint or disclosure statement before the court lifts the seal is a fatal procedural error.
3. **HIPAA Whistleblower Safe Harbor**:
   Under **45 C.F.R. § 164.502(j)(1)**, disclosures of Protected Health Information (PHI) by a workforce member or business associate are permissible only when made to a health oversight agency, public health authority, or an attorney retained by the individual for the purpose of determining legal options under the False Claims Act. OpenSeason warns against extraneous disclosure of patient identifiers.

---

### 9. Verifiable Proof of Operation

The qui tam engine is verified by comprehensive automated tests in Rust:
```bash
cargo test qui_tam
```
Output:
```
running 5 tests
test qui_tam::tests::test_rule9b_audit_scoring ... ok
test qui_tam::tests::test_records_pack_generation ... ok
test qui_tam::tests::test_provenance_manifest_hashing ... ok
test qui_tam::tests::test_disclosure_packet_compilation ... ok
test qui_tam::tests::test_standardized_calling_card_and_dual_track_formatting ... ok

test result: ok. 5 passed; 0 failed; finished in 0.91s
```
Together with `openseason doctor`, federal agencies and relator counsel can immediately verify that local cryptographic libraries, airgap enforcement, and CMS/USAspending connectors are 100% operational.

