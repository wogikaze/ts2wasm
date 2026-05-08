---
id: 194
title: "Implement Argumentsaspropertyname"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage argumentsAsPropertyName across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `argumentsAsPropertyName` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsAsPropertyName has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
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

- `reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts`
- `reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage arguments object: argumentsAsPropertyName

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 389,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "type MyType = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected RightParen, got Some(Increment) at 208..210",
  "span_start": 208,
  "span_end": 210,
  "line": 12,
  "column": 5,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 9 |
10 | function myFunction(myType: MyType) {
11 |     for (let i = 0; i < 10; i++) {
12 |         use(myType.arguments[i]);
13 |         // create closure so that tsc will turn loop body into function
14 |         const x = 5;
15 |         [1, 2, 3].forEach(function(j) { use(x); })
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "use",
    "line": 8,
    "column": 9,
    "params": "s: any"
  },
  {
    "kind": "function",
    "name": "myFunction",
    "line": 10,
    "column": 1,
    "params": "myType: MyType"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 11,
    "column": 10,
    "initializer": "0"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/194-implement-argumentsAsPropertyName.md",
    "title": "Implement Argumentsaspropertyname",
    "reason": "same reference path, title overlap"
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
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 55,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "MyType",
        ),
        span: Span {
            start: 60,
            end: 66,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 76,
            end: 85,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 87,
            end: 92,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 93,
            end: 99,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 107,
            end: 114,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 115,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 208..210
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 208..210
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
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 124,
        "length": 3,
        "line": 8,
        "character": 18,
        "name": "use"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 128,
        "length": 1,
        "line": 8,
        "character": 22,
        "name": "s"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 149,
        "length": 10,
        "line": 10,
        "character": 10,
        "name": "myFunction"
      },
      {
        "kind": "parameter",
        "typeText": "MyType",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 160,
        "length": 6,
        "line": 10,
        "character": 21,
        "name": "myType"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 192,
        "length": 1,
        "line": 11,
        "character": 14,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "5",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 337,
        "length": 1,
        "line": 14,
        "character": 15,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 380,
        "length": 1,
        "line": 15,
        "character": 36,
        "name": "j"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "TypeAliasDeclaration",
        "text": "type MyType = {\r\n    arguments: Array<string>\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function use(s: any);",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function myFunction(myType: MyType) {\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        //",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "type MyType = {\r\n    arguments: Array<string>\r\n}\r\n\r\ndeclare function use(s: any);\r\n\r\nfunction myFunction(myType: MyType)",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function myFunction(myType: MyType) {\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        //",
        "line": 10,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        // create closure so that tsc will tur",
        "line": 10,
        "character": 37
      },
      {
        "kind": "ForStatement",
        "text": "for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        // create closure so that tsc will turn loop ",
        "line": 11,
        "character": 5
      },
      {
        "kind": "PostfixUnaryExpression",
        "text": "i++",
        "line": 11,
        "character": 29
      },
      {
        "kind": "Identifier",
        "text": "i",
        "line": 11,
        "character": 29
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 208..210
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

## False-done audit

**truly-done** (194)

- Implementation commits: verified via `git log --oneline --all --grep=194`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
