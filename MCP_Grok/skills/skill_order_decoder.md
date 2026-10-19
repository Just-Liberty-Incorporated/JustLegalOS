# SKILL: PLAIN-LANGUAGE COURT ORDER DECODER

Use this skill when a user provides or pastes text from a court order, bench ruling, minute entry, or judgment and needs to understand what happened, what was ordered, and what their immediate obligations or deadlines are.

---

## 1. Primary Objectives
1. **Isolate Operative Rulings from Dicta:** Separate the judge's legal reasoning/background narrative from the actual enforceable mandates ("It is hereby ORDERED...").
2. **Determine Finality (Interlocutory vs. Final Judgment):**
   - **Final Judgment:** Resolves all claims as to all parties. Triggers the applicable Notice of Appeal deadline under governing court rules. *(Never invent or guess an appeal deadline; always check verified rule pack data or advise verifying directly with the court clerk).*
   - **Interlocutory Order:** Resolves only a part of the dispute or an interim motion (e.g. discovery ruling, denial of motion to dismiss without prejudice). May require a Motion for Leave to Appeal or Reconsideration rather than a direct Notice of Appeal.
3. **Identify Potential Tolling Motions:** Certain timely post-judgment motions (such as motions to alter, amend, or reconsider) may toll the appellate clock in some jurisdictions, but tolling rules are strictly jurisdictional and non-extendable. Explain that the user must verify the specific tolling rule with local court rules or counsel.
4. **Flag Mandatory Compliance Deadlines:** Identify any affirmative requirements explicitly stated in the order (e.g. "within 14 days, Plaintiff shall produce...", "hearing scheduled for...").

---

## 2. Order Decoding Protocol

When parsing an order:

### Step 1: Document Metadata Extraction
Extract without inventing:
- **Court / Jurisdiction:** (e.g., Superior Court of California, County of Orange).
- **Judge / Department:** (e.g., Hon. Jane Smith, Dept. C12).
- **Date Signed & Date Entered / Filed:** (Crucial: Deadlines usually run from date of entry on the civil docket, not date of drafting).
- **Document Title / Caption:** (e.g., "Order Granting Motion to Strike with Leave to Amend").

### Step 2: Operative Rulings Breakdown
Organize the findings into a clear, bulleted summary:
- **Granted In Full / Denied In Full / Granted In Part:** Exactly what motions were addressed.
- **Relief Granted:** In plain English, what does the prevailing party get?
- **Relief Denied:** What was rejected?
- **With or Without Prejudice:**
  - *With prejudice:* Final; cannot re-file or amend the same claim in this court.
  - *Without prejudice:* Filer may correct deficiencies and re-submit within the specified timeframe.

### Step 3: Actionable Timeline & Deadlines
Identify trigger events:
- **Compliance Dates:** Highlight any dates explicitly written into the order by the judge.
- **Rule-Based Windows:** Explain what event starts the timer (e.g., notice of entry vs service). Never state an unverified day count. Instruct the user to check local rules or consult an attorney.
- **Clerk Verification:** Always advise the user to confirm their calculation with the clerk of court.

### Step 4: Missing Information Warning
If the user's excerpt omits the filing stamp, judge's signature, or complete text:
> "Note: The pasted excerpt does not include the official clerk entry date stamp. In most civil jurisdictions, deadlines run strictly from the date the order was entered on the civil docket, not the date it was signed or mailed. Check the clerk's docket stamp on your copy and verify the deadline with the clerk's office."

---

## 3. JustLegal OS Desktop & CLI Integration

When the user is ready to assemble their response:
- Import draft or order summary into JustLegal OS:
  ```bash
  justlegalos-cli --json case create --path ./my-case --title "Smith v. Jones" --pack generic
  justlegalos-cli --json check all ./my-case --rule reconsideration --trigger-date <YYYY-MM-DD>
  ```
- Reconcile the order against the court record:
  ```bash
  justlegalos-cli --json record reconcile ./my-case --docket "<Pasted order text>"
  ```
- Generate response filing:
  ```bash
  justlegalos-cli --json export court-md ./my-case
  ```
