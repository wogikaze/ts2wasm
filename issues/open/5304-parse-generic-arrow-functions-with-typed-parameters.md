---
id: 5304
title: "Parse generic arrow functions with typed parameters"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse TypeScript generic arrow function expressions with typed parameters, such
as `const fn = <T>(value: Box<T>): T => value;`, by erasing the type-only
parameter and return annotations while preserving the arrow expression.

## Problem

The representative reference case starts with `const fn1 = <Params>(...)`.
The parser recognizes the generic arrow shape enough to enter the parameter
list, but it expects an untyped JavaScript parameter list and fails at the colon
after `params`.

Problem: generic arrow functions with typed parameters currently fail with
`UnsupportedSyntax` before AST construction.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected RightParen, got Some(Colon) at 71..72
line 4, column 14
```

Source context:

```ts
const fn1 = <Params>(
    params: Pick<Params, Exclude<keyof Params, never>>,
): Params => params;
```

Compiler evidence:

```text
tokens: ok; Const, Ident("fn1"), Equal, Less, Ident("Params"), Greater, LeftParen, Ident("params"), Colon, ...
ast: fails before AST construction with expected RightParen at the parameter type colon
resolved: fails with the same parser diagnostic
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
binding fn1 type: <Params>(params: Pick<Params, Exclude<keyof Params, never>>) => Params
parameter params type: Pick<Params, Exclude<keyof Params, never>>
AST shape: VariableStatement -> VariableDeclaration fn1 -> ArrowFunction with type parameter Params, typed parameter params, return type Params
```

## Desired final state

The parser accepts generic arrow function expressions with TypeScript parameter
type annotations and return type annotations, erases the type-only syntax, and
continues parsing the arrow body.

## Scope

In scope:

- [x] Parse `<T>(value: Type) => expr` as a generic arrow function expression.
- [x] Erase TypeScript parameter type annotations inside the generic arrow parameter list.
- [x] Erase optional typed parameters such as `y?: K` in the generic arrow
  parameter list.
- [x] Erase the return type annotation between `)` and `=>`.
- [x] Add focused parser coverage for `const fn = <T>(value: Box<T>): T => value;`.
- [x] Re-run the representative triage and record any next blocker separately.

Out of scope:

- Conditional type evaluation for `Exclude`, `Extract`, or `TemplatedConditional`.
- Angle-bracket type assertion statements, tracked by `issues/open/5154-parse-angle-bracket-type-assertion-statements.md`.
- JSX parsing.
- Generic arrow function body lowering beyond the existing arrow-function support.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- runtime/backend lowering
- TypeScript checker behavior
- coverage dashboard generated data

## Acceptance criteria

- [x] A focused parser test accepts `const fn = <T>(value: Box<T>): T => value;`.
- [x] `contextualSignatureInstantiation1.ts` no longer reports
  `expected RightParen, got Some(Colon)` at `var e = <K>(x: string, y?: K) =>`.
- [x] `conditionalTypesSimplifyWhenTrivial.ts` no longer reports `expected RightParen, got Some(Colon)` at the `params:` annotation.
- [x] `contextuallyTypedByDiscriminableUnion2.ts` no longer reports
  `expected RightParen, got Some(Colon)` at
  `<I extends Identifiable>(props: MyComponentProps<I>) =>`.
- [x] Existing arrow-function and angle-bracket assertion parser tests continue to pass.
- [x] If parsing advances to a new blocker, that next blocker is recorded separately.
- [x] Issue state stays synchronized with `issues/index.md`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend generic_arrow_typed_parameter
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypesSimplifyWhenTrivial.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from generated bucket `issues/open/1429-implement-conditionalTypesSimplifyWhenTrivial.md`.
Related but non-identical slice `issues/open/5154-parse-angle-bracket-type-assertion-statements.md`
explicitly excludes ambiguous generic arrow parsing.

2026-05-07 fold-in:

- `issues/open/1502-implement-contextualSignatureInstantiation-unknown-unsupported.md`
  is the same parser boundary in `contextualSignatureInstantiation1.ts`.
- Current diagnostic: `UnsupportedSyntax: expected RightParen, got Some(Colon)`
  at `var e = <K>(x: string, y?: K) => x.length;`.
- TypeScript oracle accepts the generic arrow and infers
  `<K>(x: string, y?: K | undefined) => number`.

2026-05-07 fold-in:

- `issues/open/1540-implement-contextuallyTypedByDiscriminableUnion-unknown-unsupported.md`
  is the same parser boundary in `contextuallyTypedByDiscriminableUnion2.ts`.
- Current diagnostic: `UnsupportedSyntax: expected RightParen, got Some(Colon)`
  at `const MyComponent = <I extends Identifiable>(props: MyComponentProps<I>) => {};`.
- TypeScript oracle accepts the generic arrow and infers
  `<I extends Identifiable>(props: MyComponentProps<I>) => void`.

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

**truly-done** (5304)

- Implementation commits: verified via `git log --oneline --all --grep=5304`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
