---
id: 703
title: "Implement Arrowfunctionsmissingtokens"
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

Triage arrowFunctionsMissingTokens across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrowFunctionsMissingTokens` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionsMissingTokens has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arrowFunctionsMissingTokens

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1197,
  "lines": 68,
  "extension": ".ts",
  "first_code_line": "namespace missingArrowsWithCurly {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 41..50",
  "span_start": 41,
  "span_end": 50,
  "line": 4,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | 
4 | namespace missingArrowsWithCurly {
5 |     var a = () { };
6 | 
7 |     var b = (): void { }
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
            start: 41,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "missingArrowsWithCurly",
        ),
        span: Span {
            start: 51,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 81,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
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
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: RightBrace,
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
        kind: Var,
        span: Span {
            start: 104,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 116,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 132,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedTok
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 41..50
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 41..50
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
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 92,
        "length": 1,
        "line": 5,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 121,
        "length": 1,
        "line": 7,
        "character": 22
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 144,
        "length": 1,
        "line": 9,
        "character": 17
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 187,
        "length": 1,
        "line": 11,
        "character": 36
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 236,
        "length": 1,
        "line": 13,
        "character": 42
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 337,
        "length": 3,
        "line": 18,
        "character": 23
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 382,
        "length": 3,
        "line": 20,
        "character": 29
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 421,
        "length": 3,
        "line": 22,
        "character": 24
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 480,
        "length": 3,
        "line": 24,
        "character": 43
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 545,
        "length": 3,
        "line": 26,
        "character": 49
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 584,
        "length": 3,
        "line": 28,
        "character": 23
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 663,
        "length": 1,
        "line": 32,
        "character": 23
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 697,
        "length": 1,
        "line": 34,
        "character": 29
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 725,
        "length": 1,
        "line": 36,
        "character": 24
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 773,
        "length": 1,
        "line": 38,
        "character": 43
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 827,
        "length": 1,
        "line": 40,
        "character": 49
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 855,
        "length": 1,
        "line": 42,
        "character": 23
      },
      {
        "code": 1128,
        "category": "Error",
        "message": "Declaration or statement expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 862,
        "length": 1,
        "line": 43,
        "character": 5
      },
      {
        "code": 1128,
        "category": "Error",
        "message": "Declaration or statement expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 865,
        "length": 1,
        "line": 44,
        "character": 1
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 927,
        "length": 1,
        "line": 47,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 953,
        "length": 1,
        "line": 49,
        "character": 21
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 971,
        "length": 1,
        "line": 51,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 1012,
        "length": 1,
        "line": 53,
        "character": 35
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'=>' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 1057,
        "length": 1,
        "line": 55,
        "character": 41
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionsMissingTokens.ts",
        "start": 85,
        "length": 1,
        "line": 5,
        "character": 9,
        "name": "a"
      },
      {
        "k
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 41..50
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
