---
id: 267
title: "Implement interactive web UI for test results"
type: feature
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Build a Playwright-like interactive web UI for displaying test results, coverage, and performance metrics with filtering, historical comparison, and real-time updates.

## Problem

Current test reporting is limited to text/markdown output with no interactive visualization, filtering, or historical comparison capabilities. Developers cannot easily explore test results, identify regressions, or track coverage trends.

Problem: No interactive web UI for test results, coverage visualization, and historical comparison.

## Current failure

```bash
# Current output is text-only
mise run reference-coverage -- test262 --limit 50
# Generates markdown tables, no interactive UI
```

## Desired final state

Interactive web UI with:
- Test result browser with filtering/search
- Coverage visualization with charts
- Historical comparison and regression detection
- Performance metrics tracking
- Real-time updates during test runs
- Export capabilities

## Scope

In scope:

- [x] Web UI framework setup (Vite + React + TypeScript)
- [x] Test result display with filtering/search
- [x] Coverage visualization with interactive charts
- [x] Historical comparison view
- [x] Performance metrics dashboard
- [x] Real-time test run updates
- [x] Export functionality (JSON, CSV, PDF)
- [x] Responsive design
- [x] Dark/light theme support

Out of scope:

- Native mobile apps
- Authentication/authorization
- CI/CD platform integration (initially)

## Affected paths

Expected:

- `web-ui/` (new directory for web UI)
- `scripts/gen/` (new web UI report generation script)
- `scripts/run/test262.py` (add web UI output option)
- `docs/` (new web UI usage documentation)

Do not touch:

- Existing test infrastructure
- Core compiler implementation

## Acceptance criteria

- [x] Web UI can be built and served locally
- [x] Test results display with pass/fail/skip status
- [x] Filtering by test suite, status, and search
- [x] Coverage charts show implementation progress
- [x] Historical comparison shows regressions
- [x] Performance metrics displayed with trends
- [x] Real-time updates during test runs
- [x] Export functionality works for all views
- [x] Documentation covers usage and deployment
- [x] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
cd web-ui && npm run build
```

Impacted commands:

```sh
mise run reference-coverage -- --web-ui
mise run test262 -- --web-ui
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/18-web-ui-reporting.md`

Current state:

- not affected

Follow-up issues:

- [x] created/updated:
  - `issues/done/267a-web-ui-data-generation-and-script-integration.md`
  - `issues/done/267b-web-ui-interactive-charts-regression-and-performance-trends.md`
  - `issues/done/267c-web-ui-real-time-test-run-updates.md`
  - `issues/done/267d-web-ui-export-theme-and-usage-docs.md`

## Notes

Tech stack proposal:
- Vite for fast development and build
- React + TypeScript for UI
- Tailwind CSS for styling
- Chart.js or Recharts for visualization
- WebSocket for real-time updates

Data sources:
- `artifacts/coverage/results/test262.json`
- `test_report.json` schema
- `artifacts/coverage/reference-coverage-matrix.md`

## Progress evidence

Review date: 2026-04-29.

Implemented in `web-ui/`:

- Vite + React + TypeScript project exists with locked dependencies.
- Static test-result browser loads `public/data/test-results.json`.
- Test table displays pass/fail/skip statuses and supports search plus status filtering.
- Coverage tab displays implementation totals and priority breakdown from `public/data/coverage.json`.
- History tab displays run rows with pass/fail/skip counts and compile/runtime fields from `public/data/history.json`.
- `cd web-ui && npm ci` completed on 2026-04-29 with Node engine warnings for Node `v23.6.0`.
- `cd web-ui && npm run build` passed on 2026-04-29.
- `cargo fmt --all --check` passed on 2026-04-29.

Close blockers found during review:

- No suite filter exists; current filters cover search and status only.
- Coverage visualization is progress bars only, not interactive charts.
- History view is a table only; no regression detection or historical diff UI is implemented.
- Performance metrics are displayed as raw fields only; no trends are implemented.
- Real-time updates are not implemented; `web-ui/README.md` explicitly marks them planned.
- Export is a non-functional header button; JSON, CSV, and PDF export are not implemented.
- Dark/light theme toggle is not implemented; `web-ui/README.md` explicitly marks it planned.
- The documented web UI data generator, metadata output, and `--web-ui` command integration referenced by docs/issue are absent.
- Web UI usage docs exist only in `web-ui/README.md`; no final-state `docs/` usage/deployment document exists.

Close evidence added on 2026-04-29:

- issue-267a added web-ui data generation and command integration.
- issue-267b added interactive charts, historical deltas, regression flags, performance trends, and responsive visual review.
- issue-267d added export controls, theme persistence, and final-state web UI usage/deployment docs.
- issue-267c added local live mode with `?live=1`, summary/row refresh without reload, connection state, and static JSON fallback.
- The Test Results tab now supports search, status filtering, and suite filtering.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `acace0a6` issue-267a data generator
- `c855f96` issue-267a command integration
- `4d0aa35f` issue-267b charts/trends
- `5548af98` issue-267b responsive close
- `15295019` issue-267d export controls
- `ee1184ef` issue-267d theme toggle
- `949aad8d` issue-267d docs close
- close commit records issue-267c live mode and parent issue lifecycle evidence

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cd web-ui && npm run build
result: PASS
date: 2026-04-29

command: Playwright desktop/mobile web-ui review and live-mode check
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
