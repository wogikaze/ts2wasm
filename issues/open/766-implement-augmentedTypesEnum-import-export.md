---
id: 766
title: "Implement Augmentedtypesenum Import Export"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage augmentedTypesEnum-import-export across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `augmentedTypesEnum-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentedTypesEnum-import-export has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

### Smart triage: Triage import export: augmentedTypesEnum3

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 180,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "namespace E {"
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
2 | namespace E {
3 |     var t;
4 | }
5 | enum E { }
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
    "state": "open",
    "path": "issues/open/732-implement-assignmentCompatability-import-export.md",
    "title": "Implement Assignmentcompatability Import Export",
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
            "E",
        ),
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 39,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "t",
        ),
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 50,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 64,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "F",
        ),
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 76,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "F",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 88,
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
            "t",
        ),
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 102,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 114,
            end: 115,
        },
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
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 166,
        "length": 1,
        "line": 17,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 43,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "t"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 94,
        "length": 1,
        "line": 8,
        "character": 19,
        "name": "t"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 125,
        "length": 1,
        "line": 11,
        "character": 9,
        "name": "o"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 195,
        "length": 1,
        "line": 20,
        "character": 9,
        "name": "p"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace E {\r\n    var t;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum E { }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum F { }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace F { var t; }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace A {\r\n    var o;\r\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum A {\r\n    b\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum A {\r\n    c\r\n}",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace A {\r\n    var p;\r\n}",
        "line": 19,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace E {\r\n    var t;\r\n}\r\nenum E { }\r\n\r\nenum F { }\r\nnamespace F { var t; }\r\n\r\nnamespace A {\r\n    var o;\r\n}\r\nenum A {",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace E {\r\n    var t;\r\n}",
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
