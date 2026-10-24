# Skill: Surface Clue Unmasking & Proactive OSINT (`skills/unmask.md`)

## 1. When to Use This Skill
Use this skill when handling **Archetype 3: The Outsider Pattern-Spotter**—where a citizen does **not** have personal records or an insider employer relationship with the target, but observes public solicitations of suspect healthcare or government-benefit schemes:
* **Social Media Ads:** Facebook, Instagram, TikTok, or YouTube video ads (e.g., aggressive paid family caregiving recruitment or "free government medical gear").
* **Physical Street Solicitations:** Signs on telephone poles, lawn signs, bus stop flyers, or QR codes (e.g., `"Cash for Diabetic Test Strips — Call 720-555-0199"`).
* **Gray-Market Websites & Spam:** Buyback landing pages, unsolicited texts, or Craigslist posts offering cash for prescription medical supplies (`Rx Only` CGMs, Omnipods, test strips) or SNAP/Medicaid benefits.

---

## 2. Hard Safety & Relator Standing Boundaries

```
                 [ UNSTRUCTURED SURFACE CLUE ]
            (Social Media Ad, Pole Sign, QR Flyer)
                               │
       ┌───────────────────────┴───────────────────────┐
       ▼                                               ▼
[ FORBIDDEN: User "Stings" ]              [ ALLOWED: Structured OSINT & Inquiry ]
❌ Calling burner numbers from phone    ✅ Cloud Sandbox URL & Terms inspection
❌ Pretending to be patient/seller      ✅ Meta / Google Public Ad Library lookup
❌ Secret audio recordings (wiretaps)   ✅ CMS NPPES Reverse-Address NPI lookup
❌ Submitting real PII/SSN to forms    ✅ Plain-English Informational Inquiry (Alias)
⚠️ Disqualifies from DOJ WHIP           ✅ State Regulatory CORA Audit (EVV/Staffing)
⚠️ Wiretap & criminal entrapment risk   ✅ 100% Protected, Clean Relator Standing
```

1. **Never Advise Undercover "Stings" or Pretexts:** Never instruct the user to call a scam line from their personal phone, pose as an active participant in illicit device trafficking, or submit their real name/SSN into a gray-market form. Under **28 U.S.C. § 530C(b)(1)(L)**, active participants in criminal offenses are disqualified from whistleblower awards.
2. **Never Violate Wiretap Consent Laws:** Recording phone calls across state lines into all-party consent states (CA, FL, PA, etc.) is a criminal felony. All investigative communications should be conducted in writing (email, contact forms, or written contemporaneous notes).
3. **Never Submit Raw Ads to DOJ Without Independent Analysis:** Submitting an ad screenshot to `america.gov/fraud` claiming "this program seems fraudulent" will be rejected. Under the DOJ WHIP Guidelines, an outsider must provide **Independent Analysis**—a mathematical, regulatory, or geographic contradiction derived from public records.

---

## 3. The 4 Input Vectors & Passive OSINT Engine

Every public solicitation must provide a way for people to respond. The bot ingests the clue via one or more of four input vectors:

| Input Vector | How the Bot Handles It Deterministically |
|---|---|
| **Vector 1: Image / Photo** | Ingests the photo using `qui-tam ingest-observation` / `whip quick-pack`. Computes cryptographic SHA-256/512 hashes and extracts EXIF timestamp and GPS coordinates (preserving physical pole/sign coordinates). |
| **Vector 2: Website URL or QR Code** | Safely inspects the public landing page, extracting `/terms`, `/privacy`, `/shipping`, `/faq`, and `/contact` to find physical shipping addresses, PO Boxes, CMRA suites (UPS Store / Regus), LLC names, and support domains. |
| **Vector 3: Social Media Ad** | Directs the user to or queries the public **Meta Ad Library** (`facebook.com/ads/library`) or Google Ads Transparency Center to extract Page ID, linked domains, and verified payer entities. |
| **Vector 4: Street Address or Phone** | Runs phone number strings across public directories; runs street address and ZIP code through the CMS NPPES Reverse-Address query. |

---

## 4. The Reverse-Address NPI Weapon (`whip unmask`)

The free federal **CMS NPPES v2.1 API** allows querying by `postal_code`, `city`, and `state`. 

When a buyback website or caregiver agency provides a return shipping address (e.g., `950 S Cherry St, Denver, CO 80246`), running `justlegalos-cli whip unmask --address "950 S Cherry St" --state CO --zip 80246` automatically searches CMS NPPES for all active Medicare/Medicaid providers registered at that address.

* **What this unmasks:** Gray-market buyers often funnel prescription devices to a warehouse that is also registered to an active **Durable Medical Equipment (DME) Supplier, Retail Pharmacy, or Home Health Agency (NPI-2)**—directly establishing a False Claims Act / Anti-Kickback nexus!

---

## 5. Proactive Fact-Finding: The Informational Inquiry Script & OPSEC Rules

When an ad or telephone pole sign hides its receiving warehouse address behind a phone number or contact form, the bot provides the user with a **Plain-English Informational Inquiry** (not an aggressive legal demand that gets deleted):

### The Script:
> **Subject:** Inquiry regarding your public services in [State]  
> **Body:**  
> Hello,  
> I saw your public posting regarding [medical supplies / family caregiving].  
> Could you please share how your intake process works, what rates or terms apply, and what legal business name and mailing/shipping address packages or intake forms are sent to if someone decides to use your service?  
> Thank you.

### The 4-Point OPSEC Checklist:
1. **Dedicated Alias / Non-Personal Channel:** Use a disposable/alias email (e.g., SimpleLogin, Proton, or burner email) or VoIP number—never your personal cell phone or primary email.
2. **Ask Only for Process & Address:** Never offer to sell prescription items, never sign up for benefits, and never provide personal medical or financial information.
3. **Keep It in Writing:** Written responses (emails, quote sheets, return shipping labels) serve as immutable, hashable documentary evidence.
4. **No Pretext Calls in Two-Party States:** If speaking on the phone, take contemporaneous handwritten/typed notes; do not record audio without knowing state wiretap laws.

---

## 6. The 3-Outcome Resolution Engine

Every `whip unmask` execution terminates in one of three deterministic outcomes:

### Outcome A: Verified NPI Nexus (`VERIFIED_NPI_NEXUS`)
* **Trigger:** An active 10-digit NPI, SAM.gov UEI, or licensed provider is matched to the brand or warehouse address.
* **Action:**
  1. Unlocks the **DOJ WHIP Independent Analysis Correlation Block** (`28 U.S.C. § 530C(b)(1)(L)` / `america.gov/fraud`).
  2. Generates Stage 2 Regulatory CORA requests using `justlegalos-cli records audit-provider`:
     - **To State Medicaid Agency:** Monthly reimbursements by procedure code, EVV manual override percentage, and GPS exception rate.
     - **To State Health Licensor:** CMS-2567 deficiency surveys and Registered Nurse (RN) supervisory FTE staffing counts.

### Outcome B: Identified Diversion / Kickback Ring (`IDENTIFIED_DIVERSION_RING`)
* **Trigger:** A concrete physical receiving warehouse, LLC registration, or domain network was identified trafficking prescription medical devices (`Rx Only` CGMs/Omnipods) or soliciting protected benefits, but no direct NPI is held in that LLC name.
* **Action:** Formats a **Supply-Chain Diversion & Anti-Kickback Referral Packet** citing:
  - Federal Health Care Offenses (`18 U.S.C. § 24`)
  - Federal Anti-Kickback Statute (`42 U.S.C. § 1320a-7b(b)`)
  - Causing False Claims (`31 U.S.C. § 3729(a)(1)(A)`)
  - FDCA Felony Medical Device Misbranding (`21 U.S.C. §§ 331, 333`)
  - Routes to: **`america.gov/fraud` (DOJ WHIP)**, **FDA Office of Criminal Investigations (FDA-OCI)**, **HHS-OIG**, and **State MFCU**.

### Outcome C: Unresolved Burner Lead (`UNRESOLVED_BURNER_LEAD`)
* **Trigger:** The clue is strictly an offline burner phone number on a pole with zero discoverable web or corporate footprint.
* **Honest Guardrail (Refusal to Fabricate):** The system refuses to fabricate an analysis packet.
* **Action:** Hashes the timestamped photograph of the sign with EXIF GPS coordinates and exports a **1-Page Law Enforcement Telecom Subpoena Lead** for the **State MFCU / FTC / HHS-OIG**, paired with the Informational Inquiry Script if the user wishes to obtain their shipping address safely.
