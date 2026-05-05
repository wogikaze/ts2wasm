---
id: 145
title: "Implement Ambientenum"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [059]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage ambientEnum across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientEnum` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientEnum has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
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

- `reference/typescript/tests/cases/compiler/ambientEnum1.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientEnum1

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientEnum1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 162,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "declare enum E1 {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"enum\")) at 32..36",
  "span_start": 32,
  "span_end": 36,
  "line": 2,
  "column": 14,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 |     declare enum E1 {
3 |         y = 4.23
4 |     }
5 |
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
    "path": "issues/open/145-implement-ambientEnum.md",
    "title": "Implement Ambientenum",
    "reason": "same reference path"
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
            "declare",
        ),
        span: Span {
            start: 24,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 32,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "E1",
        ),
        span: Span {
            start: 37,
            end: 39,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Number(
            4,
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Number(
            23,
        ),
        span: Span {
            start: 57,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 120,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 128,
            end: 132,
        },
    },
    SpannedToken {
        kind: Ident(
            "E2"
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
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
        "code": 1066,
        "category": "Error",
        "message": "In ambient enum declarations member initializer must be constant expression.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientEnum1.ts",
        "start": 151,
        "length": 12,
        "line": 8,
        "character": 13
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }",
        "line": 2,
        "character": 5
      },
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E2 {\r\n        x = 'foo'.length\r\n    }",
        "line": 7,
        "character": 5
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }\r\n    \r\n    // Ambient enum with computer member\r\n    declare enum E2 {\r\n     ",
        "line": 2,
        "character": 5
      },
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }",
        "line": 2,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
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
---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/done/` as part of a batch close cycle without actual triage completion. The done/ copy only differs from open/ in checkbox state ([ ] → [x]) with no "Status" note, no child issues created, no implementation commits, and empty completion evidence. The checkboxes were batch-checked without evidence that the triage was actually performed.

**True-done checklist** (all must pass):

1. Perform actual triage review of the reference failure case
2. Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. Fill in completion evidence section with triage results
4. Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
