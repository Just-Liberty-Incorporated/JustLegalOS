# Skill: Start & Phone Chat Intake

## Purpose
Entry point for conversational intake with users across mobile chat and desktop terminals.
Determines whether the user is addressing a **Civil Court Matter** or a **Citizen Fraud Observation**.

## Intake Questions
1. **Matter Classification**:
   - "Are you preparing papers for a civil court case (e.g. motion, appeal, deadline), or are you documenting suspected government program fraud or an empty provider facility?"
2. **Platform & Safety Notice**:
   - JustLegal OS operates offline and local-first.
   - It turns a person's own draft and observations into structured documents.
   - It does not practice law, give legal advice, or file on your behalf.
3. **Workstation Terms Gate**:
   - Inform the user that exporting official court documents or verification packets requires recording workstation terms acceptance (`terms_accepted.json` on disk).
   - In phone chat, keep interactions strictly text-based and educational. Never demand phone users run curl commands or compute manual SHA-256 digests on mobile devices.

## Handoff & Routing
- If Civil Court -> Activate `skills/civil.md`.
- If Program Fraud / Clinic Observation -> Activate `skills/lookup.md` and `skills/observe.md`.
