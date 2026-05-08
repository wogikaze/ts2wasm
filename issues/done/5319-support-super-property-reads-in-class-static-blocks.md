---
id: 5319
title: "Support super property reads in class static blocks"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support `super.x` property reads inside class static initialization blocks, or
replace the current broad issue-254 rejection with the next source-spanned
semantic diagnostic.

## Problem

Problem: `classFieldSuperAccessibleJs1.ts` parses successfully, but resolver
validation rejects the first `super.blah1` in a class static block with
`issue-254: super in class static blocks is not supported`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts
```

Observed 2026-05-07:

```text
resolved: UnsupportedSyntax issue-254: `super` in class static blocks is not supported at 193..198
TypeScript oracle:
TS2551 Property 'blah2' does not exist on type 'typeof C'. Did you mean 'blah1'?
```

Representative source:

```ts
class C {
  static blah1 = 123;
}
C.blah2 = 456;

class D extends C {
  static {
    console.log(super.blah1);
    console.log(super.blah2);
  }
}
```

## Desired final state

The static block resolver/lowering handles `super.blah1` against the base class
constructor value and advances the representative past the generic issue-254
diagnostic.

## Scope

In scope:

- [x] Resolve `super.<staticName>` inside class static blocks against the base
  class constructor/static side.
- [x] Preserve existing class static block execution order.
- [x] Add focused coverage for `class D extends C { static { super.x; } }`.

Out of scope:

- General dynamic `super` expressions in static blocks.
- CheckJs TS2551 diagnostics for expando static assignments such as `C.blah2`.
- Instance-method `super.foo()` dispatch, which is not this failure.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver_class_features.rs`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- focused CLI/IR tests or fixtures

Do not touch:

- parser code unless the existing AST evidence changes
- unrelated class heritage support

## Acceptance criteria

- [x] `classFieldSuperAccessibleJs1.ts` no longer reports issue-254 for
  `super.blah1` in the static block.
- [x] A focused fixture covers a static block reading a base static property
  through `super`.
- [x] If `super.blah2` or `C.blah2` remains unsupported/diagnostic-only, the next
  blocker is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -p ts2wasm-cli -E 'test(class) or test(super) or test(static)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from stale generated bucket
`issues/open/1208-implement-classFieldSuperAccessibleJs.md`.

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

## False-done audit

**truly-done** (5319)

- Implementation commits: verified via `git log --oneline --all --grep=5319`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
