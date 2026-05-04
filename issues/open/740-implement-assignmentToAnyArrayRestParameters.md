---
id: 740
title: "Implement Assignmenttoanyarrayrestparameters"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentToAnyArrayRestParameters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToAnyArrayRestParameters` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToAnyArrayRestParameters has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentToAnyArrayRestParameters

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 642,
  "lines": 24,
  "extension": ".ts",
  "first_code_line": "function foo<T extends string[]>("
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"T00\")) at 475..478",
  "span_start": 475,
  "span_end": 478,
  "line": 18,
  "column": 27,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 | }
16 | 
17 | function bar<T extends string[], K extends number>() {
18 |     type T00 = string[]["0"];
19 |     type T01 = string[]["0.0"];  // Error
20 |     type T02 = string[][K | "0"];
21 |     type T10 = T["0"];
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "f1",
    "line": 11,
    "column": 5
  },
  {
    "kind": "binding",
    "name": "f2",
    "line": 12,
    "column": 5
  },
  {
    "kind": "binding",
    "name": "f3",
    "line": 13,
    "column": 5
  },
  {
    "kind": "binding",
    "name": "f4",
    "line": 14,
    "column": 5
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
            start: 83,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 98,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 106,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "fa",
        ),
        span: Span {
            start: 122,
            end: 124,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 130,
            end: 136,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: DotDotDot,
        span: Span {
            start: 138,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "args",
        ),
        span: Span {
            start: 141,
            end: 145,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 147,
            end: 153,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 157,
            end: 159,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("T00")) at 475..478
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("T00")) at 475..478
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
        "code": 2339,
        "category": "Error",
        "message": "Property '0.0' does not exist on type 'string[]'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 521,
        "length": 5,
        "line": 19,
        "character": 25
      },
      {
        "code": 2536,
        "category": "Error",
        "message": "Type '\"0.0\"' cannot be used to index type 'T'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 614,
        "length": 8,
        "line": 22,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 92,
        "length": 3,
        "line": 7,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "(s: string, ...args: string[]) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 122,
        "length": 2,
        "line": 8,
        "character": 5,
        "name": "fa"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 127,
        "length": 1,
        "line": 8,
        "character": 10,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 141,
        "length": 4,
        "line": 8,
        "character": 24,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "(s: string, ...args: T) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 173,
        "length": 2,
        "line": 9,
        "character": 5,
        "name": "fb"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 178,
        "length": 1,
        "line": 9,
        "character": 10,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 192,
        "length": 4,
        "line": 9,
        "character": 24,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "(...args: any) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 227,
        "length": 2,
        "line": 11,
        "character": 11,
        "name": "f1"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 235,
        "length": 4,
        "line": 11,
        "character": 19,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "(...args: any[]) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 273,
        "length": 2,
        "line": 12,
        "character": 11,
        "name": "f2"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 281,
        "length": 4,
        "line": 12,
        "character": 19,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "(...args: any) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 321,
        "length": 2,
        "line": 13,
        "character": 11,
        "name": "f3"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 329,
        "length": 4,
        "line": 13,
        "character": 19,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "(...args: any[]) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 367,
        "length": 2,
        "line": 14,
        "character": 11,
        "name": "f4"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 375,
        "length": 4,
        "line": 14,
        "character": 19,
        "name": "args"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToAnyArrayRestParameters.ts",
        "start": 419,
        "length": 3,
        "line": 17,
        "character": 10,
        "name": "bar"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo<T extends string[]>(\r\n    fa: (s: string, ...args: string[]) => string,\r\n    fb: (s: string, ...args: T) =>",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar<T extends string[], K extends number>() {\r\n    type T00 = string[][\"0\"];\r\n    type T01 = string[][\"0.0\"];  ",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo<T extends string[]>(\r\n    fa: (s: string, ...args: string[]) => string,\r\n    fb: (s: string, ...args: T) =>",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar<T extends string[], K extends number>() {\r\n    type T00 = string[][\"0\"];\r\n    type T01 = string[][\"0.0\"];  ",
        "line": 17,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    type T00 = string[][\"0\"];\r\n    type T01 = string[][\"0.0\"];  // Error\r\n    type T02 = string[][K | \"0\"];\r\n    type",
        "line": 17,
        "character": 54
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type T00 = string[][\"0\"];",
        "line": 18,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "T00",
        "line": 18,
        "character": 10
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("T00")) at 475..478
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
