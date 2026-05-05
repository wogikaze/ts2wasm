---
id: 735
title: "Implement Assignmentindexedtoprimitives"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentIndexedToPrimitives across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentIndexedToPrimitives` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentIndexedToPrimitives has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentIndexedToPrimitives

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 352,
  "lines": 16,
  "extension": ".ts",
  "first_code_line": "const n1: number = [0];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected identifier or string literal as object key, got Some(Number(0)) at 260..261",
  "span_start": 260,
  "span_end": 261,
  "line": 14,
  "column": 3,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | 
12 | const no1: number = { 0: 1 };
13 | 
14 | const so1: string = { 0: 1 };
15 | const so2: string = { "0": 1 };
16 | const so3: string = { 0: "1" };
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "n1",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "n2",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "n3",
    "line": 4,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "n4",
    "line": 5,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "s1",
    "line": 7,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "s2",
    "line": 8,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "s3",
    "line": 9,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "s4",
    "line": 10,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "no1",
    "line": 12,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Const,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "n1",
        ),
        span: Span {
            start: 26,
            end: 28,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 30,
            end: 36,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 45,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "n2",
        ),
        span: Span {
            start: 51,
            end: 53,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: String(
            "0",
        ),
        span: Span {
            start: 65,
            end: 68,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 72,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "n3",
        ),
        span: Span {
            start: 78,
            end: 80,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 82,
            end: 88,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 93,
            end:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Number(0)) at 260..261
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Number(0)) at 260..261
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": false,
    "diagnostics": [
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number[]' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 26,
        "length": 2,
        "line": 2,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string[]' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '(string | number)[]' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 78,
        "length": 2,
        "line": 4,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number[]' is not assignable to type '0'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 108,
        "length": 2,
        "line": 5,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number[]' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string[]' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 155,
        "length": 2,
        "line": 8,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '(string | number)[]' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 182,
        "length": 2,
        "line": 9,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string[]' is not assignable to type '\"01\"'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 212,
        "length": 2,
        "line": 10,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ 0: number; }' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 244,
        "length": 3,
        "line": 12,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ 0: number; }' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 277,
        "length": 3,
        "line": 14,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ \"0\": number; }' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 308,
        "length": 3,
        "line": 15,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ 0: string; }' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 341,
        "length": 3,
        "line": 16,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 26,
        "length": 2,
        "line": 2,
        "character": 7,
        "name": "n1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 7,
        "name": "n2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 78,
        "length": 2,
        "line": 4,
        "character": 7,
        "name": "n3"
      },
      {
        "kind": "binding",
        "typeText": "0",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 108,
        "length": 2,
        "line": 5,
        "character": 7,
        "name": "n4"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 7,
        "name": "s1"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 155,
        "length": 2,
        "line": 8,
        "character": 7,
        "name": "s2"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 182,
        "length": 2,
        "line": 9,
        "character": 7,
        "name": "s3"
      },
      {
        "kind": "binding",
        "typeText": "\"01\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 212,
        "length": 2,
        "line": 10,
        "character": 7,
        "name": "s4"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 244,
        "length": 3,
        "line": 12,
        "character": 7,
        "name": "no1"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 277,
        "length": 3,
        "line": 14,
        "character": 7,
        "name": "so1"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 308,
        "length": 3,
        "line": 15,
        "character": 7,
        "name": "so2"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentIndexedToPrimitives.ts",
        "start": 341,
        "length": 3,
        "line": 16,
        "character": 7,
        "name": "so3"
      }
    ],
    "types
```

Stack trace:

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Number(0)) at 260..261
```

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
