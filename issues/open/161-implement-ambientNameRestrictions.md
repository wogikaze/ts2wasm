---
id: 161
title: "Implement Ambientnamerestrictions"
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

Triage ambientNameRestrictions across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientNameRestrictions` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientNameRestrictions has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts
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

- `reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientNameRestrictions

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 78,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "export declare namespace Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 20..26",
  "span_start": 20,
  "span_end": 26,
  "line": 2,
  "column": 2,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | export declare namespace Foo {
3 |   export var static: any;
4 | }
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
    "path": "issues/open/161-implement-ambientNameRestrictions.md",
    "title": "Implement Ambientnamerestrictions",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Export,
        span: Span {
            start: 20,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 27,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 35,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 45,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 54,
            end: 60,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 20..26
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 20..26
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
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientNameRestrictions.ts",
        "start": 65,
        "length": 6,
        "line": 3,
        "character": 14,
        "name": "static"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "export declare namespace Foo {\r\n  export var static: any;\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export declare namespace Foo {\r\n  export var static: any;\r\n}\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "export declare namespace Foo {\r\n  export var static: any;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 20..26
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
