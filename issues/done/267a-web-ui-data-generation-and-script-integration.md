---
id: 267a
title: "Implement web UI data generation and script integration"
type: feature
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: [267]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Add the missing data generation path that turns existing test/coverage artifacts into the JSON files consumed by `web-ui/`.

## Problem

The current `web-ui/README.md` references a web UI data generator, metadata output, and test command integration, but the generator and `--web-ui` integration are absent.

Problem: Web UI data is currently static sample data, not generated from repository test artifacts.

## Current failure

Review evidence:

- The expected generator file under `scripts/gen/` is absent.
- `rg -n "web-ui|--web-ui|metadata.json" scripts mise.toml web-ui` finds only web UI README/package references and no script integration.

## Desired final state

`web-ui/public/data/` can be generated from repository test and coverage artifacts through a documented command, and the `--web-ui` command surface produces or refreshes the expected data for local viewing.

## Scope

In scope:

- [x] Add a generator for `test-results.json`, `coverage.json`, `history.json`, and `metadata.json`.
- [x] Wire the generator into the intended `reference-coverage` / `test262` command surface or a documented mise task.
- [x] Keep the generated JSON schema compatible with the existing React data loaders.

Out of scope:

- UI layout redesign.
- Real-time streaming.
- Export formats beyond generated UI input data.

## Affected paths

Expected:

- `scripts/gen/`
- `scripts/run/reference-coverage.py`
- `web-ui/public/data/`
- `web-ui/README.md`
- `docs/`

Do not touch:

- Compiler/runtime Rust files.
- Unrelated fixtures.

## Acceptance criteria

- [x] A command generates all `web-ui/public/data/*.json` files required by the UI.
- [x] Generated test records include suite, case/name, target/status, duration when available, and error/reason when available.
- [x] Generated coverage records are derived from existing coverage artifacts rather than hard-coded sample data.
- [x] `mise run reference-coverage -- --web-ui` or a documented equivalent succeeds.
- [x] Documentation names the command and generated files.
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
mise run reference-coverage -- --web-ui
mise run test262 -- --web-ui
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `web-ui/README.md` web UI data generation and runner integration documentation

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Start from the existing `web-ui/src/hooks/useData.ts` shapes and avoid changing the UI contract unless the generator cannot represent required repository data.

## Progress evidence

2026-04-29:

- Added `mise run web-ui-data`, dispatched through `scripts/manager.py`, to generate `web-ui/public/data/test-results.json`, `coverage.json`, `history.json`, and `metadata.json`.
- Added `scripts/gen/web-ui-data.py`; it reads coverage totals from `artifacts/coverage/results/test262.json`, `artifacts/coverage/results/tsc.json`, and `artifacts/coverage/results/tsgo.json`, and can merge per-case JSONL test records with `--test-jsonl`.
- Verified `mise run web-ui-data` succeeds and refreshes all required web UI data files from existing artifacts.
- Remaining integration: `mise run reference-coverage -- --web-ui` and `mise run test262 -- --web-ui` are not wired in this slice; use the documented equivalent `mise run web-ui-data`.

2026-04-29 command-integration slice:

- Added `--web-ui` support to `mise run reference-coverage -- <suite> ...`; the command writes the selected suite summary under `artifacts/coverage/results/` and refreshes `web-ui/public/data/` while preserving the command's stdout contract.
- Added `--web-ui` support to `mise run test262 -- ...`; the command refreshes `web-ui/public/data/` from the run JSONL stored in the coverage results directory while keeping JSONL stdout unchanged.
- The narrow command validation is run in a throwaway worktree because these commands intentionally refresh tracked coverage/data artifacts for the selected sample.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `acace0a6` (`issue-267a: add web ui data generator`)
- `c855f96` (`merge: issue-267a web ui command progress`)
- close commit records final regenerated data, docs, and issue lifecycle evidence

Validation result:

```text
command: mise run web-ui-data
result: pass; generated 4 files under web-ui/public/data from artifacts/coverage/results/*.json
date: 2026-04-29

command: cd web-ui && npm run build
result: pass after `npm ci` restored lockfile dependencies; Vite production build completed
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated after moving 267a to done
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md is up to date
date: 2026-04-29

command: mise run check issues
result: pass; issue index and issue health checks passed
date: 2026-04-29
```

Remaining risks:

- none
