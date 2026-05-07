---
id: 5362
title: "Report strict-mode static constructor parameter name"
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

Report a source-spanned strict-mode reserved-word diagnostic for
`constructor(static)` inside a class, instead of the generic issue-247 parser
binding failure.

## Problem

`constructorStaticParamName.ts` contains a class constructor parameter named
`static`. TypeScript parses the parameter and reports TS1213 because class
definitions are strict mode; ts2wasm stops before AST construction.

Problem: the compiler currently reports an unsupported parser failure,
`issue-247: expected binding identifier or pattern, got Some(Static)`, at the
parameter token.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(Static) at 152..158
```

TypeScript oracle evidence:

```text
TS1213: Identifier expected. 'static' is a reserved word in strict mode. Class definitions are automatically in strict mode.
```

Representative source:

```ts
class test {
    constructor (static) { }
}
```

## Desired final state

The parser recognizes `static` in this constructor parameter-name position and
emits a source-spanned strict-mode reserved-word diagnostic rather than a
generic binding parser failure.

## Scope

In scope:

- [ ] Detect `static` as a parameter name in class constructor parameter lists.
- [ ] Emit a source-spanned strict-mode reserved-word diagnostic at the `static` token.
- [ ] Re-run the representative triage and confirm it no longer reports issue-247.

Out of scope:

- Invalid constructor parameter modifiers such as `static a` / `public static a`, tracked by issue 5355.
- General reserved-word-as-identifier policy outside class constructor parameters.
- Runtime constructor behavior.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/diagnostic.rs`

Do not touch:

- backend/runtime code
- module system code

## Acceptance criteria

- [ ] `constructorStaticParamName.ts` no longer reports `issue-247: expected binding identifier or pattern`.
- [ ] A focused parser/frontend test covers `class C { constructor(static) {} }`.
- [ ] Existing invalid modifier diagnostics from issue 5355 remain separate and are not widened by this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parameter) or test(constructor) or test(class)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamName.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/done/1479-implement-constructorStaticParamName.md` on
2026-05-07.

Related but not exact:

- `issues/open/5355-report-invalid-constructor-parameter-modifiers.md` owns
  invalid modifier forms where `static` or `export` appears before a parameter
  binding.

## Completion evidence

Fill only when implemented.
