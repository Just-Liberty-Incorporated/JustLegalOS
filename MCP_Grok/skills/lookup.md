# Skill: Public Registry Entity Lookup & Funding Verification

## Purpose
Verifies healthcare entity credentials, corporate registrations, and federal award funding streams using keyless public API queries and official government registries.

---

## 1. Authoritative Public Registries Queried

The assistant guides lookups across three primary federal public registries:

### 1. CMS NPPES Provider Registry
- **Portal / API:** [https://npiregistry.cms.hhs.gov](https://npiregistry.cms.hhs.gov)
- **Lookup Parameters:** Target legal business name, DBA, or 10-digit National Provider Identifier (NPI).
- **Extracted Fields:**
  - Enumeration Type (Type 1 Individual vs. Type 2 Organization).
  - Primary Healthcare Provider Taxonomy code and classification description (e.g. Hospice, Home Health, Adult Day Care).
  - Registered Physical Practice Location vs. Mailing Address.
  - Authorized Official Name, Title, and Phone Number.
  - Active enumeration status and deactivation history.

### 2. USAspending.gov Federal Awards Registry
- **Portal / API:** [https://www.usaspending.gov](https://www.usaspending.gov) | [https://api.usaspending.gov](https://api.usaspending.gov)
- **Lookup Parameters:** Entity legal name, UEI, or Federal Award ID (FAID).
- **Extracted Fields:** Prime federal procurement contracts, direct grant awards, Assistance Listing Numbers (ALNs), and total obligated dollar amounts.
- **State Funding Distinction:** Direct federal awards on USAspending reflect prime federal contracts (e.g. DOD, VA, direct HHS grants). State-administered programs (such as Medicaid and State Unemployment Insurance) flow through state health and labor departments. If USAspending returns $0.00 direct awards, report:
  > *"Direct federal prime awards: $0.00. (The entity may bill through state-administered Medicaid, CHIP, or sub-awarded state grants rather than direct federal procurement)."*

### 3. SAM.gov Entity Management
- **Portal:** [https://sam.gov](https://sam.gov)
- **Lookup Parameters:** 12-character alphanumeric Unique Entity Identifier (UEI), CAGE code, or corporate name.
- **Extracted Fields:** Active federal registration status, entity structure, and exclusion/debarment records.

---

## 2. Privacy & Airgap Architecture

- **Public Queries Only:** Public registry searches query *only* the entity name, state, or NPI typed by the user.
- **Zero Case Upload:** The user's case folder, private notes, observation photos, drafts, and evidence files are **never** uploaded to any network service or registry API.
