---
id: 719
title: "Implement Assigntoinvalidlhs"
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

Triage assignToInvalidLHS across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignToInvalidLHS` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignToInvalidLHS has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts
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

- `reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignToInvalidLHS

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 201,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "declare var y:any;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Equal) at 201..202",
  "span_start": 201,
  "span_end": 202,
  "line": 5,
  "column": 19,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | declare var y:any;
3 | 
4 | // Below is actually valid JavaScript (see http://es5.github.com/#x8.7 ), even though will always fail at runtime with 'invalid left-hand side'
5 | var x = new y = 5;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "y",
    "line": 2,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 5,
    "column": 1,
    "initializer": "new y = 5"
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
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 28,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
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
            "any",
        ),
        span: Span {
            start: 34,
            end: 37,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 187,
            end: 190,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 193,
            end: 194,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 195,
            end: 198,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 201,
            end: 202,
        },
    },
    SpannedToken {
        kind: Number(
            5,
        ),
        span: Span {
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 204,
            end: 205,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Equal) at 201..202
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Equal) at 201..202
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
        "code": 2364,
        "category": "Error",
        "message": "The left-hand side of an assignment expression must be a variable or a property access.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts",
        "start": 195,
        "length": 5,
        "line": 5,
        "character": 9
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts",
        "start": 32,
        "length": 1,
        "line": 2,
        "character": 13,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToInvalidLHS.ts",
        "start": 191,
        "length": 1,
        "line": 5,
        "character": 5,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare var y:any;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = new y = 5;",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare var y:any;\r\n\r\n// Below is actually valid JavaScript (see http://es5.github.com/#x8.7 ), even though will always ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = new y = 5;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var x = new y = 5",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "x = new y = 5",
        "line": 5,
        "character": 5
      },
      {
        "kind": "BinaryExpression",
        "text": "new y = 5",
        "line": 5,
        "character": 9
      },
      {
        "kind": "FirstAssignment",
        "text": "=",
        "line": 5,
        "character": 15
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Equal) at 201..202
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
