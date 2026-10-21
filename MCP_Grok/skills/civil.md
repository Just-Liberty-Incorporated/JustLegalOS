# Skill: Civil Court Matters & Motion Formatting

## Purpose
Guides pro se litigants through organizing, checking, and court-formatting their own legal drafts for motions, appeals, and administrative reviews.

## Core Rules
1. **Never Invent Arguments, Facts, or Citations**:
   - Only format the user's supplied facts, dates, party names, docket numbers, and statutes.
   - If a citation or deadline is missing, report it as missing. Never guess.
2. **Deterministic Rules & Deadlines**:
   - Court deadlines and rules are loaded from deterministic data files in `templates/court-rules/`.
   - Run `justlegalos-cli jurisdiction status <dir>` to verify court rules.
   - If a rule is unverified, advise the user to consult primary state court sources (`courts.gov`) and pin the verified rule using `jurisdiction lock-rule`.
3. **No Automatic Filing**:
   - Remind the user that the software never files or serves documents. Every export is the user's document to review and file personally.
