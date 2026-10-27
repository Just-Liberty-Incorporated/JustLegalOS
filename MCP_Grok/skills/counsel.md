# Skill: Retained Counsel Package & Conflict-Check Roster

## Purpose
Prepares factual consultation dossiers and conflict-check summaries for licensed qui tam or civil rights attorneys when insider records are held, ensuring compliance with professional conduct rules and preserving prospective-client confidentiality.

---

## 1. Conflict-Check Roster (`whip conflict-roster`)

Before a law firm can examine case facts or evaluate whistleblower claims, ethical rules require counsel to verify they do not currently represent the target entity, its corporate affiliates, or its principals (ABA Model Rule 1.7). Sending substantive allegations before a conflict check can disqualify the law firm.

The **Conflict-Check Roster** outputs:
```text
======================================================================
JUSTLEGAL COUNSEL CONFLICT-CHECK ROSTER
Provided pursuant to ABA Model Rule 1.18 (Duties to Prospective Client)
======================================================================
• TARGET LEGAL NAME: [Target Business Entity Name]
• ENUMERATION / IDENTIFIER: [CMS NPI-1 / NPI-2 / SAM.gov UEI]
• PRIMARY PRACTICE ADDRESS: [Physical Street Address, City, State, ZIP]
• GOVERNING JURISDICTION: [State / Federal Judicial District]
• PROSPECTIVE RELATOR / CLIENT: [Client Legal Name & Contact Info]

CRITICAL PRIVILEGE & ETHICAL NOTICE:
This roster contains ONLY party names and identifying registration data.
It contains ZERO factual narrative, allegations, or evidence files.
Transmitted strictly to facilitate firm conflict-of-interest screening.
======================================================================
```

CLI Command:
```bash
justlegalos-cli --json whip conflict-roster ./my-case
```

---

## 2. Prospective-Client Factual Index (ABA Model Rule 1.18)

Once the law firm confirms no conflicts exist, the client transmits the encrypted factual evidence index:
- **Prospective Client Confidentiality:** Communications sent to counsel for the purpose of seeking legal representation are protected under ABA Model Rule 1.18, even if the attorney ultimately declines representation.
- **Airgapped Storage:** All underlying documents remain encrypted in the client's local vault.
- **Standard Verified Consultation Disclaimer:**
  > *"This factual record index was compiled offline by the client for legal consultation. Just Liberty Incorporated is not legal counsel. This document is provided to retained counsel to facilitate legal evaluation."*
- **No Unsworn Perjury Stamps:** The software never attaches 28 U.S.C. § 1746 penalty-of-perjury statements to unsworn informal drafts. Sworn declarations are drafted and reviewed directly with retained counsel prior to filing under seal.
