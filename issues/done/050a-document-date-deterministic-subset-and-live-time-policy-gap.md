---
id: 050a
title: "Document Date deterministic subset and live-time policy gap"
type: docs
area: runtime/builtins
class: docs-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Date has validated deterministic slices, but the open epic still presents live time, timezone formatting, frontend recognition, and Annex B behavior as one implementation target.

## Summary

Record the current supported Date subset and split the remaining Date work into policy-backed child issues so implementers can select one behavior at a time.

## Scope

In scope:

- [x] Document deterministic `new Date(<epoch-ms integer>)`, `getTime()`, and `valueOf()` support.
- [x] Document that `new Date()` and `Date.now()` require an auditable live-time capability policy.
- [x] Create follow-up issues for live time, timezone-aware `toString`, and Annex B Date legacy methods if they do not already exist.

Out of scope:

- Implementing live host time.
- Implementing timezone formatting.
- Implementing new Date runtime behavior.

## Affected paths

Expected:

- `current-state.md`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/ir/src/`

## Acceptance criteria

- [x] Current Date support is described without implying full Date support.
- [x] Live time remains blocked on explicit capability policy.
- [x] Issue 050 remains blocked as an epic with closeable child issues.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(date)'
```

Not run:

- none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit on branch `agent/050a-date-contract-20260428T235026Z` (hash recorded in cycle report)

Validation result:

```text
command: cargo nextest run -E 'test(date)'
result: passed; 16 tests passed, 402 skipped
date: 2026-04-29

command: mise run update-issue-index
result: passed; issues/index.md updated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: failed on unrelated pre-existing missing report paths in issues/open/052-implement-json.md and issues/done/228-implement-logical-assignment-operators.md; no Date issue errors were reported
date: 2026-04-29

command: mise run check-agent-state
result: passed
date: 2026-04-29
```

Remaining risks:

- Discord delivery may be deferred if the webhook command is unavailable or fails.
