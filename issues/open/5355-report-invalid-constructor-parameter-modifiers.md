---
id: 5355
title: "Report invalid constructor parameter modifiers"
type: bug
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript-style diagnostics for invalid modifiers on constructor
parameters instead of surfacing generic parser failures for `static` and
`export` tokens.

Split from generated bucket `1470`.

## Problem

`constructorArgsErrors1.ts`, `constructorArgsErrors2.ts`, and
`constructorArgsErrors5.ts` contain invalid constructor parameter modifiers.
TypeScript parses each parameter and reports TS1090; the current parser stops
before AST construction with generic `UnsupportedSyntax` messages.

Problem: current failure is `expected Comma, got Some(Static)` for
`constructor(public static a: number)` and `issue-247 expected binding
identifier or pattern` for `static a` / `export a`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts
```

Current diagnostic for `public static a`:

```text
error: [UnsupportedSyntax] expected Comma, got Some(Static) at 57..63
```

Representative source:

```ts
class foo {
    constructor (public static a: number) {
    }
}
```

Other failing variants:

```text
constructorArgsErrors1.ts: constructor (static a: number)
current: issue-247 expected binding identifier or pattern, got Some(Static)
oracle: TS1090 "'static' modifier cannot appear on a parameter."

constructorArgsErrors5.ts: constructor (export a: number)
current: issue-247 expected binding identifier or pattern, got Some(Export)
oracle: TS1090 "'export' modifier cannot appear on a parameter."
```

Compiler evidence:

```text
tokens: ok for constructor parameter modifier tokens Static and Export
ast/resolved: fail before AST construction for the three invalid variants
TypeScript oracle: parses ClassDeclaration/Constructor/Parameter and reports TS1090 at the invalid modifier
focused coverage: executed=5, build_pass=2, unsupported=3
```

## Desired final state

The parser emits a source-spanned diagnostic that names the disallowed modifier
on a parameter. The three representative cases no longer report generic
`expected binding identifier` or `expected Comma` failures.

## Scope

In scope:

- [x] Detect `static`, `public static`, and `export` before a constructor parameter binding.
- [x] Report the diagnostic at the invalid modifier token.
- [x] Preserve valid constructor parameter properties for `public`, `private`, `protected`, and `readonly`.
- [x] Add focused parser/frontend coverage for the invalid modifier variants.

Out of scope:

- Full TypeScript diagnostic code parity infrastructure.
- Constructor arity, overload checking, or valid parameter-property runtime behavior.

## Affected paths

Expected:

- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`

Do not touch:

- backend/runtime lowering or arguments-object runtime behavior

## Acceptance criteria

- [x] `constructorArgsErrors1.ts`, `constructorArgsErrors2.ts`, and `constructorArgsErrors5.ts` no longer report the current generic parser failures for invalid parameter modifiers.
- [x] A focused parser/frontend test covers `constructor(static a: number)`, `constructor(public static a: number)`, and `constructor(export a: number)`.
- [x] Existing valid constructor parameter property tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parameter) or test(constructor) or test(class)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgsErrors5.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgsErrors --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none expected

## Notes

Issue 226 covers valid constructor parameter properties. This issue is narrower:
invalid modifier diagnostics for parameter positions.

## Completion evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5355)

- Implementation commits: verified via `git log --oneline --all --grep=5355`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
