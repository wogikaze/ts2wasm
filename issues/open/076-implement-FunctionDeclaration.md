---
id: 076
title: "Implement Functiondeclaration"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage FunctionDeclaration across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `FunctionDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: FunctionDeclaration has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts
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

- `reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts`
- `reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts`
- `reference/typescript/tests/cases/compiler/FunctionDeclaration6.ts`
- `reference/typescript/tests/cases/compiler/FunctionDeclaration7.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: FunctionDeclaration3

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 34,
  "lines": 2,
  "extension": ".ts",
  "first_code_line": "function foo();"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Semicolon) at 33..34",
  "span_start": 33,
  "span_end": 34,
  "line": 2,
  "column": 15,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | function foo();
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "foo",
    "line": 2,
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
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/076-implement-FunctionDeclaration.md",
    "title": "Implement Functiondeclaration",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 19,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 28,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 33,
            end: 34,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 33..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 33..34
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts",
        "start": 28,
        "length": 3,
        "line": 2,
        "character": 10
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts",
        "start": 28,
        "length": 3,
        "line": 2,
        "character": 10,
        "name": "foo"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo();",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo();",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo();",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 33..34
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
