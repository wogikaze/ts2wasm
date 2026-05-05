---
id: 665
title: "Implement Arraybestcommontypes"
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

Triage arrayBestCommonTypes across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayBestCommonTypes` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayBestCommonTypes has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts
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

- `reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arrayBestCommonTypes

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 4533,
  "lines": 107,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 23..32",
  "span_start": 23,
  "span_end": 32,
  "line": 2,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | ﻿// @target: es2015
2 | namespace EmptyTypes {
3 |     interface iface { }
4 |     class base implements iface { }
5 |     class base2 implements iface { }
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
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
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
            "namespace",
        ),
        span: Span {
            start: 23,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "EmptyTypes",
        ),
        span: Span {
            start: 33,
            end: 43,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 51,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "iface",
        ),
        span: Span {
            start: 61,
            end: 66,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 76,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "base",
        ),
        span: Span {
            start: 82,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "implements",
        ),
        span: Span {
            start: 87,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "iface",
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
        kind: RightBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 113,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "base2",
        ),
        span: Span {
            start: 119,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "implements",
        ),
        span: Span {
            start: 125,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "iface",
        ),
        span: Span {
            start: 136,
            end: 141,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 151,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "derived",
        ),
        span: Span {
            start: 157,
            end: 164,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 165,
            end: 172,
        },
    },
    SpannedToken {
        kind: Ident(
            "base",
        ),
        span: Span {
            start: 173,
            end: 177,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 178,
            end: 179,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Spa
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 23..32
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 23..32
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
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'boolean'.\n      Type 'undefined' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'string'.\n      Type 'undefined' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'number'.\n      Type 'undefined' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 554,
        "length": 20,
        "line": 17,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'boolean'.\n      Type 'undefined' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'string'.\n      Type 'undefined' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | undefined' is not assignable to parameter of type 'number'.\n      Type 'undefined' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 615,
        "length": 20,
        "line": 18,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'boolean'.\n      Type 'null' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'string'.\n      Type 'null' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'number'.\n      Type 'null' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 676,
        "length": 15,
        "line": 19,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'boolean'.\n      Type 'null' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'string'.\n      Type 'null' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null' is not assignable to parameter of type 'number'.\n      Type 'null' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 732,
        "length": 15,
        "line": 20,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null | undefined' is not assignable to parameter of type 'boolean'.\n      Type 'undefined' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null | undefined' is not assignable to parameter of type 'string'.\n      Type 'undefined' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'number | null | undefined' is not assignable to parameter of type 'number'.\n      Type 'undefined' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 788,
        "length": 23,
        "line": 21,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'boolean'.\n      Type 'undefined' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'string'.\n      Type 'undefined' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'number'.\n      Type 'undefined' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 907,
        "length": 23,
        "line": 24,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'boolean'.\n      Type 'undefined' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'string'.\n      Type 'undefined' is not assignable to type 'string'.\n  Overload 3 of 3, '(x: number, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | undefined' is not assignable to parameter of type 'number'.\n      Type 'undefined' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBestCommonTypes.ts",
        "start": 971,
        "length": 23,
        "line": 25,
        "character": 37
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 3, '(x: boolean, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | null' is not assignable to parameter of type 'boolean'.\n      Type 'null' is not assignable to type 'boolean'.\n  Overload 2 of 3, '(x: string, y?: boolean | undefined): number', gave the following error.\n    Argument of type 'string | null' is not assignable to param
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 23..32
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
