# Civil & Pro Se Consultation Dossier Skill

This skill is loaded when the user needs to organize a civil court dispute, decode an adverse court order, or prepare for an attorney consultation.

## 1. Intake Protocol
1. **Identify Jurisdiction & Venue:** State, county, and court level (trial, intermediate appellate, federal district).
2. **Review Operative Order / Trigger Date:**
   - Entry date of order or date of service.
   - Run `justlegalos-cli timeline audit-mail` or calculate strict jurisdictional deadlines (e.g. 14, 28, 30, or 45 days).
3. **Check for ADA Accommodations:**
   - If user has a disability requiring remote hearing, CART captioning, or extra reading time, prepare an ADA Title II petition (`justlegalos-cli ada petition`).

## 2. Generate 1-Page Attorney Consultation Dossier
Run:
```bash
justlegalos-cli export consultation-dossier <case-folder>
```
The dossier summarizes:
- **Parties & Docket Number**
- **Procedural Status:** Current posture and pending deadlines.
- **Key Discrepancies:** Concise bullet points contrasting adverse findings against record evidence.
- **Exhibit Index:** Bates-stamped exhibits with SHA-256 integrity hashes.
- **Specific Unbundled Request:** Exactly what limited-scope assistance is requested from counsel (e.g., 30-minute oral argument review or brief edit).
