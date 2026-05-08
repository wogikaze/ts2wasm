---
id: 688
title: "Implement Arrayofexportedclass"
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

Triage arrayOfExportedClass across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayOfExportedClass` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayOfExportedClass has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts
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

- `reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arrayOfExportedClass

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 391,
  "lines": 24,
  "extension": ".ts",
  "first_code_line": "class Car {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 119..125",
  "span_start": 119,
  "span_end": 125,
  "line": 8,
  "column": 8,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 5 |     foo: string;
 6 | }
 7 | 
 8 | export = Car;
 9 | 
10 | // @Filename: arrayOfExportedClass_1.ts
11 | ///<reference path='arrayOfExportedClass_0.ts'/>
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Car",
    "line": 4,
    "column": 1
  }
]
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
        kind: Class,
        span: Span {
            start: 83,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "Car",
        ),
        span: Span {
            start: 89,
            end: 92,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 100,
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
            "string",
        ),
        span: Span {
            start: 105,
            end: 111,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 119,
            end: 125,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "Car",
        ),
        span: Span {
            start: 128,
            end: 131,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 227,
            end: 233,
        },
    },
    SpannedToken {
        kind: Ident(
            "Car",
        ),
        span: Span {
            start: 234,
            end: 237,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 238,
            end: 239,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 240,
            end: 247,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 247,
            end: 248,
        },
    },
    SpannedToken {
        kind: String(
            "./arrayOfExportedClass_0",
        ),
        span: Span {
            start: 248,
            end: 274,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 274,
            end: 275,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 275,
            end: 276,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 280,
            end: 285,
        },
    },
    SpannedToken {
        kind: Ident(
            "Road",
        ),
        span: Span {
            start: 286,
            end: 290,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 291,
            end: 292,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 300,
            end: 306,
        },
    },
    SpannedToken {
        kind: Ident(
            "cars",
        ),
        span: Span {
            start: 307,
            end: 311,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 119..125
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 119..125
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
        "message": "Property 'foo' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 100,
        "length": 3,
        "line": 5,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'export='.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 128,
        "length": 3,
        "line": 8,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './arrayOfExportedClass_0' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 248,
        "length": 26,
        "line": 12,
        "character": 22
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'cars' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 307,
        "length": 4,
        "line": 16,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'export='.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 408,
        "length": 4,
        "line": 24,
        "character": 10
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "Car[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfExportedClass.ts",
        "start": 342,
        "length": 4,
        "line": 18,
        "character": 20,
        "name": "cars"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Car {\r\n    foo: string;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Car;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import Car = require('./arrayOfExportedClass_0');",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Road {\r\n\r\n    public cars: Car[];\r\n\r\n    public AddCars(cars: Car[]) {\r\n\r\n        this.cars = cars;\r\n    }\r\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Road;",
        "line": 24,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Car {\r\n    foo: string;\r\n}\r\n\r\nexport = Car;\r\n\r\n// @Filename: arrayOfExportedClass_1.ts\r\n///<reference path='arrayO",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Car;",
        "line": 8,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 119..125
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
