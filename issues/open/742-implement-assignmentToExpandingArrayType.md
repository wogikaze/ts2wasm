---
id: 742
title: "Implement Assignmenttoexpandingarraytype"
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

Triage assignmentToExpandingArrayType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToExpandingArrayType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToExpandingArrayType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentToExpandingArrayType

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 637,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "let x = []"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"x\")) at 96..97",
  "span_start": 96,
  "span_end": 97,
  "line": 5,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @noImplicitAny: true
3 | // Fixes exponential time/space in #14628
4 | let x = []
5 | x[0] = { foo: 'hi' }
6 | x[0] = { foo: 'hi' }
7 | x[0] = { foo: 'hi' }
8 | x[0] = { foo: 'hi' }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 4,
    "column": 1,
    "initializer": "[]"
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
    "path": "issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/243-implement-numeric-literal-separator-parser.md",
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
        kind: Let,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 105,
            end: 108,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: String(
            "hi",
        ),
        span: Span {
            start: 110,
            end: 114,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 126,
            end: 129,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: String(
            "hi",
        ),
        span: Span {
            start: 131,
            end: 135,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("x")) at 96..97
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("x")) at 96..97
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
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 96,
        "length": 4,
        "line": 5,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 117,
        "length": 4,
        "line": 6,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 138,
        "length": 4,
        "line": 7,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 159,
        "length": 4,
        "line": 8,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 180,
        "length": 4,
        "line": 9,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 201,
        "length": 4,
        "line": 10,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 222,
        "length": 4,
        "line": 11,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 243,
        "length": 4,
        "line": 12,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 264,
        "length": 4,
        "line": 13,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 285,
        "length": 4,
        "line": 14,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 306,
        "length": 4,
        "line": 15,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 327,
        "length": 4,
        "line": 16,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 348,
        "length": 4,
        "line": 17,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 406,
        "length": 4,
        "line": 18,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 427,
        "length": 4,
        "line": 19,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 448,
        "length": 4,
        "line": 20,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 469,
        "length": 4,
        "line": 21,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 490,
        "length": 4,
        "line": 22,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 511,
        "length": 4,
        "line": 23,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 532,
        "length": 4,
        "line": 24,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 553,
        "length": 4,
        "line": 25,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ foo: string; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToExpandingArrayType.ts",
        "start": 574,
        "length": 4,
        "line": 26,
        "character": 1
      },
      {
        "code": 2322,
        "
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("x")) at 96..97
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
