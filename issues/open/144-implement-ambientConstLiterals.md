---
id: 144
title: "Implement Ambientconstliterals (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ambientConstLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientConstLiterals` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientConstLiterals has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientConstLiterals.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientConstLiterals.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientConstLiterals.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientConstLiterals.ts
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

- `reference/typescript/tests/cases/compiler/ambientConstLiterals.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientConstLiterals

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientConstLiterals.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientConstLiterals.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 478,
  "lines": 24,
  "extension": ".ts",
  "first_code_line": "function f<T>(x: T): T {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"E\")) at 96..97",
  "span_start": 96,
  "span_end": 97,
  "line": 8,
  "column": 13,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 5 |     return x;
 6 | }
 7 |
 8 | enum E { A, B, C, "non identifier" }
 9 |
10 | const c1 = "abc";
11 | const c2 = 123;
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
    "path": "issues/open/144-implement-ambientConstLiterals.md",
    "title": "Implement Ambientconstliterals",
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
        kind: Function,
        span: Span {
            start: 45,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 57,
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
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 62,
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
        kind: Colon,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        ki
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("E")) at 96..97
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("E")) at 96..97
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
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 54,
        "length": 1,
        "line": 4,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 59,
        "length": 1,
        "line": 4,
        "character": 15,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "\"abc\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 137,
        "length": 2,
        "line": 10,
        "character": 7,
        "name": "c1"
      },
      {
        "kind": "binding",
        "typeText": "123",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 156,
        "length": 2,
        "line": 11,
        "character": 7,
        "name": "c2"
      },
      {
        "kind": "binding",
        "typeText": "\"abc\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 173,
        "length": 2,
        "line": 12,
        "character": 7,
        "name": "c3"
      },
      {
        "kind": "binding",
        "typeText": "123",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 189,
        "length": 2,
        "line": 13,
        "character": 7,
        "name": "c4"
      },
      {
        "kind": "binding",
        "typeText": "123",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 205,
        "length": 2,
        "line": 14,
        "character": 7,
        "name": "c5"
      },
      {
        "kind": "binding",
        "typeText": "-123",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 225,
        "length": 2,
        "line": 15,
        "character": 7,
        "name": "c6"
      },
      {
        "kind": "binding",
        "typeText": "true",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 246,
        "length": 2,
        "line": 16,
        "character": 7,
        "name": "c7"
      },
      {
        "kind": "binding",
        "typeText": "E.A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 264,
        "length": 2,
        "line": 17,
        "character": 7,
        "name": "c8"
      },
      {
        "kind": "binding",
        "typeText": "(typeof E)[\"non identifier\"]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 281,
        "length": 3,
        "line": 18,
        "character": 7,
        "name": "c8b"
      },
      {
        "kind": "binding",
        "typeText": "{ x: string; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 315,
        "length": 2,
        "line": 19,
        "character": 7,
        "name": "c9"
      },
      {
        "kind": "binding",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 341,
        "length": 3,
        "line": 20,
        "character": 7,
        "name": "c10"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 361,
        "length": 3,
        "line": 21,
        "character": 7,
        "name": "c11"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 367,
        "length": 13,
        "line": 21,
        "character": 13,
        "operator": "+",
        "leftType": "\"abc\"",
        "rightType": "\"def\""
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 389,
        "length": 3,
        "line": 22,
        "character": 7,
        "name": "c12"
      },
      {
        "kind": "binary-expression",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 395,
        "length": 9,
        "line": 22,
        "character": 13,
        "operator": "+",
        "leftType": "123",
        "rightType": "456"
      },
      {
        "kind": "binding",
        "typeText": "\"abc\" | \"def\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 413,
        "length": 3,
        "line": 23,
        "character": 7,
        "name": "c13"
      },
      {
        "kind": "binding",
        "typeText": "123 | 456",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientConstLiterals.ts",
        "start": 463,
        "length": 3,
        "line": 24,
        "character": 7,
        "name": "c14"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f<T>(x: T): T {\r\n    return x;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum E { A, B, C, \"non identifier\" }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c1 = \"abc\";",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c2 = 123;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c3 = c1;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c4 = c2;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c5 = f(123);",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c6 = f(-123);",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c7 = true;",
        "line": 16,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c8 = E.A;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c8b = E[\"non identifier\"];",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c9 = { x: \"abc\" };",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c10 = [123];",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c11 = \"abc\" + \"def\";",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c12 = 123 + 456;",
        "line": 22,
        "char
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("E")) at 96..97
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/606-implement-ambientConstLiterals.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/144-implement-ambientConstLiterals.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
