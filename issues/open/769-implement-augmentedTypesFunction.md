---
id: 769
title: "Implement Augmentedtypesfunction"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage augmentedTypesFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `augmentedTypesFunction` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentedTypesFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: augmentedTypesFunction

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 894,
  "lines": 39,
  "extension": ".ts",
  "first_code_line": "function y1() { } // error"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Function) at 265..273",
  "span_start": 265,
  "span_end": 273,
  "line": 14,
  "column": 14,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | var y2a = () => { } // error
12 | 
13 | // function then class
14 | function y3() { } // error
15 | class y3 { } // error
16 | 
17 | function y3a() { } // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "function",
    "line": 2,
    "column": 18
  },
  {
    "kind": "function",
    "name": "y1",
    "line": 3,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "y1",
    "line": 4,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "function",
    "name": "y2",
    "line": 7,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "y2",
    "line": 8,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "y2a",
    "line": 10,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "y2a",
    "line": 11,
    "column": 1,
    "initializer": "() => { } // error"
  },
  {
    "kind": "class",
    "name": "function",
    "line": 13,
    "column": 18
  },
  {
    "kind": "function",
    "name": "y3",
    "line": 14,
    "column": 1,
    "params": ""
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
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/767-implement-augmentedTypesEnum-parser-syntax.md",
    "title": "Implement Augmentedtypesenum Parser Syntax",
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
        kind: Function,
        span: Span {
            start: 42,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 51,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 70,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 74,
            end: 76,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 121,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2",
        ),
        span: Span {
            start: 130,
            end: 132,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 133,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 149,
            end: 157,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2",
        ),
        span: Span {
            start: 158,
            end: 160,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 179,
            end: 187,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2a",
        ),
        span: Span {
            start: 188,
            end: 191,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedT
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
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
        "message": "Duplicate identifier 'y1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 74,
        "length": 2,
        "line": 4,
        "character": 5
      },
      {
        "code": 2393,
        "category": "Error",
        "message": "Duplicate function implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 10
      },
      {
        "code": 2393,
        "category": "Error",
        "message": "Duplicate function implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 158,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y2a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 188,
        "length": 3,
        "line": 10,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y2a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 213,
        "length": 3,
        "line": 11,
        "character": 5
      },
      {
        "code": 2814,
        "category": "Error",
        "message": "Function with bodies can only merge with classes that are ambient.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 274,
        "length": 2,
        "line": 14,
        "character": 10
      },
      {
        "code": 2813,
        "category": "Error",
        "message": "Class declaration cannot implement overload list for 'y3'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 299,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2814,
        "category": "Error",
        "message": "Function with bodies can only merge with classes that are ambient.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 327,
        "length": 3,
        "line": 17,
        "character": 10
      },
      {
        "code": 2813,
        "category": "Error",
        "message": "Class declaration cannot implement overload list for 'y3a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 353,
        "length": 3,
        "line": 18,
        "character": 7
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 422,
        "length": 2,
        "line": 21,
        "character": 10
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 446,
        "length": 2,
        "line": 22,
        "character": 6
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 10,
        "name": "y1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 74,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "y1"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 10,
        "name": "y2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 158,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "y2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 188,
        "length": 3,
        "line": 10,
        "character": 10,
        "name": "y2a"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 213,
        "length": 3,
        "line": 11,
        "character": 5,
        "name": "y2a"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 274,
        "length": 2,
        "line": 14,
        "character": 10,
        "name": "y3"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 327,
        "length": 3,
        "line": 17,
        "character": 10,
        "name": "y3a"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 422,
        "length": 2,
        "line": 21,
        "character": 10,
        "name": "y4"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 512,
        "length": 2,
        "line": 25,
        "character": 10,
        "name": "y5"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 590,
        "length": 3,
        "line": 28,
        "character": 10,
        "name": "y5a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 621,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 664,
        "length": 3,
        "line": 31,
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
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
