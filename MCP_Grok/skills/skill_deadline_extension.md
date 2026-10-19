# SKILL: PROCEDURES FOR DEADLINE EXTENSION & ENLARGEMENT OF TIME

Use this skill when a user is facing an imminent court deadline (such as filing an Answer, an Appellate Brief, or responding to a motion) and needs to explore procedural options to request additional time from opposing counsel or the court to gather records, seek legal advice, or prepare their filing.

---

## 1. Procedural Framework: FRCP Rule 6 & State Court Rules (Court Rules, Not Statutes)
Under Federal Rule of Civil Procedure 6(b) and equivalent state court procedural rules (e.g., California CRC 3.110(d), New York CPLR 2004, Texas TRCP 5):
1. **Court Rules vs. Statutes:** These are rules of judicial administration, not legislative statutes. Their application depends on local court rules and judicial discretion.
2. **Request Before Expiration:** A motion or stipulation made *before* the original deadline expires requires only a showing of **"good cause"** (e.g. gathering necessary records, retaining counsel, illness, complexity of issues).
3. **Request After Expiration:** A motion made *after* the deadline has already passed faces a much higher standard: **"excusable neglect"**.
4. **Summary & Expedited Matters:** In summary proceedings (such as unlawful detainer / eviction actions or expedited injunction hearings), time extensions are frequently restricted, disfavored, or subject to shortened response windows.
5. **Non-Extendable Jurisdictional Deadlines:** Warn the user immediately that certain deadlines **CANNOT be extended by the court**, including:
   - Motions for a New Trial (FRCP 59(b))
   - Motions to Alter or Amend a Judgment / Reconsideration (FRCP 59(e))
   - Renewed Motions for Judgment as a Matter of Law (FRCP 50(b))
   - Jurisdictional notices of appeal (strict deadlines under appellate rules with very limited relief).

---

## 2. The 2-Tier Extension Sequence

### Tier 1: Consent / Stipulation from Opposing Party
Most court rules require or encourage parties to confer before burdening the judge with a motion.
1. **Initial Courtesy Inquiry:** Send a formal, polite email or letter to opposing counsel requesting an enlargement of time (typically 14 to 30 days, as permitted by local court rules).
   - Explain good cause: newly received records, recent adverse event, need to ensure full and accurate presentation.
   - Ask for written confirmation by a specific business day/time.
2. **Drafting a Stipulation:** If opposing counsel agrees:
   - Prepare a simple "Stipulation and Order Extending Time to File [Pleading/Brief]".
   - Both parties sign (or pro se filer notes opposing counsel's consent).
   - Submit immediately to the clerk or judge for endorsement.

### Tier 2: Unilateral Motion for Enlargement of Time
If opposing counsel refuses, ignores the request, or time is too short:
1. **Notice of Motion for Extension of Time:** File a formal motion with the clerk before the expiration date.
2. **Supporting Declaration:** Filer signs a certification under penalty of perjury outlining:
   - Specific date original pleading or order was received.
   - Good cause why additional time is necessary.
   - Detail of efforts to confer with opposing counsel and their response or lack thereof.
   - Assurance that the request is made in good faith and not for purposes of undue delay.
   - Length of requested extension (standard: 14 to 30 days).
3. **Proposed Order:** Include a clean, one-page proposed order granting the extension.

---

## 3. JustLegal OS Integration

Litigants can use JustLegal OS to prepare extension papers:
```bash
# Verify rules and deadlines for the venue
justlegalos-cli --json check venue ./my-case

# Check applicable time limits and extension rules
justlegalos-cli --json guide find --state <STATE> --situation extension

# Assemble motion and certification
justlegalos-cli --json export court-md ./my-case
```
