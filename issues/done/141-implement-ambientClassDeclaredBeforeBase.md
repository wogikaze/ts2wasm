---
id: 141
title: "Implement Ambientclassdeclaredbeforebase (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5003]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ambientClassDeclaredBeforeBase across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientClassDeclaredBeforeBase` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientClassDeclaredBeforeBase has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
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

- `reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage declaration emit: ambientClassDeclaredBeforeBase

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 123,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "declare namespace ns {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"namespace\")) at 52..61",
  "span_start": 52,
  "span_end": 61,
  "line": 4,
  "column": 12,
  "feature_label": "declaration-emit",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @filename: a.d.ts
3 |
4 | declare namespace ns {
5 |   class SecondNS extends FirstNS { }
6 |   class FirstNS { }
7 | }
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
    "path": "issues/done/141-implement-ambientClassDeclaredBeforeBase.md",
    "title": "Implement Ambientclassdeclaredbeforebase",
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
            start: 44,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 52,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "ns",
        ),
        span: Span {
            start: 62,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "SecondNS",
        ),
        span: Span {
            start: 76,
            end: 84,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 85,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "FirstNS",
        ),
        span: Span {
            start: 93,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 108,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "FirstNS",
        ),
        span: Span {
            start: 114,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 52..61
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 52..61
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}\r\n",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 52..61
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/605-implement-ambientClassDeclaredBeforeBase.md` に統合されました。
そちらを参照してください。
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
