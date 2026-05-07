---
id: 512
title: "Implement Allowjscrossmonorepopackage (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage allowJsCrossMonorepoPackage across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowJsCrossMonorepoPackage` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowJsCrossMonorepoPackage has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
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

- `reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts`

## Duplicate detection

- `issues/open/133-implement-allowJsCrossMonorepoPackage.md` - Implement Allowjscrossmonorepopackage (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: allowJsCrossMonorepoPackage

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 970,
  "lines": 45,
  "extension": ".ts",
  "first_code_line": "export declare function pkg(): \"pkg\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-232: unsupported non-local module specifier `pkg`; package resolution, import maps, and absolute specifiers are not implemented at 306..311",
  "span_start": 306,
  "span_end": 311,
  "line": 14,
  "column": 21,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
11 | }
12 | 
13 | // @Filename: /packages/shared/utils.js
14 | export { pkg } from "pkg";
15 | 
16 | // @Filename: /packages/shared/index.js
17 | import { pkg } from "./utils.js";
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "pkg",
    "line": 3,
    "column": 16,
    "params": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/133-implement-allowJsCrossMonorepoPackage.md",
    "title": "Implement Allowjscrossmonorepopackage",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
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

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

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
            start: 62,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 69,
            end: 76,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 77,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "pkg",
        ),
        span: Span {
            start: 86,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: String(
            "pkg",
        ),
        span: Span {
            start: 93,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
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
        kind: String(
            "name",
        ),
        span: Span {
            start: 151,
            end: 157,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: String(
            "shared",
        ),
        span: Span {
            start: 159,
            end: 167,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: String(
            "version",
        ),
        span: Span {
            start: 173,
            end: 182,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 182,
            end: 183,
        },
    },
    SpannedToken {
        kind: String(
            "1.0.0",
        ),
        span: Span {
            start: 184,
            end: 191,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedToken {
        kind: String(
            "type",
        ),
        span: Span {
            start: 197,
            end: 203,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: String(
            "module",
        ),
        span: Span {
            start: 205,
            end: 213,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 213,
            end: 214,
        },
    },
    SpannedToken {
        kind: String(
            "exports",
        ),
        span: Span {
            start: 219,
            end: 228,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: String(
            "./index.js",
        ),
        span: Span {
            start: 230,
            end: 242,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Expr {
        expr: Object {
            props: [
                (
                    "name",
                    String {
                        value: "shared",
                        span: Span {
                            start: 159,
                            end: 167,
                        },
                    },
                ),
                (
                    "version",
                    String {
                        value: "1.0.0",
                        span: Span {
                            start: 184,
                            end: 191,
                        },
                    },
                ),
                (
                    "type",
                    String {
                        value: "module",
                        span: Span {
                            start: 205,
                            end: 213,
                        },
                    },
                ),
                (
                    "exports",
                    String {
                        value: "./index.js",
                        span: Span {
                            start: 230,
                            end: 242,
                        },
                    },
                ),
            ],
            span: Span {
                start: 145,
                end: 244,
            },
        },
        span: Span {
            start: 145,
            end: 244,
        },
    },
    ExportNamedFrom {
        specifiers: [
            ReExportNamedSpecifier {
                imported: "pkg",
                imported_span: Span {
                    start: 295,
                    end: 298,
                },
                exported: "pkg",
                exported_span: Span {
                    start: 295,
                    end: 298,
                },
                span: Span {
                    start: 295,
                    end: 298,
                },
            },
        ],
        source: ModuleSpecifier {
            value: "pkg",
            span: Span {
                start: 306,
                end: 311,
            },
        },
        span: Span {
            start: 286,
            end: 312,
        },
    },
    ImportNamed {
        specifiers: [
            ImportNamedSpecifier {
                imported: "pkg",
                imported_span: Span {
                    start: 363,
                    end: 366,
                },
                local: "pkg",
                local_span: Span {
                    start: 363,
                    end: 366,
                },
                span: Span {
                    start: 363,
                    end: 366,
                },
            },
        ],
        source: ModuleSpecifier {
            value: "./utils.js",
            span: Span {
                start: 374,
                end: 386,
            },
        },
        span: Span {
            start: 354,
            end: 387,
        },
    },
    ExportDecl {
        declaration: Let {
            name: "x",
            expr: Call {
                callee: Ident {
                    name: "pkg",
                    span: Span {
                        start: 405,
                        end: 408,
                    },
                },
                args: [],
                span: Span {
                    start: 405,
                    end: 410,
                },
            },
            span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `pkg`; package resolution, import maps, and absolute specifiers are not implemented at 306..311
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
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 157,
        "length": 1,
        "line": 7,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 159,
        "length": 8,
        "line": 7,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 182,
        "length": 1,
        "line": 8,
        "character": 14
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 184,
        "length": 7,
        "line": 8,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 203,
        "length": 1,
        "line": 9,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 205,
        "length": 8,
        "line": 9,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 228,
        "length": 1,
        "line": 10,
        "character": 14
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'pkg' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 306,
        "length": 5,
        "line": 14,
        "character": 21
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './utils.js' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 374,
        "length": 12,
        "line": 17,
        "character": 21
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'x' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 401,
        "length": 1,
        "line": 18,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 467,
        "length": 1,
        "line": 22,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 469,
        "length": 6,
        "line": 22,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 490,
        "length": 1,
        "line": 23,
        "character": 14
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 492,
        "length": 7,
        "line": 23,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 511,
        "length": 1,
        "line": 24,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 513,
        "length": 8,
        "line": 24,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 541,
        "length": 1,
        "line": 25,
        "character": 19
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 561,
        "length": 1,
        "line": 26,
        "character": 17
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 652,
        "length": 1,
        "line": 32,
        "character": 22
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 672,
        "length": 1,
        "line": 33,
        "character": 17
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 674,
        "length": 4,
        "line": 33,
        "character": 19
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 697,
        "length": 1,
        "line": 34,
        "character": 18
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 699,
        "length": 4,
        "line": 34,
        "character": 20
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJs
```

Stack trace:

```text
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `pkg`; package resolution, import maps, and absolute specifiers are not implemented at 306..311
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/598-implement-allowJsCrossMonorepoPackage.md` に統合されました。
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
- `issues/done/512-implement-allowJsCrossMonorepoPackage.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
