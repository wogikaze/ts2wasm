---
id: 135
title: "Implement Allowsyntheticdefaultimports (dup)"
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

Triage allowSyntheticDefaultImports across 10 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 10 cases fail in directory `allowSyntheticDefaultImports` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowSyntheticDefaultImports has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
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

- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports10.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports2.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports3.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports4.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports5.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports6.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports7.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports8.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports9.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage import export: allowSyntheticDefaultImports1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 223,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "import Namespace from \"./b\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138",
  "span_start": 132,
  "span_end": 138,
  "line": 6,
  "column": 6,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @module: commonjs
4 | // @Filename: a.ts
5 | import Namespace from "./b";
6 | export var x = new Namespace.Foo();
7 |
8 | // @Filename: b.d.ts
9 | export class Foo {
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "./b",
    "line": 5,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/135-implement-allowSyntheticDefaultImports.md",
    "title": "Implement Allowsyntheticdefaultimports",
    "reason": "same reference path"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
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
        kind: Import,
        span: Span {
            start: 102,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "Namespace",
        ),
        span: Span {
            start: 109,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 119,
            end: 123,
        },
    },
    SpannedToken {
        kind: String(
            "./b",
        ),
        span: Span {
            start: 124,
            end: 129,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 132,
            end: 138,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 147,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "Namespace",
        ),
        span: Span {
            start: 151,
            end: 160,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 161,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
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
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './b' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 124,
        "length": 5,
        "line": 5,
        "character": 23
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'member' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 214,
        "length": 6,
        "line": 10,
        "character": 2
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 143,
        "length": 1,
        "line": 6,
        "character": 12,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import Namespace from \"./b\";",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var x = new Namespace.Foo();",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Foo {\r\n\tmember: string;\r\n}",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import Namespace from \"./b\";\r\nexport var x = new Namespace.Foo();\r\n\r\n// @Filename: b.d.ts\r\nexport class Foo {\r\n\tmember: ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var x = new Namespace.Foo();",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/600-implement-allowSyntheticDefaultImports.md` に統合されました。
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
