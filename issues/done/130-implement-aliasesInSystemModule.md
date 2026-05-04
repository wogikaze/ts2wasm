---
id: 130
title: "Implement Aliasesinsystemmodule (dup)"
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

Triage aliasesInSystemModule across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `aliasesInSystemModule` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasesInSystemModule has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts
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

- `reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts`
- `reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasesInSystemModule1

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 350,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "import alias = require('foo');"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 65..71",
  "span_start": 65,
  "span_end": 71,
  "line": 5,
  "column": 1,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @module: system
3 | // @isolatedModules: true
4 |
5 | import alias = require('foo');
6 | import cls = alias.Class;
7 | export import cls2 = alias.Class;
8 |
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
    "path": "issues/done/130-implement-aliasesInSystemModule.md",
    "title": "Implement Aliasesinsystemmodule",
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
        kind: Import,
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 72,
            end: 77,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 80,
            end: 87,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: String(
            "foo",
        ),
        span: Span {
            start: 88,
            end: 93,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 96,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "cls",
        ),
        span: Span {
            start: 103,
            end: 106,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 109,
            end: 114,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    S
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 65..71
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 65..71
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
        "message": "Cannot find module 'foo' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 88,
        "length": 5,
        "line": 5,
        "character": 24
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 161,
        "length": 1,
        "line": 9,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 188,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 207,
        "length": 1,
        "line": 11,
        "character": 5,
        "name": "z"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 279,
        "length": 1,
        "line": 15,
        "character": 7,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 308,
        "length": 1,
        "line": 16,
        "character": 7,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts",
        "start": 330,
        "length": 1,
        "line": 17,
        "character": 7,
        "name": "z"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import alias = require('foo');",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import cls = alias.Class;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "export import cls2 = alias.Class;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let x = new alias.Class();",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let y = new cls();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let z = new cls2();",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n  export import cls = alias.Class;\n  let x = new alias.Class();\n  let y = new cls(); \n  let z = new cls2()",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import alias = require('foo');\nimport cls = alias.Class;\nexport import cls2 = alias.Class;\n\nlet x = new alias.Class();\nl",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import alias = require('foo');",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 65..71
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/595-implement-aliasesInSystemModule.md` に統合されました。
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
