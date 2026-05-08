---
id: 707
title: "Implement Asibreak"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage asiBreak across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiBreak` with diagnostics: break-continue. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiBreak has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiBreak.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiBreak.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiBreak.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiBreak.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5133-implement-single-statement-loop-body-break-continue.md`

## Notes

Split to implementation-ready child issue
`issues/done/5133-implement-single-statement-loop-body-break-continue.md`.

## Affected test files

- `reference/typescript/tests/cases/compiler/asiBreak.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage break continue: asiBreak

- Issue class: `triage-needed`
- Feature label: `break-continue`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asiBreak.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiBreak.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 40,
  "lines": 2,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Break) at 35..40",
  "span_start": 35,
  "span_end": 40,
  "line": 2,
  "column": 16,
  "feature_label": "break-continue",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | ﻿// @target: es2015
2 | while (true) break
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
    "path": "issues/open/418-implement-break-continue.md",
    "title": "Implement break/continue",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/035-implement-break-continue.md",
    "title": "Implement break and continue statements",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/209-implement-labeled-break-continue.md",
    "title": "Implement labeled break and continue",
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
        kind: While,
        span: Span {
            start: 22,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 29,
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
        kind: Break,
        span: Span {
            start: 35,
            end: 40,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Break) at 35..40
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Break) at 35..40
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
        "kind": "WhileStatement",
        "text": "while (true) break",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "while (true) break",
        "line": 2,
        "character": 1
      },
      {
        "kind": "WhileStatement",
        "text": "while (true) break",
        "line": 2,
        "character": 1
      },
      {
        "kind": "BreakStatement",
        "text": "break",
        "line": 2,
        "character": 14
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Break) at 35..40
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

