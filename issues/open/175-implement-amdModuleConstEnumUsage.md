---
id: 175
title: "Implement Amdmoduleconstenumusage"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage amdModuleConstEnumUsage across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `amdModuleConstEnumUsage` with diagnostics: module-system-amd. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdModuleConstEnumUsage has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts
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

- `reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module system amd: amdModuleConstEnumUsage

- Issue class: `triage-needed`
- Feature label: `module-system-amd`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 326,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "export const enum CharCode {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "const declarations require an initializer at 132..136",
  "span_start": 132,
  "span_end": 136,
  "line": 6,
  "column": 19,
  "feature_label": "module-system-amd",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @preserveConstEnums: true
4 | // @baseUrl: /proj
5 | // @filename: /proj/defs/cc.ts
6 | export const enum CharCode {
7 |     A,
8 |     B
9 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "enum",
    "line": 6,
    "column": 8
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/175-implement-amdModuleConstEnumUsage.md",
    "title": "Implement Amdmoduleconstenumusage",
    "reason": "same reference path, title overlap"
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
        kind: Export,
        span: Span {
            start: 119,
            end: 125,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 126,
            end: 131,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 132,
            end: 136,
        },
    },
    SpannedToken {
        kind: Ident(
            "CharCode",
        ),
        span: Span {
            start: 137,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 208,
            end: 214,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 215,
            end: 216,
        },
    },
    SpannedToken {
        kind: Ident(
            "CharCode",
        ),
        span: Span {
            start: 217,
            end: 225,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 226,
            end: 227,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] const declarations require an initializer at 132..136
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] const declarations require an initializer at 132..136
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
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'CharCode' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts",
        "start": 137,
        "length": 8,
        "line": 6,
        "character": 19
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'CharCode' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts",
        "start": 217,
        "length": 8,
        "line": 12,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'defs/cc' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts",
        "start": 233,
        "length": 9,
        "line": 12,
        "character": 26
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdModuleConstEnumUsage.ts",
        "start": 277,
        "length": 5,
        "line": 14,
        "character": 12,
        "name": "input"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "export const enum CharCode {\r\n    A,\r\n    B\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { CharCode } from 'defs/cc';",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class User {\r\n    method(input: number) {\r\n        if (CharCode.A === input) {}\r\n    }\r\n}",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export const enum CharCode {\r\n    A,\r\n    B\r\n}\r\n// @filename: /proj/component/file.ts\r\n\r\nimport { CharCode } from 'defs/",
        "line": 6,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "export const enum CharCode {\r\n    A,\r\n    B\r\n}",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] const declarations require an initializer at 132..136
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
