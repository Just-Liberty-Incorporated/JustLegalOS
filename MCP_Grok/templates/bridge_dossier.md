# Standardized Inter-Bot Bridge Dossier Template

When handing off a matter from one assistant or session to another, generate and present this exact block to the user so they can copy and paste it into the destination bot or counsel consultation:

```text
======================================================================
JUSTLEGAL BRIDGE DOSSIER: INTER-BOT & COUNSEL HANDOFF
======================================================================
• DATE: [YYYY-MM-DD]
• STATE / JURISDICTION: [e.g. Colorado / California / Minnesota / Federal]
• TARGET ENTITY: [Entity Legal Name] (NPI: [10-Digit] | UEI: [12-Char])
• CASE DIRECTORY: [/workspace/cases/YYYYMMDD-entity-slug/]
• 4-STATE VERIFICATION STATUS:
  - Primary Entity: [VERIFIED / USER_STATED / LOOKUP_FAILED]
  - Funding Nexus: [VERIFIED: $Amount / State Flow / MISSING]
  - Evidence Hashes: [VERIFIED: Full 64-char SHA-256 Manifest]
• ROUTE HOLD FORK:
  - Insider Records Held: [true / false]
  - Public Portal Blocked: [true / false]
• SUMMARY OF USER-STATED FACTS:
  - [1-3 concise bullet points summarizing facts established so far]
• SECURED EVIDENCE:
  - [List of files and full 64-character SHA-256 digests]
• REASON FOR TRANSFER:
  - [e.g., Matter involves non-public records requiring sealed False Claims Act attorney consultation]
• STATUTORY NOTICES:
  - [Public Disclosure Bar (31 U.S.C. § 3730(e)(4)) / Wiretap consent rule / Not legal advice]
======================================================================
```
