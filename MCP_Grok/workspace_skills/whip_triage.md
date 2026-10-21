# WHIP Fast-Track Triage Skill

This skill is loaded when the user investigates potential government program or healthcare fraud, or when they reply STAY to Bot 2.

## 1. Safety & Anti-Viral Warning (Turn 1)
If the user observes a suspicious facility or program:
1. Warn them immediately:
   > "Stay safe, and **do NOT post this to social media (X, TikTok, YouTube)**. Posting publicly can destroy your whistleblower standing under the Public Disclosure Bar (31 U.S.C. § 3730(e)(4)) and tips off the fraudsters. Keep your observations private."
2. Ask the 3 Plain-English Intake Questions:
   - What city and state is this in?
   - What is the exact business name on the sign or awning?
   - What did you see (e.g. empty building during posted hours, suspicious billing, fake daycare)?

## 2. State Trap Verification
Before advising any photography or recording:
Run:
```bash
justlegalos-cli check state-traps --state <ST>
```
Summarize the findings in plain English:
- If 2-party recording state: explicitly warn never to record audio without consent.
- If California: cite California AB 2624 (Cal. Gov. Code § 6218.20) and warn that publishing photos/videos of workers on social media carries fines, but private submission to federal authorities (america.gov/fraud) or an attorney is protected whistleblowing.
- Reiterate that observations must be made strictly from public sidewalks or public parking lots (no trespassing).

## 3. Case Folder & Entity Verification
Create the case folder and verify public registry data:
```bash
CASE_DIR="/workspace/cases/$(date +%Y%m%d)-entity-slug"
mkdir -p "$CASE_DIR/evidence"
justlegalos-cli qui-tam verify-entity --name "<name>" --state <ST>
```

## 4. Evidence Ingestion & Scoring
When the user attaches photos or documents:
1. Save them into `$CASE_DIR/evidence/`.
2. Ingest and hash:
   ```bash
   justlegalos-cli qui-tam ingest-observation "$CASE_DIR" --file "$CASE_DIR/evidence/<filename>" --notes "<user observation>"
   ```
3. Run the dual audit:
   ```bash
   justlegalos-cli whip audit "$CASE_DIR"
   ```

## 5. Scoring Gates & Actions
- **Tier 1 (Score 0–45 / Raw Hunch):** Explain what is missing. Provide a 2-step checklist (e.g., photograph posted hours plaque, observe on a second business day). Offer the raw unverified tip submission if they prefer not to do homework.
- **Tier 2 (Score 46–79 / Developing Record):** Prompt for state licensing or public records verification (`records draft`).
- **Tier 3 (Score 80–100 / Actionable Dossier):**
  1. Generate the Independent Analysis block via `justlegalos-cli whip prepare "$CASE_DIR"`.
  2. Present `/workspace/templates/four_door_menu.md` in plain English.
  3. If loss exceeds $1M with internal billing records, provide `/workspace/templates/bridge_dossier.md` to Bot 3.
