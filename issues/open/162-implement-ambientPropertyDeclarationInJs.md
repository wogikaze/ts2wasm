---
id: 162
title: "Implement Ambientpropertydeclarationinjs"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage ambientPropertyDeclarationInJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientPropertyDeclarationInJs` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientPropertyDeclarationInJs has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
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

- `reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientPropertyDeclarationInJs

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 231,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "class Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"prop\")) at 171..175",
  "span_start": 171,
  "span_end": 175,
  "line": 12,
  "column": 13,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 9 |         this.prop = {};
10 |     }
11 |
12 |     declare prop: string;
13 |
14 |     method() {
15 |         this.prop.foo
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 7,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/162-implement-ambientPropertyDeclarationInJs.md",
    "title": "Implement Ambientpropertydeclarationinjs",
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
        kind: Class,
        span: Span {
            start: 96,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 102,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 112,
            end: 123,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 136,
            end: 140,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop",
        ),
        span: Span {
            start: 141,
            end: 145,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Semico
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
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
        "code": 2322,
        "category": "Error",
        "message": "Type '{}' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts",
        "start": 136,
        "length": 9,
        "line": 9,
        "character": 9
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts",
        "start": 219,
        "length": 3,
        "line": 15,
        "character": 19
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "declare prop: string;",
        "line": 12,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "prop",
        "line": 12,
        "character": 13
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
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
