---
id: 178
title: "Implement Anonterface (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage anonterface across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonterface` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonterface has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonterface.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonterface.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
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

- `reference/typescript/tests/cases/compiler/anonterface.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: anonterface

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anonterface.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 221,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"M\")) at 30..31",
  "span_start": 30,
  "span_end": 31,
  "line": 2,
  "column": 12,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace M {
3 |     export class C {
4 |         m(fn:{ (n:number):string; },n2:number):string {
5 |             return fn(n2);
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/178-implement-anonterface.md",
    "title": "Implement Anonterface",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
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
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 39,
            end: 45,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 52,
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
        kind: Ident(
            "m",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 67,
            end: 69,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 30..31
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 30..31
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
        "typeText": "(n: number) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 67,
        "length": 2,
        "line": 4,
        "character": 11,
        "name": "fn"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 73,
        "length": 1,
        "line": 4,
        "character": 17,
        "name": "n"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 93,
        "length": 2,
        "line": 4,
        "character": 37,
        "name": "n2"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 169,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 196,
        "length": 1,
        "line": 11,
        "character": 14,
        "name": "n"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 208,
        "length": 11,
        "line": 11,
        "character": 26,
        "operator": "+",
        "leftType": "\"hello: \"",
        "rightType": "number"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c=new M.C();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c.m(function(n) { return \"hello: \"+n; },18);",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "M",
        "line": 2,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 30..31
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/636-implement-anonterface.md` に統合されました。
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
- `issues/done/178-implement-anonterface.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
