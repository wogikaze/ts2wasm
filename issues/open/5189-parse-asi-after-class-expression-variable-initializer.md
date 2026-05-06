---
id: 5189
title: "Parse ASI after class expression variable initializer"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`blockScopedVariablesUseBeforeDef.ts` reaches `let y = class { static a = x; }` without a trailing semicolon, then fails at the following `let x;` with `expected Semicolon, got Some(Let)`.

## Problem

The current parser accepts enough of the anonymous class expression to reach the next declaration, but it requires an explicit semicolon after the `let` declaration initializer:

```ts
function foo9() {
    let y = class {
        static a = x;
    }
    let x;
}
```

TypeScript accepts the ASI boundary and reports later TDZ diagnostics. The compiler stops earlier at the next `let`, so the `blockScopedVariablesUseBeforeDef` bucket cannot be triaged for its intended scope-analysis behavior.

Problem: semicolonless variable declarations whose initializer is an anonymous class expression do not accept ASI before the next statement.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Let) at 718..721
```

Compiler evidence:

- Tokens succeed through `let y = class { static a = x; }` and the following `Let`.
- AST/resolved construction fails at the following `let x;`.
- Visible-symbol extraction reaches `foo9` and the `y` binding before the failure.

TypeScript oracle evidence:

```text
TS2448: Block-scoped variable 'x' used before its declaration.
```

The oracle includes a diagnostic at the `x` inside the static class field initializer.

## Desired final state

The parser accepts ASI after a completed variable declaration whose initializer is an anonymous class expression. The representative reference should advance past the current `expected Semicolon, got Some(Let)` parser blocker.

## Scope

In scope:

- [ ] Accept ASI after `let name = class { ... }` before a later-line statement starter.
- [ ] Preserve explicit semicolon behavior for same-line continuation tokens where ASI must not apply.
- [ ] Add focused parser coverage for an anonymous class expression variable initializer followed by `let`.

Out of scope:

- General TDZ/scope-analysis diagnostics after parsing advances.
- Runtime class expression lowering.
- Broader ASI policy for every variable declaration initializer.
- Class static field semantics beyond parsing this initializer.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/ir/src/`
- backend/runtime class lowering

## Acceptance criteria

- [ ] `parse_program("let y = class { static a = x; }\\nlet x;")` succeeds as two statements.
- [ ] Parser tests cover ASI after an anonymous class expression variable initializer.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts` no longer reports `expected Semicolon, got Some(Let)` at `718..721`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedVariablesUseBeforeDef.ts
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

Split from generated bucket `1079` on 2026-05-06. Existing issue `5169` covers ASI after expression statements, not variable declarations with class-expression initializers.

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
