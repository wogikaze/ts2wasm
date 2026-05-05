---
id: 758
title: "Implement Asyncfunctionsandstrictnullchecks"
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

Triage asyncFunctionsAndStrictNullChecks across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionsAndStrictNullChecks` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionsAndStrictNullChecks has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: asyncFunctionsAndStrictNullChecks

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1152,
  "lines": 27,
  "extension": ".ts",
  "first_code_line": "declare namespace Windows.Foundation {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 55..64",
  "span_start": 55,
  "span_end": 64,
  "line": 4,
  "column": 12,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es6
2 | // @strictNullChecks: true
3 | 
4 | declare namespace Windows.Foundation {
5 |     interface IPromise<TResult> {
6 |         then<U>(success?: (value: TResult) => IPromise<U>, error?: (error: any) => IPromise<U>, progress?: (progress: any) => void): IPromise<U>;
7 |         then<U>(success?: (value: TResult) => IPromise<U>, error?: (error: any) => U, progress?: (progress: any) => void): IPromise<U>;
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
            "declare",
        ),
        span: Span {
            start: 47,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 55,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "Windows",
        ),
        span: Span {
            start: 65,
            end: 72,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foundation",
        ),
        span: Span {
            start: 73,
            end: 83,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 91,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "IPromise",
        ),
        span: Span {
            start: 101,
            end: 109,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "TResult",
        ),
        span: Span {
            start: 110,
            end: 117,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "then",
        ),
        span: Span {
            start: 130,
            end: 134,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "U",
        ),
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "success",
        ),
        span: Span {
            start: 138,
            end: 145,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 149,
            end: 154,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Ident(
            "TResult",
        ),
        span: Span {
            start: 156,
            end: 163,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 55..64
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 55..64
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
        "typeText": "((value: TResult) => IPromise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 138,
        "length": 7,
        "line": 6,
        "character": 17,
        "name": "success"
      },
      {
        "kind": "parameter",
        "typeText": "TResult",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 149,
        "length": 5,
        "line": 6,
        "character": 28,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "((error: any) => IPromise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 181,
        "length": 5,
        "line": 6,
        "character": 60,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 190,
        "length": 5,
        "line": 6,
        "character": 69,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "((progress: any) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 218,
        "length": 8,
        "line": 6,
        "character": 97,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 230,
        "length": 8,
        "line": 6,
        "character": 109,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "((value: TResult) => IPromise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 285,
        "length": 7,
        "line": 7,
        "character": 17,
        "name": "success"
      },
      {
        "kind": "parameter",
        "typeText": "TResult",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 296,
        "length": 5,
        "line": 7,
        "character": 28,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "((error: any) => U) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 328,
        "length": 5,
        "line": 7,
        "character": 60,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 337,
        "length": 5,
        "line": 7,
        "character": 69,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "((progress: any) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 355,
        "length": 8,
        "line": 7,
        "character": 87,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 367,
        "length": 8,
        "line": 7,
        "character": 99,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "((value: TResult) => U) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 422,
        "length": 7,
        "line": 8,
        "character": 17,
        "name": "success"
      },
      {
        "kind": "parameter",
        "typeText": "TResult",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 433,
        "length": 5,
        "line": 8,
        "character": 28,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "((error: any) => IPromise<U>) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 455,
        "length": 5,
        "line": 8,
        "character": 50,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 464,
        "length": 5,
        "line": 8,
        "character": 59,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "((progress: any) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 492,
        "length": 8,
        "line": 8,
        "character": 87,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 504,
        "length": 8,
        "line": 8,
        "character": 99,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "((value: TResult) => U) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 559,
        "length": 7,
        "line": 9,
        "character": 17,
        "name": "success"
      },
      {
        "kind": "parameter",
        "typeText": "TResult",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 570,
        "length": 5,
        "line": 9,
        "character": 28,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "((error: any) => U) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 592,
        "length": 5,
        "line": 9,
        "character": 50,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 601,
        "length": 5,
        "line": 9,
        "character": 59,
        "name": "error"
      },
      {
        "kind": "parameter",
        "typeText": "((progress: any) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 619,
        "length": 8,
        "line": 9,
        "character": 77,
        "name": "progress"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAndStrictNullChecks.ts",
        "start": 631,
        "length": 8,
        "line": 9,
        "character": 89,
        "name": "progres
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 55..64
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
