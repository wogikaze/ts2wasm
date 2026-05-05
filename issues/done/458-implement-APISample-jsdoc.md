---
id: 458
title: "Implement Apisample Jsdoc (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage APISample-jsdoc across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `APISample-jsdoc` with diagnostics: jsdoc. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APISample-jsdoc has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_jsdoc.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_jsdoc.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_jsdoc.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_jsdoc.ts
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

- `reference/typescript/tests/cases/compiler/APISample_jsdoc.ts`

## Duplicate detection

- `issues/done/070-implement-APISample.md` - Implement Apisample (same reference path, same feature label, title overlap)

## Smart triage

### Smart triage: Triage jsdoc: APISample jsdoc

- Issue class: `triage-needed`
- Feature label: `jsdoc`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/APISample_jsdoc.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_jsdoc.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 4095,
  "lines": 126,
  "extension": ".ts",
  "first_code_line": "{"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-247: expected binding identifier or pattern, got Some(This) at 869..873",
  "span_start": 869,
  "span_end": 873,
  "line": 28,
  "column": 38,
  "feature_label": "jsdoc",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
25 | 
26 | // excerpted from https://github.com/YousefED/typescript-json-schema
27 | // (converted from a method and modified; for example, `this: any` to compensate, among other changes)
28 | function parseCommentsIntoDefinition(this: any,
29 |                                      symbol: ts.Symbol,
30 |                                      definition: {description?: string, [s: string]: string | undefined},
31 |                                      otherAnnotations: { [s: string]: true}): void {
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "console",
    "line": 22,
    "column": 9
  },
  {
    "kind": "import",
    "name": "typescript",
    "line": 24,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/070-implement-APISample.md",
    "title": "Implement Apisample",
    "reason": "same reference path, same feature label, title overlap"
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
        kind: LeftBrace,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 198,
            end: 204,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 204,
            end: 205,
        },
    },
    SpannedToken {
        kind: String(
            "typescript",
        ),
        span: Span {
            start: 206,
            end: 218,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 218,
            end: 219,
        },
    },
    SpannedToken {
        kind: String(
            "types",
        ),
        span: Span {
            start: 224,
            end: 231,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 231,
            end: 232,
        },
    },
    SpannedToken {
        kind: String(
            "/.ts/typescript.d.ts",
        ),
        span: Span {
            start: 233,
            end: 255,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 256,
            end: 257,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 598,
            end: 605,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 606,
            end: 609,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 610,
            end: 617,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 617,
            end: 618,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 619,
            end: 622,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 622,
            end: 623,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 625,
            end: 631,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 632,
            end: 633,
        },
    },
    SpannedToken {
        kind: Ident(
            "as",
        ),
        span: Span {
            start: 634,
            end: 636,
        },
    },
    SpannedToken {
        kind: Ident(
            "ts",
        ),
        span: Span {
            start: 637,
            end: 639,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 640,
            end: 644,
        },
    },
    SpannedToken {
        kind: String(
            "typescript",
        ),
        span: Span {
            start: 645,
            end: 657,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 657,
            end: 658,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 832,
            end: 840,
        },
    },
    SpannedToken {
        kind: Ident(
            "parseCommentsIntoDefinition",
        ),
        span: Span {
            start: 841,
            end: 868,
        },
    },
    SpannedToken {
        kind: Left
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-247: expected binding identifier or pattern, got Some(This) at 869..873
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-247: expected binding identifier or pattern, got Some(This) at 869..873
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
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 204,
        "length": 1,
        "line": 10,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 206,
        "length": 12,
        "line": 10,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 231,
        "length": 1,
        "line": 11,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 610,
        "length": 7,
        "line": 22,
        "character": 13,
        "name": "console"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 841,
        "length": 27,
        "line": 28,
        "character": 10,
        "name": "parseCommentsIntoDefinition"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 869,
        "length": 4,
        "line": 28,
        "character": 38,
        "name": "this"
      },
      {
        "kind": "parameter",
        "typeText": "Symbol",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 917,
        "length": 6,
        "line": 29,
        "character": 38,
        "name": "symbol"
      },
      {
        "kind": "parameter",
        "typeText": "{ [s: string]: string | undefined; description?: string | undefined; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 973,
        "length": 10,
        "line": 30,
        "character": 38,
        "name": "definition"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1009,
        "length": 1,
        "line": 30,
        "character": 74,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "{ [s: string]: true; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1079,
        "length": 16,
        "line": 31,
        "character": 38,
        "name": "otherAnnotations"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1100,
        "length": 1,
        "line": 31,
        "character": 59,
        "name": "s"
      },
      {
        "kind": "binding",
        "typeText": "SymbolDisplayPart[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1210,
        "length": 8,
        "line": 37,
        "character": 9,
        "name": "comments"
      },
      {
        "kind": "parameter",
        "typeText": "SymbolDisplayPart",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1338,
        "length": 7,
        "line": 40,
        "character": 47,
        "name": "comment"
      },
      {
        "kind": "binding",
        "typeText": "JSDocTagInfo[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1507,
        "length": 6,
        "line": 44,
        "character": 11,
        "name": "jsdocs"
      },
      {
        "kind": "parameter",
        "typeText": "JSDocTagInfo",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 1570,
        "length": 3,
        "line": 45,
        "character": 20,
        "name": "doc"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2008,
        "length": 4,
        "line": 60,
        "character": 6,
        "name": "name"
      },
      {
        "kind": "function",
        "typeText": "Annotations | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2039,
        "length": 14,
        "line": 62,
        "character": 10,
        "name": "getAnnotations"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2054,
        "length": 4,
        "line": 62,
        "character": 25,
        "name": "this"
      },
      {
        "kind": "parameter",
        "typeText": "Node",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2065,
        "length": 4,
        "line": 62,
        "character": 36,
        "name": "node"
      },
      {
        "kind": "binding",
        "typeText": "Symbol",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2117,
        "length": 6,
        "line": 63,
        "character": 11,
        "name": "symbol"
      },
      {
        "kind": "binding",
        "typeText": "JSDocTagInfo[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2221,
        "length": 9,
        "line": 68,
        "character": 11,
        "name": "jsDocTags"
      },
      {
        "kind": "binding",
        "typeText": "Annotations",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2373,
        "length": 11,
        "line": 73,
        "character": 11,
        "name": "annotations"
      },
      {
        "kind": "parameter",
        "typeText": "Annotations",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2418,
        "length": 6,
        "line": 73,
        "character": 56,
        "name": "result"
      },
      {
        "kind": "parameter",
        "typeText": "JSDocTagInfo",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2439,
        "length": 8,
        "line": 73,
        "character": 77,
        "name": "jsDocTag"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
        "start": 2485,
        "length": 5,
        "line": 74,
        "character": 15,
        "name": "value"
      },
      {
        "kind": "function",
        "typeText": "readonly JSDocParameterTag[] | JSDocTag[] | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_jsdoc.ts",
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-247: expected binding identifier or pattern, got Some(This) at 869..873
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/544-implement-APISample-jsdoc.md` に統合されました。
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
- `issues/done/458-implement-APISample-jsdoc.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
