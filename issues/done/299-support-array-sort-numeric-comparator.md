---
id: 299
title: "Support Array.sort numeric comparator slice"
type: feature
area: runtime/builtins
class: done
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-30
status: done
completed: 2026-04-30
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

- [x] Recognize `.sort((a, b) => a - b)` on known dense array locals.
- [x] Implement a small in-place numeric sort runtime/lowering path.
- [x] Add focused Node/iwasm differential coverage.
- [x] Verify ABC451 advances past the `sort` blocker and record the next
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

- [x] Focused fixture with `values.sort((a, b) => a - b)` matches Node output
  under `iwasm`.
- [x] Unsupported sort forms remain issue-linked diagnostics.
- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past
  `unknown receiver class for method sort`.
- [x] No code path detects the ABC451 source text or substitutes another
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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a simple O(n^2) in-place sort over dense tagged small-int numbers if it
keeps the slice small and testable.

Progress on 2026-04-30:

- Added lowering for known dense-array `.sort((a, b) => a - b)` calls to the
  new `ArraySortNumeric` runtime helper.
- Added a small in-place numeric array sort helper over the current tagged
  small-int representation. Unsupported comparator/default sort forms remain
  `issue-299` diagnostics.
- Added focused Node/iwasm coverage in
  `fixtures/core-semantics/array-sort-numeric-comparator.ts` and unsupported
  diagnostic coverage in
  `fixtures/core-semantics/array-sort-default-unsupported.ts`.
- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` no longer stops at
  `unknown receiver class for method sort`. The next visible blocker is
  `error: [NumberOutOfRange] number literal 1000000000 is out of small-int
  tagged range (-268435456..=268435455)`.

## Completion evidence

Commits:

- `4bdfb86d` issue-299 numeric array sort implementation.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli array_sort_numeric_comparator_fixture_matches_node_output_under_iwasm array_sort_unsupported_forms_report_issue_299
result: pass, 2 passed
date: 2026-04-30

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-sort-child.wasm --host-deny
result: advanced past issue-211 sort blocker; next blocker is NumberOutOfRange for 1000000000
date: 2026-04-30

command: cargo fmt --all --check
result: pass
date: 2026-04-30
```

Remaining risks:

- This is intentionally limited to dense numeric arrays and comparator shape
  `(a, b) => a - b`; broader ECMAScript sort semantics remain out of scope.
