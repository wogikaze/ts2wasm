---
id: 159
title: "Implement Ambientmodulewithtemplateliterals"
type: spike
area: reference
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage ambientModuleWithTemplateLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientModuleWithTemplateLiterals` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientModuleWithTemplateLiterals has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail
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

- unrelated runtime/backend code unless the triage report proves the failure is not parser/frontend

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
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

- `reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientModuleWithTemplateLiterals

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 307,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "declare namespace Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"namespace\")) at 28..37",
  "span_start": 28,
  "span_end": 37,
  "line": 2,
  "column": 10,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare namespace Foo {
3 |     enum Bar {
4 |         a = `1`,
5 |         b = '2',
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/159-implement-ambientModuleWithTemplateLiterals.md",
    "title": "Implement Ambientmodulewithtemplateliterals",
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
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 28,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 38,
            end: 41,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 49,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 54,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: TemplateLiteral(
            "1",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Equal,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 28..37
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 28..37
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
        "typeText": "\"string\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 140,
        "length": 1,
        "line": 9,
        "character": 18,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "\"template\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 172,
        "length": 1,
        "line": 10,
        "character": 18,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "Bar.a",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 208,
        "length": 1,
        "line": 12,
        "character": 18,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "Bar.b",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 237,
        "length": 1,
        "line": 13,
        "character": 18,
        "name": "d"
      },
      {
        "kind": "binding",
        "typeText": "Bar.c",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 269,
        "length": 1,
        "line": 14,
        "character": 18,
        "name": "e"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.a;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.b;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.c;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.d;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.e;",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 28..37
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
