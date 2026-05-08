---
id: 5430
title: "W6: Full test262 corpus re-run and coverage update"
type: feature
area: coverage
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Run full test262 corpus now that batches 1-3 are merged, update coverage matrix, generate triage issues for new unsupported categories. Measure the cumulative impact of 5411-5426.

## Problem

Coverage matrix data is from the pre-batch-3 run. Need to re-run to measure the real impact of parser syntax fixes (5423), name resolution round 2 (5424), builtin API routing (5425), and async/await (5426).

## Scope

In scope:

- [ ] Run `mise run reference-coverage -- test262`
- [ ] Run `mise run update-coverage-matrix`
- [ ] Compare results with previous run
- [ ] If new unsupported categories appear, generate triage issues
- [ ] Update current-state.md with new numbers
- [ ] Commit and push

Out of scope:

- Fixing any newly discovered issues (separate work items)
- Changes to the coverage runner scripts

## Affected paths

Expected:

- `artifacts/coverage/reference-coverage-matrix.md` — updated
- `issues/open/` — new triage issues if needed
- `current-state.md` — updated numbers

Do not touch:

- `crates/` — Rust code out of scope
- `scripts/` — scripts out of scope (unless minor fix needed)
- `docs/` — docs out of scope

## Validation

```sh
mise run reference-coverage -- test262
mise run update-coverage-matrix -- --check
```
