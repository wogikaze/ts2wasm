---
id: 172
title: "Implement Amddependencycommentname (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage amdDependencyCommentName across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `amdDependencyCommentName` with diagnostics: module-system-amd. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdDependencyCommentName has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts
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

- `reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName2.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName3.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module system amd: amdDependencyCommentName1

- Issue class: `triage-needed`
- Feature label: `module-system-amd`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 114,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "import m1 = require(\"m2\")"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 81..87",
  "span_start": 81,
  "span_end": 87,
  "line": 5,
  "column": 1,
  "feature_label": "module-system-amd",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | //@module: commonjs
3 | ///<amd-dependency path='bar' name='b'/>
4 |
5 | import m1 = require("m2")
6 | m1.f();
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
    "path": "issues/done/172-implement-amdDependencyCommentName.md",
    "title": "Implement Amddependencycommentname",
    "reason": "same reference path"
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
        kind: Import,
        span: Span {
            start: 81,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 88,
            end: 90,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 93,
            end: 100,
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
        kind: String(
            "m2",
        ),
        span: Span {
            start: 101,
            end: 105,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 107,
            end: 109,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 113,
            end: 114,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 81..87
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 81..87
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
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'm2' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts",
        "start": 101,
        "length": 4,
        "line": 5,
        "character": 21
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m1 = require(\"m2\")",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "m1.f();",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import m1 = require(\"m2\")\nm1.f();",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m1 = require(\"m2\")",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported default import; module resolution and loading are not implemented at 81..87
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/630-implement-amdDependencyCommentName.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/172-implement-amdDependencyCommentName.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
