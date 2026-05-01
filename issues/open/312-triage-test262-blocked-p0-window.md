---
id: 312
title: "Triage test262 blocked P0 window"
type: spike
area: reference
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

The previous test262 aggregate run reports 44 blocked cases in the 18,000-case
window. The web UI priority model counts `fail + blocked` as P0, so these
blocked cases need explicit triage instead of remaining only as an aggregate
number.

This issue is for recovering exact blocked-case evidence and splitting
implementation-ready children. Do not implement directly from this bucket.

## Problem

The tracked aggregate artifact records the blocked count but not the exact
blocked case list. Without per-case blocked evidence, the P0 queue cannot tell
whether the blockers are timeouts, runtime traps, harness gaps, OOM/memory
policy, or unsupported reference-runner behavior.

Problem: `artifacts/coverage/results/test262.json` reports `blocked=44` for the
previous `test262 --limit 18000` window, but those P0 blockers are not yet
represented as actionable issue slices.

## Current failure

Previous aggregate evidence:

```text
source: artifacts/coverage/results/test262.json
evidence: mise run reference-coverage -- test262 --limit 18000
denominator=53445
executed=18000
build_pass=209
semantic_pass=150
fail=1
unsupported=17746
blocked=44
```

Web UI priority evidence:

```text
source: web-ui/public/data/coverage.json
byPriority.p0=45
```

Only the semantic failure is available as a per-case JSONL row in the current
workspace. The blocked 44 cases must be regenerated with detail output or a
persisted per-case result before implementation children can be safely scoped.

## Desired final state

Each repeated blocked family from the 18,000-case test262 window is either:

- split into an implementation-ready child with exact case paths, command,
  stderr/stdout/trap/timeout evidence, and expected ownership; or
- marked as duplicate/superseded by an existing open issue with matching
  evidence.

The aggregate `blocked=44` count is no longer the only record of those P0
blockers.

## Scope

In scope:

- [ ] Regenerate blocked-case detail for the previous 18,000-case test262
      window.
- [ ] Group blocked rows by observable failure mode: timeout, iwasm trap,
      runtime OOM, harness/setup, or compiler invariant.
- [ ] Check each group against existing open/done issues before creating
      children.
- [ ] Create the smallest implementation-ready child issues for the top
      repeated blocked families.
- [ ] Preserve the exact command and representative paths in every child issue.

Out of scope:

- Direct implementation of blocked case fixes from this broad triage issue.
- Changing P0 priority accounting.
- Relaxing reference-coverage, web-ui, or gate rules.

## Affected paths

Expected:

- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/results/test262-results.jsonl` when intentionally
  refreshed as a generated artifact
- `web-ui/public/data/coverage.json`
- `web-ui/public/data/test-results.json`
- `issues/open/`

Do not touch:

- `scripts/check/architecture-rules.py`
- `.githooks/pre-push`
- unrelated compiler/runtime files unless a child issue is being implemented

## Acceptance criteria

- [ ] A detail command records the 44 blocked cases or proves that the blocked
      count changed in the current tree.
- [ ] At least one child issue is created for a repeated blocked family, or this
      issue records why all 44 cases are duplicates of existing open issues.
- [ ] Each child issue includes representative reference paths and exact
      stderr/stdout/trap/timeout evidence.
- [ ] `issues/index.md` is regenerated and the new child issues appear in the
      correct Ready/Blocked queue.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 18000 --detail
python3 scripts/gen/web-ui-data.py
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] created/updated: `issues/open/...`

## Notes

The P0 count is derived by `scripts/gen/web-ui-data.py` as `failed + blocked`.
This issue deliberately preserves that rule and turns the aggregate P0 blocked
debt into auditable work orders.

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
