---
id: 5235
title: "Erase type predicate object return types"
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

Parse and erase TypeScript type predicate return annotations whose asserted
type is an object type literal, such as `x is { a: string; }`, without
consuming the object type as the JavaScript function body.

## Problem

`checkTypePredicateForRedundantProperties.ts` contains a function returning a
type predicate with an object type literal. The parser currently treats the
type literal fields as labeled statements inside the function body and then
parses the real body's `return true;` as a top-level return.

Problem: `function f(x: any): x is { a: string; a: string; } { return true; }` reports `InvalidTopLevelReturn` because the return type is misparsed as the function body.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
```

Source context:

```ts
function addProp2(x: any): x is { a: string; a: string; } {
    return true;
}
```

Compiler evidence:

```text
tokens: ok
ast: Function body incorrectly contains Labeled("a": Ident("string")) twice
ast: following Return(true) is emitted as a top-level statement
validate_ast: InvalidTopLevelReturn at return true
TypeScript oracle: TS2300 Duplicate identifier 'a' on the two object type properties
```

## Desired final state

The parser erases the full type predicate return annotation, including object
type literal members, and starts the function body at the real `{ return true;
}` block. The representative case should no longer report `InvalidTopLevelReturn`.

## Scope

In scope:

- [ ] Parse and erase return annotations of the form `param is { ... }`.
- [ ] Correctly skip semicolon-delimited duplicate object type properties inside the type literal.
- [ ] Preserve existing parsing for ordinary function bodies that start with `{`.
- [ ] Add focused parser/AST coverage for a type predicate object return annotation followed by a real body.
- [ ] Re-run the representative reference triage and confirm the top-level return boundary is gone.

Out of scope:

- Full TypeScript type predicate semantics or control-flow narrowing.
- Emitting TS2300 duplicate-property diagnostics for object type literals.
- Type aliases, interfaces, and broader object type validation.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- runtime/backend code
- type-checking semantics beyond parser erasure

## Acceptance criteria

- [ ] A focused parser test accepts `function f(x: any): x is { a: string; a: string; } { return true; }`.
- [ ] The AST function body contains only the real `return true;` body, not labeled statements from the object type literal.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts` no longer reports `InvalidTopLevelReturn`.
- [ ] Existing object literal expression parsing remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1145-implement-checkTypePredicateForRedundantProperties.md`.

Related but broader:

- `issues/open/4523-implement-typeInferenceTypePredicate-type-system.md`
- `issues/open/4564-implement-typePredicateStructuralMatch.md`
- `issues/open/4565-implement-typePredicateTopLevelTypeParameter.md`

## Completion evidence

Fill when implemented.
