---
id: 711
title: "Implement Asireturn"
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

Triage asiReturn across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiReturn` with diagnostics: top-level-return. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiReturn has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiReturn.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiReturn.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiReturn.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiReturn.ts
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

- `reference/typescript/tests/cases/compiler/asiReturn.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage top level return: asiReturn

- Issue class: `triage-needed`
- Feature label: `top-level-return`
- Diagnostic: `InvalidTopLevelReturn` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/asiReturn.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiReturn.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 123,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "InvalidTopLevelReturn",
  "message": "top-level return is not supported at 119..125",
  "span_start": 119,
  "span_end": 125,
  "line": 3,
  "column": 5,
  "feature_label": "top-level-return",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
1 | ﻿// @target: es2015
2 | // This should be an error for using a return outside a function, but ASI should work properly
3 | return
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[]
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
        kind: Return,
        span: Span {
            start: 119,
            end: 125,
        },
    },
]
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Return {
        expr: Undefined {
            span: Span {
                start: 119,
                end: 125,
            },
        },
        span: Span {
            start: 119,
            end: 125,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [InvalidTopLevelReturn] top-level return is not supported at 119..125
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
        "code": 1108,
        "category": "Error",
        "message": "A 'return' statement can only be used within a function body.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiReturn.ts",
        "start": 116,
        "length": 6,
        "line": 3,
        "character": 1
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ReturnStatement",
        "text": "return",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "return",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ReturnStatement",
        "text": "return",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [InvalidTopLevelReturn] top-level return is not supported at 119..125
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
