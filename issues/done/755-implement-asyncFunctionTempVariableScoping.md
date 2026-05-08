---
id: 755
title: "Implement Asyncfunctiontempvariablescoping"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncFunctionTempVariableScoping across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionTempVariableScoping` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionTempVariableScoping has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: asyncFunctionTempVariableScoping

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionTempVariableScoping.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 163,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "async ({ foo, bar, ...rest }) => bar(await foo);"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 120, end: 125 } }) at 126..127",
  "span_start": 126,
  "span_end": 127,
  "line": 6,
  "column": 12,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @lib: es2015
4 | // https://github.com/Microsoft/TypeScript/issues/19187
5 | 
6 | async ({ foo, bar, ...rest }) => bar(await foo);
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
    "path": "issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md",
    "title": "Implement Arraytolocalestringes Unknown Unsupported",
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
        kind: Async,
        span: Span {
            start: 120,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 129,
            end: 132,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 134,
            end: 137,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: DotDotDot,
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "rest",
        ),
        span: Span {
            start: 142,
            end: 146,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 150,
            end: 152,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 153,
            end: 156,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: Await,
        span: Span {
            start: 157,
            end: 162,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 163,
            end: 166,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 167,
            end: 168,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 120, end: 125 } }) at 126..127
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 120, end: 125 } }) at 126..127
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "async ({ foo, bar, ...rest }) => bar(await foo);",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "async ({ foo, bar, ...rest }) => bar(await foo);",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "async ({ foo, bar, ...rest }) => bar(await foo);",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "async ({ foo, bar, ...rest }) => bar(await foo)",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 120, end: 125 } }) at 126..127
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
