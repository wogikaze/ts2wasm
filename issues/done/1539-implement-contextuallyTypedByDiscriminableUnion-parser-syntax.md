---
id: 1539
title: "Implement Contextuallytypedbydiscriminableunion Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5388]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypedByDiscriminableUnion-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedByDiscriminableUnion-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypedByDiscriminableUnion-parser-syntax has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5388-support-discriminated-union-parameter-method-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`
  is related, but owns interface-typed erased locals such as
  `var s: Sequence<string>; s.groupBy(...)`; this bucket fails on a function
  parameter typed as a discriminated union alias.
- `issues/open/435-implement-method-call.md` is a broad method-call bucket, not
  a fixed acceptance owner for the discriminated union parameter receiver.
- Generated parser-syntax duplicates listed below were no-match because current
  triage proves parsing succeeds and lowering reports issue-211:
  `issues/open/442-implement-parser-syntax.md`,
  `issues/done/464-implement-FunctionDeclaration-parser-syntax.md`,
  `issues/done/550-implement-FunctionDeclaration-parser-syntax.md`,
  `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md`,
  `issues/open/734-implement-assignmentCompatability-parser-syntax.md`,
  `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md`,
  `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md`,
  `issues/open/059-implement-parser-syntax-extensions.md`,
  `issues/done/065-implement-parser-syntax.md`, and
  `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md`.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts

result:
Triage class: contextuallyTypedByDiscriminableUnion
Feature label: class
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current diagnostic: issue-211: unknown receiver class for method method at 238..253

source context:
function invoke(item: ADT) {
    if (item.kind === "a") {
        item.method("");
    }
    else {
        item.method(42);
    }
}

visible symbols:
function invoke, params item: ADT

compiler evidence:
tokens: ok
ast: ok; includes discriminated union type alias, function invoke(item: ADT),
     strict equality check item.kind === "a", and calls item.method("") /
     item.method(42)
resolved/lowered: fails with UnsupportedSyntax issue-211 unknown receiver class
     for method `method`
TypeScript AST: PropertyAccessExpression item.method at line 14
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5388-support-discriminated-union-parameter-method-calls.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts
result:
pass; reproduced issue-211 unknown receiver class for method `method` on discriminated union parameter receiver
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5388 handles
  discriminated union parameter method calls or reports a more precise
  source-spanned semantic diagnostic.
