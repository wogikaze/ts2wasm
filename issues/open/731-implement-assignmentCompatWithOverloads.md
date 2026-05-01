---
id: 731
title: "Implement Assignmentcompatwithoverloads"
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

Triage assignmentCompatWithOverloads across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatWithOverloads` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatWithOverloads has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatWithOverloads

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 493,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "function f1(x: string): number { return null; }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Semicolon) at 442..443",
  "span_start": 442,
  "span_end": 443,
  "line": 26,
  "column": 23,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
23 | 
24 | class C {
25 |     constructor(x: string);
26 | constructor(x: any) {}
27 | }
28 | 
29 | var d: new(x: number) => void;
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f1",
    "line": 2,
    "column": 1,
    "params": "x: string"
  },
  {
    "kind": "function",
    "name": "f2",
    "line": 4,
    "column": 1,
    "params": "x: string"
  },
  {
    "kind": "function",
    "name": "f3",
    "line": 6,
    "column": 1,
    "params": "x: number"
  },
  {
    "kind": "function",
    "name": "f4",
    "line": 8,
    "column": 1,
    "params": "x: string"
  },
  {
    "kind": "function",
    "name": "f4",
    "line": 10,
    "column": 1,
    "params": "x: number"
  },
  {
    "kind": "function",
    "name": "f4",
    "line": 12,
    "column": 1,
    "params": "x: any"
  },
  {
    "kind": "binding",
    "name": "g",
    "line": 14,
    "column": 1
  },
  {
    "kind": "class",
    "name": "C",
    "line": 24,
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
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
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
    "state": "done",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
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
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
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
        kind: Function,
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "f1",
        ),
        span: Span {
            start: 29,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 35,
            end: 41,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 44,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 60,
            end: 64,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 71,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "f2",
        ),
        span: Span {
            start: 80,
            end: 82,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 86,
            end: 92,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 95,
            end: 101,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 104,
            end: 110,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 111,
            end: 115,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 442..443
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 442..443
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
        "message": "Type 'null' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 53,
        "length": 6,
        "line": 2,
        "character": 34
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'null' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 104,
        "length": 6,
        "line": 4,
        "character": 34
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'null' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 155,
        "length": 6,
        "line": 6,
        "character": 34
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '(x: string) => string' is not assignable to type '(s1: string) => number'.\n  Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 345,
        "length": 1,
        "line": 18,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '(x: number) => number' is not assignable to type '(s1: string) => number'.\n  Types of parameters 'x' and 's1' are incompatible.\n    Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 365,
        "length": 1,
        "line": 20,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ (x: string): string; (x: number): number; }' is not assignable to type '(s1: string) => number'.\n  Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 385,
        "length": 1,
        "line": 22,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'typeof C' is not assignable to type 'new (x: number) => void'.\n  Types of construct signatures are incompatible.\n    Type 'new (x: string) => C' is not assignable to type 'new (x: number) => void'.\n      Types of parameters 'x' and 'x' are incompatible.\n        Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 508,
        "length": 1,
        "line": 31,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 29,
        "length": 2,
        "line": 2,
        "character": 10,
        "name": "f1"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 32,
        "length": 1,
        "line": 2,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 80,
        "length": 2,
        "line": 4,
        "character": 10,
        "name": "f2"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 83,
        "length": 1,
        "line": 4,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 131,
        "length": 2,
        "line": 6,
        "character": 10,
        "name": "f3"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 134,
        "length": 1,
        "line": 6,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 182,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "f4"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 185,
        "length": 1,
        "line": 8,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 217,
        "length": 2,
        "line": 10,
        "character": 10,
        "name": "f4"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 220,
        "length": 1,
        "line": 10,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 252,
        "length": 2,
        "line": 12,
        "character": 10,
        "name": "f4"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 255,
        "length": 1,
        "line": 12,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "(s1: string) => number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 297,
        "length": 1,
        "line": 14,
        "character": 5,
        "name": "g"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 301,
        "length": 2,
        "line": 14,
        "character": 9,
        "name": "s1"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 432,
        "length": 1,
        "line": 25,
        "character": 17,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatWithOverloads.ts",
        "start": 457,
        "length": 1,
        "line": 26,
        "character":
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 442..443
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
