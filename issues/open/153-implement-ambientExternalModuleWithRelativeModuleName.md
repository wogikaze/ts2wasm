---
id: 153
title: "Implement Ambientexternalmodulewithrelativemodulename"
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

Triage ambientExternalModuleWithRelativeModuleName across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithRelativeModuleName` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithRelativeModuleName has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientExternalModuleWithRelativeModuleName

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 134,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "declare module \"./relativeModule\" {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"module\")) at 28..34",
  "span_start": 28,
  "span_end": 34,
  "line": 2,
  "column": 10,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare module "./relativeModule" {
3 |     var x: string;
4 | }
5 |
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
    "path": "issues/open/153-implement-ambientExternalModuleWithRelativeModuleName.md",
    "title": "Implement Ambientexternalmodulewithrelativemodulename",
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
            "module",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: String(
            "./relativeModule",
        ),
        span: Span {
            start: 35,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 68,
            end: 74,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 82,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 90,
            end: 96,
        },
    },
    SpannedToken {
        kind: String(
            ".\\re
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
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
        "code": 2436,
        "category": "Error",
        "message": "Ambient module declaration cannot specify relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 35,
        "length": 18,
        "line": 2,
        "character": 16
      },
      {
        "code": 2436,
        "category": "Error",
        "message": "Ambient module declaration cannot specify relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 97,
        "length": 19,
        "line": 6,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 65,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 128,
        "length": 1,
        "line": 7,
        "character": 9,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \".\\\\relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}\r\n\r\ndeclare module \".\\\\relativeModule\" {\r\n    var x: string;\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
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
