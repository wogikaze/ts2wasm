---
id: 489
title: "Implement Accessorsinambientcontext (audit reopened #489)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage accessorsInAmbientContext across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorsInAmbientContext` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorsInAmbientContext has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts
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

- `reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts`

## Duplicate detection

- `issues/done/108-implement-accessorsInAmbientContext.md` - Implement Accessorsinambientcontext (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: accessorsInAmbientContext

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 307,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "declare namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 35..44",
  "span_start": 35,
  "span_end": 44,
  "line": 3,
  "column": 11,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es5, es2015
2 | 
3 | declare namespace M {
4 |     class C {
5 |         get X() { return 1; }
6 |         set X(v) { }
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
    "path": "issues/done/108-implement-accessorsInAmbientContext.md",
    "title": "Implement Accessorsinambientcontext",
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
            start: 27,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 35,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 54,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 104,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 111,
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
        kind: RightBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 128,
            end: 134,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 135,
            end: 138,
        },
    },
    SpannedToke
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 35..44
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 35..44
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
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 81,
        "length": 1,
        "line": 5,
        "character": 17
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 113,
        "length": 1,
        "line": 6,
        "character": 18
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 143,
        "length": 1,
        "line": 8,
        "character": 24
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 182,
        "length": 1,
        "line": 9,
        "character": 25
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 230,
        "length": 1,
        "line": 14,
        "character": 13
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 258,
        "length": 1,
        "line": 15,
        "character": 14
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 284,
        "length": 1,
        "line": 17,
        "character": 20
      },
      {
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 319,
        "length": 1,
        "line": 18,
        "character": 21
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 110,
        "length": 1,
        "line": 6,
        "character": 15,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 179,
        "length": 1,
        "line": 9,
        "character": 22,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 255,
        "length": 1,
        "line": 15,
        "character": 11,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorsInAmbientContext.ts",
        "start": 316,
        "length": 1,
        "line": 18,
        "character": 18,
        "name": "v"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    class C {\r\n        get X() { return 1; }\r\n        set X(v) { }\r\n\r\n        static get Y() { re",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class C {\r\n    get X() { return 1; }\r\n    set X(v) { }\r\n\r\n    static get Y() { return 1; }\r\n    static set Y(v) ",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace M {\r\n    class C {\r\n        get X() { return 1; }\r\n        set X(v) { }\r\n\r\n        static get Y() { re",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    class C {\r\n        get X() { return 1; }\r\n        set X(v) { }\r\n\r\n        static get Y() { re",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 35..44
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## Status

Superseded by issue #108. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- open issue file before this move
- `issues/done/489-implement-accessorsInAmbientContext.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
