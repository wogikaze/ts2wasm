---
id: 763
title: "Implement Augmentexportequals"
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

Triage augmentExportEquals across 12 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 12 cases fail in directory `augmentExportEquals` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentExportEquals has 12 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentExportEquals1.ts --detail
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
mise run reference-coverage -- tsc --limit 24
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentExportEquals1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts
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

- `reference/typescript/tests/cases/compiler/augmentExportEquals1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals2_1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals2.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals1_1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals4.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals3_1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals4_1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals3.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals6_1.ts`
- `reference/typescript/tests/cases/compiler/augmentExportEquals6.ts`
- ... and 2 more files

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: augmentExportEquals1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentExportEquals1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 410,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "var x = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 97..103",
  "span_start": 97,
  "span_end": 103,
  "line": 6,
  "column": 6,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @module: commonjs
4 | // @filename: file1.ts
5 | var x = 1;
6 | export = x;
7 | 
8 | // @filename: file2.ts
9 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 5,
    "column": 1,
    "initializer": "1"
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
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 97,
            end: 103,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 138,
            end: 144,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 149,
            end: 156,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: String(
            "./file1",
        ),
        span: Span {
            start: 157,
            end: 166,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 269,
            end: 276,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 277,
            end: 283,
        },
    },
    SpannedToken {
        kind: String(
            "./file1",
        ),
        span: Span {
            start: 284,
            end: 293,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 294,
            end: 295,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 301,
            end: 310,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 311,
            end: 312,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 313,
            end: 314,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 315,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 97..103
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 97..103
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 89,
        "length": 1,
        "line": 5,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 145,
        "length": 1,
        "line": 10,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './file1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 157,
        "length": 9,
        "line": 10,
        "character": 20
      },
      {
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module './file1' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 284,
        "length": 9,
        "line": 14,
        "character": 16
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 356,
        "length": 1,
        "line": 19,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './file1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 368,
        "length": 9,
        "line": 19,
        "character": 20
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of './file2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 388,
        "length": 9,
        "line": 20,
        "character": 8
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 89,
        "length": 1,
        "line": 5,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "x.A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentExportEquals1.ts",
        "start": 404,
        "length": 1,
        "line": 21,
        "character": 5,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var x = 1;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = x;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import x = require(\"./file1\");",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"./file1\" {\r\n    interface A { a }\r\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import x = require(\"./file1\");",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"./file2\";",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let a: x.A;",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var x = 1;\r\nexport = x;\r\n\r\n// @filename: file2.ts\r\n\r\nimport x = require(\"./file1\"); \r\n\r\n// augmentation for './file1'\r\n/",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = x;",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 97..103
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
