# Skill: Public Registry Entity Lookup & Funding Verification

## Purpose
Verifies healthcare entity credentials, corporate registrations, and federal award funding streams using keyless public API queries and official government registries.

---

## 1. Automated Keyless Verification Command

When running in a workspace with consent, or guiding the user on desktop CLI:
```bash
justlegalos-cli --json qui-tam verify-entity --name "<Target Entity Name>" --state <ST>
```
This automated command runs keyless public lookups against official CMS NPPES and USAspending.gov APIs simultaneously, returning structured entity details, matching NPIs, taxonomy codes, and direct federal prime award obligations.

---

## 2. Authoritative Public Registries Queried

The assistant guides lookups across authoritative federal public registries:

### 1. CMS NPPES Provider Registry (Automated via CLI)
- **Portal / API:** [https://npiregistry.cms.hhs.gov](https://npiregistry.cms.hhs.gov)
- **Lookup Parameters:** Target legal business name, DBA, or 10-digit National Provider Identifier (NPI).
- **Extracted Fields:**
  - Enumeration Type (Type 1 Individual vs. Type 2 Organization).
  - Primary Healthcare Provider Taxonomy code and classification description (e.g. Hospice, Home Health, Adult Day Care).
  - Registered Physical Practice Location vs. Mailing Address.
  - Authorized Official Name, Title, and Phone Number.
  - Active enumeration status and deactivation history.

### 2. USAspending.gov Federal Awards Registry (Automated via CLI)
- **Portal / API:** [https://www.usaspending.gov](https://www.usaspending.gov) | [https://api.usaspending.gov](https://api.usaspending.gov)
- **Lookup Parameters:** Entity legal name, UEI, or Federal Award ID (FAID).
- **Extracted Fields:** Prime federal procurement contracts, direct grant awards, Assistance Listing Numbers (ALNs), and total obligated dollar amounts.
- **State Funding Distinction:** Direct federal awards on USAspending reflect prime federal contracts (e.g. DOD, VA, direct HHS grants). State-administered programs (such as Medicaid and State Unemployment Insurance) flow through state health and labor departments. If USAspending returns $0.00 direct awards, report:
  > *"Direct federal prime awards: $0.00. (The entity may bill through state-administered Medicaid, CHIP, or sub-awarded state grants rather than direct federal procurement)."*

### 3. SAM.gov Entity Management (Manual Browser Reference)
- **Portal:** [https://sam.gov](https://sam.gov)
- **Note:** SAM.gov public search requires manual web browsing or registered API credentials. Guide the user to manually verify active entity status, CAGE codes, and exclusion/debarment records directly on sam.gov.

---

## 3. Privacy & Airgap Architecture

- **Public Queries Only:** Public registry searches query *only* the entity name, state, or NPI typed by the user.
- **Zero Case Upload:** The user's case folder, private notes, observation photos, drafts, and evidence files are **never** uploaded to any network service or registry API.
