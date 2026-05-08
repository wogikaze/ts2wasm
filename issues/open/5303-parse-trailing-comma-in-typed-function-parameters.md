---
id: 5303
title: "Parse trailing comma in typed function parameters"
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

Accept a trailing comma before `)` in exported function declaration parameter
lists after TypeScript type annotations, generic type parameters, and default
initializers.

## Problem

The representative TypeScript reference case reaches an exported generic
function declaration with typed parameters and a default `[]` initializer. The
parser consumes the comma after the final parameter and then requires another
binding identifier or pattern, failing on the closing `)`.

Problem: typed function declaration parameter lists with a trailing comma
currently fail with `UnsupportedSyntax` before AST construction.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(RightParen) at 139298..139299
line 8013, column 6
```

Source context:

```ts
export function makeThing<T extends BigUnion['name']>(
    name: T,
    children: ChildrenOf<WithName<T>>[] = [],
) { }
```

Compiler evidence:

```text
tokens: ok
ast: fails before AST construction with issue-247 at the closing RightParen
resolved: fails with the same parser diagnostic
TypeScript oracle: ok, diagnostics=[]
```

TypeScript AST evidence:

```text
TypeAliasDeclaration BigUnion
TypeAliasDeclaration DiscriminateUnion
TypeAliasDeclaration WithName
TypeAliasDeclaration ChildrenOf
FunctionDeclaration export function makeThing<T extends BigUnion['name']>(...)
```

## Desired final state

Function declaration parsing accepts a trailing comma immediately before `)` in
typed/generic parameter lists, including a final parameter with a default
initializer. Type-only annotations continue to be erased for runtime lowering.

## Scope

In scope:

- [x] Accept a trailing comma before `)` in function declaration parameter lists.
- [x] Preserve existing support for generic type parameter lists after the function name.
- [x] Preserve existing support for parameter type annotations and default initializers.
- [x] Add focused parser coverage for `export function f<T>(x: T, y: T[] = [],) {}`.
- [x] Re-run the representative triage and record any next blocker separately.

Out of scope:

- Full conditional type semantics for `DiscriminateUnion` or `WithName`.
- Typed class method trailing commas, tracked by `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`.
- Comment-specific trailing comma coverage, tracked by `issues/done/5278-parse-trailing-comma-in-function-parameters-with-comments.md`.
- Rest parameter trailing comma diagnostics.

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

- [x] A focused parser test accepts `export function f<T>(x: T, y: T[] = [],) {}`.
- [x] `conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts` no longer reports `issue-247: expected binding identifier or pattern, got Some(RightParen)`.
- [x] Existing parameter-list parser tests continue to pass.
- [x] If parsing advances to a new blocker, that next blocker is recorded separately.
- [x] Issue state stays synchronized with `issues/index.md`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend typed_function_parameter_trailing_comma
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1425-implement-conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.md`.
Related but non-identical slices are `issues/done/5278-parse-trailing-comma-in-function-parameters-with-comments.md`
and `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`.

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

**truly-done** (5303)

- Implementation commits: verified via `git log --oneline --all --grep=5303`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
