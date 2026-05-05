---
id: 074
title: "Implement Declarationerrorsnoemitonerror"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-05-06
status: done
---

## Summary

Triage DeclarationErrorsNoEmitOnError across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `DeclarationErrorsNoEmitOnError` with diagnostics: type-annotation. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: DeclarationErrorsNoEmitOnError has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts --detail
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

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
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

## Triage findings

2026-05-06:

- This generated bucket contains one reference case and must not be selected directly.
- The embedded smart triage evidence shows the concrete parser gap:
  - Path: `reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts`
  - Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
  - Failure: `unterminated TypeScript type alias declaration at 92..96`
  - Source context: `type T = { x : number }` followed by `export interface I { f: T; }`
- Child issue 5140 owns the executable parser slice for semicolonless TypeScript object-type alias parsing before an exported interface.

## Affected test files

- `reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type annotation: DeclarationErrorsNoEmitOnError

- Issue class: `triage-needed`
- Feature label: `type-annotation`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 146,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "type T = { x : number }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unterminated TypeScript type alias declaration at 92..96",
  "span_start": 92,
  "span_end": 96,
  "line": 6,
  "column": 6,
  "feature_label": "type-annotation",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @declaration: true
4 | // @noEmitOnError: true
5 |
6 | type T = { x : number }
7 | export interface I {
8 |     f: T;
9 | }
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
    "path": "issues/done/074-implement-DeclarationErrorsNoEmitOnError.md",
    "title": "Implement Declarationerrorsnoemitonerror",
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
            start: 92,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 107,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 117,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 124,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "I",
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unterminated TypeScript type alias declaration at 92..96
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unterminated TypeScript type alias declaration at 92..96
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
        "kind": "TypeAliasDeclaration",
        "text": "type T = { x : number }",
        "line": 6,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "export interface I {\r\n    f: T;   \r\n}",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "type T = { x : number }\r\nexport interface I {\r\n    f: T;   \r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type T = { x : number }",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unterminated TypeScript type alias declaration at 92..96
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- issue-state commit closing this generated bucket after child issue 5140 split

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-06
```

Remaining risks:

- The parser implementation remains open in issue 5140.
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
