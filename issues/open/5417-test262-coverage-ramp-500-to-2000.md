---
id: 5417
title: "W6: test262 coverage ramp from 500 to 2000"
type: feature
area: coverage
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Run test262 coverage ramp from current limit=500 to limit=2000, collect triage data, auto-generate issues for new unsupported features, and update the coverage matrix and dashboard.

## Problem

Current test262 executed=500 out of 53,445 total (0.9%). To reach Gate D (executed >= 2000), we need to ramp to limit=2000. This will reveal new unsupported feature categories beyond the current known blockers.

Problem: test262 executed=500/53445. Gate D requires >= 2000.

## Current failure

```sh
mise run reference-coverage -- test262 --limit 500
# test262 | 53449 | 500 | 0.11% | 0.06% | 61 | 31 | 0 | 439 | 1
# Need: | 53449 | 2000 | ... | ... | ...
```

## Desired final state

- `mise run reference-coverage -- test262 --limit 2000` passes
- Coverage matrix updated: `artifacts/coverage/reference-coverage-matrix.md`
- New unsupported features from the expanded window are triaged
- Auto-generated issues for any new unsupported diagnostic codes or feature labels
- Dashboard data regenerated

## Scope

In scope:

- [ ] Run `mise run reference-coverage -- test262 --limit 2000`
- [ ] If 2000 fails (too many errors), find the actual achievable limit > 500
- [ ] Run with `--detail` to capture feature breakdown at limit=2000
- [ ] Run `mise run gen-issues-from-coverage -- --suite test262` on the new results
- [ ] Run `mise run update-coverage-matrix` to regenerate the matrix
- [ ] Run `mise run coverage-dashboard-data` to regenerate dashboard JSON
- [ ] Document new unsupported categories discovered at the higher limit
- [ ] Update `docs/15-coverage-matrix.md` if ramp policy needs adjustment

Out of scope:

- Semantic fix for any discovered unsupported features (separate implementation issues)
- Performance optimization of the ramp pipeline
- Parallel test execution improvement
- Automatic CI ramp (manual run for this issue)

## Affected paths

Expected:

- `artifacts/coverage/reference-coverage-matrix.md` — updated
- `issues/open/` — new auto-generated triage issues
- `docs/15-coverage-matrix.md` — may need ramp policy update
- `crates/cli/tests/differential_jsonl.rs` — may need minor fixes for larger batch
- `scripts/` — may need script adjustments for 2000-limit runs

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
- `crates/backend-wasm/` — runtime out of scope
- `crates/cli/tests/m*_*.rs` — no fixture test changes

## Acceptance criteria

- [ ] `mise run reference-coverage -- test262 --limit 2000` completes without hard failure
- [ ] Coverage matrix shows executed >= 2000, with build_pass and semantic_pass recorded
- [ ] New unsupported features (if any) have auto-generated tracking issues
- [ ] `mise run update-coverage-matrix -- --check` passes
- [ ] Dashboard JSON data regenerated without errors

## Validation

Required commands:

```sh
mise run reference-coverage -- test262 --limit 2000
mise run update-coverage-matrix
mise run update-coverage-matrix -- --check
mise run coverage-dashboard-data
```

Impacted commands:

```sh
# Verify detail output
mise run reference-coverage -- test262 --limit 2000 --detail | grep "feature-table"
# Verify issue generation
mise run gen-issues-from-coverage -- --suite test262 --limit 2000
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/15-coverage-matrix.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created/updated: auto-generated triage issues from new unsupported features
- [ ] created/updated: `issues/open/` → triage issues per new feature label

## Notes

- The ramp runs all test262 files up to the limit in index order
- Expect increased runtime for 2000 files vs 500 files — may need 2-5x longer
- Use `time mise run reference-coverage -- test262 --limit 2000` to record duration
- If 2000 is too aggressive, find the max achievable limit > 500 and use that
- The `--detail` flag produces feature-level breakdown needed for issue generation
