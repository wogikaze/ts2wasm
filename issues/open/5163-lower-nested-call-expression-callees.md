---
id: 5163
title: "Lower nested call expression callees"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The parser already builds AST for nested call expressions such as `foo()(1).toString()`, but lowering rejects call expressions whose callee is not a simple identifier. This blocks `betterErrorForAccidentalCall.ts` before the compiler can report a TypeScript-compatible accidental call diagnostic.

## Problem

Problem: `reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts` currently reports `UnsupportedSyntax: only identifier calls are supported in expression context` for `foo()(1 as number)`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: only identifier calls are supported in expression context at 52..70
```

Representative source:

```ts
declare function foo(): string;

foo()(1 as number).toString();
foo()   (1 as number).toString();
foo()
(1 as number).toString();
```

Current compiler evidence:

- Tokens and AST succeed.
- The AST records nested `Call` expressions where the outer call callee is another `Call`.
- The pipeline reaches `lower_program` and fails in `crates/ir/src/lowered/resolver_expr.rs`.

TypeScript oracle evidence:

```text
TS2349: This expression is not callable.
  Type 'String' has no call signatures.
```

TypeScript reports TS2349 for all five accidental-call variants in the reference file.

## Desired final state

Lowering supports, or explicitly diagnoses, call expressions whose callee is another expression rather than an identifier. The representative reference case should no longer hit the generic "only identifier calls" unsupported diagnostic.

## Scope

In scope:

- [ ] Handle `Expr::Call { callee: Expr::Call { ... } }` in expression lowering with a source-spanned diagnostic or runtime-supported callable path.
- [ ] Preserve existing identifier-call behavior.
- [ ] Add focused coverage for `foo()(1).toString()` and the whitespace/newline accidental-call variants.
- [ ] Re-run the representative triage and confirm the current generic unsupported diagnostic is gone.

Out of scope:

- Full TypeScript call-signature checking.
- General callable object semantics beyond this nested call-expression boundary.
- `super[...]()` call semantics, which remain under the broader issue 420 parent unless current triage proves the same child fixes them.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/lowered/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

Do not touch:

- backend/runtime code unless the lowered representation requires it after focused implementation.

## Acceptance criteria

- [ ] `foo()(1).toString()` no longer reports `only identifier calls are supported in expression context`.
- [ ] Whitespace and newline accidental-call variants from `betterErrorForAccidentalCall.ts` reach the same new diagnostic or lowered path.
- [ ] Existing simple identifier calls continue to pass.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts` no longer reports the current generic unsupported diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli call
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `1045` on 2026-05-06. The broad call-expression parent `420` remains blocked for unrelated `super[...]()` and other call-expression feature families.

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
