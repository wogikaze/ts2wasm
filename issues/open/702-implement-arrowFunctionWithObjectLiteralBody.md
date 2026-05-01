---
id: 702
title: "Implement Arrowfunctionwithobjectliteralbody"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrowFunctionWithObjectLiteralBody across 6 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 6 cases fail in directory `arrowFunctionWithObjectLiteralBody` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionWithObjectLiteralBody has 6 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts --detail
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
mise run reference-coverage -- tsc --limit 12
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts`
- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody1.ts`
- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody2.ts`
- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody5.ts`
- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody6.ts`
- `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody4.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage object literal: arrowFunctionWithObjectLiteralBody3

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 54,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var v = a => <any>{}"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 49, end: 50 } }) at 50..53",
  "span_start": 50,
  "span_end": 53,
  "line": 3,
  "column": 17,
  "feature_label": "object-literal",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @strict: false
2 | // @target: es6
3 | var v = a => <any>{}
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "v",
    "line": 3,
    "column": 1,
    "initializer": "a => <an"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/374-design-broader-object-toprimitive-for-bigint-comparisons.md",
    "title": "Design broader object ToPrimitive for mixed BigInt comparisons",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/441-implement-object-literal.md",
    "title": "Implement object literal enhancements",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/355-dynamic-object-enumeration-spread.md",
    "title": "Implement dynamic object property enumeration spread",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md",
    "title": "Implement BigInt object ToPrimitive non-BigInt primitive returns",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md",
    "title": "Handle BigInt object ToPrimitive invalid and out-of-range string returns",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 36,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 46,
            end: 48,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 50,
            end: 53,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 49, end: 50 } }) at 50..53
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 49, end: 50 } }) at 50..53
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
        "typeText": "(a: any) => any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts",
        "start": 40,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionWithObjectLiteralBody3.ts",
        "start": 44,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var v = a => <any>{}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var v = a => <any>{}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var v = a => <any>{}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var v = a => <any>{}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "v = a => <any>{}",
        "line": 3,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "a => <any>{}",
        "line": 3,
        "character": 9
      },
      {
        "kind": "TypeAssertionExpression",
        "text": "<any>{}",
        "line": 3,
        "character": 14
      },
      {
        "kind": "AnyKeyword",
        "text": "any",
        "line": 3,
        "character": 15
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 49, end: 50 } }) at 50..53
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
