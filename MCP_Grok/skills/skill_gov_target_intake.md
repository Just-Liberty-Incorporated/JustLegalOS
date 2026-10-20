# Skill: Government Target Intake & Legal Viability Classification

## Purpose
Classify whether a target is a state agency, private contractor, or individual official, map its federal Assistance Listing Number (ALN) and contracts, and enforce the *Vermont ANR v. Stevens* rule.

---

## 1. Trigger Conditions
* User reports fraud, waste, system crashes, or non-delivery in a state or federally funded program (e.g., UI, Medicaid, SNAP, Child Care, DMV, Highways).
* User identifies a state department (e.g., CDLE, EDD, NYSDOL, HHSC) or a tech/billing contractor (Deloitte, Accenture, CGI, ID.me, Maximus, Conduent).

---

## 2. Target Classification Schema

| Target Type | Legal Definition | Qui Tam Viable? (31 U.S.C. § 3729) | Primary Action Route |
|---|---|---|---|
| `gov-agency` | State department, board, commission, or agency | **NO** (*Stevens*, 529 U.S. 765) | Systemic Oversight (DOL-OIG, State Auditor, CORA/FOIA, § 1983 prospective relief) |
| `gov-contractor` | Private corporation delivering tech, billing, or administration | **YES** (31 U.S.C. § 3729(b)(2)(A)(ii)) | False Claims Act Qui Tam + DOJ Disclosure Packet |
| `private-provider` | Direct clinic, doctor, daycare, or vendor billing grants | **YES** (Direct false claims) | False Claims Act Qui Tam + HHS-OIG Portal |
| `official-individual` | Agency commissioner, director, or hearing officer | **NO** for damages (Eleventh Amendment) | Ethics complaint, Judicial conduct, or § 1983 prospective injunction |

---

## 3. Execution Commands

1. **Set Case Profile & Target Parameters:**
   ```bash
   justlegalos-cli --json case profile ./my-case \
     --target-type gov-contractor \
     --program ui \
     --aln 17.225 \
     --contract-id "CDLE-2020-001" \
     --tier investigation
   ```

2. **Verify Entity / Contractor Public Footprint:**
   ```bash
   justlegalos-cli --json qui-tam verify-entity --name "Deloitte Consulting LLP" --state CO
   ```

3. **Check Stevens Immunity Status:**
   If target is `gov-agency`, explain gently:
   > *"Under Supreme Court precedent (Vermont ANR v. Stevens), state agencies cannot be sued for money bounties under the False Claims Act. But don't worry—we don't hit a dead end. We split this into two tracks: (1) we follow the federal money to the private contractor who built the system, and (2) we hit the state agency through the State Auditor, federal OIG, and open records."*
