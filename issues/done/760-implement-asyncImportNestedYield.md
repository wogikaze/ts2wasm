---
id: 760
title: "Implement Asyncimportnestedyield"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncImportNestedYield across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncImportNestedYield` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncImportNestedYield has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts
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

- `reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: asyncImportNestedYield

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 130,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "async function* foo() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 58..72",
  "span_start": 58,
  "span_end": 72,
  "line": 4,
  "column": 3,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @module: commonjs
2 | // @target: es2015
3 | // @lib: esnext
4 | async function* foo() {
5 |     import((await import(yield "foo")).default);
6 | }
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
    "path": "issues/open/754-implement-asyncFunctionReturnType-runtime-subset.md",
    "title": "Implement Asyncfunctionreturntype Runtime Subset",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Async,
        span: Span {
            start: 58,
            end: 63,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 64,
            end: 72,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 74,
            end: 77,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 87,
            end: 93,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Await,
        span: Span {
            start: 95,
            end: 100,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 101,
            end: 107,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "yield",
        ),
        span: Span {
            start: 108,
            end: 113,
        },
    },
    SpannedToken {
        kind: String(
            "foo",
        ),
        span: Span {
            start: 114,
            end: 119,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 122,
            end: 129,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 133,
            end: 134,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 58..72
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 58..72
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
        "kind": "function",
        "typeText": "AsyncGenerator<string, void, string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncImportNestedYield.ts",
        "start": 74,
        "length": 3,
        "line": 4,
        "character": 17,
        "name": "foo"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "async function* foo() {\r\n    import((await import(yield \"foo\")).default);\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "async function* foo() {\r\n    import((await import(yield \"foo\")).default);\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function* foo() {\r\n    import((await import(yield \"foo\")).default);\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "AsyncKeyword",
        "text": "async",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 58..72
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
