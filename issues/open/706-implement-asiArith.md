---
id: 706
title: "Implement Asiarith"
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

Triage asiArith across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiArith` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiArith has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiArith.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiArith.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiArith.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiArith.ts
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

- `reference/typescript/tests/cases/compiler/asiArith.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asiArith

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asiArith.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiArith.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 116,
  "lines": 34,
  "extension": ".ts",
  "first_code_line": "var x = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Var) at 86..89",
  "span_start": 86,
  "span_end": 89,
  "line": 21,
  "column": 7,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
18 | 
19 | var a = 1;
20 | 
21 | var b = 1;
22 | 
23 | var c =
24 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "y",
    "line": 4,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "z",
    "line": 6,
    "column": 1,
    "initializer": "x"
  },
  {
    "kind": "binding",
    "name": "a",
    "line": 19,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "b",
    "line": 21,
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
        kind: Var,
        span: Span {
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 24,
            end: 25,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 34,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 48,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "z",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 86,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    S
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 86..89
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 86..89
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 38,
        "length": 1,
        "line": 4,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 52,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "z"
      },
      {
        "kind": "binary-expression",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 59,
        "length": 21,
        "line": 8,
        "character": 1,
        "operator": "+",
        "leftType": "number",
        "rightType": "number",
        "candidate": "number-add-fast-path"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 90,
        "length": 1,
        "line": 19,
        "character": 5,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 104,
        "length": 1,
        "line": 21,
        "character": 5,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiArith.ts",
        "start": 118,
        "length": 1,
        "line": 23,
        "character": 5,
        "name": "c"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var x = 1;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y = 1;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var z =\r\n\r\nx\r\n\r\n+\r\n\r\n+\r\n\r\n+\r\n\r\ny",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var a = 1;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var b = 1;",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c =\r\n\r\nx\r\n\r\n-\r\n\r\n-\r\n\r\n-\r\n\r\ny",
        "line": 23,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var x = 1;\r\n\r\nvar y = 1;\r\n\r\nvar z =\r\n\r\nx\r\n\r\n+\r\n\r\n+\r\n\r\n+\r\n\r\ny\r\n\r\n\r\nvar a = 1;\r\n\r\nvar b = 1;\r\n\r\nvar c =\r\n\r\nx\r\n\r\n-\r\n\r\n-\r\n\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var a = 1;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var a = 1",
        "line": 19,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 86..89
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
