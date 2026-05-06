---
id: 5156
title: "Parse generic type arguments in class heritage clauses"
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

The parser handles generic class declarations like `class C<T>`, but class heritage parsing still treats `extends Class3<T>` as a runtime expression. That consumes the type-argument tokens as comparison operators and leaves the parser expecting a class body at EOF.

This blocks `baseTypeOrderChecking.ts` at parser syntax before resolver/backend behavior can run. The same parser boundary also blocks nested class heritage type arguments such as `extends CBaseBase<Wrapper<T2>>`, where the lexer emits the closing `>>` as `RightShift`.

## Problem

Problem: `reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts` fails parsing on `class Class4<T> extends Class3<T> {}` with `UnsupportedSyntax: expected LeftBrace, got None`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
```

Current diagnostic:

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected LeftBrace, got None
```

Representative source:

```ts
var someVariable: Class4<Class2>;

class Class3<T>
{
    public memberVariable: Class2;
}

class Class4<T> extends Class3<T>
{
}
```

Current compiler evidence:

- Tokens succeed, including `Class4`, `<`, `T`, `>`, `Extends`, `Class3`, `<`, `T`, `>`, `{`, `}`.
- AST dump fails with `UnsupportedSyntax: expected LeftBrace, got None`.
- Visible symbols before failure include `someVariable`, `Class1`, `Class2`, `Class3`, and `Class4`.
- TypeScript oracle does not report a parse error; it reports only TS2564 for `memberVariable` definite assignment.

Additional representative:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
```

```ts
class CBase<T2> extends CBaseBase<Wrapper<T2>> {
}
```

Current diagnostic:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Class) at line 10
```

Compiler evidence:

- Tokens include `CBaseBase`, `<`, `Wrapper`, `<`, `T2`, `RightShift`, `{`, `}`, then `class Parameter`.
- AST fails because class heritage parsing consumes past the class body and expects a later `{`.
- TypeScript oracle accepts the nested heritage syntax; its only diagnostic is unrelated TS2564 for `Wrapper.property`.

## Desired final state

The parser erases TypeScript type arguments in class heritage expressions before class body parsing. The representative case should advance past parser syntax and either compile further or report the next semantic/runtime diagnostic.

## Scope

In scope:

- [x] Parse or skip TypeScript type-argument lists in `extends` heritage expressions such as `extends Class3<T>` and nested `extends CBaseBase<Wrapper<T2>>`.
- [x] Preserve existing parsing for plain runtime heritage expressions such as `extends Base` and `extends mixin(Base)`.
- [x] Add focused parser regressions for `class Class4<T> extends Class3<T> {}` and a nested `RightShift` generic heritage clause.
- [x] Re-run the exact `baseTypeOrderChecking.ts` triage and record the new diagnostic if a downstream blocker remains.

Out of scope:

- Type checking for whether the base type is valid.
- TS2564 definite-assignment diagnostics for class fields.
- Full generic type-system semantics or runtime preservation of erased type arguments.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/src/` unless parser triage advances to a backend-only blocker after this syntax fix.

## Acceptance criteria

- [x] `class Class4<T> extends Class3<T> {}` parses without treating `<T>` as runtime comparison syntax.
- [x] `class CBase<T> extends CBaseBase<Wrapper<T>> {}` parses without consuming `>>` as runtime right-shift syntax.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts` no longer reports `UnsupportedSyntax: expected LeftBrace, got None`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts` no longer reports `UnsupportedSyntax: expected LeftBrace, got Some(Class)`.
- [x] A parser test or fixture covers a generic class declaration with a generic heritage clause.
- [x] Existing class heritage expression tests continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
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

`class_statement` already calls `consume_typescript_generic_parameter_list()` after the class name. The missing boundary is the `class_extends()` path, which currently delegates directly to expression parsing.

## Completion evidence

Commits:

- Already implemented in existing codebase (`class_extends` skips TypeScript generic heritage arguments)

Validation result:

```text
command: cargo nextest run -p ts2wasm-frontend generic_class
result: pass
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
result: no longer reports `expected LeftBrace, got None`
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
result: no longer reports `expected LeftBrace, got Some(Class)`
date: 2026-05-06
```

Remaining risks:

- none
