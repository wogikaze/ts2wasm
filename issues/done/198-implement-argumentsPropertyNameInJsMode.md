---
id: 198
title: "Implement Argumentspropertynameinjsmode (dup)"
type: spike
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage argumentsPropertyNameInJsMode across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsPropertyNameInJsMode` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsPropertyNameInJsMode has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
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

- `reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage arguments object: argumentsPropertyNameInJsMode1

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 238,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "const foo = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Function) at 173..181",
  "span_start": 173,
  "span_end": 181,
  "line": 13,
  "column": 1,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
10 |    f1: (params) => { }
11 | }
12 |
13 | function f2(x) {
14 |   foo.f1({ x, arguments: [] });
15 | }
16 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "foo",
    "line": 9,
    "column": 1,
    "initializer": "{"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/198-implement-argumentsPropertyNameInJsMode.md",
    "title": "Implement Argumentspropertynameinjsmode",
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
        kind: Const,
        span: Span {
            start: 133,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Ident(
            "f1",
        ),
        span: Span {
            start: 150,
            end: 152,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Ident(
            "params",
        ),
        span: Span {
            start: 155,
            end: 161,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 163,
            end: 165,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Function
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
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
        "code": 2554,
        "category": "Error",
        "message": "Expected 1 arguments, but got 3.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 231,
        "length": 4,
        "line": 17,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ f1: (params: any) => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 139,
        "length": 3,
        "line": 9,
        "character": 7,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 155,
        "length": 6,
        "line": 10,
        "character": 9,
        "name": "params"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 182,
        "length": 2,
        "line": 13,
        "character": 10,
        "name": "f2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 185,
        "length": 1,
        "line": 13,
        "character": 13,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const foo = {\n   f1: (params) => { }\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f2(x) {\n  foo.f1({ x, arguments: [] });\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f2(1, 2, 3);",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const foo = {\n   f1: (params) => { }\n}\n\nfunction f2(x) {\n  foo.f1({ x, arguments: [] });\n}\n\nf2(1, 2, 3);\n",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f2(x) {\n  foo.f1({ x, arguments: [] });\n}",
        "line": 13,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/652-implement-argumentsPropertyNameInJsMode.md` に統合されました。
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
- `issues/done/198-implement-argumentsPropertyNameInJsMode.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
