---
id: 5307
title: "Report var/function duplicate identifier diagnostics"
type: bug
area: frontend/resolver
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Align diagnostics for same-name `var` declarations and function declarations
with the TypeScript duplicate identifier shape for the representative reference
cases.

## Problem

`reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts`
parses successfully, but validation stops with a generic duplicate-local
message at the first function keyword. Fresh triage for
`reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts` also
parses successfully, but validation stops with a generic duplicate-local
message for `function y1() { }` followed by `var y1 = 1;`. TypeScript reports
duplicate identifier diagnostics on the duplicated names.

Problem: var/function declaration collisions report generic `DuplicateLocal`
spans instead of source-spanned duplicate identifier diagnostics for the
duplicated identifier.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
```

Current compiler diagnostic:

```text
DuplicateLocal: top-level function `foo` conflicts with existing lexical binding at 38..46
```

Source context:

```text
var foo: string;
function foo(): number { }
function foo(): number { }
```

TypeScript oracle:

```text
TS2300: Duplicate identifier 'foo'.
TS2393: Duplicate function implementation.
```

Additional reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Current compiler diagnostic:

```text
DuplicateLocal: top-level lexical binding `y1` conflicts with function declaration at 70..81
```

Source context:

```text
function y1() { } // error
var y1 = 1; // error
```

## Desired final state

The resolver/validator reports this conflict at the duplicate `foo` identifier
span with a diagnostic message that clearly matches the TypeScript duplicate
identifier rule. The parser and AST shape should remain unchanged.

## Scope

In scope:

- [ ] Adjust the duplicate function-vs-var binding diagnostic span to the duplicate identifier.
- [ ] Handle both declaration orders: `var foo; function foo() { }` and `function foo() { } var foo = 1;`.
- [ ] Preserve duplicate-local rejection for the representative conflict.
- [ ] Add focused regressions for typed `var foo` followed by `function foo`
  and concrete `function y1` followed by `var y1`.

Out of scope:

- Compatible `var` redeclarations, tracked by issue 5162.
- Incompatible same-scope `var` type diagnostics, tracked by issue 5205.
- Broad declaration merging semantics.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/`
- `fixtures/`

Do not touch:

- backend/runtime lowering

## Acceptance criteria

- [ ] `conflictingTypeAnnotatedVar.ts` no longer reports the first blocker at the `function` keyword span `38..46`.
- [ ] `augmentedTypesFunction.ts` no longer reports the first blocker as a
  generic `DuplicateLocal` at the whole `var y1 = 1;` span `70..81`.
- [ ] The duplicate diagnostic is source-spanned at a `foo` identifier and names the duplicate identifier rule.
- [ ] The duplicate diagnostic is source-spanned at a `y1` identifier and names the duplicate identifier rule.
- [ ] Focused regressions preserve rejection for `var foo: string; function foo(): number { }` and `function y1() { } var y1 = 1;`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
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

Split from generated bucket `issues/done/1436-implement-conflictingTypeAnnotatedVar.md`.
Also supersedes stale parser-syntax bucket
`issues/done/769-implement-augmentedTypesFunction.md`.

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
