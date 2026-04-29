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

- [ ] Web UI framework setup (Vite + React + TypeScript)
- [ ] Test result display with filtering/search
- [ ] Coverage visualization with interactive charts
- [ ] Historical comparison view
- [ ] Performance metrics dashboard
- [ ] Real-time test run updates
- [ ] Export functionality (JSON, CSV, PDF)
- [ ] Responsive design
- [ ] Dark/light theme support

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

- [ ] Web UI can be built and served locally
- [ ] Test results display with pass/fail/skip status
- [ ] Filtering by test suite, status, and search
- [ ] Coverage charts show implementation progress
- [ ] Historical comparison shows regressions
- [ ] Performance metrics displayed with trends
- [ ] Real-time updates during test runs
- [ ] Export functionality works for all views
- [ ] Documentation covers usage and deployment
- [ ] Docs/current-state/issues are synchronized when status or design changes

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

- [ ] not affected
- [ ] updated: `docs/` (new web UI usage documentation)

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [x] created/updated:
  - `issues/done/267a-web-ui-data-generation-and-script-integration.md`
  - `issues/open/267b-web-ui-interactive-charts-regression-and-performance-trends.md`
  - `issues/open/267c-web-ui-real-time-test-run-updates.md`
  - `issues/open/267d-web-ui-export-theme-and-usage-docs.md`

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
