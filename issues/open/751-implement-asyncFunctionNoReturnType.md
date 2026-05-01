---
id: 751
title: "Implement Asyncfunctionnoreturntype"
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

Triage asyncFunctionNoReturnType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionNoReturnType` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionNoReturnType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: asyncFunctionNoReturnType

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionNoReturnType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 95,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "async () => {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 47, end: 52 } }) at 53..54",
  "span_start": 53,
  "span_end": 54,
  "line": 3,
  "column": 7,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @noImplicitReturns: true
3 | async () => {
4 |     if (window)
5 |         return;
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
            start: 47,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 56,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: If,
        span: Span {
            start: 65,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "window",
        ),
        span: Span {
            start: 69,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 85,
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
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 47, end: 52 } }) at 53..54
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 47, end: 52 } }) at 53..54
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
        "text": "async () => {\n    if (window)\n        return;\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "async () => {\n    if (window)\n        return;\n}\n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "async () => {\n    if (window)\n        return;\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "async () => {\n    if (window)\n        return;\n}",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 47, end: 52 } }) at 53..54
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
