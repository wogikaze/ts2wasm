---
id: 650
title: "Implement Argumentsobjectcreatesrestforjs"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage argumentsObjectCreatesRestForJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsObjectCreatesRestForJs` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsObjectCreatesRestForJs has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts
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

- `reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts`

## Duplicate detection

- `issues/open/196-implement-argumentsObjectCreatesRestForJs.md` - Implement Argumentsobjectcreatesrestforjs (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage arguments object: argumentsObjectCreatesRestForJs

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 469,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "function allRest() { arguments; }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-062d: `arguments` together with rest parameters is not supported in this milestone",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
// @strict: false
// @checkJs: true
// @allowJs: true
// @Filename: main.js
// @noemit: true
function allRest() { arguments; }
allRest();
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "allRest",
    "line": 7,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "someRest",
    "line": 10,
    "column": 1,
    "params": "x, y"
  },
  {
    "kind": "function",
    "name": "jsdocced",
    "line": 17,
    "column": 1,
    "params": "x"
  },
  {
    "kind": "function",
    "name": "dontDoubleRest",
    "line": 20,
    "column": 1,
    "params": "x, ...y"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/196-implement-argumentsObjectCreatesRestForJs.md",
    "title": "Implement Argumentsobjectcreatesrestforjs",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/311-fix-test262-arguments-object-index-assignment.md",
    "title": "Fix test262 arguments object index assignment semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/646-implement-arguments.md",
    "title": "Implement Arguments",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/647-implement-argumentsAsPropertyName-arguments-object.md",
    "title": "Implement Argumentsaspropertyname Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/291-provide-object-global-binding-for-test262.md",
    "title": "Provide Object global binding for test262 cases",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 112,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "allRest",
        ),
        span: Span {
            start: 121,
            end: 128,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 133,
            end: 142,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "allRest",
        ),
        span: Span {
            start: 146,
            end: 153,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "allRest",
        ),
        span: Span {
            start: 157,
            end: 164,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 175,
            end: 183,
        },
    },
    SpannedToken {
        kind: Ident(
            "someRest",
        ),
        span: Span {
            start: 184,
            end: 192,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 193,
            end: 194,
        },
    },
    SpannedToken {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "allRest",
        params: [],
        body: [
            Expr {
                expr: Ident {
                    name: "arguments",
                    span: Span {
                        start: 133,
                        end: 142,
                    },
                },
                span: Span {
                    start: 133,
                    end: 143,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 112,
            end: 143,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "allRest",
                span: Span {
                    start: 146,
                    end: 153,
                },
            },
            args: [],
            span: Span {
                start: 146,
                end: 155,
            },
        },
        span: Span {
            start: 146,
            end: 156,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "allRest",
                span: Span {
                    start: 157,
                    end: 164,
                },
            },
            args: [
                Number {
                    value: 1,
                    span: Span {
                        start: 165,
                        end: 166,
                    },
                },
                Number {
                    value: 2,
                    span: Span {
                        start: 168,
                        end: 169,
                    },
                },
                Number {
                    value: 3,
                    span: Span {
                        start: 171,
                        end: 172,
                    },
                },
            ],
            span: Span {
                start: 157,
                end: 173,
            },
        },
        span: Span {
            start: 157,
            end: 174,
        },
    },
    Function {
        name: "someRest",
        params: [
            (
                "x",
                None,
                false,
            ),
            (
                "y",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: Ident {
                    name: "arguments",
                    span: Span {
                        start: 201,
                        end: 210,
                    },
                },
                span: Span {
                    start: 201,
                    end: 211,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 175,
            end: 211,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "someRest",
                span: Span {
                    start: 214,
                    end: 222,
                },
            },
            args: [],
            span: Span {
                start: 214,
                end: 224,
            },
        },
        span: Span {
            start: 214,
            end: 225,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "someRest",
                span: Span {
                    start: 286,
                    end: 294,
                },
            },
            args: [
                Number {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-062d: `arguments` together with rest parameters is not supported in this milestone
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
        "code": 2554,
        "category": "Error",
        "message": "Expected 0 arguments, but got 3.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 165,
        "length": 7,
        "line": 9,
        "character": 9
      },
      {
        "code": 2554,
        "category": "Error",
        "message": "Expected 2 arguments, but got 0.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 214,
        "length": 8,
        "line": 11,
        "character": 1
      },
      {
        "code": 2554,
        "category": "Error",
        "message": "Expected 2 arguments, but got 3.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 301,
        "length": 1,
        "line": 12,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 121,
        "length": 7,
        "line": 7,
        "character": 10,
        "name": "allRest"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 184,
        "length": 8,
        "line": 10,
        "character": 10,
        "name": "someRest"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 193,
        "length": 1,
        "line": 10,
        "character": 19,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 196,
        "length": 1,
        "line": 10,
        "character": 22,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 354,
        "length": 8,
        "line": 17,
        "character": 10,
        "name": "jsdocced"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 363,
        "length": 1,
        "line": 17,
        "character": 19,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 404,
        "length": 14,
        "line": 20,
        "character": 10,
        "name": "dontDoubleRest"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 419,
        "length": 1,
        "line": 20,
        "character": 25,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectCreatesRestForJs.ts",
        "start": 425,
        "length": 1,
        "line": 20,
        "character": 31,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
