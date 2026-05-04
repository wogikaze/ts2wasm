---
id: 613
title: "Implement Ambientexternalmodulewithrelativeexternalimportdeclaration"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage ambientExternalModuleWithRelativeExternalImportDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithRelativeExternalImportDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithRelativeExternalImportDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts`

## Duplicate detection

- `issues/done/152-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` - Implement Ambientexternalmodulewithrelativeexternalimportdeclaration (same reference path, same group key, title overlap)
- `issues/done/527-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` - Implement Ambientexternalmodulewithrelativeexternalimportdeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientExternalModuleWithRelativeExternalImportDeclaration

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 267,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "declare module \"OuterModule\" {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 28..34",
  "span_start": 28,
  "span_end": 34,
  "line": 2,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare module "OuterModule" {
3 |     import m2 = require("./SubModule");
4 |     class SubModule {
5 |         public static StaticVar: number;
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
    "path": "issues/done/152-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithrelativeexternalimportdeclaration",
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
    "state": "open",
    "path": "issues/done/527-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithrelativeexternalimportdeclaration",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: String(
            "OuterModule",
        ),
        span: Span {
            start: 35,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 56,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "m2",
        ),
        span: Span {
            start: 63,
            end: 65,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 68,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: String(
            "./SubModule",
        ),
        span: Span {
            start: 76,
            end: 89,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 97,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "SubModule",
        ),
        span: Span {
            start: 103,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 124,
            end: 130,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 131,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "StaticVar",
        ),
        span: Span {
            start: 138,
            end: 147,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 149,
            end: 155,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 166,
            end: 172,
        },
    },
    SpannedToken {
        kind: Ident(
            "InstanceVar",
        ),
        span: Span {
            start: 173,
            end: 184,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 184,
            end: 185,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
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
        "code": 2439,
        "category": "Error",
        "message": "Import or export declaration in an ambient module declaration cannot reference module through relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts",
        "start": 56,
        "length": 35,
        "line": 3,
        "character": 5
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './SubModule' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts",
        "start": 76,
        "length": 13,
        "line": 3,
        "character": 25
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
```

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

## Status

Superseded by issue #152. Duplicate from separate coverage run.
