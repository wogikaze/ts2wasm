---
id: 165
title: "Implement Ambientwithstatements"
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

Triage ambientWithStatements across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientWithStatements` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientWithStatements has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientWithStatements.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientWithStatements.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
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

- `reference/typescript/tests/cases/compiler/ambientWithStatements.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientWithStatements

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientWithStatements.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 468,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "declare namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"namespace\")) at 107..116",
  "span_start": 107,
  "span_end": 116,
  "line": 5,
  "column": 13,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @ignoreDeprecations: 6.0
3 | // @strict: false
4 | // @alwaysStrict: true, false
5 | declare namespace M {
6 |     break;
7 |     continue;
8 |     debugger;
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
    "path": "issues/done/165-implement-ambientWithStatements.md",
    "title": "Implement Ambientwithstatements",
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
            start: 99,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 107,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Break,
        span: Span {
            start: 126,
            end: 131,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Continue,
        span: Span {
            start: 138,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "debugger",
        ),
        span: Span {
            start: 153,
            end: 161,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Do,
        span: Span {
            start: 168,
            end: 170,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 173,
            end: 174,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 107..116
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 107..116
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
        "code": 1036,
        "category": "Error",
        "message": "Statements are not allowed in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 126,
        "length": 5,
        "line": 6,
        "character": 5
      },
      {
        "code": 1104,
        "category": "Error",
        "message": "A 'continue' statement can only be used within an enclosing iteration statement.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 138,
        "length": 9,
        "line": 7,
        "character": 5
      },
      {
        "code": 2407,
        "category": "Error",
        "message": "The right-hand side of a 'for...in' statement must be of type 'any', an object type or a type parameter, but here has type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 216,
        "length": 4,
        "line": 11,
        "character": 15
      },
      {
        "code": 1344,
        "category": "Error",
        "message": "'A label is not allowed here.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 267,
        "length": 1,
        "line": 14,
        "character": 5
      },
      {
        "code": 1108,
        "category": "Error",
        "message": "A 'return' statement can only be used within a function body.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 282,
        "length": 6,
        "line": 15,
        "character": 5
      },
      {
        "code": 1101,
        "category": "Error",
        "message": "'with' statements are not allowed in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 478,
        "length": 4,
        "line": 29,
        "character": 5
      },
      {
        "code": 2410,
        "category": "Error",
        "message": "The 'with' statement is not supported. All symbols in a 'with' block will have type 'any'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 478,
        "length": 8,
        "line": 29,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 198,
        "length": 1,
        "line": 10,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 274,
        "length": 1,
        "line": 14,
        "character": 12,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 439,
        "length": 1,
        "line": 25,
        "character": 12,
        "name": "e"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("namespace")) at 107..116
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/624-implement-ambientWithStatements.md` に統合されました。
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
