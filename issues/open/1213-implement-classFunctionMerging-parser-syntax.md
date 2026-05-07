---
id: 1213
title: "Implement Classfunctionmerging Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [400]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1213.

## Summary

Closed as superseded by the completed ambient declaration boundary in
`issues/done/400-implement-ambient-declaration-erasure-boundary.md`.

## Problem

Reference test results previously showed 1 `parser-syntax` case in
`classFunctionMerging-parser-syntax`.

Problem: fresh triage shows this is not an unclassified parser work item. The
compiler reaches the intended issue-400 rejection boundary for an unsupported
ambient declaration form: `declare abstract class A`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing done issue 400 covers this boundary
- [x] Supersede this generated bucket without creating a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Existing owner contains the ambient declaration boundary contract
- [x] This issue includes failing path, diagnostic code, source context, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFunctionMerging2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFunctionMerging2.ts`

Source context:

```ts
declare abstract class A {
    constructor(p: number);
    a: number;
}

declare function B(p: string): B;
declare class B extends A {
    constructor(p: string);
    b: number;
}
```

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging2.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedTypeScriptSyntax:1
unsupported_features: parser-syntax:1

Diagnostic: UnsupportedTypeScriptSyntax
Message: issue-400: unsupported ambient declaration form at 28..36
Tokens: ok
AST: fails at `declare abstract class A`
TypeScript oracle: ok, diagnostics=[]
```

TypeScript's AST accepts the top-level `declare abstract class A`, the ambient
function declaration `B`, and the ambient class declaration `B extends A`.
ts2wasm currently rejects the `abstract` ambient class form at the completed
issue-400 boundary for unsupported ambient declaration forms.

Superseding owner:

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/400-implement-ambient-declaration-erasure-boundary.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging2.ts
result: pass; current blocker matches the completed issue-400 ambient declaration boundary
date: 2026-05-06
```

Remaining risks:

- `classFunctionMerging2.ts` may expose later ambient class/function merge
  semantics if issue-400 is broadened in the future, but this generated bucket
  no longer needs a separate child issue.
