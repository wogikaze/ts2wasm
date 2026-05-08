---
id: 128
title: "Implement Aliasusedasnamevalue (dup)"
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

Triage aliasUsedAsNameValue across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasUsedAsNameValue` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasUsedAsNameValue has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts
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

- `reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasUsedAsNameValue

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 539,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "export var id: number;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89",
  "span_start": 83,
  "span_end": 89,
  "line": 4,
  "column": 4,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | // @Filename: aliasUsedAsNameValue_0.ts
4 | export var id: number;
5 |
6 | // @Filename: aliasUsedAsNameValue_1.ts
7 | export function b(a: any): any { return null; }
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
    "path": "issues/open/128-implement-aliasUsedAsNameValue.md",
    "title": "Implement Aliasusedasnamevalue",
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
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 90,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 94,
            end: 96,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 98,
            end: 104,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 150,
            end: 156,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 157,
            end: 165,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 171,
            end: 174,
        },
    },
    Spanne
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
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
        "message": "Cannot find module './aliasUsedAsNameValue_0' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 365,
        "length": 26,
        "line": 12,
        "character": 22
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsedAsNameValue_1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 414,
        "length": 26,
        "line": 13,
        "character": 20
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'b' does not exist on type '(a: any) => any'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 545,
        "length": 1,
        "line": 17,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 94,
        "length": 2,
        "line": 4,
        "character": 12,
        "name": "id"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 166,
        "length": 1,
        "line": 7,
        "character": 17,
        "name": "b"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 168,
        "length": 1,
        "line": 7,
        "character": 19,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsedAsNameValue.ts",
        "start": 458,
        "length": 1,
        "line": 15,
        "character": 12,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export var id: number;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function b(a: any): any { return null; }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import mod = require(\"./aliasUsedAsNameValue_0\");",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import b = require(\"./aliasUsedAsNameValue_1\");",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var a = function () {\r\n    //var x = mod.id; // TODO needed hack that mod is loaded\r\n    b.b(mod);\r\n}",
        "line": 15,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export var id: number;\r\n\r\n// @Filename: aliasUsedAsNameValue_1.ts\r\nexport function b(a: any): any { return null; }\r\n\r\n//",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var id: number;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/593-implement-aliasUsedAsNameValue.md` に統合されました。
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
- `issues/open/128-implement-aliasUsedAsNameValue.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
