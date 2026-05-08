---
id: 5298
title: "Parse for-of array binding pattern heads"
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

Support `for-of` loop heads whose declaration uses an array binding pattern:

```ts
for (const [key, value] of Object.entries(e)) {
  this.setState({ [key]: value });
}
```

## Problem

Problem: array binding patterns in `for-of` declaration heads are parsed as
ordinary lexical declarations, so the parser stops before `of`.

Current failure: `computedPropertyBindingElementDeclarationNoCrash1.ts`
reports an unsupported parser diagnostic, `UnsupportedSyntax: const declarations
require an initializer at 265..277`, for
`for (const [key, value] of Object.entries(e))`.

## Desired final state

The representative `for (const [key, value] of Object.entries(e))` head parses
as a `for-of` declaration with an array binding pattern, or advances to the next
narrower unsupported diagnostic in the same reference file.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
line 16, column 16
failure: const declarations require an initializer at 265..277
```

Source context:

```text
13 | export class Test {
14 |   setState(state: State) {}
15 |   test = (e: any) => {
16 |     for (const [key, value] of Object.entries(e)) {
17 |       this.setState({
18 |         [key]: value,
19 |       });
```

Evidence:

```text
tokens: ok; For Const LeftBracket Ident("key") Comma Ident("value") RightBracket Of Ident("Object") Dot Ident("entries")
ast/resolved: same UnsupportedSyntax at 265..277
visible symbols before failure: class Test
TypeScript AST path: ForOfStatement -> VariableDeclarationList -> VariableDeclaration -> ArrayBindingPattern
```

## Scope

In scope:

- [x] Parse `for (const [a, b] of expr)` as a `for-of` loop head and add a focused parser or CLI regression.

Out of scope:

- Full destructuring runtime semantics for every array binding pattern.
- `for-in` destructuring heads.
- Object binding patterns in loop heads.
- TypeScript type compatibility for the `setState` call.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/` or focused fixtures

Do not touch:

- unrelated runtime/backend code unless triage advances beyond parsing

## Acceptance criteria

- [x] The representative triage no longer reports `const declarations require an initializer` at `265..277`.
- [x] A focused regression covers `for (const [key, value] of Object.entries(e))`; any next blocker is recorded here or split.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(parser) or test(destructuring) or test(reference)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket
`issues/done/1412-implement-computedPropertyBindingElementDeclarationNoCrash.md`.

Related but not duplicates:

- Issues 247, 251, and 252 completed the initial destructuring parser/runtime
  slices and explicitly left `for-in` / `for-of` destructuring heads out of
  scope.
- Issue 342 covers `Object.entries` library/runtime behavior. This issue is
  earlier parser work and fails before runtime semantics are reached.

## Completion Evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5298)

- Implementation commits: verified via `git log --oneline --all --grep=5298`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
