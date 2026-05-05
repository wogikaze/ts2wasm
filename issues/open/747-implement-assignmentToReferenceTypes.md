---
id: 747
title: "Implement Assignmenttoreferencetypes"
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

Triage assignmentToReferenceTypes across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToReferenceTypes` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToReferenceTypes has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: assignmentToReferenceTypes

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 216,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 67..76",
  "span_start": 67,
  "span_end": 76,
  "line": 5,
  "column": 5,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | // @strict: false
3 | // Should all be allowed
4 | 
5 | namespace M {
6 | }
7 | M = null;
8 |
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
            start: 67,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 89,
            end: 93,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 98,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 116,
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
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 142,
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
        kind: Function,
        span: Span {
            start: 151,
            end: 159,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            st
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 67..76
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 67..76
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
        "code": 2708,
        "category": "Error",
        "message": "Cannot use namespace 'M' as a value.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 85,
        "length": 1,
        "line": 7,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'C' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 112,
        "length": 1,
        "line": 11,
        "character": 1
      },
      {
        "code": 2628,
        "category": "Error",
        "message": "Cannot assign to 'E' because it is an enum.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 138,
        "length": 1,
        "line": 15,
        "character": 1
      },
      {
        "code": 2630,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a function.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 169,
        "length": 1,
        "line": 18,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'null' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 194,
        "length": 1,
        "line": 21,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 160,
        "length": 1,
        "line": 17,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 186,
        "length": 1,
        "line": 20,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 216,
        "length": 1,
        "line": 23,
        "character": 10,
        "name": "g"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToReferenceTypes.ts",
        "start": 218,
        "length": 1,
        "line": 23,
        "character": 12,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "M = null;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "C = null;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum E {\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "E = null;",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f() { }",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f = null;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = 1;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "x = null;",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function g(x) {\r\n    x = null;\r\n}",
        "line": 23,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n}\r\nM = null;\r\n\r\nclass C {\r\n}\r\nC = null;\r\n\r\nenum E {\r\n}\r\nE = null;\r\n\r\nfunction f() { }\r\nf = null;\r\n\r\nvar x",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n}",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 67..76
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
