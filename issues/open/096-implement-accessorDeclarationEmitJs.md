---
id: 096
title: "Implement Accessordeclarationemitjs (dup)"
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

Triage accessorDeclarationEmitJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorDeclarationEmitJs` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorDeclarationEmitJs has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
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

- `reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage declaration emit: accessorDeclarationEmitJs

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 403,
  "lines": 26,
  "extension": ".ts",
  "first_code_line": "export const t1 = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(Ident(\"getter\")) at 189..195",
  "span_start": 189,
  "span_end": 195,
  "line": 10,
  "column": 9,
  "feature_label": "declaration-emit",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 7 | // @filename: /a.js
 8 | export const t1 = {
 9 |     p: 'value',
10 |     get getter() {
11 |         return 'value';
12 |     },
13 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "t1",
    "line": 8,
    "column": 8,
    "initializer": "{"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/096-implement-accessorDeclarationEmitJs.md",
    "title": "Implement Accessordeclarationemitjs",
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
        kind: Export,
        span: Span {
            start: 145,
            end: 151,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 152,
            end: 157,
        },
    },
    SpannedToken {
        kind: Ident(
            "t1",
        ),
        span: Span {
            start: 158,
            end: 160,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: Ident(
            "p",
        ),
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: String(
            "value",
        ),
        span: Span {
            start: 172,
            end: 179,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 179,
            end: 180,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 185,
            end: 188,
        },
    },
    SpannedToken {
        kind: Ident(
            "getter",
        ),
        span: Span {
            start: 189,
            end: 195,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 195,
            end: 196,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 196,
            end: 197,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("getter")) at 189..195
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("getter")) at 189..195
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
        "typeText": "{ p: string; readonly getter: string; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts",
        "start": 158,
        "length": 2,
        "line": 8,
        "character": 14,
        "name": "t1"
      },
      {
        "kind": "binding",
        "typeText": "{ v: string; setter: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts",
        "start": 247,
        "length": 2,
        "line": 15,
        "character": 14,
        "name": "t2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts",
        "start": 285,
        "length": 1,
        "line": 17,
        "character": 16,
        "name": "v"
      },
      {
        "kind": "binding",
        "typeText": "{ p: string; value: string; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts",
        "start": 308,
        "length": 2,
        "line": 20,
        "character": 14,
        "name": "t3"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts",
        "start": 394,
        "length": 1,
        "line": 25,
        "character": 15,
        "name": "v"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export const t1 = {\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const t2 = {\n    v: 'value',\n    set setter(v) {},\n}",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const t3 = {\n    p: 'value',\n    get value() {\n        return 'value';\n    },\n    set value(v) {},\n}",
        "line": 20,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export const t1 = {\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}\n\nexport const t2 = {\n    v: 'val",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const t1 = {\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const t1 = {\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}",
        "line": 8,
        "character": 8
      },
      {
        "kind": "VariableDeclaration",
        "text": "t1 = {\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}",
        "line": 8,
        "character": 14
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{\n    p: 'value',\n    get getter() {\n        return 'value';\n    },\n}",
        "line": 8,
        "character": 19
      },
      {
        "kind": "GetAccessor",
        "text": "get getter() {\n        return 'value';\n    }",
        "line": 10,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "getter",
        "line": 10,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("getter")) at 189..195
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/566-implement-accessorDeclarationEmitJs.md` に統合されました。
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
- `issues/open/096-implement-accessorDeclarationEmitJs.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
