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
---

Problem: Date has validated deterministic slices, but the open epic still presents live time, timezone formatting, frontend recognition, and Annex B behavior as one implementation target.

## Summary

Record the current supported Date subset and split the remaining Date work into policy-backed child issues so implementers can select one behavior at a time.

## Scope

In scope:

- [ ] Document deterministic `new Date(<epoch-ms integer>)`, `getTime()`, and `valueOf()` support.
- [ ] Document that `new Date()` and `Date.now()` require an auditable live-time capability policy.
- [ ] Create follow-up issues for live time, timezone-aware `toString`, and Annex B Date legacy methods if they do not already exist.

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

- [ ] Current Date support is described without implying full Date support.
- [ ] Live time remains blocked on explicit capability policy.
- [ ] Issue 050 remains blocked as an epic with closeable child issues.

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
