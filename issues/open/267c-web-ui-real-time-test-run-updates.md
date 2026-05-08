---
id: 267c
title: "Implement web UI real-time test run updates"
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

Implement the live update path for test runs in the web UI.

## Problem

Issue 267 requires real-time updates during test runs, but the current UI loads static JSON once during mount. `web-ui/README.md` marks real-time WebSocket updates as planned.

Problem: Web UI does not update while tests are running.

## Current failure

Evidence from `web-ui/src/hooks/useData.ts`:

- `useTestData`, `useCoverageData`, and `useHistoricalData` fetch static JSON once in `useEffect`.
- No WebSocket, EventSource, polling, or run-status subscription exists.

## Desired final state

During a test run, the UI can receive incremental result updates and refresh the visible summary/table without a manual page reload.

## Scope

In scope:

- [x] Define a minimal live update transport for local development.
- [x] Update test summary and visible result rows as events arrive.
- [x] Display connection/running/error state.
- [x] Keep static JSON loading as the fallback for built reports.

Out of scope:

- CI/CD platform integration.
- Authentication/authorization.
- Native desktop or mobile clients.

## Affected paths

Expected:

- `web-ui/src/**`
- `web-ui/README.md`
- `docs/`
- `scripts/**` only if needed for the local event source

Do not touch:

- Compiler/runtime Rust files.
- Unrelated fixtures.

## Acceptance criteria

- [x] UI can consume incremental test-result updates while a local run is active.
- [x] Pass/fail/skip counters update without page reload.
- [x] Test rows update or append without page reload.
- [x] Static report mode still works from `web-ui/public/data/*.json`.
- [x] Documentation explains how to run live mode locally.
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

The transport may be WebSocket, Server-Sent Events, or local polling if the contract is documented and works with the repo's test runner.

## Progress evidence

2026-04-29:

- Added `?live=1` local live mode for Test Results.
- Live mode polls `web-ui/public/data/test-results.json`, refreshes summary counters and visible rows without a page reload, and shows connection/error state plus last refresh time.
- Static report mode remains the default when `live=1` is absent.
- Documented local live mode in `web-ui/README.md` and `docs/18-web-ui-reporting.md`.
- Added suite filtering in the Test Results tab to satisfy the remaining issue-267 parent filtering criterion.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- close commit records live mode implementation, docs, validation, and issue lifecycle evidence

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cd web-ui && npm run build
result: PASS
date: 2026-04-29

command: Playwright `http://127.0.0.1:5174/?live=1&liveIntervalMs=500`
result: PASS; Test Results displayed `Live mode: connected` and last refresh time
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

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/267c-web-ui-real-time-test-run-updates.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
