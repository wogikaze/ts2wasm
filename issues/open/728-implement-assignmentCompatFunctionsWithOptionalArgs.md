---
id: 728
title: "Implement Assignmentcompatfunctionswithoptionalargs"
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

Triage assignmentCompatFunctionsWithOptionalArgs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatFunctionsWithOptionalArgs` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatFunctionsWithOptionalArgs has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatFunctionsWithOptionalArgs

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 288,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "function foo(x: { id: number; name?: string; }): void;"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "unterminated TypeScript type annotation at 247..248",
  "span_start": 247,
  "span_end": 248,
  "line": 6,
  "column": 28,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | foo({ id: 1234 });                 // Ok
4 | foo({ id: 1234, name: "hello" });  // Ok
5 | foo({ id: 1234, name: false });    // Error, name of wrong type
6 | foo({ name: "hello" });            // Error, id required but missing
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "foo",
    "line": 2,
    "column": 1,
    "params": "x: { id: number; name?: string; }"
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

- Create a child issue around this exact path and diagnostic before broadening the reference window.

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
            "foo",
        ),
        span: Span {
            start: 29,
            end: 32,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 38,
            end: 40,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 50,
            end: 54,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 57,
            end: 63,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 69,
            end: 73,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 76,
            end: 79,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 82,
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
        kind: Number(
            1234,
        ),
        span: Span {
            start: 86,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] unterminated TypeScript type annotation at 247..248
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] unterminated TypeScript type annotation at 247..248
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
        "code": 2391,
        "category": "Error",
        "message": "Function implementation is missing or not immediately following the declaration.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts",
        "start": 29,
        "length": 3,
        "line": 2,
        "character": 10
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'boolean' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts",
        "start": 176,
        "length": 4,
        "line": 5,
        "character": 17
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type '{ name: string; }' is not assignable to parameter of type '{ id: number; name?: string | undefined; }'.\n  Property 'id' is missing in type '{ name: string; }' but required in type '{ id: number; name?: string | undefined; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts",
        "start": 229,
        "length": 17,
        "line": 6,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts",
        "start": 29,
        "length": 3,
        "line": 2,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "{ id: number; name?: string | undefined; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatFunctionsWithOptionalArgs.ts",
        "start": 33,
        "length": 1,
        "line": 2,
        "character": 14,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo(x: { id: number; name?: string; }): void;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo({ id: 1234 });",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo({ id: 1234, name: \"hello\" });",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo({ id: 1234, name: false });",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo({ name: \"hello\" });",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo(x: { id: number; name?: string; }): void;\r\nfoo({ id: 1234 });                 // Ok\r\nfoo({ id: 1234, name: ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo({ name: \"hello\" });",
        "line": 6,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "foo({ name: \"hello\" })",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] unterminated TypeScript type annotation at 247..248
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
