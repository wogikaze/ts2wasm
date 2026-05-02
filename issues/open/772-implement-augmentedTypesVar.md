---
id: 772
title: "Implement Augmentedtypesvar"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage augmentedTypesVar across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `augmentedTypesVar` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentedTypesVar has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesVar.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesVar.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesVar.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: augmentedTypesVar

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesVar.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 673,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "var x1 = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Var) at 203..206",
  "span_start": 203,
  "span_end": 206,
  "line": 14,
  "column": 14,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | var x3 = () => { } // error
12 | 
13 | // var then class
14 | var x4 = 1; // error
15 | class x4 { } // error
16 | 
17 | var x4a = 1; // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "then",
    "line": 2,
    "column": 4
  },
  {
    "kind": "binding",
    "name": "var",
    "line": 2,
    "column": 13
  },
  {
    "kind": "binding",
    "name": "x1",
    "line": 4,
    "column": 1,
    "initializer": "2"
  },
  {
    "kind": "binding",
    "name": "then",
    "line": 6,
    "column": 4
  },
  {
    "kind": "binding",
    "name": "x2",
    "line": 7,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "function",
    "name": "x2",
    "line": 8,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "x3",
    "line": 10,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "x3",
    "line": 11,
    "column": 1,
    "initializer": "() => { } // error"
  },
  {
    "kind": "binding",
    "name": "then",
    "line": 13,
    "column": 4
  },
  {
    "kind": "class",
    "name": "var",
    "line": 13,
    "column": 13
  },
  {
    "kind": "binding",
    "name": "x4",
    "line": 14,
    "column": 1,
    "initializer": "1"
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
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/767-implement-augmentedTypesEnum-parser-syntax.md",
    "title": "Implement Augmentedtypesenum Parser Syntax",
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
        kind: Var,
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 41,
            end: 43,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 50,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 54,
            end: 56,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "x2",
        ),
        span: Span {
            start: 91,
            end: 93,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 109,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "x2",
        ),
        span: Span {
            start: 118,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "x3",
        ),
        span: Span {
            start: 143,
            end: 145,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 149,
            end: 150,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 91,
        "length": 2,
        "line": 7,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 118,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2403,
        "category": "Error",
        "message": "Subsequent variable declarations must have the same type.  Variable 'x3' must be of type 'number', but here has type '() => void'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 157,
        "length": 2,
        "line": 11,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 207,
        "length": 2,
        "line": 14,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 231,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 254,
        "length": 3,
        "line": 17,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 279,
        "length": 3,
        "line": 18,
        "character": 7
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 338,
        "length": 2,
        "line": 21,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 352,
        "length": 2,
        "line": 22,
        "character": 6
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 461,
        "length": 3,
        "line": 28,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 490,
        "length": 3,
        "line": 29,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 544,
        "length": 3,
        "line": 31,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 573,
        "length": 3,
        "line": 32,
        "character": 11
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 41,
        "length": 2,
        "line": 3,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 54,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 91,
        "length": 2,
        "line": 7,
        "character": 5,
        "name": "x2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 118,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 143,
        "length": 2,
        "line": 10,
        "character": 5,
        "name": "x3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 157,
        "length": 2,
        "line": 11,
        "character": 5,
        "name": "x3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 207,
        "length": 2,
        "line": 14,
        "character": 5,
        "name": "x4"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 254,
        "length": 3,
        "line": 17,
        "character": 5,
        "name": "x4a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 338,
        "length": 2,
        "line": 21,
        "character": 5,
        "name": "x5"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 399,
        "length": 2,
        "line": 25,
        "character": 5,
        "name": "x6"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 461,
        "length": 3,
        "line": 28,
        "character": 5,
        "name": "x6a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 500,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/re
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
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
