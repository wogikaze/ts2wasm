---
id: 148
title: "Implement Ambientexportdefaulterrors (dup)"
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

Triage ambientExportDefaultErrors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExportDefaultErrors` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExportDefaultErrors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts
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

- `reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientExportDefaultErrors

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 665,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "export default 2 + 2;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 131..137",
  "span_start": 131,
  "span_end": 137,
  "line": 7,
  "column": 6,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 | // @noUncheckedSideEffectImports: false
 5 |
 6 | export default 2 + 2;
 7 | export as namespace Foo;
 8 |
 9 | // @filename: foo2.d.ts
10 | export = 2 + 2;
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
    "path": "issues/done/148-implement-ambientExportDefaultErrors.md",
    "title": "Implement Ambientexportdefaulterrors",
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
            start: 108,
            end: 114,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 115,
            end: 122,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 131,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "as",
        ),
        span: Span {
            start: 138,
            end: 140,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 141,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 151,
            end: 154,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 184,
            end: 190,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 191,
            end: 192,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 131..137
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 131..137
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
        "code": 1315,
        "category": "Error",
        "message": "Global module exports may only appear in declaration files.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 131,
        "length": 24,
        "line": 7,
        "character": 1
      },
      {
        "code": 2309,
        "category": "Error",
        "message": "An export assignment cannot be used in a module with other exported elements.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 184,
        "length": 15,
        "line": 10,
        "character": 1
      },
      {
        "code": 1315,
        "category": "Error",
        "message": "Global module exports may only appear in declaration files.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 201,
        "length": 25,
        "line": 11,
        "character": 1
      },
      {
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'indirect' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 314,
        "length": 10,
        "line": 15,
        "character": 16
      },
      {
        "code": 2714,
        "category": "Error",
        "message": "The expression of an export assignment must be an identifier or qualified name in an ambient context.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 347,
        "length": 18,
        "line": 16,
        "character": 20
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'Foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 354,
        "length": 3,
        "line": 16,
        "character": 27
      },
      {
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'indirect2' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 459,
        "length": 11,
        "line": 21,
        "character": 16
      },
      {
        "code": 2714,
        "category": "Error",
        "message": "The expression of an export assignment must be an identifier or qualified name in an ambient context.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 487,
        "length": 11,
        "line": 22,
        "character": 14
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'Foo2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 494,
        "length": 4,
        "line": 22,
        "character": 21
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'indirect'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 631,
        "length": 10,
        "line": 28,
        "character": 8
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 651,
        "length": 5,
        "line": 29,
        "character": 8
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'indirect2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 666,
        "length": 11,
        "line": 30,
        "character": 8
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'foo2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 687,
        "length": 6,
        "line": 31,
        "character": 8
      }
    ],
    "hints": [
      {
        "kind": "binary-expression",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 123,
        "length": 5,
        "line": 6,
        "character": 16,
        "operator": "+",
        "leftType": "2",
        "rightType": "2"
      },
      {
        "kind": "binary-expression",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExportDefaultErrors.ts",
        "start": 193,
        "length": 5,
        "line": 10,
        "character": 10,
        "operator": "+",
        "leftType": "2",
        "rightType": "2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExportAssignment",
        "text": "export default 2 + 2;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "NamespaceExportDeclaration",
        "text": "export as namespace Foo;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = 2 + 2;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "NamespaceExportDeclaration",
        "text": "export as namespace Foo2;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"indirect\" {\r\n    export default typeof Foo.default;\r\n}",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"indirect2\" {\r\n    export = typeof Foo2;\r\n}",
        "line": 21,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"indirect\";",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"foo\";",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"indirect2\";",
        "line": 30,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"foo2\";",
        "line": 31,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export default 2 + 2;\r\nexport as namespace Foo;\r\n\r\n// @filename: foo2.d.ts\r\nexport = 2 + 2;\r\nexport as namespace Foo2;\r\n",
        "line": 6,
        "character": 1
      },
      {
        "kind": "NamespaceExportDeclaration",
        "text": "export as namespace Foo;",
        "line": 7,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 131..137
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/609-implement-ambientExportDefaultErrors.md` に統合されました。
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
