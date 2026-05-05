---
id: 184
title: "Implement Anydeclare"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5003]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage anyDeclare across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyDeclare` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyDeclare has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyDeclare.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyDeclare.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyDeclare.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyDeclare.ts
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

- `reference/typescript/tests/cases/compiler/anyDeclare.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage declaration emit: anyDeclare

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anyDeclare.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyDeclare.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 98,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "declare var x: any;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Var) at 28..31",
  "span_start": 28,
  "span_end": 31,
  "line": 2,
  "column": 10,
  "feature_label": "declaration-emit",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare var x: any;
3 | namespace myMod {
4 |     var myFn;
5 |     function myFn() {  }
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
    "path": "issues/done/184-implement-anyDeclare.md",
    "title": "Implement Anydeclare",
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
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 28,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 35,
            end: 38,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 41,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "myMod",
        ),
        span: Span {
            start: 51,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 64,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "myFn",
        ),
        span: Span {
            start: 68,
            end: 72,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 79,
            end: 87,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 28..31
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 28..31
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'myFn'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyDeclare.ts",
        "start": 68,
        "length": 4,
        "line": 4,
        "character": 9
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'myFn'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyDeclare.ts",
        "start": 88,
        "length": 4,
        "line": 5,
        "character": 14
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyDeclare.ts",
        "start": 32,
        "length": 1,
        "line": 2,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyDeclare.ts",
        "start": 68,
        "length": 4,
        "line": 4,
        "character": 9,
        "name": "myFn"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyDeclare.ts",
        "start": 88,
        "length": 4,
        "line": 5,
        "character": 14,
        "name": "myFn"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare var x: any;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace myMod {\r\n    var myFn;\r\n    function myFn() {  }\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare var x: any;\r\nnamespace myMod {\r\n    var myFn;\r\n    function myFn() {  }\r\n}\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var x: any;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var x: any",
        "line": 2,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 28..31
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/642-implement-anyDeclare.md` に統合されました。
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
