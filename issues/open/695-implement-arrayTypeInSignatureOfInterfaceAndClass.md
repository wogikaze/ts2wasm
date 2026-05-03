---
id: 695
title: "Implement Arraytypeinsignatureofinterfaceandclass"
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

Triage arrayTypeInSignatureOfInterfaceAndClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayTypeInSignatureOfInterfaceAndClass` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayTypeInSignatureOfInterfaceAndClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts
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

- `reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arrayTypeInSignatureOfInterfaceAndClass

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 894,
  "lines": 26,
  "extension": ".ts",
  "first_code_line": "declare namespace WinJS {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37",
  "span_start": 28,
  "span_end": 37,
  "line": 2,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare namespace WinJS {
3 |     class Promise<T> {
4 |         then<U>(success?: (value: T) => Promise<U>, error?: (error: any) => Promise<U>, progress?: (progress: any) => void): Promise<U>;
5 |     }
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
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
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
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
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
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 28,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "WinJS",
        ),
        span: Span {
            start: 38,
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
        kind: Class,
        span: Span {
            start: 51,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "Promise",
        ),
        span: Span {
            start: 57,
            end: 64,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "then",
        ),
        span: Span {
            start: 79,
            end: 83,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "U",
        ),
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "success",
        ),
        span: Span {
            start: 87,
            end: 94,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 98,
            end: 103,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 108,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "Promise",
        ),
        span: Span {
            start: 111,
            end: 118,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "((value: T) => Promise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 87,
        "length": 7,
        "line": 4,
        "character": 17,
        "name": "success"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 98,
        "length": 5,
        "line": 4,
        "character": 28,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "((error: any) => Promise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 123,
        "length": 5,
        "line": 4,
        "character": 53,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 132,
        "length": 5,
        "line": 4,
        "character": 62,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "((progress: any) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 159,
        "length": 8,
        "line": 4,
        "character": 89,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 171,
        "length": 8,
        "line": 4,
        "character": 101,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 622,
        "length": 7,
        "line": 20,
        "character": 23,
        "name": "indices"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 641,
        "length": 7,
        "line": 20,
        "character": 42,
        "name": "options"
      },
      {
        "kind": "parameter",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 844,
        "length": 7,
        "line": 24,
        "character": 30,
        "name": "indices"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayTypeInSignatureOfInterfaceAndClass.ts",
        "start": 863,
        "length": 7,
        "line": 24,
        "character": 49,
        "name": "options"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace WinJS {\r\n    class Promise<T> {\r\n        then<U>(success?: (value: T) => Promise<U>, error?: (error: a",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Data {\r\n    export interface IListItem<T> {\r\n        itemIndex: number;\r\n        key: any;\r\n        da",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace WinJS {\r\n    class Promise<T> {\r\n        then<U>(success?: (value: T) => Promise<U>, error?: (error: a",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace WinJS {\r\n    class Promise<T> {\r\n        then<U>(success?: (value: T) => Promise<U>, error?: (error: a",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
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
