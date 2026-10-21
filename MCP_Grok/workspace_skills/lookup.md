# Skill: Public Registry Entity Lookup & Funding Verification

## Purpose
Verifies healthcare entity credentials and federal award funding streams using keyless public API queries against official .gov registries.

## Public Registries Queried
1. **CMS NPPES NPI Registry (`npiregistry.cms.hhs.gov`)**:
   - Queries target organization name or 10-digit National Provider Identifier (NPI).
   - Identifies:
     - Registered legal business name.
     - Physical practice location vs. mailing address.
     - Primary healthcare taxonomy / provider classification code.
     - Authorized corporate official.
2. **USAspending.gov API (`api.usaspending.gov`)**:
   - Queries direct federal obligations, grant awards, and Assistance Listing Numbers (ALNs).
   - Direct federal awards are recorded verbatim.
   - If obligations return $0.00, report: "Direct federal awards: $0.00 (Program may be state-administered Medicaid/UI or sub-awarded)."

## Privacy & Network Rules
- Public lookups query ONLY the entity name, state, or NPI typed by the user.
- The case folder, local drafts, notes, and photos are NEVER uploaded to any network or cloud server.
