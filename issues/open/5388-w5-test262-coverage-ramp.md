---
id: 5388
title: "W5: test262 coverage ramp from 100 to 500 executed"
type: feature
area: coverage
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Ramp test262 executed count from the current 100 to 500, increasing `build_pass`
and `semantic_pass` across all test262 shards. This is the primary W5 activity
to progress toward Gate D (100+ executed) and Gate E (50+ build_pass, 20+ semantic_pass).

## Problem

W5 (Differential testing and coverage) requires continuous ramp of test262
execution. Current ramp is at 100 executed, mostly unsupported/blocked.
To progress toward Gates D and E, the executed count must increase to 500
with measurable build_pass and semantic_pass gains.

## Scope

In scope:

- [ ] Run test262 with --limit 500, collect results
- [ ] Categorize new failures and update known-unsupported lists
- [ ] Fix the highest-impact unsupported cases (most common diagnostics)
- [ ] Increase build_pass by at least 20 cases
- [ ] Update reference coverage matrix
- [ ] Commit updated artifacts

Out of scope:

- test262 semantic_pass improvements (separate issue)
- TypeScript/tsc coverage ramp
- Performance benchmarking

## Affected paths

Expected:

- `artifacts/coverage/reference-coverage-matrix.md`
- `scripts/data/` (known-features/unsupported lists)

## Acceptance criteria

- [ ] test262 executed count >= 500
- [ ] build_pass increased over current baseline
- [ ] Reference coverage matrix updated and passes --check
- [ ] All new failures categorized with reason/tracking

## Validation

```sh
mise run reference-coverage -- test262 --limit 500 --detail
mise run update-coverage-matrix -- --check
```
