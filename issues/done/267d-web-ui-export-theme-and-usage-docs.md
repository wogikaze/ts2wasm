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

- [x] Implement JSON export for test, coverage, and history views.
- [x] Implement CSV export for tabular views.
- [x] Implement PDF export or a documented browser-print/PDF export path that is exposed from the UI.
- [x] Add dark/light theme toggle with persisted preference.
- [x] Add final-state usage/deployment documentation under `docs/`.

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

- [x] Export control produces JSON for each view.
- [x] Export control produces CSV for test results and history.
- [x] Export control provides PDF output or a documented equivalent from the UI.
- [x] Theme toggle switches dark/light mode and persists across reloads.
- [x] `docs/` contains web UI usage and deployment instructions.
- [x] Docs/current-state/issues are synchronized when status or design changes.

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

- [x] updated: `docs/18-web-ui-reporting.md`

Current state:

- not affected

Follow-up issues:

- none

## Notes

Keep export behavior local/static-host friendly; issue 267 explicitly excludes authentication and CI/CD integration.

## Progress evidence

2026-04-29:

- Added active-tab JSON export for test results, coverage, and history data.
- Added active-tab CSV export for test rows, coverage summary/suite rows, and history rows with deltas/regression flags.
- Connected the PDF control to the browser print flow as the local/static-host friendly PDF path.
- Updated `web-ui/README.md` to document the export controls and current chart/trend status.
- Remaining scope before close: dark/light theme toggle, persisted theme preference, and final-state `docs/` usage/deployment documentation.

2026-04-29 theme slice:

- Added a dark/light theme toggle in the header.
- Persisted the selected theme in `localStorage` and restored it on reload, falling back to host `prefers-color-scheme`.
- Updated `web-ui/README.md` to document the theme control.
- Remaining scope before close: final-state `docs/` usage/deployment documentation and final close validation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `15295019` issue-267d export controls
- `ee1184ef` issue-267d theme toggle
- close commit records final `docs/` usage/deployment documentation and issue lifecycle evidence

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cd web-ui && npm run build
result: PASS
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/267d-web-ui-export-theme-and-usage-docs.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
