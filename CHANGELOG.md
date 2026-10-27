# Changelog

## 0.7.4 - 2026-10-26

- Release workflow: set GH_REPO for the publish step. The v0.7.3 run built every asset but could not publish because gh had no repository context, so no v0.7.3 release exists. v0.7.4 contains the same engine and skill changes as the v0.7.3 tag.
- Version bumped to 0.7.4.

## 0.7.3 - 2026-10-26

- Court gateway and `jurisdiction lock-rule --fetch`: fetches are limited to whitelisted official court domains over HTTPS only. Plain http URLs, open `.us` domains, and another state's court domains are rejected. Fetched rule text must contain the quoted rule text before it is locked.
- Jurisdiction: a federal court venue stays federal and is no longer converted to the state where the court sits.
- Gate 5 (`whip approve`): scripted approval is refused on Windows when the parent process is a versioned Python executable such as `python3.13.exe`. Fixed in both the Python fallback (`scripts/whip_gate5.py`) and the Rust CLI (`handlers_whip.rs`).
- Release workflow: publishes only to a new tag, refuses to publish over `v0.7.2` or `latest`, names installers for the tag's version, and checks the published files against `SHA256SUMS.txt`.
- Version bumped to 0.7.3 across the Rust CLI, desktop app, Python fallback, MCP server, and the Linux installer.

## 0.7.2

- See the v0.7.2 GitHub release.