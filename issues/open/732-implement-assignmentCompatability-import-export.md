---
id: 732
title: "Implement Assignmentcompatability Import Export"
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

Triage assignmentCompatability-import-export across 43 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 43 cases fail in directory `assignmentCompatability-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatability-import-export has 43 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability1.ts --detail
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
mise run reference-coverage -- tsc --limit 86
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability1.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatability1.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability10.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability11.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability13.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability12.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability16.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability15.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability14.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability17.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability18.ts`
- ... and 33 more files

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

### Smart triage: Triage import export: assignmentCompatability1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatability1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 352,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "namespace __test1__ {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 19..28",
  "span_start": 19,
  "span_end": 28,
  "line": 2,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace __test1__ {
3 |     export interface interfaceWithPublicAndOptional<T,U> { one: T; two?: U; };  var obj4: interfaceWithPublicAndOptional<number,string> = { one: 1 };;
4 |     export var __val__obj4 = obj4;
5 | }
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
            start: 19,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "__test1__",
        ),
        span: Span {
            start: 29,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 45,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 52,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "interfaceWithPublicAndOptional",
        ),
        span: Span {
            start: 62,
            end: 92,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "U",
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "one",
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
            "T",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "two",
        ),
        span: Span {
            start: 108,
            end: 111,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "U",
        ),
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 121,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj4",
        ),
        span: Span {
            start: 125,
            end: 129,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 19..28
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 19..28
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
        "kind": "binding",
        "typeText": "interfaceWithPublicAndOptional<number, string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability1.ts",
        "start": 125,
        "length": 4,
        "line": 3,
        "character": 85,
        "name": "obj4"
      },
      {
        "kind": "binding",
        "typeText": "interfaceWithPublicAndOptional<number, string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability1.ts",
        "start": 207,
        "length": 11,
        "line": 4,
        "character": 16,
        "name": "__val__obj4"
      },
      {
        "kind": "binding",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability1.ts",
        "start": 266,
        "length": 2,
        "line": 7,
        "character": 16,
        "name": "aa"
      },
      {
        "kind": "binding",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability1.ts",
        "start": 291,
        "length": 9,
        "line": 8,
        "character": 16,
        "name": "__val__aa"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace __test1__ {\n    export interface interfaceWithPublicAndOptional<T,U> { one: T; two?: U; };  var obj4: interfac",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace __test2__ {\n    export var aa = {};;\n    export var __val__aa = aa;\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "__test2__.__val__aa = __test1__.__val__obj4",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace __test1__ {\n    export interface interfaceWithPublicAndOptional<T,U> { one: T; two?: U; };  var obj4: interfac",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace __test1__ {\n    export interface interfaceWithPublicAndOptional<T,U> { one: T; two?: U; };  var obj4: interfac",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 19..28
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
