---
id: 615
title: "Implement Ambientexternalmodulewithoutinternalimportdeclaration"
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

Triage ambientExternalModuleWithoutInternalImportDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithoutInternalImportDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithoutInternalImportDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts`

## Duplicate detection

- `issues/done/154-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` - Implement Ambientexternalmodulewithoutinternalimportdeclaration (same reference path, same group key, title overlap)
- `issues/done/529-implement-ambientExternalModuleWithoutInternalImportDeclaration.md` - Implement Ambientexternalmodulewithoutinternalimportdeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientExternalModuleWithoutInternalImportDeclaration

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 439,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "declare module 'M' {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 118..124",
  "span_start": 118,
  "span_end": 124,
  "line": 4,
  "column": 12,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: amd
3 | // @Filename: ambientExternalModuleWithoutInternalImportDeclaration_0.ts
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
    "path": "issues/done/154-implement-ambientExternalModuleWithoutInternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithoutinternalimportdeclaration",
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
    "path": "issues/done/529-implement-ambientExternalModuleWithoutInternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithoutinternalimportdeclaration",
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
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
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
            start: 110,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 118,
            end: 124,
        },
    },
    SpannedToken {
        kind: String(
            "M",
        ),
        span: Span {
            start: 125,
            end: 128,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 136,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 159,
            end: 165,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 166,
            end: 169,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 173,
            end: 179,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 179,
            end: 180,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 186,
            end: 187,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 193,
            end: 198,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 201,
            end: 202,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 212,
            end: 215,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 215,
            end: 216,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 216,
            end: 217,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 217,
            end: 218,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 219,
            end: 223,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 237,
            end: 243,
        },
    },
    SpannedToken {
        kind: Equal,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 118..124
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 118..124
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts",
        "start": 125,
        "length": 3,
        "line": 4,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'M' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts",
        "start": 433,
        "length": 3,
        "line": 17,
        "character": 20
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts",
        "start": 170,
        "length": 1,
        "line": 6,
        "character": 20,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithoutInternalImportDeclaration.ts",
        "start": 444,
        "length": 1,
        "line": 18,
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
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c = new A();",
        "line": 18,
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
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 118..124
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
