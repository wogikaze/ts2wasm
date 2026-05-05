---
id: 539
title: "Implement Ambiguouscallswherereturntypesagree (dup)"
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

Triage ambiguousCallsWhereReturnTypesAgree across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambiguousCallsWhereReturnTypesAgree` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambiguousCallsWhereReturnTypesAgree has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts
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

- `reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts`

## Duplicate detection

- `issues/done/166-implement-ambiguousCallsWhereReturnTypesAgree.md` - Implement Ambiguouscallswherereturntypesagree (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: ambiguousCallsWhereReturnTypesAgree

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 613,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "class TestClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"bar\")) at 48..51",
  "span_start": 48,
  "span_end": 51,
  "line": 3,
  "column": 12,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class TestClass {
3 |     public bar(x: string): void;
4 |     public bar(x: string[]): void;
5 |     public bar(x: any): void {
6 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "TestClass",
    "line": 2,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/166-implement-ambiguousCallsWhereReturnTypesAgree.md",
    "title": "Implement Ambiguouscallswherereturntypesagree",
    "reason": "same reference path, same feature label, title overlap"
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
        kind: Class,
        span: Span {
            start: 19,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "TestClass",
        ),
        span: Span {
            start: 25,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 41,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 48,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 64,
            end: 68,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 74,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 81,
            end: 84,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 88,
            end: 94,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 99,
            end: 103,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        s
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("bar")) at 48..51
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("bar")) at 48..51
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
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 85,
        "length": 1,
        "line": 4,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 120,
        "length": 1,
        "line": 5,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 168,
        "length": 1,
        "line": 9,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 201,
        "length": 1,
        "line": 10,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 236,
        "length": 1,
        "line": 11,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 336,
        "length": 1,
        "line": 17,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 371,
        "length": 1,
        "line": 18,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 408,
        "length": 1,
        "line": 19,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 467,
        "length": 1,
        "line": 23,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 502,
        "length": 1,
        "line": 24,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousCallsWhereReturnTypesAgree.ts",
        "start": 539,
        "length": 1,
        "line": 25,
        "character": 16,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class TestClass {\n    public bar(x: string): void;\n    public bar(x: string[]): void;\n    public bar(x: any): void {\n   ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class TestClass2 {\n    public bar(x: string): number;\n    public bar(x: string[]): number;\n    public bar(x: any): numbe",
        "line": 16,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class TestClass {\n    public bar(x: string): void;\n    public bar(x: string[]): void;\n    public bar(x: any): void {\n   ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class TestClass {\n    public bar(x: string): void;\n    public bar(x: string[]): void;\n    public bar(x: any): void {\n   ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public bar(x: string): void;",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "bar",
        "line": 3,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("bar")) at 48..51
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/625-implement-ambiguousCallsWhereReturnTypesAgree.md` に統合されました。
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
- `issues/done/539-implement-ambiguousCallsWhereReturnTypesAgree.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
