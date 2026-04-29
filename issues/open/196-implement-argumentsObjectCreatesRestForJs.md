---
id: 196
title: "Implement Argumentsobjectcreatesrestforjs"
type: spike
area: reference
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage argumentsObjectCreatesRestForJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsObjectCreatesRestForJs` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

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

- unrelated runtime/backend code unless the triage report proves the failure is not parser/frontend

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

## Smart triage

### Smart triage: Triage name resolution: argumentsObjectCreatesRestForJs

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
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
  "code": "UnresolvedName",
  "message": "unresolved name: `arguments` at 133..142",
  "span_start": 133,
  "span_end": 142,
  "line": 7,
  "column": 22,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 4 | // @allowJs: true
 5 | // @Filename: main.js
 6 | // @noemit: true
 7 | function allRest() { arguments; }
 8 | allRest();
 9 | allRest(1, 2, 3);
10 | function someRest(x, y) { arguments; }
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
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/196-implement-argumentsObjectCreatesRestForJs.md",
    "title": "Implement Argumentsobjectcreatesrestforjs",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Automatic repair sketch:

```rust
// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});
```

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
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `arguments` at 133..142
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
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function allRest() { arguments; }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "allRest();",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "allRest(1, 2, 3);",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function someRest(x, y) { arguments; }",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "someRest();",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "someRest(1, 2, 3);",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function jsdocced(x) { arguments; }",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "jsdocced(1);",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function dontDoubleRest(x, ...y) { arguments; }",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "dontDoubleRest(1, 2, 3);",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function allRest() { arguments; }\nallRest();\nallRest(1, 2, 3);\nfunction someRest(x, y) { arguments; }\nsomeRest(); // x a",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function allRest() { arguments; }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{ arguments; }",
        "line": 7,
        "character": 20
      },
      {
        "kind": "ExpressionStatement",
        "text": "arguments;",
        "line": 7,
        "character": 22
      },
      {
        "kind": "Identifier",
        "text": "arguments",
        "line": 7,
        "character": 22
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `arguments` at 133..142
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
