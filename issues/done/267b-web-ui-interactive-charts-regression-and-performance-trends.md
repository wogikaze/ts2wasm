---
id: 267b
title: "Implement web UI interactive charts, regression detection, and performance trends"
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

Upgrade the current coverage/history tables and progress bars into the interactive visualization and trend views required by issue 267.

## Problem

The current UI shows coverage as static progress bars and history as a table. It does not show interactive charts, regression detection, historical comparison deltas, or performance trends.

Problem: Web UI visualization is currently static and does not expose regressions or trends.

## Current failure

Evidence from `web-ui/src/App.tsx`:

- Coverage view renders progress bars and priority counts only.
- History view renders a table of runs only.
- No Recharts components are used despite `recharts` being installed.
- No regression/delta calculation exists.

## Desired final state

Coverage, historical comparison, and performance panels provide interactive charts with explicit pass/fail/skip deltas, regression flags, and compile/runtime trend views.

## Scope

In scope:

- [x] Add interactive coverage charts for implementation progress and priority breakdown.
- [x] Add historical comparison deltas between selected or adjacent runs.
- [x] Flag regressions when failures increase, passed count drops, or performance worsens beyond a documented threshold.
- [x] Add compile/runtime performance trend visualization.

Out of scope:

- Real-time updates.
- Data generator integration.
- Export file generation.

## Affected paths

Expected:

- `web-ui/src/**`
- `web-ui/public/data/**`
- `web-ui/README.md`
- `docs/`

Do not touch:

- Compiler/runtime Rust files.
- Unrelated fixtures.

## Acceptance criteria

- [x] Coverage tab renders at least one interactive chart backed by coverage data.
- [x] History tab shows pass/fail/skip deltas between runs.
- [x] Regression indicators are visible for worsening test counts or performance metrics.
- [x] Performance trends are displayed as charted compile/runtime series.
- [x] Responsive layout remains usable on desktop and mobile widths.
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

- not affected; issue-267d added `docs/18-web-ui-reporting.md`

Current state:

- not affected

Follow-up issues:

- none

## Notes

Prefer the existing `recharts` dependency before introducing another charting library.

## Progress evidence

2026-04-29:

- Added Recharts-backed coverage mix, suite coverage, and priority charts to the Coverage tab.
- Added historical result and performance trend charts to the History tab.
- Added adjacent-run pass/fail/skip and compile/runtime deltas plus regression flags for increased failures, dropped passes, or performance increases above the documented 20% threshold.
- Added responsive layout bounds for header/actions, summary cards, filter controls, tables, coverage bars, and chart containers.
- Playwright visual review covered desktop and mobile Test Results, Coverage, and History tabs with no document-level horizontal overflow.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `4d0aa35f` issue-267b charts, deltas, regressions, and trend views
- close commit records responsive polish, visual review evidence, and issue lifecycle evidence

Validation result:

```text
command: cd web-ui && npm run build
result: PASS
date: 2026-04-29

command: Playwright desktop screenshots for Test Results, Coverage, History
result: PASS; charts and tables render without incoherent overlap
date: 2026-04-29

command: Playwright mobile screenshots for Test Results, Coverage, History
result: PASS; document scrollWidth matched viewport width
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
- `issues/done/267b-web-ui-interactive-charts-regression-and-performance-trends.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
