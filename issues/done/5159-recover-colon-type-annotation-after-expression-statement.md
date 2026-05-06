---
id: 5159
title: "Recover colon type annotations after expression statements"
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

`bases.ts` and `autoLift2.ts` both contain constructor-body statements like `this.y: any;`. TypeScript parses the property access as an expression statement and reports source diagnostics for the unexpected colon/type tokens, but the compiler currently stops in the parser with `UnsupportedSyntax: expected Semicolon, got Some(Colon)`.

## Problem

Problem: constructor-body statements of the form `this.<name>: any;` currently fail as parser-unsupported before the reference runner can observe TypeScript-compatible diagnostics or continue to later semantic checks.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bases.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoLift2.ts
```

Current compiler diagnostic for `bases.ts`:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 113..114
```

Representative source:

```ts
class B {
    constructor() {
        this.y: any;
    }
}
```

Current parser evidence:

- Tokens succeed and include `This`, `Dot`, identifier `y`, `Colon`, identifier `any`, and `Semicolon`.
- AST dump fails at the colon before producing the `ClassDeclaration` for `B`.
- Visible symbols before failure contain only `class B`.

TypeScript oracle evidence:

```text
TS2339: Property 'y' does not exist on type 'B'.
TS1005: ';' expected.
TS2693: 'any' only refers to a type, but is being used as a value here.
```

The TypeScript AST path at the failing span is:

```text
ClassDeclaration -> Constructor -> Block -> ExpressionStatement -> PropertyAccessExpression -> Identifier(y)
```

`autoLift2.ts` fails on the same parser boundary at `this.foo: any;` and `this.bar: any;`.

## Desired final state

The parser reports or recovers from invalid colon type annotations after expression statements in constructor/function bodies without classifying the construct as unsupported syntax. The representative reference cases should advance past the current `expected Semicolon, got Some(Colon)` blocker.

## Scope

In scope:

- [x] Add parser recovery or a source-spanned frontend diagnostic for `expr: type;` after an expression statement, with focused coverage for `this.foo: any;`.
- [x] Preserve normal expression statement parsing and ASI behavior for valid statements.
- [x] Re-run triage for both `bases.ts` and `autoLift2.ts` and confirm the current `UnsupportedSyntax` colon blocker is gone.

Out of scope:

- Full TypeScript type checking for missing properties.
- Derived-class `super()` ordering diagnostics.
- General class/interface heritage semantics.
- Broader auto-lift behavior beyond the shared constructor-body colon statement boundary.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/src/`
- unrelated runtime or ABI code

## Acceptance criteria

- [x] A focused parser/frontend test covers `class B { constructor() { this.y: any; } }`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bases.ts` no longer reports `UnsupportedSyntax: expected Semicolon, got Some(Colon)`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoLift2.ts` no longer reports `UnsupportedSyntax: expected Semicolon, got Some(Colon)`.
- [x] Valid expression statements and ASI parser tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bases.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoLift2.ts
```

Impacted commands:

```sh
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

Split from generated buckets `1042` and `774` on 2026-05-06. This child intentionally targets the shared parser boundary so the remaining TypeScript oracle diagnostics can be triaged separately if they remain after parser recovery.

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
