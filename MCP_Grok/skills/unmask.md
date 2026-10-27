# Skill: Public Registry Entity Lookup & Surface Clue Triage (`skills/unmask.md`)

## 1. When to Use This Skill
Use this skill when handling **Archetype 3: The Outsider Pattern-Spotter** - where a citizen does **not** have personal recipient records or an insider employee relationship with the target, but observes public solicitations of suspect healthcare or government-benefit schemes:
* **Social Media Ads:** Public video/image ads (e.g., paid family caregiving recruitment or "free government medical supplies").
* **Physical Solicitations:** Public signs on telephone poles, lawn signs, bus stop flyers, or QR codes.
* **Gray-Market Websites & Online Notices:** Public buyback landing pages or solicitations offering cash for prescription medical equipment (`Rx Only` CGMs, Omnipods, test strips) or public benefits.

---

## 2. Hard Anti-Doxxing & Relator Standing Boundaries

```
                 [ UNSTRUCTURED SURFACE CLUE ]
            (Public Video Ad, Pole Sign, QR Flyer)
                                │
        ┌───────────────────────┴───────────────────────┐
        ▼                                               ▼
[ FORBIDDEN: Pretext & Doxxing ]             [ ALLOWED: Public Registry Verification ]
❌ Investigating private citizens/caregivers ✅ Corporate Entity & LLC Registry lookup
❌ Targeting individual practitioners (NPI-1)✅ CMS NPPES Organizational Provider lookup (NPI-2)
❌ Deceptive alias personas or entrapment    ✅ Plain-English written inquiry for business terms
❌ Publishing personal residential addresses ✅ State open records requests (CORA/FOIA)
❌ Wiretap/covert audio recordings           ✅ Cryptographic photo hashing with GPS stripped
⚠️ Doxxing / harassment risk                 ✅ 100% Protected, Clean Submitter Standing
```

1. **Organizational Entities Only (NPI-2 Rule):** The system strictly matches against **Organizational Providers (`NPI-2`)** - facilities, home health agencies, medical supply corporations, and clinics. Individual healthcare workers, individual clinicians, and private caregivers (`NPI-1`) are never targeted from street ads or phone numbers.
2. **Unconditional GPS Stripping:** Whenever photos are ingested, EXIF GPS coordinates are unconditionally stripped to protect residential and observer privacy.
3. **No Undercover Stings or Deceptive Pretexts:** Never instruct the user to pose as an active participant in illegal trafficking, construct deceptive personas, or submit false identity data. Under **28 U.S.C. § 530C(b)(1)(L)**, active participants in criminal offenses are disqualified from whistleblower awards.
4. **No Wiretap Violations:** All fact-finding communications must be in writing (email, contact forms, or written contemporaneous notes). Recording phone calls across state lines into all-party consent states (CA, FL, PA, etc.) is a criminal felony.
5. **No Submission Without Independent Analysis:** Submitting an ad screenshot to `america.gov/fraud` claiming "this looks fraudulent" will be rejected. Under DOJ guidelines, an outsider must provide **Independent Analysis** - a mathematical, regulatory, or geographic contradiction derived from public records.

---

## 3. The 4 Input Vectors & Public Registry Ingestion

Every public solicitation must provide a way for the public to respond. The system ingests clues via four standard input vectors:

| Input Vector | Deterministic Processing |
|---|---|
| **Vector 1: Image / Photo** | Ingests the photo using `qui-tam ingest-observation` / `whip quick-pack`. Computes cryptographic SHA-256/512 hashes. EXIF GPS coordinates are stripped immediately. |
| **Vector 2: Website URL or QR Code** | Safely inspects the public landing page, extracting `/terms`, `/privacy`, `/shipping`, `/faq`, and `/contact` to identify corporate names, registered business addresses, and parent entities. |
| **Vector 3: Social Media Ad** | Queries or directs the user to public ad transparency repositories (Meta Ad Library, Google Ads Transparency) to identify registered business advertisers and payer entities. |
| **Vector 4: Street Address or Phone** | Runs business address and ZIP code through the CMS NPPES Organizational Provider (`NPI-2`) query. |

---

## 4. Public Registry Entity Lookup (`whip unmask`)

The federal **CMS NPPES v2.1 API** allows querying active healthcare organizations by `postal_code`, `city`, and `state`. 

When a service or agency advertises a public address (e.g., `100 Example Ave, Suite 200, Sample City, CO 80200`), running `justlegalos-cli whip unmask --address "100 Example Ave" --state CO --zip 80200` automatically searches CMS NPPES for active organizational providers (`NPI-2`) registered at that location.

* **What this verifies:** Verifies whether the entity operates as a licensed Home Health Agency, Durable Medical Equipment (DME) Supplier, or Clinic with active Medicaid/Medicare billing numbers.

---

## 5. Proactive Fact-Finding: Written Informational Inquiry & OPSEC

When an ad or public sign provides a contact form or email, the user may send a **Standard Written Informational Inquiry** (not an accusatory demand):

### The Script:
> **Subject:** Inquiry regarding your public services in [State]  
> **Body:**  
> Hello,  
> I saw your public posting regarding [medical supplies / family caregiving].  
> Could you please share how your intake process works, what rates or terms apply, and what legal business name and mailing address packages or intake forms are sent to if someone decides to use your service?  
> Thank you.

### OPSEC & Compliance Rules:
1. **Official Channels:** Keep inquiries in writing through normal contact forms or emails.
2. **Ask Only for Terms & Business Name:** Never offer to sell prescription items, never sign up for benefits, and never provide personal medical or financial information.
3. **No Pretext Calls in Two-Party States:** If speaking on the phone, take contemporaneous handwritten/typed notes. Never advise anyone to make an audio recording.
4. **Public Registry Verification:** Once a legal business name or address is confirmed, verify it against state Secretary of State corporate records and CMS NPPES.

---

## 6. The 3-Outcome Resolution Engine

Every `whip unmask` execution terminates in one of three deterministic outcomes:

### Outcome A: Verified NPI Nexus (`VERIFIED_NPI_NEXUS`)
* **Trigger:** An active 10-digit organizational NPI (`NPI-2`) is matched to the brand or operating address.
* **Action:**
  1. Unlocks the **DOJ WHIP Independent Analysis Correlation Block** (`28 U.S.C. § 530C(b)(1)(L)` / `america.gov/fraud`).
  2. Generates Stage 2 Regulatory CORA/FOIA requests using `justlegalos-cli records audit-provider`:
     - **To State Medicaid Agency:** Monthly reimbursements by procedure code, EVV manual override percentage, and GPS exception rate.
     - **To State Health Licensor:** CMS-2567 deficiency surveys and Registered Nurse (RN) supervisory staffing counts.

### Outcome B: Identified Corporate Entity (`IDENTIFIED_NON_NPI_CORPORATE_ENTITY`)
* **Trigger:** A concrete physical receiving facility, registered LLC, or corporate domain is identified operating without an active healthcare provider NPI.
* **Action:**
  1. Logs the registered business name, state corporate SOS registry ID, and registered agent.
  2. Prepares Stage 2 open-records inquiries (state checkbook expenditures, state contracting awards, business filings) to establish financial nexus.
  3. Enforces the sequential stage lock: substantive statutory violations (e.g., 18 U.S.C. § 1347 or 31 U.S.C. § 3729) are only evaluated at Stage 4 after primary records are obtained.

### Outcome C: Unresolved Public Lead (`UNRESOLVED_PUBLIC_LEAD`)
* **Trigger:** The clue is strictly an unverified contact phone or flyer on a pole with zero discoverable corporate or registry footprint.
* **Anti-Doxxing Guardrail (Refusal to Target Individuals):** The system refuses to fabricate an analysis packet and does not attempt to track, unmask, or subpoena private telephone subscribers.
