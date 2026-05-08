---
id: 717
title: "Implement Assigntoexistingclass"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignToExistingClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignToExistingClass` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignToExistingClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToExistingClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToExistingClass.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToExistingClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToExistingClass.ts
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

- `reference/typescript/tests/cases/compiler/assignToExistingClass.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: assignToExistingClass

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignToExistingClass.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToExistingClass.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 317,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "namespace Test {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29",
  "span_start": 20,
  "span_end": 29,
  "line": 2,
  "column": 2,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace Test {
3 |     class Mocked {
4 |         myProp: string;
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
            "namespace",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "Test",
        ),
        span: Span {
            start: 30,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 42,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "Mocked",
        ),
        span: Span {
            start: 48,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "myProp",
        ),
        span: Span {
            start: 66,
            end: 72,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 74,
            end: 80,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 96,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "Tester",
        ),
        span: Span {
            start: 102,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "willThrowError",
        ),
        span: Span {
            start: 120,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "Mocked",
        ),
        span: Span {
            start: 152,
            end: 158,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "Mocked",
        ),
        span: Span {
            start: 161,
            end: 167,
        },
    },
    SpannedToken {
        kind: OrOr,
        span: Span {
            start: 168,
            end: 170,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 171,
            end: 179,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 180,
            end: 181,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'myProp' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToExistingClass.ts",
        "start": 66,
        "length": 6,
        "line": 4,
        "character": 9
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'Mocked' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToExistingClass.ts",
        "start": 152,
        "length": 6,
        "line": 9,
        "character": 13
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace Test {\r\n    class Mocked {\r\n        myProp: string;\r\n    }\r\n\r\n    class Tester {\r\n        willThrowError() {\r\n",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace Test {\r\n    class Mocked {\r\n        myProp: string;\r\n    }\r\n\r\n    class Tester {\r\n        willThrowError() {\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace Test {\r\n    class Mocked {\r\n        myProp: string;\r\n    }\r\n\r\n    class Tester {\r\n        willThrowError() {\r\n",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
