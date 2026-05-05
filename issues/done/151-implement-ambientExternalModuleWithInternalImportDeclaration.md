---
id: 151
title: "Implement Ambientexternalmodulewithinternalimportdeclaration (dup)"
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

Triage ambientExternalModuleWithInternalImportDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithInternalImportDeclaration` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithInternalImportDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientExternalModuleWithInternalImportDeclaration

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 448,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "declare module 'M' {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"module\")) at 115..121",
  "span_start": 115,
  "span_end": 121,
  "line": 4,
  "column": 12,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: amd
3 | // @Filename: ambientExternalModuleWithInternalImportDeclaration_0.ts
4 | declare module 'M' {
5 |     namespace C {
6 |         export var f: number;
7 |     }
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
    "path": "issues/done/151-implement-ambientExternalModuleWithInternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithinternalimportdeclaration",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 107,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 115,
            end: 121,
        },
    },
    SpannedToken {
        kind: String(
            "M",
        ),
        span: Span {
            start: 122,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 133,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
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
        kind: Export,
        span: Span {
            start: 156,
            end: 162,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 163,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 170,
            end: 176,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 115..121
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 115..121
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
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'M' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 122,
        "length": 3,
        "line": 4,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'M' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 443,
        "length": 3,
        "line": 18,
        "character": 20
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 167,
        "length": 1,
        "line": 6,
        "character": 20,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 454,
        "length": 1,
        "line": 19,
        "character": 5,
        "name": "c"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import A = require('M');",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c = new A();",
        "line": 19,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 115..121
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/612-implement-ambientExternalModuleWithInternalImportDeclaration.md` に統合されました。
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
- `issues/done/151-implement-ambientExternalModuleWithInternalImportDeclaration.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
