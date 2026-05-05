---
id: 136
title: "Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (dup)"
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

Triage allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage import export: allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 319,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "interface Color {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 203..209",
  "span_start": 203,
  "span_end": 209,
  "line": 11,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 8 | export default Color;
 9 | // @filename: file1.ts
10 | import Color from "./color";
11 | export declare function styled(): Color;
12 | // @filename: file2.ts
13 | import { styled }  from "./file1";
14 | export const A = styled();
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "./color",
    "line": 10,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md",
    "title": "Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration",
    "reason": "same reference path, title overlap"
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
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 88,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 98,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 114,
            end: 120,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 126,
            end: 132,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 133,
            end: 140,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 141,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 173,
            end: 179,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 203..209
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 203..209
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
        "message": "Cannot find module './color' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 191,
        "length": 9,
        "line": 10,
        "character": 19
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './file1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 293,
        "length": 9,
        "line": 13,
        "character": 25
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "Color",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 227,
        "length": 6,
        "line": 11,
        "character": 25,
        "name": "styled"
      },
      {
        "kind": "binding",
        "typeText": "Color",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 318,
        "length": 1,
        "line": 14,
        "character": 14,
        "name": "A"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Color {\r\n    c: string;\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export default Color;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import Color from \"./color\";",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export declare function styled(): Color;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { styled }  from \"./file1\";",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const A = styled();",
        "line": 14,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Color {\r\n    c: string;\r\n}\r\nexport default Color;\r\n// @filename: file1.ts\r\nimport Color from \"./color\";\r\nexpor",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export declare function styled(): Color;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 11,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 203..209
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/601-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` に統合されました。
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
- `issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
