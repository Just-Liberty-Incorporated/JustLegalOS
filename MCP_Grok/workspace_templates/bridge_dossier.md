# Standardized Inter-Bot Bridge Dossier Template

When handing off a matter from one GrokBot to another, generate and present this exact block to the user so they can copy and paste it into the destination bot:

```text
======================================================================
[JUSTLEGAL BRIDGE DOSSIER — INTER-BOT HANDOFF]
======================================================================
• ORIGINATING BOT: [Civil Navigator | WHIP Scout | Qui Tam Architect]
• TARGET BOT: [Civil Navigator | WHIP Scout | Qui Tam Architect]
• DATE: [YYYY-MM-DD]
• STATE / JURISDICTION: [e.g. California / Minnesota / Colorado]
• TARGET ENTITY: [Entity Name] (NPI: [10-Digit] | UEI: [12-Char])
• CASE DIRECTORY: [/workspace/cases/YYYYMMDD-entity-slug/]
• CURRENT SCORE: [WHIP Readiness: X/100 | Qui Tam 9(b): Y/100]
• SUMMARY OF FACTS:
  - [1-3 concise bullet points summarizing the facts established so far]
• SECURED EVIDENCE IN WORKSPACE:
  - [List of files and SHA-256 hashes stored in the case folder]
• REASON FOR TRANSFER:
  - [e.g., Matter involves >$1M in federal healthcare billing with non-public records requiring sealed False Claims Act evaluation in Bot 3]
• STATUTORY WARNINGS:
  - [Public Disclosure Bar notice / State trap reminder / Not legal advice]
======================================================================
```
