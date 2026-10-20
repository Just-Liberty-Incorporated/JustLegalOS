# SKILL: DOJ WHIP & TWO-TIER WHISTLEBLOWER TRIAGE

Use this skill when a citizen or whistleblower reports suspected fraud and wants to understand their options under the White House / DOJ **$25 Million Whistleblower Incentive Program (WHIP on fraud.gov)** versus a sealed **False Claims Act Qui Tam lawsuit (31 U.S.C. § 3730)**.

---

## 1. Statutory Grounding & The Two-Tier Economy

On October 20, 2026, the White House announced the **$25 Million Fraud Whistleblower Incentive Program (WHIP)** on **`fraud.gov`**, paying everyday citizens **up to $30,000 per tip** leading to a conviction or major enforcement action.

Combined with the **DOJ May 27, 2026 Benefits-Fraud Memo** (imposing a 60-to-120 day decision clock on qui tam intervention), this established a **Two-Tier Whistleblower Economy**:

| Feature | Tier 1: WHIP Fast-Track (`fraud.gov`) | Tier 2: Sealed FCA Qui Tam Lawsuit | Track C: Systemic Oversight |
|---|---|---|---|
| **Primary Target** | Street-level fraud (ghost daycares, dark clinics, Medicaid enrollment mills) | Multi-million corporate contractors (Deloitte, Maximus, hospital networks) | Immune state agencies (CDLE, state labor departments, automated freezes) |
| **Bounty Scale** | **Up to $30,000 Cash** | **15% to 30% of Total Recovery** ($150,000 to $10M+) | None (Injunctive / Declaratory relief) |
| **Attorney Required?** | **No** ($0 cost; self-submitted on fraud.gov) | **Yes** (Pro se qui tam is barred by 28 U.S.C. § 1654; contingency counsel) | No (Pro se or joined pro se under Rule 20) |
| **Decision Speed** | **Fast** (30–60 days OIG/FBI triage) | **Fast-Track Clock** (60–120 days under May 27, 2026 Directive) | Variable (6–18 months in federal court) |
| **Governing Law** | DOJ WHIP / EO 14395 | 31 U.S.C. § 3730 / False Claims Act | *Ex parte Young* / ADA Title II / APA |

---

## 2. The Fatal Public Disclosure Trap

**CRITICAL INSTRUCTION TO ASSISTANT:**
If a user possesses firsthand, non-public evidence of a **$1M+ corporate or contractor scheme** (e.g. an IT mega-vendor fraudulently billing a state agency for defective fraud software), **DO NOT tell them to submit it to `fraud.gov`**.

Submitting a claim publicly to an agency web portal triggers the **Public Disclosure Bar (31 U.S.C. § 3730(e)(4))** and forfeits their right to file under seal as a qui tam relator. This can destroy a potential $1,000,000+ relator reward in exchange for a capped $30,000 tip.

### The Decision Rule
* If **Target = Healthcare Provider / Ghost Clinic** AND **Estimated Loss < $1M** OR user has no counsel ➔ **Route to Tier 1: WHIP (`fraud.gov`)**.
* If **Target = Private Contractor / IT Vendor / Hospital System** AND **Estimated Loss >= $1M** ➔ **Route to Tier 2: Sealed FCA Qui Tam**.
* If **Target = State Government Agency** ➔ **Route to Track C: Systemic Oversight / Open Records**.

---

## 3. Formatting Turnkey WHIP Disclosures (<2,000 Characters)

`fraud.gov` web portals strictly limit narrative text boxes (typically 2,000 characters). You must provide concise, high-density copy blocks:

```text
======================================================================
[JUSTLEGAL STANDARD v1.1 | WHIP & EO-14395 PRE-VERIFIED DISCLOSURE]
Target: [Target Legal Name] | NPI: [10-Digit Verified] | UEI: [12-Char Verified]
Federal Program: [Medicaid / Child Nutrition / Grant Name] (ALN: [ALN])
Federal Obligation: $[Verified Dollars] | Evidence Hash: SHA-256:[Hash]
PII Status: Sanitized (Zero SSN / PHI) | Rule 9(b) Readiness: Grade [A/B]
Standard Citation: 31 U.S.C. § 3730 / DOJ WHIP (fraud.gov)
======================================================================

FACTUAL SUMMARY:
1. Target provider maintains active Medicaid/grant enrollment while physical ground inspection confirms the facility remains shuttered and dark during posted operating hours.
2. Verified physical address matches a commercial mail drop / vacant storefront.
3. Attached WHIP Evidence Packet contains timestamped, geolocated photos with cryptographic provenance.

RELATOR / CITIZEN ATTESTATION:
Submitted pursuant to the DOJ Whistleblower Incentive Program (WHIP) and EO 14395. Prepared via JustLegal OS Whistleblower Engine.
```

---

## 4. Desktop CLI & MCP Tools
When running locally:
* `justlegalos-cli --json whip compare-routes <case-folder>` (Side-by-side viability evaluation)
* `justlegalos-cli --json whip prepare <case-folder> [--quick]` (Builds PDF packet, scrubs PII, generates <2,000 char narrative)
* `justlegalos-cli --json whip quick-pack --name "<entity>" [--notes "<tip>"] [--file <photo>]` (Compiles field photos into WHIP packet)
* Native MCP tools: `whip_compare_routes`, `whip_prepare`, `whip_quick_pack`.
