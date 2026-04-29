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

- [ ] Add interactive coverage charts for implementation progress and priority breakdown.
- [ ] Add historical comparison deltas between selected or adjacent runs.
- [ ] Flag regressions when failures increase, passed count drops, or performance worsens beyond a documented threshold.
- [ ] Add compile/runtime performance trend visualization.

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

- [ ] Coverage tab renders at least one interactive chart backed by coverage data.
- [ ] History tab shows pass/fail/skip deltas between runs.
- [ ] Regression indicators are visible for worsening test counts or performance metrics.
- [ ] Performance trends are displayed as charted compile/runtime series.
- [ ] Responsive layout remains usable on desktop and mobile widths.
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

- [ ] updated: `docs/` web UI usage/deployment documentation if controls or data contracts change

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Prefer the existing `recharts` dependency before introducing another charting library.

## Progress evidence

2026-04-29:

- Added Recharts-backed coverage mix, suite coverage, and priority charts to the Coverage tab.
- Added historical result and performance trend charts to the History tab.
- Added adjacent-run pass/fail/skip and compile/runtime deltas plus regression flags for increased failures, dropped passes, or performance increases above the documented 20% threshold.
- Remaining scope before close: final responsive visual review and any docs/current-state synchronization required by broader issue 267.

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
