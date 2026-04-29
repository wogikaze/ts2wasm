---
id: 299
title: "Support Array.sort numeric comparator slice"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement a narrow `Array.prototype.sort((a, b) => a - b)` slice for dense
numeric arrays, needed by the ABC451 D fixture.

This is a work order, not a design document and not a progress log.

## Problem

Problem: after issue 298, `fixtures/atcoder/abc451-d-concat-power2.ts`
advances past repeated loop locals and stops at `allGoodInt.sort((a, b) => a -
b)` because dense array locals do not have a supported `sort` method.

## Current failure

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-loop-scope-parent.wasm --host-deny
```

Current result:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `sort` at 1200..1232
```

Representative source:

```ts
allGoodInt.sort((a, b) => a - b);
```

## Desired final state

Dense numeric arrays support in-place ascending sort for the comparator
shape `(a, b) => a - b`, and the call returns the sorted array value in the
same observable subset Node uses for numeric ascending sort.

## Scope

In scope:

- [ ] Recognize `.sort((a, b) => a - b)` on known dense array locals.
- [ ] Implement a small in-place numeric sort runtime/lowering path.
- [ ] Add focused Node/iwasm differential coverage.
- [ ] Verify ABC451 advances past the `sort` blocker and record the next
  blocker.

Out of scope:

- Default lexicographic sort.
- Arbitrary comparator callbacks.
- Sparse arrays, holes, `undefined`, `NaN`, and full ECMAScript sort stability.
- Source-specific ABC451 rewrites.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- parser syntax
- problem-specific source rewrite hooks
- broad callback allocation semantics

## Acceptance criteria

- [ ] Focused fixture with `values.sort((a, b) => a - b)` matches Node output
  under `iwasm`.
- [ ] Unsupported sort forms remain issue-linked diagnostics.
- [ ] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past
  `unknown receiver class for method sort`.
- [ ] No code path detects the ABC451 source text or substitutes another
  program.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli <new focused test name>
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-sort.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a simple O(n^2) in-place sort over dense tagged small-int numbers if it
keeps the slice small and testable.

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
