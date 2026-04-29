---
id: 267d
title: "Implement web UI export controls, theme toggle, and usage docs"
type: feature
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: [267]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Complete the user-facing web UI controls and documentation that are currently placeholders or missing from final-state docs.

## Problem

The header contains an Export button with no behavior, `web-ui/README.md` marks JSON/CSV/PDF export and dark/light theme toggle as planned, and no final-state `docs/` usage/deployment page exists.

Problem: Export, theme, and final-state web UI docs do not satisfy issue 267 acceptance.

## Current failure

Evidence from `web-ui/src/App.tsx` and `web-ui/README.md`:

- Export button has no click handler or file generation behavior.
- No JSON, CSV, or PDF export implementation exists.
- No theme state or dark/light toggle exists.
- Web UI documentation is only in `web-ui/README.md`, while issue 267 expects `docs/` usage documentation.

## Desired final state

Users can export every view in the requested formats, switch dark/light theme, and follow final-state docs to run, build, and deploy the web UI.

## Scope

In scope:

- [ ] Implement JSON export for test, coverage, and history views.
- [ ] Implement CSV export for tabular views.
- [ ] Implement PDF export or a documented browser-print/PDF export path that is exposed from the UI.
- [ ] Add dark/light theme toggle with persisted preference.
- [ ] Add final-state usage/deployment documentation under `docs/`.

Out of scope:

- Real-time update transport.
- Test runner integration.
- Authentication/authorization.

## Affected paths

Expected:

- `web-ui/src/**`
- `web-ui/README.md`
- `docs/`

Do not touch:

- Compiler/runtime Rust files.
- Unrelated fixtures.

## Acceptance criteria

- [ ] Export control produces JSON for each view.
- [ ] Export control produces CSV for test results and history.
- [ ] Export control provides PDF output or a documented equivalent from the UI.
- [ ] Theme toggle switches dark/light mode and persists across reloads.
- [ ] `docs/` contains web UI usage and deployment instructions.
- [ ] Docs/current-state/issues are synchronized when status or design changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cd web-ui && npm run build
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cd web-ui && npm run dev
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/` web UI usage/deployment documentation

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Keep export behavior local/static-host friendly; issue 267 explicitly excludes authentication and CI/CD integration.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
