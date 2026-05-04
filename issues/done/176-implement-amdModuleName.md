---
id: 176
title: "Implement Amdmodulename (dup)"
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

Triage amdModuleName across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `amdModuleName` with diagnostics: module-system-amd. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdModuleName has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleName1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleName1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
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

- `reference/typescript/tests/cases/compiler/amdModuleName1.ts`
- `reference/typescript/tests/cases/compiler/amdModuleName2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module system amd: amdModuleName1

- Issue class: `triage-needed`
- Feature label: `module-system-amd`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/amdModuleName1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 159,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "class Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Colon) at 87..88",
  "span_start": 87,
  "span_end": 88,
  "line": 5,
  "column": 6,
  "feature_label": "module-system-amd",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | //@module: amd
3 | ///<amd-module name='NamedModule'/>
4 | class Foo {
5 |     x: number;
6 |     constructor() {
7 |         this.x = 5;
8 |     }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 4,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/176-implement-amdModuleName.md",
    "title": "Implement Amdmodulename",
    "reason": "same reference path"
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
        kind: Class,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 76,
            end: 79,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 89,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 101,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 87..88
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 87..88
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
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    x: number;\n    constructor() {\n        this.x = 5;\n    }\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Foo;",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Foo {\n    x: number;\n    constructor() {\n        this.x = 5;\n    }\n}\nexport = Foo;\n",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    x: number;\n    constructor() {\n        this.x = 5;\n    }\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "x: number;",
        "line": 5,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "x",
        "line": 5,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 87..88
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/634-implement-amdModuleName.md` に統合されました。
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
