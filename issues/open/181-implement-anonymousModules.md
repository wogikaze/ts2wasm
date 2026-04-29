---
id: 181
title: "Implement Anonymousmodules"
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

Triage anonymousModules across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonymousModules` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonymousModules has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousModules.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousModules.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousModules.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousModules.ts
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

- `reference/typescript/tests/cases/compiler/anonymousModules.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage import export: anonymousModules

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anonymousModules.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousModules.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 130,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "module {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(LeftBrace) at 27..28",
  "span_start": 27,
  "span_end": 28,
  "line": 2,
  "column": 9,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | module {
3 |  export var foo = 1;
4 |
5 |  module {
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
    "path": "issues/open/181-implement-anonymousModules.md",
    "title": "Implement Anonymousmodules",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 20,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 31,
            end: 37,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 38,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 42,
            end: 45,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 67,
            end: 73,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 74,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 78,
            end: 81,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 27..28
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 27..28
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
        "code": 2591,
        "category": "Error",
        "message": "Cannot find name 'module'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 20,
        "length": 6,
        "line": 2,
        "character": 1
      },
      {
        "code": 1437,
        "category": "Error",
        "message": "Namespace must be given a name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 27,
        "length": 1,
        "line": 2,
        "character": 8
      },
      {
        "code": 2591,
        "category": "Error",
        "message": "Cannot find name 'module'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 55,
        "length": 6,
        "line": 5,
        "character": 2
      },
      {
        "code": 1437,
        "category": "Error",
        "message": "Namespace must be given a name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 62,
        "length": 1,
        "line": 5,
        "character": 9
      },
      {
        "code": 2591,
        "category": "Error",
        "message": "Cannot find name 'module'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 112,
        "length": 6,
        "line": 11,
        "character": 2
      },
      {
        "code": 1437,
        "category": "Error",
        "message": "Namespace must be given a name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 119,
        "length": 1,
        "line": 11,
        "character": 9
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 42,
        "length": 3,
        "line": 3,
        "character": 13,
        "name": "foo"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 78,
        "length": 3,
        "line": 6,
        "character": 14,
        "name": "bar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 99,
        "length": 3,
        "line": 9,
        "character": 6,
        "name": "bar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousModules.ts",
        "start": 128,
        "length": 1,
        "line": 12,
        "character": 7,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "module",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n\texport var foo = 1;\r\n\r\n\tmodule {\r\n\t\texport var bar = 1;\r\n\t}\r\n\r\n\tvar bar = 2;\r\n\r\n\tmodule {\r\n\t\tvar x = bar;\r\n\t}\r\n}",
        "line": 2,
        "character": 8
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "module {\r\n\texport var foo = 1;\r\n\r\n\tmodule {\r\n\t\texport var bar = 1;\r\n\t}\r\n\r\n\tvar bar = 2;\r\n\r\n\tmodule {\r\n\t\tvar x = bar;\r\n\t}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n\texport var foo = 1;\r\n\r\n\tmodule {\r\n\t\texport var bar = 1;\r\n\t}\r\n\r\n\tvar bar = 2;\r\n\r\n\tmodule {\r\n\t\tvar x = bar;\r\n\t}\r\n}",
        "line": 2,
        "character": 8
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 27..28
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
