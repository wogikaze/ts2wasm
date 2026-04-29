---
id: 080
title: "Implement Systemmoduleforstatementnoinitializer"
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

Triage SystemModuleForStatementNoInitializer across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `SystemModuleForStatementNoInitializer` with diagnostics: module-system-amd. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: SystemModuleForStatementNoInitializer has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts
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

- `reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module system amd: SystemModuleForStatementNoInitializer

- Issue class: `triage-needed`
- Feature label: `module-system-amd`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 173,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "export { };"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Increment, span: Span { start: 105, end: 107 } }) at 107..108",
  "span_start": 107,
  "span_end": 108,
  "line": 10,
  "column": 4,
  "feature_label": "module-system-amd",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 7 | let limit = 10;
 8 |
 9 | for (; i < limit; ++i) {
10 |     break;
11 | }
12 |
13 | for (; ; ++i) {
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "i",
    "line": 6,
    "column": 1,
    "initializer": "0"
  },
  {
    "kind": "binding",
    "name": "limit",
    "line": 7,
    "column": 1,
    "initializer": "10"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/080-implement-SystemModuleForStatementNoInitializer.md",
    "title": "Implement Systemmoduleforstatementnoinitializer",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

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
            start: 41,
            end: 47,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 56,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 68,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "limit",
        ),
        span: Span {
            start: 72,
            end: 77,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Number(
            10,
        ),
        span: Span {
            start: 80,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Increment, span: Span { start: 105, end: 107 } }) at 107..108
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Increment, span: Span { start: 105, end: 107 } }) at 107..108
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
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts",
        "start": 60,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts",
        "start": 72,
        "length": 5,
        "line": 7,
        "character": 5,
        "name": "limit"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExportDeclaration",
        "text": "export { };",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let i = 0;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let limit = 10;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ForStatement",
        "text": "for (; i < limit; ++i) {\r\n    break;\r\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ForStatement",
        "text": "for (; ; ++i) {\r\n    break;\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ForStatement",
        "text": "for (; ;) {\r\n    break;\r\n}",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export { };\r\n\r\nlet i = 0;\r\nlet limit = 10;\r\n\r\nfor (; i < limit; ++i) {\r\n    break;\r\n}\r\n\r\nfor (; ; ++i) {\r\n    break;\r\n}\r",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ForStatement",
        "text": "for (; i < limit; ++i) {\r\n    break;\r\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "PrefixUnaryExpression",
        "text": "++i",
        "line": 9,
        "character": 19
      },
      {
        "kind": "Identifier",
        "text": "i",
        "line": 9,
        "character": 21
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Increment, span: Span { start: 105, end: 107 } }) at 107..108
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
