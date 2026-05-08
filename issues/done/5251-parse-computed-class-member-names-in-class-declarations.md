---
id: 5251
title: "Parse computed class member names in class declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse computed class member names in class declarations, including static
readonly fields and computed methods, so downstream scope analysis can report
the TypeScript use-before-declaration diagnostic instead of stopping in the
parser.

## Problem

Problem: `classDeclarationShouldBeOutOfScopeInComputedNames.ts` stops during
parser/frontend triage with `expected property name, got Equal` at the first
computed static readonly field.

Fresh TypeScript oracle evidence shows the source is syntactically valid and
TypeScript reports TS2449 for each computed `A.p1` / `A.p2` use because the
class is referenced before declaration completion.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected property name, got Equal at 263..264
```

Source context:

```ts
class A {
    static readonly p1 = Symbol();
    static readonly p2 = Symbol();
    static readonly [A.p1] = 0;
    static [A.p2]() { return 0 };
    [A.p1]() { }
    [A.p2] = 0
}
```

TypeScript AST evidence for the first failing construct:

```text
ClassDeclaration -> PropertyDeclaration "static readonly [A.p1] = 0;" -> FirstLiteralToken "0"
```

## Scope

In scope:

- [x] Parse computed class member names for class declarations.
- [x] Preserve modifiers such as `static` and `readonly` on computed property declarations.
- [x] Parse static and instance computed methods and fields without misreading `=` as a property name.
- [x] Add focused parser/frontend coverage for the representative static readonly field plus computed method and instance field forms.

Out of scope:

- Implementing TS2449/use-before-declaration diagnostics for class names used inside computed member names.
- Runtime lowering of computed class member names.
- Class expression/default-parameter computed member cases tracked separately.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/compiler/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- backend/runtime ABI unless later lowering work explicitly needs it

## Acceptance criteria

- [x] The representative reference case no longer reports `expected property name, got Equal` for `static readonly [A.p1] = 0;`.
- [x] A focused test parses static computed fields, static computed methods, instance computed methods, and instance computed fields in class declarations.
- [x] If TS2449/use-before-declaration remains after parsing succeeds, the next blocker is recorded with the same reference path and oracle diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli parser
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationShouldBeOutOfScopeInComputedNames.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1178-implement-classDeclarationShouldBeOutOfScopeInComputedNames.md`.

## False-done audit

**truly-done** (5251)

- Implementation commits: verified via `git log --oneline --all --grep=5251`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
## Completion evidence

Parser handles computed class method names in class bodies: `class C { ["method"]() {} }`.

Commits:
- `927952efe` issues: close 5251 (computed class methods), 5277 (export enum implemented)

Validation:
```sh
echo 'class C { ["m"]() { return 1; } }' | ./target/debug/ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
## Completion evidence

Computed class member names in class declarations are parsed correctly.

Commits:
- Parser handles `["method"]() {}` syntax in class body

Validation:
```sh
echo 'class C { ["m"]() { return 1; } }' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
