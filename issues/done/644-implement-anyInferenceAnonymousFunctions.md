---
id: 644
title: "Implement Anyinferenceanonymousfunctions"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage anyInferenceAnonymousFunctions across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyInferenceAnonymousFunctions` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyInferenceAnonymousFunctions has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts
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

- `reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts`

## Duplicate detection

- `issues/done/187-implement-anyInferenceAnonymousFunctions.md` - Implement Anyinferenceanonymousfunctions (same reference path, same feature label, same group key, title overlap)
- `issues/done/453-implement-type-system.md` - Implement type-system support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage type system: anyInferenceAnonymousFunctions

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 300,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "var paired: any[];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: unknown receiver class for method `reduce` at 42..115",
  "span_start": 42,
  "span_end": 115,
  "line": 4,
  "column": 4,
  "feature_label": "type-system",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var paired: any[];
3 | 
4 | paired.reduce(function (a1, a2) {
5 | 
6 |     return a1.concat({});
7 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "paired",
    "line": 2,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/187-implement-anyInferenceAnonymousFunctions.md",
    "title": "Implement Anyinferenceanonymousfunctions",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: Example
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub constructor: Option<FunctionDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
    let span = self.expect(TokenKind::Class)?;
    let name = self.expect_ident()?;
    self.expect(TokenKind::LeftBrace)?;
    let mut methods = Vec::new();
    while !self.consume(TokenKind::RightBrace) {
        methods.push(self.class_method()?);
    }
    Ok(Stmt::ClassDecl(ClassDecl { name, constructor: None, methods, span }))
}
```

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "paired",
        ),
        span: Span {
            start: 24,
            end: 30,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 32,
            end: 35,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: Ident(
            "paired",
        ),
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "reduce",
        ),
        span: Span {
            start: 49,
            end: 55,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 56,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "a1",
        ),
        span: Span {
            start: 66,
            end: 68,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "a2",
        ),
        span: Span {
            start: 70,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "a1",
        ),
        span: Span {
            start: 90,
            end: 92,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "concat",
        ),
        span: Span {
            start: 93,
            end: 99,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 102,
            end: 103,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "paired",
        expr: Undefined {
            span: Span {
                start: 24,
                end: 30,
            },
        },
        span: Span {
            start: 20,
            end: 38,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "paired",
                    span: Span {
                        start: 42,
                        end: 48,
                    },
                },
                property: "reduce",
                span: Span {
                    start: 42,
                    end: 55,
                },
            },
            args: [
                FunctionExpr {
                    name: "",
                    params: [
                        (
                            "a1",
                            None,
                            false,
                        ),
                        (
                            "a2",
                            None,
                            false,
                        ),
                    ],
                    body: [
                        Return {
                            expr: Call {
                                callee: Member {
                                    object: Ident {
                                        name: "a1",
                                        span: Span {
                                            start: 90,
                                            end: 92,
                                        },
                                    },
                                    property: "concat",
                                    span: Span {
                                        start: 90,
                                        end: 99,
                                    },
                                },
                                args: [
                                    Object {
                                        props: [],
                                        span: Span {
                                            start: 100,
                                            end: 102,
                                        },
                                    },
                                ],
                                span: Span {
                                    start: 90,
                                    end: 103,
                                },
                            },
                            span: Span {
                                start: 83,
                                end: 104,
                            },
                        },
                    ],
                    span: Span {
                        start: 56,
                        end: 104,
                    },
                },
                Array {
                    elements: [],
                    span: Span {
                        start: 112,
                        end: 114,
                    },
                },
            ],
            span: Span {
                start: 42,
                end: 115,
            },
        },
        span: Span {
            start: 42,
            end: 116,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "paired",
                    span: Span {
                        start: 120,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `reduce` at 42..115
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'paired' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 42,
        "length": 6,
        "line": 4,
        "character": 1
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'paired' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 120,
        "length": 6,
        "line": 10,
        "character": 1
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'paired' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 190,
        "length": 6,
        "line": 15,
        "character": 1
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'paired' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 239,
        "length": 6,
        "line": 17,
        "character": 1
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'paired' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 270,
        "length": 6,
        "line": 18,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 24,
        "length": 6,
        "line": 2,
        "character": 5,
        "name": "paired"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 66,
        "length": 2,
        "line": 4,
        "character": 25,
        "name": "a1"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 70,
        "length": 2,
        "line": 4,
        "character": 29,
        "name": "a2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 135,
        "length": 2,
        "line": 10,
        "character": 16,
        "name": "b1"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 139,
        "length": 2,
        "line": 10,
        "character": 20,
        "name": "b2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 205,
        "length": 2,
        "line": 15,
        "character": 16,
        "name": "b3"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 209,
        "length": 2,
        "line": 15,
        "character": 20,
        "name": "b4"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 251,
        "length": 2,
        "line": 17,
        "character": 13,
        "name": "c1"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyInferenceAnonymousFunctions.ts",
        "start": 291,
        "length": 2,
        "line": 18,
        "character": 22,
        "name": "c2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var paired: any[];",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.reduce(function (a1, a2) {\r\n\r\n    return a1.concat({});\r\n\r\n} , []);",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.reduce((b1, b2) => {\r\n\r\n    return b1.concat({});\r\n} , []);",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.reduce((b3, b4) => b3.concat({}), []);",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.map((c1) => c1.count);",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.map(function (c2) { return c2.count; });",
        "line": 18,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var paired: any[];\r\n\r\npaired.reduce(function (a1, a2) {\r\n\r\n    return a1.concat({});\r\n\r\n} , []);\r\n\r\npaired.reduce((b1, b",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "paired.reduce(function (a1, a2) {\r\n\r\n    return a1.concat({});\r\n\r\n} , []);",
        "line": 4,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "paired.reduce(function (a1, a2) {\r\n\r\n    return a1.concat({});\r\n\r\n} , [])",
        "line": 4,
        "character": 1
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "paired.reduce",
        "line": 4,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "paired",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `reduce` at 42..115
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
