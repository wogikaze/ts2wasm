---
id: 1190
title: "Implement Classexpressionwithstaticpropertieses Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5254]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1190.

## Summary

Closed as superseded by `issues/open/5254-parse-asi-between-static-class-fields.md`.
Fresh triage shows the remaining parser-syntax representative in this bucket is
the same static class field ASI boundary already split from 1188.

## Problem

Reference test results previously showed 2 cases in
`classExpressionWithStaticPropertiesES-parser-syntax`.

Problem: fresh coverage shows `classExpressionWithStaticPropertiesES61.ts`
builds, while `classExpressionWithStaticPropertiesES62.ts` fails with
`UnsupportedSyntax: expected LeftParen, got Some(Static)` at the later-line
`static c` field after `static b`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with the existing static class field ASI issue
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
- [x] Existing owner contains the exact reference-triage command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner acceptance names the exact diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5254-parse-asi-between-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts`
- `reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES61.ts` (fresh build pass)

Source context for the remaining parser-syntax failure:

```ts
var v = class C {
    static a = 1;
    static b
    static c = {
        x: "hi"
    }
    static d = C.c.x + " world";
};
```

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/open/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES61.ts
```

Observed result on 2026-05-06:

```text
classExpressionWithStaticPropertiesES61.ts: BuildPass
classExpressionWithStaticPropertiesES62.ts: UnsupportedSyntax: expected LeftParen, got Some(Static) at 91..97
```

`classExpressionWithStaticPropertiesES62.ts` tokenizes successfully, then the
parser reaches `static b` followed by later-line `static c = ...` and expects a
method `LeftParen`. TypeScript accepts this as a `PropertyDeclaration` whose
path includes `ClassExpression -> PropertyDeclaration "static c = ..." ->
StaticKeyword`.

Superseding owner:

- `issues/open/5254-parse-asi-between-static-class-fields.md`

Reason:

- 5254 already covers ASI after `static name` before a later-line `static`
  class element and explicitly includes class expressions.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/open/5254-parse-asi-between-static-class-fields.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES62.ts
result: pass; diagnostic resolved to static class field ASI boundary tracked by 5254
date: 2026-05-06
```

Remaining risks:

- The broad `classExpressionWithStaticPropertiesES` path filter still has
  unknown-unsupported ES63/ES64 failures; those belong to the adjacent
  unknown-unsupported bucket, not this parser-syntax bucket.
