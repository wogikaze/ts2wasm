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

- [ ] Define a minimal live update transport for local development.
- [ ] Update test summary and visible result rows as events arrive.
- [ ] Display connection/running/error state.
- [ ] Keep static JSON loading as the fallback for built reports.

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

- [ ] UI can consume incremental test-result events while a local run is active.
- [ ] Pass/fail/skip counters update without page reload.
- [ ] Test rows update or append without page reload.
- [ ] Static report mode still works from `web-ui/public/data/*.json`.
- [ ] Documentation explains how to run live mode locally.
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

The transport may be WebSocket, Server-Sent Events, or local polling if the contract is documented and works with the repo's test runner.

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
