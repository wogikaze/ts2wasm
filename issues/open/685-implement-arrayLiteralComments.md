---
id: 685
title: "Implement Arrayliteralcomments"
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

Triage arrayLiteralComments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayLiteralComments` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayLiteralComments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralComments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralComments.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralComments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralComments.ts
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

- `reference/typescript/tests/cases/compiler/arrayLiteralComments.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayLiteralComments

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayLiteralComments.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralComments.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 245,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "var testArrayWithFunc = ["
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got None",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
var testArrayWithFunc = [
    // Function comment
    function() {
        let x = 1;
    },
    // String comment
    '1',
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "testArrayWithFunc",
    "line": 2,
    "column": 1,
    "initializer": "["
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 5,
    "column": 9,
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
        kind: Var,
        span: Span {
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "testArrayWithFunc",
        ),
        span: Span {
            start: 23,
            end: 40,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 73,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 94,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: String(
            "1",
        ),
        span: Span {
            start: 138,
            end: 141,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 201,
            end: 202,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 202,
            end: 203,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 204,
            end: 205,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 206,
            end: 207,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 207,
            end: 208,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 234,
            end: 235,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got None
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got None
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
        "typeText": "(string | number | (() => void) | number[] | { a: number; })[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralComments.ts",
        "start": 23,
        "length": 17,
        "line": 2,
        "character": 5,
        "name": "testArrayWithFunc"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralComments.ts",
        "start": 98,
        "length": 1,
        "line": 5,
        "character": 13,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
