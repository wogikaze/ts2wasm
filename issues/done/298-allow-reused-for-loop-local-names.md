---
id: 298
title: "Allow reused for-loop local names in separate loop scopes"
type: bug
area: frontend/ir
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

Allow repeated `for (let i = ...)` loop variables in separate loop scopes
without reporting a duplicate local binding.

This is a work order, not a design document and not a progress log.

## Problem

Problem: after issue 297, `fixtures/atcoder/abc451-d-concat-power2.ts`
advances beyond array map receiver tracking and stops because multiple
independent loops reuse the loop variable name `i`.

## Current failure

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-array-map-parent.wasm --host-deny
```

Current result:

```text
error: [DuplicateLocal] duplicate local binding: `i`
```

Representative source:

```ts
for (let i = 0; i < powersOfTwoStr.length; i++) {
    ...
}

for (let i = 0; 2 ** i <= 1000000000; i++) {
    ...
}

for (let i = 0; i < allGoodIntHasDup.length; i++) {
    ...
}
```

## Desired final state

Separate `for (let i = ...)` loops can reuse the same source name by receiving
distinct lowered locals or block-scoped bindings. Duplicate diagnostics remain
for true same-scope duplicate declarations.

## Scope

In scope:

- [x] Support repeated `for (let i = ...)` names in separate loop statements.
- [x] Preserve local lookup inside each loop body/update/condition.
- [x] Keep true same-scope duplicate local diagnostics intact.
- [x] Verify the ABC451 fixture advances past the duplicate-local blocker and
  record the next blocker.

Out of scope:

- Full JavaScript lexical environment semantics for every block construct.
- Closure capture of per-iteration loop bindings.
- `var` hoisting semantics.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/resolver.rs`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- parser syntax
- problem-specific source rewrite hooks
- broad closure/lexical environment redesign

## Acceptance criteria

- [x] Focused fixture with two separate `for (let i = ...)` loops matches Node
  output under `iwasm`.
- [x] Existing duplicate local tests still reject true duplicate declarations.
- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past
  `DuplicateLocal: duplicate local binding: i`.
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
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-loop-scope.wasm --host-deny
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

Prefer a narrow fix for repeated loop initializer names. Do not implement full
per-iteration closure capture semantics in this issue.

Progress on 2026-04-30:

- Lowering now gives each `ResolvedStmt::For` its own local scope for
  initializer, condition, update, and body lowering, matching the earlier name
  resolver scope and allowing independent `for (let i = ...)` statements to
  receive distinct lowered locals.
- Added `fixtures/core-semantics/reused-for-loop-local.ts` to cover two
  separate `for (let i = ...)` loops whose condition, update, and body all use
  the loop-local binding.
- Added
  `fixtures/core-semantics/duplicate-local-same-scope-unsupported.ts` and a
  CLI diagnostic test proving true same-scope duplicates still report
  `DuplicateLocal`.
- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` advances beyond the
  repeated-loop `DuplicateLocal` blocker. The current next blocker is
  `error: [UnsupportedSyntax] issue-211: unknown receiver class for method
  sort at 1200..1232`.

## Completion evidence

Commits:

- child branch final commit: reused loop-local lowered scope slice.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli reused_for_loop_local_fixture_matches_node_output_under_iwasm same_scope_duplicate_local_still_reports_duplicate_local
result: pass, 2 passed
date: 2026-04-30

command: cargo nextest run -p ts2wasm-ir test_duplicate_local_error
result: pass, 1 passed
date: 2026-04-30

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-loop-scope-child.wasm --host-deny
result: advanced past DuplicateLocal; next blocker is issue-211 unknown receiver class for method `sort` at 1200..1232
date: 2026-04-30

command: cargo fmt --all --check
result: pass
date: 2026-04-30
```

Remaining risks:

- Full per-iteration closure capture semantics remain out of scope.
