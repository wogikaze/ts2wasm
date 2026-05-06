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

Align the diagnostic for a typed `var` declaration followed by duplicate
same-name function implementations with the TypeScript duplicate identifier
shape for the representative reference case.

## Problem

`reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts`
parses successfully, but validation stops with a generic duplicate-local
message at the first function keyword. TypeScript reports duplicate identifier
diagnostics on the `foo` names and duplicate function implementation
diagnostics for the two functions.

Problem: `conflictingTypeAnnotatedVar.ts` reports a generic `DuplicateLocal` at the function declaration span instead of source-spanned duplicate identifier diagnostics for `foo`.

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

## Desired final state

The resolver/validator reports this conflict at the duplicate `foo` identifier
span with a diagnostic message that clearly matches the TypeScript duplicate
identifier rule. The parser and AST shape should remain unchanged.

## Scope

In scope:

- [ ] Adjust the duplicate function-vs-var binding diagnostic span to the duplicate identifier.
- [ ] Preserve duplicate-local rejection for the representative conflict.
- [ ] Add a focused regression for typed `var foo` followed by two `function foo` declarations.

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
- [ ] The duplicate diagnostic is source-spanned at a `foo` identifier and names the duplicate identifier rule.
- [ ] A focused regression preserves rejection for `var foo: string; function foo(): number { }`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeAnnotatedVar.ts
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
