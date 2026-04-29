---
id: 117
title: "Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage aliasOfGenericFunctionWithRestBehavedSameAsUnaliased across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasOfGenericFunctionWithRestBehavedSameAsUnaliased` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasOfGenericFunctionWithRestBehavedSameAsUnaliased has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts
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

- `reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasOfGenericFunctionWithRestBehavedSameAsUnaliased

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1163,
  "lines": 34,
  "extension": ".ts",
  "first_code_line": "type ExtendedMapper<HandledInputT, OutputT, ArgsT extends any[]> = (name : string, mixed : HandledInputT, ...args : ArgsT) => OutputT;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Equal, got Some(Less) at 119..120",
  "span_start": 119,
  "span_end": 120,
  "line": 5,
  "column": 24,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: true
3 | // the type printback for every `test` below should be "y"
4 |
5 | type ExtendedMapper<HandledInputT, OutputT, ArgsT extends any[]> = (name : string, mixed : HandledInputT, ...args : ArgsT) => OutputT;
6 | type a = ExtendedMapper<any, any, [any]>;
7 | type b = ExtendedMapper<any, any, any[]>;
8 | type test = a extends b ? "y" : "n"
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
    "path": "issues/open/117-implement-aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.md",
    "title": "Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 100,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "ExtendedMapper",
        ),
        span: Span {
            start: 105,
            end: 119,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "HandledInputT",
        ),
        span: Span {
            start: 120,
            end: 133,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 133,
            end: 134,
        },
    },
    SpannedToken {
        kind: Ident(
            "OutputT",
        ),
        span: Span {
            start: 135,
            end: 142,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "ArgsT",
        ),
        span: Span {
            start: 144,
            end: 149,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 150,
            end: 157,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 158,
            end: 161,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 119..120
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 119..120
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
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 168,
        "length": 4,
        "line": 5,
        "character": 69,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "HandledInputT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 183,
        "length": 5,
        "line": 5,
        "character": 84,
        "name": "mixed"
      },
      {
        "kind": "parameter",
        "typeText": "ArgsT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 209,
        "length": 4,
        "line": 5,
        "character": 110,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "\"y\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 363,
        "length": 5,
        "line": 9,
        "character": 5,
        "name": "check"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 463,
        "length": 4,
        "line": 13,
        "character": 6,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "HandledInputT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 478,
        "length": 5,
        "line": 13,
        "character": 21,
        "name": "mixed"
      },
      {
        "kind": "parameter",
        "typeText": "ArgsT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 504,
        "length": 4,
        "line": 13,
        "character": 47,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "\"y\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 670,
        "length": 6,
        "line": 19,
        "character": 5,
        "name": "check1"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 773,
        "length": 4,
        "line": 22,
        "character": 9,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "HandledInputT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 788,
        "length": 5,
        "line": 22,
        "character": 24,
        "name": "mixed"
      },
      {
        "kind": "parameter",
        "typeText": "ArgsT",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 814,
        "length": 4,
        "line": 22,
        "character": 50,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "\"y\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 986,
        "length": 6,
        "line": 28,
        "character": 5,
        "name": "check2"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1021,
        "length": 4,
        "line": 30,
        "character": 12,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1035,
        "length": 5,
        "line": 30,
        "character": 26,
        "name": "mixed"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1047,
        "length": 6,
        "line": 30,
        "character": 38,
        "name": "args_0"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1079,
        "length": 4,
        "line": 31,
        "character": 12,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1093,
        "length": 5,
        "line": 31,
        "character": 26,
        "name": "mixed"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1108,
        "length": 4,
        "line": 31,
        "character": 41,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "\"y\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.ts",
        "start": 1175,
        "length": 6,
        "line": 34,
        "character": 5,
        "name": "check3"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ExtendedMapper<HandledInputT, OutputT, ArgsT extends any[]> = (name : string, mixed : HandledInputT, ...args : Args",
        "line": 5,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type a = ExtendedMapper<any, any, [any]>;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type b = ExtendedMapper<any, any, any[]>;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type test = a extends b ? \"y\" : \"n\"",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let check: test = \"y\";",
        "line": 9,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ExtendedMapper1<HandledInputT, OutputT, ArgsT extends any[]> = (\r\n    (name : string, mixed : HandledInputT, ...arg",
        "line": 12,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type a1 = ExtendedMapper1<any, any, [any]>;",
        "line": 16,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type b1 = ExtendedMapper1<any, any, any[]>;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type test1 = a1 e
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 119..120
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
