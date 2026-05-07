---
id: 1111
title: "Implement Capturedletconstinloop Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
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
mise run reference-coverage -- tsc --limit 28
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5207-support-ambient-interface-filter-receiver.md`
- [x] `issues/done/5208-support-regexp-match-fallback-array-map-receiver.md`
- [x] `issues/done/5209-support-class-instance-method-receiver-calls.md`
- [x] `issues/done/5210-array-map-sparse-array-holes.md`
- [x] `issues/done/5211-sparse-array-spread-support.md`
- [x] `issues/open/5387-parse-function-expression-statements-in-nested-blocks.md`

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
- `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Fresh triage has started. The repeated `expected Semicolon, got Some(For)`
subfamily is split to
`issues/done/5207-support-ambient-interface-filter-receiver.md`; remaining
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
Child issue `issues/done/5207-support-ambient-interface-filter-receiver.md`
owns this ASI slice.

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
Child issue `issues/done/5207-support-ambient-interface-filter-receiver.md`
owns this ASI slice.

### Smart triage: capturedLetConstInLoop6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 890..893",
  "line": 64,
  "column": 17
}
```

Source context:

```text
60 | } while (1 === 1)
61 |
62 | for (let y = 0; y < 1; ++y) {
63 |     let x = 1;
```

TypeScript oracle succeeds with no diagnostics. This is the same
no-semicolon `do while` before `for` parser boundary as capturedLetConstInLoop1,
with `break` and `continue` statements already parsed inside the preceding
body. Child issue
`issues/done/5207-support-ambient-interface-filter-receiver.md` owns this ASI
slice.

### Smart triage: capturedLetConstInLoop6_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop6_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop6_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(For) at 887..890",
  "line": 64,
  "column": 17
}
```

Source context:

```text
60 | } while (1 === 1)
61 |
62 | for (let y = 0; y < 1; ++y) {
63 |     let x = 1;
```

TypeScript oracle succeeds with no diagnostics. This is the same
no-semicolon `do while` before `for` parser boundary as capturedLetConstInLoop1,
with `break` and `continue` statements already parsed inside the preceding
body. Child issue
`issues/done/5207-support-ambient-interface-filter-receiver.md` owns this ASI
slice.

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
`issues/done/5208-support-regexp-match-fallback-array-map-receiver.md` owns this
parser slice.

### Smart triage: capturedLetConstInLoop13

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop13.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop13.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Dot, got Some(Plus) at 257..258",
  "line": 13,
  "column": 35
}
```

Source context:

```text
12 |             this.bar({
13 |                 [name + ".a"]: () => { this.foo(name); },
14 |             });
```

TypeScript AST sees `PropertyAssignment -> ComputedPropertyName ->
BinaryExpression` for `name + ".a"` and reports no diagnostics. Child issue
`issues/done/5209-support-class-instance-method-receiver-calls.md` owns
this parser slice.

### Smart triage: capturedLetConstInLoop2

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop2.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(RightBrace) at 861..862",
  "line": 48,
  "column": 23
}
```

Source context includes the preceding `do { ... } while (1 === 1)` with no
explicit semicolon before the enclosing function closes. Child issue
`issues/done/5210-array-map-sparse-array-holes.md` owns
this ASI slice.

### Smart triage: capturedLetConstInLoop2_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop2_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop2_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(RightBrace) at 856..857",
  "line": 44,
  "column": 1
}
```

Source context includes the preceding `do { ... } while (1 === 1)` with no
explicit semicolon before the enclosing function closes. Child issue
`issues/done/5210-array-map-sparse-array-holes.md` owns
this ASI slice.

### Smart triage: capturedLetConstInLoop5

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop5.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"use\")) at 1176..1179",
  "line": 75,
  "column": 6
}
```

Source context includes a preceding `do { ... } while (1 === 1)` with no
explicit semicolon before the following `use(v);` expression. TypeScript
advances and reports TS2454 use-before-assigned diagnostics. Child issue
`issues/done/5210-array-map-sparse-array-holes.md` owns
this ASI slice.

### Smart triage: capturedLetConstInLoop5_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop5_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop5_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"use\")) at 1175..1178",
  "line": 75,
  "column": 6
}
```

Source context includes a preceding `do { ... } while (1 === 1)` with no
explicit semicolon before the following `use(v);` expression. TypeScript
advances and reports TS2454 use-before-assigned diagnostics. Child issue
`issues/done/5210-array-map-sparse-array-holes.md` owns
this ASI slice.

### Smart triage: capturedLetConstInLoop7

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop7.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop7.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"l4\")) at 1383..1385",
  "line": 101,
  "column": 2
}
```

Source context:

```text
94 | } while (1 === 1)
95 |
96 | l4:
97 | for (let y = 0; y < 1; ++y) {
```

TypeScript oracle accepts the no-semicolon `do while` before the labeled
statement. Child issue
`issues/done/5211-sparse-array-spread-support.md` owns this ASI
slice.

### Smart triage: capturedLetConstInLoop7_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop7_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop7_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"l4\")) at 1380..1382",
  "line": 101,
  "column": 2
}
```

Source context:

```text
94 | } while (1 === 1)
95 |
96 | l4:
97 | for (let y = 0; y < 1; ++y) {
```

TypeScript oracle accepts the no-semicolon `do while` before the labeled
statement. Child issue
`issues/done/5211-sparse-array-spread-support.md` owns this ASI
slice.

### Smart triage: capturedLetConstInLoop9

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop9.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop9.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Ident(\"x\")) at 133..134",
  "line": 8,
  "column": 4
}
```

Source context:

```text
6 |     {
7 |         let x;
8 |         (function() { return x });
9 |     }
```

TypeScript accepts the parenthesized function expression statement inside the
nested block. Child issue
`issues/open/5387-parse-function-expression-statements-in-nested-blocks.md`
owns this parser slice.

### Smart triage: capturedLetConstInLoop9_ES6

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop9_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop9_ES6.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Ident(\"x\")) at 132..133",
  "line": 9,
  "column": 4
}
```

Source context:

```text
7 |     {
8 |         let x;
9 |         (function() { return x });
10 |     }
```

TypeScript accepts the parenthesized function expression statement inside the
nested block. Child issue
`issues/open/5387-parse-function-expression-statements-in-nested-blocks.md`
owns this parser slice.

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
diagnostics. This is the same no-semicolon `do while` before `for` boundary as
capturedLetConstInLoop1, with captured `var` use in the surrounding body. Child
issue `issues/done/5207-support-ambient-interface-filter-receiver.md` owns this
ASI slice.

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
diagnostics. This is the same no-semicolon `do while` before `for` boundary as
capturedLetConstInLoop1, with captured `var` use in the surrounding body. Child
issue `issues/done/5207-support-ambient-interface-filter-receiver.md` owns this
ASI slice.

## Completion evidence

capturedLetConstInLoop parser-syntax triage is complete. All current parser
failures in the bucket are represented by focused implementation issues
5207-5212, with the import/export misbucket folded into the same evidence.

Commits:

- `2ebf81da` issues: split captured loop do-while asi parser blocker
- `2e78774a` issues: split captured loop arrow assignment parser blocker
- `2215a150` issues: split captured loop computed property parser blocker
- `1de05874` issues: split captured loop do-while asi expression blocker
- `14657477` issues: split captured loop labeled asi parser blocker
- `bba271ed` issues: split captured loop nested function parser blocker
- `d88eeccf` issues: fold captured loop asi variants into child issues

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass; child issues 5207-5212 are M-sized and ready
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none
