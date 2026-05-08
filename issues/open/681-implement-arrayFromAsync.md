---
id: 681
title: "Implement Arrayfromasync"
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

Triage arrayFromAsync across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFromAsync` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFromAsync has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFromAsync.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFromAsync.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFromAsync.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFromAsync.ts
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

- `reference/typescript/tests/cases/compiler/arrayFromAsync.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: arrayFromAsync

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayFromAsync.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFromAsync.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1377,
  "lines": 48,
  "extension": ".ts",
  "first_code_line": "export { };"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 74..88",
  "span_start": 74,
  "span_end": 88,
  "line": 6,
  "column": 6,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @target: esnext
4 | 
5 | export { };
6 | async function * asyncGen (n) {
7 |     for (let i = 0; i < n; i++)
8 |       yield i * 2;
9 |   }
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Export,
        span: Span {
            start: 61,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 74,
            end: 79,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 80,
            end: 88,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "asyncGen",
        ),
        span: Span {
            start: 91,
            end: 99,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "n",
        ),
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 111,
            end: 114,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 116,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "n",
        ),
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Increment,
        span: Span {
            start: 135,
            end: 137,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 137,
            end: 138,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 74..88
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 74..88
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
        "code": 1432,
        "category": "Error",
        "message": "Top-level 'for await' loops are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 467,
        "length": 5,
        "line": 26,
        "character": 5
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 539,
        "length": 5,
        "line": 30,
        "character": 18
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 551,
        "length": 9,
        "line": 30,
        "character": 30
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 589,
        "length": 5,
        "line": 31,
        "character": 18
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 601,
        "length": 9,
        "line": 31,
        "character": 30
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 712,
        "length": 5,
        "line": 32,
        "character": 18
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 724,
        "length": 9,
        "line": 32,
        "character": 30
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 769,
        "length": 5,
        "line": 33,
        "character": 18
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 781,
        "length": 9,
        "line": 33,
        "character": 30
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 853,
        "length": 9,
        "line": 36,
        "character": 24
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 882,
        "length": 5,
        "line": 37,
        "character": 18
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 936,
        "length": 5,
        "line": 39,
        "character": 17
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 948,
        "length": 9,
        "line": 39,
        "character": 29
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 1002,
        "length": 5,
        "line": 40,
        "character": 17
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target library? Try changing the 'lib' compiler option to 'esnext' or later.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 1014,
        "length": 9,
        "line": 40,
        "character": 29
      },
      {
        "code": 1378,
        "category": "Error",
        "message": "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'node18', 'node20', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFromAsync.ts",
        "start": 1083,
        "length": 5,
        "line": 41,
        "character": 17
      },
      {
        "code": 2550,
        "category": "Error",
        "message": "Property 'fromAsync' does not exist on type 'ArrayConstructor'. Do you need to change your target libra
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 74..88
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
