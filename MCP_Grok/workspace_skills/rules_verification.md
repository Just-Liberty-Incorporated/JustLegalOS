# Jurisdiction & Court Rule Verification Protocol

## 1. Zero-Hallucination Rule Mandate
GrokBots and autonomous agents must **never invent, assume, or guess** a court rule, filing deadline, procedural limitation, or page limit. If an agent does not have a verified citation from an installed pack or primary government source, it must transparently declare an **Unverified Gap** and provide the official `.gov` lookup URL to the user.

## 2. The 3-State Traffic-Light Readiness Model
The JustLegal OS engine tracks 5 critical statutory and procedural slots for every active case matter:
1. **State Traps & Investigation Consent:** (1-party vs 2-party recording consent, CA AB 2624 social media safe harbor, First Amendment public observation).
2. **Primary Official Sources:** Direct `.gov` statutory portal and supreme court rulebook for the governing state and federal circuit.
3. **Fraud & False Claims Act Scope:** Federal False Claims Act (31 U.S.C. § 3729) & DOJ WHIP (`america.gov/fraud`) vs state qui tam statutes.
4. **Pre-Filing Open Records Statute:** State public records laws (CORA, OPRA, CPRA, Ohio Sunshine) with exact statutory agency response clocks.
5. **Local Court & Vicinage Practice:** County standing orders, motion page limits, judge rules, and electronic filing formats.

Each slot is evaluated into one of 4 deterministic states:
* 🟢 **VERIFIED_PACK:** Verified offline data pack installed with primary statute and timestamp.
* 🟡 **PACK_AVAILABLE:** Downloadable form/court pack exists in the JustLegal catalog (run `justlegalos-cli packs install --state <ST>`).
* ⚪ **UNVERIFIED_GAP:** Specific local rule or standing order not in local cache. The CLI outputs the exact official `.gov` court site and refuses to guess.
* 🔵 **USER_LOCKED:** Researched by user or GrokBot directly from official `.gov` source and pinned into `jurisdiction_overrides.json`.

## 3. CLI & Bot Tool Verification Commands
To inspect the jurisdiction readiness of a case:
```bash
justlegalos-cli --json jurisdiction status /workspace/cases/<case_id>
```
Or via MCP:
`jurisdiction_status(path: "/workspace/cases/<case_id>")`

To lock in a verified rule after checking the official `.gov` site:
```bash
justlegalos-cli --json jurisdiction lock-rule /workspace/cases/<case_id> \
  --slot local_court_rules \
  --cite "Middlesex County Standing Order 24-02: 25-page brief limit" \
  --url "https://www.njcourts.gov/middlesex"
```
Or via MCP:
`jurisdiction_lock_rule(path: "/workspace/cases/<case_id>", slot: "local_court_rules", cite: "...", url: "...")`

## 4. State-Specific Scope Cheat Sheet
* **Ohio (6th Cir.):** Ohio has **no general private qui tam relator statute**. State-only fraud cannot yield a private relator bounty in state court; it routes to Ohio Auditor Fraud Hotline (O.R.C. § 117.103) or AG MFCU. **To qualify for financial rewards (WHIP $10k–$30k or FCA 15%–30%), the relator must identify federal dollars** (Medicaid ALN 93.778, Title I, ARPA, HHS block grants).
* **Colorado (10th Cir.):** Colorado False Claims Act (C.R.S. § 24-31-1201) and Medicaid FCA (C.R.S. § 25.5-4-303.5) permit state court qui tam relator actions. CORA responses must occur within 3 working days.
* **New Jersey (3rd Cir.):** Part II Court Rules govern Appellate Division; OPRA custodian response within 7 business days.
* **California (9th Cir.):** Cal. Gov. Code § 12650 permits state qui tam relator actions. AB 2624 imposes penalties for posting photos of healthcare/service workers online, but explicitly exempts confidential filings to law enforcement (`america.gov/fraud`).
