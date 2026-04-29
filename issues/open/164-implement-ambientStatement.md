---
id: 164
title: "Implement Ambientstatement"
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

Triage ambientStatement across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientStatement` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientStatement has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientStatement1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientStatement1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientStatement1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientStatement1.ts
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

- `reference/typescript/tests/cases/compiler/ambientStatement1.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientStatement1

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientStatement1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientStatement1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 108,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "declare namespace M1 {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"namespace\")) at 32..41",
  "span_start": 32,
  "span_end": 41,
  "line": 2,
  "column": 14,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 |     declare namespace M1 {
3 |      while(true);
4 |
5 |      export var v1 = () => false;
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
    "path": "issues/open/164-implement-ambientStatement.md",
    "title": "Implement Ambientstatement",
    "reason": "same reference path"
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
            start: 24,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 32,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "M1",
        ),
        span: Span {
            start: 42,
            end: 44,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: While,
        span: Span {
            start: 53,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 59,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 78,
            end: 84,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "v1",
        ),
        span: Span {
            start: 89,
            end: 91,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: LeftParen,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 32..41
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 32..41
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
        "code": 1036,
        "category": "Error",
        "message": "Statements are not allowed in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientStatement1.ts",
        "start": 53,
        "length": 5,
        "line": 3,
        "character": 6
      },
      {
        "code": 1039,
        "category": "Error",
        "message": "Initializers are not allowed in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientStatement1.ts",
        "start": 94,
        "length": 11,
        "line": 5,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientStatement1.ts",
        "start": 89,
        "length": 2,
        "line": 5,
        "character": 17,
        "name": "v1"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M1 {\r\n    \twhile(true);\r\n    \r\n    \texport var v1 = () => false;\r\n    }",
        "line": 2,
        "character": 5
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace M1 {\r\n    \twhile(true);\r\n    \r\n    \texport var v1 = () => false;\r\n    }",
        "line": 2,
        "character": 5
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M1 {\r\n    \twhile(true);\r\n    \r\n    \texport var v1 = () => false;\r\n    }",
        "line": 2,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 32..41
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
