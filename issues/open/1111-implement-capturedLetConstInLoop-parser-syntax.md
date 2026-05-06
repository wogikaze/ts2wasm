---
id: 1111
title: "Implement Capturedletconstinloop Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage capturedLetConstInLoop-parser-syntax across the original 14 failing reference test cases plus 2 folded capturedLetConstInLoop import/export misbucket cases, then split this bucket into implementation-ready child issues.

## Problem

Reference test results show 14 cases fail in directory `capturedLetConstInLoop-parser-syntax` with diagnostics: parser-syntax. Fresh triage of stale bucket #1109 adds 2 more capturedLetConstInLoop cases whose current blocker is also parser-syntax, not import/export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedLetConstInLoop-parser-syntax has 14 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 28
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] `issues/open/5207-parse-do-while-asi-before-following-for.md`
- [ ] `issues/open/5208-parse-arrow-body-destructuring-assignments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop1_ES6.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop12.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop13.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop2.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop2_ES6.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop4.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop4_ES6.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop5_ES6.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop5.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop7.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop6.ts`
- ... and 4 more files

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Fresh triage has started. The repeated `expected Semicolon, got Some(For)`
subfamily is split to
`issues/open/5207-parse-do-while-asi-before-following-for.md`; remaining
subfamilies still need child issues.

### Smart triage: capturedLetConstInLoop1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 487..490",
  "line": 34,
  "column": 3
}
```

Source context:

```text
31 | } while (1 === 1)
32 |
33 | for (let y = 0; y < 1; ++y) {
34 |     let x = 1;
```

TypeScript oracle succeeds with no diagnostics.

### Smart triage: capturedLetConstInLoop1_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop1_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 446..449",
  "line": 32,
  "column": 1
}
```

Source context:

```text
29 | } while (1 === 1)
30 |
31 | for (let y = 0; y < 1; ++y) {
32 |     let x = 1;
```

TypeScript oracle succeeds with no diagnostics.

### Smart triage: capturedLetConstInLoop12

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop12.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop12.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected RightParen, got Some(Equal) at 129..130",
  "line": 7,
  "column": 26
}
```

Source context:

```text
6 |     for (let i = 0; i < 4; i++) {
7 |         (() => [i] = [i + 1])();
8 |     }
```

TypeScript AST sees the arrow body as a `BinaryExpression` assignment
`[i] = [i + 1]` and reports no diagnostics. Child issue
`issues/open/5208-parse-arrow-body-destructuring-assignments.md` owns this
parser slice.

### Folded triage from #1109: capturedLetConstInLoop4

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop4.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop4.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 692..695",
  "line": 43,
  "column": 12
}
```

Source context:

```text
42 | for (let y = 0; y < 1; ++y) {
43 |     let x = 1;
44 |     var v4 = x;
45 |     (function() { return x + v4});
46 |     (() => x);
```

Visible symbols before failure include `exportedFoo`, loop/captured bindings
`x`, `v0`, `v00`, `v1`, `v2`, `v3`, `y`, and the failing block-local `x`.
Compiler tokens succeed; AST and resolved dumps fail with the same
`UnsupportedSyntax` parser error. TypeScript oracle succeeds with no
diagnostics.

### Folded triage from #1109: capturedLetConstInLoop4_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop4_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop4_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 669..672",
  "line": 42,
  "column": 11
}
```

Source context:

```text
41 | for (let y = 0; y < 1; ++y) {
42 |     let x = 1;
43 |     var v4 = x;
44 |     (function() { return x + v4});
45 |     (() => x);
```

Visible symbols before failure include `exportedFoo`, loop/captured bindings
`x`, `v0`, `v00`, `v1`, `v2`, `v3`, `y`, and the failing block-local `x`.
Compiler tokens succeed; AST and resolved dumps fail with the same
`UnsupportedSyntax` parser error. TypeScript oracle succeeds with no
diagnostics.

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
