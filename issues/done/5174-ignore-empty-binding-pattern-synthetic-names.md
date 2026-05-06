---
id: 5174
title: "Ignore empty binding pattern synthetic names"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Stop treating empty destructuring binding patterns such as `const {} = ...` as duplicate local names.

## Problem

`bindingPatternCannotBeOnlyInferenceSource.ts` contains more than one empty binding pattern. The parser currently represents those declarations as synthetic names like `{}` and `[]`, and AST validation reports `DuplicateLocal` when the same synthetic name appears again.

Problem: empty binding patterns do not declare a local binding, but the compiler currently registers their display text as if it were a real local name.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts
```

Current compiler diagnostic:

```text
DuplicateLocal: duplicate local binding: `{}` at 937..1148
```

Source context:

```ts
const {} = f();
const [] = f();
const {} = useReduxDispatch1(
    (d, f) => ({ /* ... */ })
);
```

Compiler evidence:

- Tokens and AST succeed.
- AST contains `Let { name: "{}" }` and `Let { name: "[]" }` for empty binding patterns.
- Resolved pipeline fails during `validate_ast` with `DuplicateLocal` for the repeated synthetic `{}` name.
- TypeScript reports type/inference diagnostics on the destructuring expressions, not duplicate local declarations.

## Desired final state

Empty object and array binding patterns are represented or validated as declarations with no bound names, so repeated empty destructuring declarations do not trigger duplicate-local diagnostics.

## Scope

In scope:

- [x] Avoid registering `{}` and `[]` synthetic binding-pattern display names as local bindings.
- [x] Preserve duplicate-local checks for real names inside non-empty binding patterns.
- [x] Add focused coverage for repeated `const {} = f();` and `const [] = f();`.

Out of scope:

- Full TypeScript inference diagnostics for generic destructuring.
- Runtime destructuring semantics for non-empty object or array patterns.
- Rest/spread binding pattern lowering.

## Affected paths

Expected:

- `crates/frontend/src/parser/binding_patterns.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/binding_pattern.rs`
- focused frontend/IR tests

Do not touch:

- Type inference implementation for `f<T>()`
- runtime destructuring lowering beyond avoiding the false duplicate-local diagnostic

## Acceptance criteria

- [x] A focused test accepts repeated `const {} = f();` without `DuplicateLocal`.
- [x] A focused test accepts repeated `const [] = f();` without `DuplicateLocal`.
- [x] Existing duplicate-local tests for repeated real identifiers still fail.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts` no longer reports `duplicate local binding: {}`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir binding_pattern
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts
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

Split from generated bucket `1060` on 2026-05-06. Later TypeScript inference diagnostics in this file should be triaged after the false duplicate-local diagnostic is gone.

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

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in . This issue has repo-local close evidence
(implementation commit or completion evidence).

Future-work tracking: none identified.
