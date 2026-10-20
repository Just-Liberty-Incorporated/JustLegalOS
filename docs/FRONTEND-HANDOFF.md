# Frontend redesign handoff

The desktop is the durable workspace; the user's external agent operates the local CLI/MCP engine. The AI page provides a copyable operating briefing with 99 command help entries, installation context, workflows, and confidentiality boundaries. There is no embedded chatbot.

The light application shell now provides compact navigation, a searchable matter list, simplified matter creation, contextual matter sections, a forms-first library, and a split draft/preview view. Specialist operations remain under the relevant matter sections and Tools & settings. Notices are contextual or expandable.

Visible workspaces refresh on focus and periodically. Detected external changes cannot silently replace unsaved edits; the user can explicitly discard local edits and reload. This is frontend conflict detection, not an atomic backend lock.

## Verification

- `npm run check`: zero errors; 42 remaining warnings in 10 files.
- `node --test tests/frontend-workflow.test.cjs`: all four tests pass.
- `git diff --check`: no whitespace errors.
- Browser checks: briefing copy, matter creation, non-litigation navigation, two-tab external-change conflict and explicit reload, forms empty state, narrow and desktop layouts.

The browser uses sample data. Installed forms, native file reveal, vault operations, real CLI-driven refresh, and packaged executable path detection must be checked in the rebuilt desktop application. No native executable was rebuilt for this change.

## CLI reference maintenance

Run `node scripts/generate-cli-reference.mjs` after updating the repository's CLI executable to refresh `src/lib/data/cli-reference.json`. The generator only reads command help. The briefing instructs agents to verify the installed help before operating because installed capabilities can differ from the bundled reference.
