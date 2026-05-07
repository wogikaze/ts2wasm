---
id: 748
title: "Implement Asyncarrowinclasses"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncArrowInClassES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncArrowInClassES` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncArrowInClassES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts
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

- `reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asyncArrowInClassES5

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 207,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "class Test {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Equal) at 184..185",
  "span_start": 184,
  "span_end": 185,
  "line": 8,
  "column": 26,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
5 | // Should capture `this`
6 | 
7 | class Test {
8 |     static member = async (x: string) => { };
9 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Test",
    "line": 7,
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
        kind: Class,
        span: Span {
            start: 152,
            end: 157,
        },
    },
    SpannedToken {
        kind: Ident(
            "Test",
        ),
        span: Span {
            start: 158,
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
        kind: Static,
        span: Span {
            start: 170,
            end: 176,
        },
    },
    SpannedToken {
        kind: Ident(
            "member",
        ),
        span: Span {
            start: 177,
            end: 183,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 184,
            end: 185,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 186,
            end: 191,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 193,
            end: 194,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 194,
            end: 195,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 196,
            end: 202,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 202,
            end: 203,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 204,
            end: 206,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 207,
            end: 208,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 209,
            end: 210,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 210,
            end: 211,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 213,
            end: 214,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 184..185
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 184..185
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
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncArrowInClassES5.ts",
        "start": 193,
        "length": 1,
        "line": 8,
        "character": 28,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Test {\r\n    static member = async (x: string) => { };\r\n}",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Test {\r\n    static member = async (x: string) => { };\r\n}\r\n",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Test {\r\n    static member = async (x: string) => { };\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "static member = async (x: string) => { };",
        "line": 8,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 184..185
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
