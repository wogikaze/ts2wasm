---
id: 5289
title: "Validate commentsOverloads top-level functions"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept the first top-level function overload group in `commentsOverloads.ts`
when comments/trivia appear around overload signatures and parameters.

## Problem

`commentsOverloads.ts` tokenizes and parses to AST, but `validate_ast` treats
the second bodyless `f1` overload signature as a duplicate function definition.
TypeScript accepts the group as two overload signatures followed by one
implementation.

Problem: `reference/typescript/tests/cases/compiler/commentsOverloads.ts`
reports `DuplicateFunction` for `function f1(b: string): number;`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOverloads.ts
```

Current diagnostic:

```text
error: [DuplicateFunction] duplicate function definition: `f1` at 166..174
```

Source context:

```ts
/** this is signature 1*/
function f1(/**param a*/a: number): number;
function f1(b: string): number;
function f1(aOrb: any) {
    return 10;
}
```

Smart triage evidence:

```text
tokens: ok
ast: ok; Function f1(a), Function f1(b), Function f1(aOrb) with body
validate_ast: DuplicateFunction for second bodyless overload signature
TypeScript oracle: ok, diagnostics: []
```

## Desired final state

The first `commentsOverloads.ts` top-level `f1` overload group is accepted as
overload signatures plus one implementation, so triage can reach the next
overload/comment blocker in the file.

## Scope

In scope:

- [ ] Accept two bodyless top-level `f1` overload signatures followed by one implementation.
- [ ] Preserve comments/trivia as non-semantic for overload grouping.
- [ ] Preserve duplicate diagnostics for multiple concrete top-level implementations.
- [ ] Record the next `commentsOverloads.ts` blocker after `f1` advances.

Out of scope:

- Full comment emit fidelity.
- Interface call signature overloads.
- Class method overloads.
- Constructor overloads.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime call semantics

## Acceptance criteria

- [ ] `commentsOverloads.ts` no longer reports `DuplicateFunction` for the second bodyless top-level `f1` overload signature.
- [ ] A focused fixture covers the commented `f1` overload group shape.
- [ ] Existing duplicate concrete function implementation diagnostics remain covered.
- [ ] The next blocker from `commentsOverloads.ts` is recorded if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOverloads.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOverloads.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1379-implement-commentsOverloads.md`.
Related but broader issues:

- `issues/open/5200-validate-top-level-function-overload-implementations.md`
- `issues/open/5280-validate-commented-top-level-function-overloads.md`

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
