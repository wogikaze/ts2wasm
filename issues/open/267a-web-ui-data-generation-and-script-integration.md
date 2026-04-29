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

- [ ] Add a generator for `test-results.json`, `coverage.json`, `history.json`, and `metadata.json`.
- [ ] Wire the generator into the intended `reference-coverage` / `test262` command surface or a documented mise task.
- [ ] Keep the generated JSON schema compatible with the existing React data loaders.

Out of scope:

- UI layout redesign.
- Real-time streaming.
- Export formats beyond generated UI input data.

## Affected paths

Expected:

- `scripts/gen/`
- `scripts/run/test262.py`
- `web-ui/public/data/`
- `web-ui/README.md`
- `docs/`

Do not touch:

- Compiler/runtime Rust files.
- Unrelated fixtures.

## Acceptance criteria

- [ ] A command generates all `web-ui/public/data/*.json` files required by the UI.
- [ ] Generated test records include suite, case/name, target/status, duration when available, and error/reason when available.
- [ ] Generated coverage records are derived from existing coverage artifacts rather than hard-coded sample data.
- [ ] `mise run reference-coverage -- --web-ui` or a documented equivalent succeeds.
- [ ] Documentation names the command and generated files.
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
mise run reference-coverage -- --web-ui
mise run test262 -- --web-ui
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

Start from the existing `web-ui/src/hooks/useData.ts` shapes and avoid changing the UI contract unless the generator cannot represent required repository data.

## Progress evidence

2026-04-29:

- Added `mise run web-ui-data`, dispatched through `scripts/manager.py`, to generate `web-ui/public/data/test-results.json`, `coverage.json`, `history.json`, and `metadata.json`.
- Added `scripts/gen/web-ui-data.py`; it reads coverage totals from `artifacts/coverage/results/test262.json`, `artifacts/coverage/results/tsc.json`, and `artifacts/coverage/results/tsgo.json`, and can merge per-case JSONL test records with `--test-jsonl`.
- Verified `mise run web-ui-data` succeeds and refreshes all required web UI data files from existing artifacts.
- Remaining integration: `mise run reference-coverage -- --web-ui` and `mise run test262 -- --web-ui` are not wired in this slice; use the documented equivalent `mise run web-ui-data`.

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
