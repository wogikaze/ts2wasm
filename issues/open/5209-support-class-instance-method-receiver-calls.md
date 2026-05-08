---
id: 5209
title: "Support class instance method receiver calls"
type: feature
area: ir/lowered
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support the narrow class instance method receiver call shape exposed by `avoidCycleWithVoidExpressionReturnedFromArrow.ts` after unary `void` lowering was fixed.

## Problem

The frontend parses the representative `class Howl` and the `instance.once("unlock", () => {})` call, but lowering rejects the member call because class instance receiver classification does not yet cover this shape.

Problem: class instance method calls such as `instance.once(...)` currently fail with `issue-211: unknown receiver class for method`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
```

Current diagnostic after issue 5143:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `once` at 394..427
```

Source context:

```text
const instance = new Howl({
  onplayerror: () => void instance.once("unlock", () => {}),
});
```

## Desired final state

The compiler classifies supported `new Class(...)` locals as class instance receivers so direct method calls such as `instance.once(...)` lower and execute with the correct receiver.

## Scope

In scope:

- [x] Track enough class constructor result information for direct local instance receiver classification.
- [x] Lower direct class instance method calls without falling into the generic unknown receiver diagnostic.
- [x] Add or update a focused fixture that proves `new Class(...).method(...)` or local instance method calls match Node output.
- [x] Re-run the representative `avoidCycleWithVoidExpressionReturnedFromArrow.ts` triage and confirm it advances past `unknown receiver class for method`.

Out of scope:

- Full class inheritance, private fields, static `this`, or overload/type-check semantics.
- Extracted method calls and dynamic method names.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver_class_features.rs`
- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/core-semantics/`

Do not touch:

- unrelated class syntax parsing unless a focused regression proves the AST is wrong.

## Acceptance criteria

- [x] A focused class instance method receiver fixture matches Node output under iwasm.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts` no longer reports `issue-211: unknown receiver class for method`.
- [x] Existing object method receiver fixtures still pass.
- [x] Issue index and readiness checks pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli class_new_expression_method_call
cargo nextest run -p ts2wasm-cli unary_void_operator
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
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

Split while closing issue 5143. Issue 211 completed object receiver-bound `this` semantics and explicitly left constructor/class `this` semantics out of scope.

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

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
