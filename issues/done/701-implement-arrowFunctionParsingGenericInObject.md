---
id: 701
title: "Implement Arrowfunctionparsinggenericinobject"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrowFunctionParsingGenericInObject across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrowFunctionParsingGenericInObject` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionParsingGenericInObject has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage type system: arrowFunctionParsingGenericInObject

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 862,
  "lines": 40,
  "extension": ".ts",
  "first_code_line": "const fn1 = () => ({"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 52, end: 53 } }) at 53..54",
  "span_start": 53,
  "span_end": 54,
  "line": 3,
  "column": 14,
  "feature_label": "type-system",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: esnext
2 | const fn1 = () => ({
3 |     test: <T = undefined>(value: T): T => value,
4 |     extraValue: () => {},
5 | })
6 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "fn1",
    "line": 2,
    "column": 1,
    "initializer": "() => ({"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
    "reason": "same feature label, title overlap"
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
        kind: Const,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn1",
        ),
        span: Span {
            start: 26,
            end: 29,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 35,
            end: 37,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 38,
            end: 39,
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
        kind: Ident(
            "test",
        ),
        span: Span {
            start: 46,
            end: 50,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Undefined,
        span: Span {
            start: 57,
            end: 66,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 68,
            end: 73,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 81,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 84,
            end: 89,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "extraValue",
        ),
        span: Span {
            start: 96,
            end: 106,
        },
    },
    Spann
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 52, end: 53 } }) at 53..54
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 52, end: 53 } }) at 53..54
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
        "typeText": "() => { test: <T = undefined>(value: T) => T; extraValue: () => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 26,
        "length": 3,
        "line": 2,
        "character": 7,
        "name": "fn1"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 68,
        "length": 5,
        "line": 3,
        "character": 27,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { test: <T = undefined>(value: T) => Promise<T>; extraValue: () => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 131,
        "length": 8,
        "line": 7,
        "character": 7,
        "name": "fn1async"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 184,
        "length": 5,
        "line": 8,
        "character": 33,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { test: <T>(value: T) => T; extraValue: () => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 256,
        "length": 3,
        "line": 12,
        "character": 7,
        "name": "fn2"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 286,
        "length": 5,
        "line": 13,
        "character": 15,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { test: <T>(value: T) => Promise<T>; extraValue: () => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 349,
        "length": 8,
        "line": 17,
        "character": 7,
        "name": "fn2async"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 390,
        "length": 5,
        "line": 18,
        "character": 21,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { extraValue: () => void; test: <T = undefined>(value: T) => T; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 462,
        "length": 3,
        "line": 22,
        "character": 7,
        "name": "fn3"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 531,
        "length": 5,
        "line": 24,
        "character": 27,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { extraValue: () => void; test: <T = undefined>(value: T) => Promise<T>; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 567,
        "length": 8,
        "line": 27,
        "character": 7,
        "name": "fn3async"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 647,
        "length": 5,
        "line": 29,
        "character": 33,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { extraValue: string; test: <T = undefined>(value: T) => T; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 692,
        "length": 3,
        "line": 32,
        "character": 7,
        "name": "fn4"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 755,
        "length": 5,
        "line": 34,
        "character": 27,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "() => { extraValue: string; test: <T = undefined>(value: T) => Promise<T>; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 791,
        "length": 8,
        "line": 37,
        "character": 7,
        "name": "fn4async"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingGenericInObject.ts",
        "start": 865,
        "length": 5,
        "line": 39,
        "character": 33,
        "name": "value"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const fn1 = () => ({\r\n    test: <T = undefined>(value: T): T => value,\r\n    extraValue: () => {},\r\n})",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn1async = () => ({\r\n    test: async <T = undefined>(value: T): Promise<T> => value,\r\n    extraValue: () => {},\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn2 = () => ({\r\n    test: <T>(value: T): T => value,\r\n    extraValue: () => {},\r\n})",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn2async = () => ({\r\n    test: async <T>(value: T): Promise<T> => value,\r\n    extraValue: () => {},\r\n})",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn3 = () => ({\r\n    extraValue: () => {},\r\n    test: <T = undefined>(value: T): T => value,\r\n})",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn3async = () => ({\r\n    extraValue: () => {},\r\n    test: async <T = undefined>(value: T): Promise<T> => value,\r\n}",
        "line": 27,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn4 = () => ({\r\n    extraValue: '',\r\n    test: <T = undefined>(value: T): T => value,\r\n})",
        "line": 32,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn4async = () => ({\r\n    extraValue: '',\r\n    test: async <T = undefined>(value: T): Promise<T> => value,\r\n})",
        "line": 37,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const fn1 = () => ({\r\n    test: <T = undefined>(value: T): T => value,\r\n    extraValue: () => {},\r\n})\r\n\r\nconst fn1async ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fn1 = () => ({\r\n    test: <T = undefined>(value: T): T => value,\r\n    extraValue: () => {},\r\n})",
        "line": 2,
        "character": 1
      },
      {
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 52, end: 53 } }) at 53..54
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
