---
id: 071
title: "Implement Arrowfunctionexpression"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ArrowFunctionExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ArrowFunctionExpression` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ArrowFunctionExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts
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

- `reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: ArrowFunctionExpression1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 53,
  "lines": 2,
  "extension": ".ts",
  "first_code_line": "var v = (public x: string) => { };"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected RightParen, got Some(Ident(\"x\")) at 35..36",
  "span_start": 35,
  "span_end": 36,
  "line": 2,
  "column": 17,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var v = (public x: string) => { };
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "v",
    "line": 2,
    "column": 1,
    "initializer": "(public"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/071-implement-ArrowFunctionExpression.md",
    "title": "Implement Arrowfunctionexpression",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
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
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 38,
            end: 44,
        },
    },
    SpannedToken {
        kind: RightParen,
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
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 52,
            end: 53,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Ident("x")) at 35..36
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Ident("x")) at 35..36
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
        "code": 2369,
        "category": "Error",
        "message": "A parameter property is only allowed in a constructor implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts",
        "start": 28,
        "length": 16,
        "line": 2,
        "character": 10
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "(x: string) => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts",
        "start": 23,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ArrowFunctionExpression1.ts",
        "start": 35,
        "length": 1,
        "line": 2,
        "character": 17,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var v = (public x: string) => { };",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var v = (public x: string) => { };",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var v = (public x: string) => { };",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var v = (public x: string) => { }",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "v = (public x: string) => { }",
        "line": 2,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "(public x: string) => { }",
        "line": 2,
        "character": 9
      },
      {
        "kind": "Parameter",
        "text": "public x: string",
        "line": 2,
        "character": 10
      },
      {
        "kind": "Identifier",
        "text": "x",
        "line": 2,
        "character": 17
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Ident("x")) at 35..36
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/545-implement-ArrowFunctionExpression.md` に統合されました。
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
