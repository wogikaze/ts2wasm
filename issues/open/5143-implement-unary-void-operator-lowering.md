---
id: 5143
title: "Implement unary void operator lowering"
type: feature
area: ir/lowered
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Implement the narrow unary `void` operator slice after parsing, so expressions such as `() => void instance.once("unlock", () => {})` lower successfully.

## Problem

The frontend already tokenizes and parses `void` as `UnaryOp::Void`. The representative reference case reaches lowering, then fails because `lower_unary_op` currently rejects `UnaryOp::Void`.

Problem: unary `void` currently fails with `UnsupportedSyntax` in lowering even though TypeScript accepts the reference case with no diagnostics.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unary operator Void not yet supported
```

Source context:

```text
// @target: es2015
// @strict: true
// @noEmit: true

type HowlErrorCallback = (soundId: number, error: unknown) => void;
```

Relevant compiler evidence:

```text
tokens: Token::Void is emitted for the `void` keyword
AST: object literal callback contains Unary { op: Void, expr: Call(Member(Ident("instance"), "once"), ...) }
Pipeline: validate_ast -> module_graph -> resolve_names -> resolve_builtins -> build_typed_ir -> lower_program
Failure: lower_program reports `unary operator Void not yet supported`
TypeScript oracle: ok, no diagnostics
```

## Desired final state

The compiler lowers unary `void expr` by evaluating `expr` for side effects and producing `undefined`, matching JavaScript/TypeScript semantics for the supported expression subset.

## Scope

In scope:

- [x] Add a `LoweredUnaryOp::Void` or equivalent lowering path that preserves operand side effects.
- [x] Ensure the expression result is `undefined` regardless of operand value.
- [x] Add a focused fixture for `void sideEffect()` in an arrow/function body.
- [x] Re-run the representative `avoidCycleWithVoidExpressionReturnedFromArrow.ts` triage and confirm it advances past the unary `Void` diagnostic.

Out of scope:

- `void` in unsupported expression forms that fail earlier for independent reasons.
- Broad callback or class-constructor semantics not needed for the focused fixture.
- Other unsupported unary operators such as bigint bitwise-not.

## Affected paths

Expected:

- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/types.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/core-semantics/`

Do not touch:

- frontend lexer/parser unless a focused regression proves the existing `UnaryOp::Void` AST is no longer produced
- unrelated operator implementations

## Acceptance criteria

- [x] A focused Node/iwasm fixture for `void sideEffect()` matches Node output and proves the side effect ran.
- [x] The fixture proves the expression result is `undefined`.
- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts` no longer reports `unary operator Void not yet supported`.
- [x] Existing supported unary operators still pass their focused tests.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli unary_void_operator
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5209-support-class-instance-method-receiver-calls.md`

## Notes

Split from generated bucket `issues/open/1014-implement-avoidCycleWithVoidExpressionReturnedFromArrow.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- close commit: fixtures: close unary void lowering issue

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli unary_void_operator
result: pass (1 passed)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
result: pass for issue 5143; the unary `Void` diagnostic is gone and the next blocker is `issue-211: unknown receiver class for method once`
date: 2026-05-06
```

Remaining risks:

- The representative reference now stops at class instance method receiver classification, tracked by issue 5209.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

