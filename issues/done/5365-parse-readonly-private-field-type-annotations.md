---
id: 5365
title: "Parse readonly private field type annotations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse TypeScript `readonly #privateField: T;` class fields, covering the first blocker in `constructorWithParameterPropertiesAndPrivateFields.es2015.ts`.

## Problem

Problem: `constructorWithParameterPropertiesAndPrivateFields.es2015.ts` currently reports `UnsupportedSyntax: expected property name, got PrivateIdentifier("privateField")` at `readonly #privateField: string;`.

The lexer tokenizes `#privateField`, but AST construction rejects the class field after the `readonly` modifier:

```text
UnsupportedSyntax: expected property name, got PrivateIdentifier("privateField") at 114..115
```

Private identifier parsing and basic private fields are already handled by issue 248, and runtime private field semantics are tracked separately. This issue owns the missing parser shape where a readonly modifier precedes a private field with a TypeScript type annotation.

## Current Failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
```

Equivalent mise task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
```

Source context:

```ts
class A {
  readonly #privateField: string;

  constructor(arg: { key: string }, public exposedField: number) {
    ({ key: this.#privateField } = arg);
  }
}
```

Smart triage evidence on 2026-05-07:

```text
tokens: ok; readonly, PrivateIdentifier(privateField), Colon, Ident(string)
AST/resolved: fail; expected property name, got PrivateIdentifier("privateField")
visible symbols: class A
TypeScript oracle: ok, diagnostics=[]
coverage: executed=1, build_pass=0, unsupported=1
```

## Desired Final State

The parser accepts readonly private field declarations with erased TypeScript type annotations, so the reference path can advance to the constructor parameter property and private-field assignment behavior.

## Scope

In scope:

- [x] Parse `readonly #name: Type;` in class bodies.
- [x] Preserve existing support for non-readonly private fields and public readonly fields.
- [x] Erase the TypeScript type annotation consistently with other class fields.
- [x] Add a focused parser/AST regression for `class C { readonly #x: string; }`.
- [x] Re-run the representative reference path and split any next blocker separately if outside this parser shape.

Out of scope:

- Runtime private field storage or brand semantics.
- Definite assignment diagnostics for private fields.
- Parameter property lowering; issue 226 owns the basic supported parameter-property path.
- Derived constructor parameter properties; tracked by issue 5268.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch: unrelated runtime/backend code.

## Acceptance Criteria

- [x] `constructorWithParameterPropertiesAndPrivateFields.es2015.ts` no longer reports `expected property name, got PrivateIdentifier` at `readonly #privateField`.
- [x] A focused parser regression covers `readonly #x: string;`.
- [x] Existing private field parser tests from issue 248 still pass.
- [x] Any next blocker from the reference path is recorded here or split to a follow-up.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(private) or test(class) or test(field)'
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithParameterPropertiesAndPrivateFields.es2015.ts --detail --no-dashboard-data
```

## Docs / Current-State / Issue Sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/open/1483-implement-constructorWithParameterPropertiesAndPrivateFields.md` on 2026-05-07.

Related but distinct issues:

- `issues/done/248-implement-private-class-element-parser.md` owns private identifier tokenization and basic private element parsing.
- `issues/done/255-implement-private-class-element-runtime-semantics.md` owns runtime private field semantics.
- `issues/done/5268-support-derived-constructor-parameter-properties-after-super.md` owns derived constructor parameter-property ordering.

## Completion Evidence

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
