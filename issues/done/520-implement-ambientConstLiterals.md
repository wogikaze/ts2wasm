---
id: 520
title: "Implement Ambientconstliterals (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage ambientConstLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientConstLiterals` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

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
mise run reference-coverage -- tsc --limit 2
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

- `issues/done/144-implement-ambientConstLiterals.md` - Implement Ambientconstliterals (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: ambientConstLiterals

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
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
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 91..95",
  "span_start": 91,
  "span_end": 95,
  "line": 8,
  "column": 8,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
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
    "path": "issues/done/144-implement-ambientConstLiterals.md",
    "title": "Implement Ambientconstliterals",
    "reason": "same reference path, title overlap"
  },
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
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
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
  },
  {
    "state": "done",
    "path": "issues/done/246-implement-optional-chaining-parser-support.md",
    "title": "Implement optional chaining parser support",
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
        kind: Return,
        span: Span {
            start: 75,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 91,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 91..95
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 91..95
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
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 91..95
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

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/520-implement-ambientConstLiterals.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
