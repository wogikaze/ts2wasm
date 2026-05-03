---
id: 183
title: "Implement Anyasreturntypefornewoncall"
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

Triage anyAsReturnTypeForNewOnCall across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyAsReturnTypeForNewOnCall` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyAsReturnTypeForNewOnCall has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
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

- `reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class: anyAsReturnTypeForNewOnCall

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 135,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "function Point(x, y) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: `this` is only supported inside receiver-bound class constructors and instance methods at 62..66",
  "span_start": 62,
  "span_end": 66,
  "line": 5,
  "column": 2,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: false
3 | function Point(x, y) {
4 |
5 |  this.x = x;
6 |
7 |  this.y = y;
8 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "Point",
    "line": 3,
    "column": 1,
    "params": "x, y"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/183-implement-anyAsReturnTypeForNewOnCall.md",
    "title": "Implement Anyasreturntypefornewoncall",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/045-implement-class-syntax.md",
    "title": "Implement class declaration and expression",
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
        kind: Function,
        span: Span {
            start: 37,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "Point",
        ),
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 62,
            end: 66,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        ki
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "Point",
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
                expr: PropertyAssign {
                    object: This {
                        span: Span {
                            start: 62,
                            end: 66,
                        },
                    },
                    property: "x",
                    value: Ident {
                        name: "x",
                        span: Span {
                            start: 71,
                            end: 72,
                        },
                    },
                    span: Span {
                        start: 62,
                        end: 73,
                    },
                },
                span: Span {
                    start: 62,
                    end: 73,
                },
            },
            Expr {
                expr: PropertyAssign {
                    object: This {
                        span: Span {
                            start: 76,
                            end: 80,
                        },
                    },
                    property: "y",
                    value: Ident {
                        name: "y",
                        span: Span {
                            start: 85,
                            end: 86,
                        },
                    },
                    span: Span {
                        start: 76,
                        end: 87,
                    },
                },
                span: Span {
                    start: 76
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: `this` is only supported inside receiver-bound class constructors and instance methods at 62..66
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
        "code": 2683,
        "category": "Error",
        "message": "'this' implicitly has type 'any' because it does not have a type annotation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 62,
        "length": 4,
        "line": 5,
        "character": 2
      },
      {
        "code": 2683,
        "category": "Error",
        "message": "'this' implicitly has type 'any' because it does not have a type annotation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 76,
        "length": 4,
        "line": 7,
        "character": 2
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 46,
        "length": 5,
        "line": 3,
        "character": 10,
        "name": "Point"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 55,
        "length": 1,
        "line": 3,
        "character": 19,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 96,
        "length": 1,
        "line": 11,
        "character": 5,
        "name": "o"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 122,
        "length": 2,
        "line": 13,
        "character": 5,
        "name": "xx"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function Point(x, y) {\n\n this.x = x;\n\n this.y = y;\n\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var o = new Point(3, 4);",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var xx = o.x;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function Point(x, y) {\n\n this.x = x;\n\n this.y = y;\n\n}\n\nvar o = new Point(3, 4);\n\nvar xx = o.x;\n\n \n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function Point(x, y) {\n\n this.x = x;\n\n this.y = y;\n\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n\n this.x = x;\n\n this.y = y;\n\n}",
        "line": 3,
        "character": 22
      },
      {
        "kind": "ExpressionStatement",
        "text": "this.x = x;",
        "line": 5,
        "character": 2
      },
      {
        "kind": "BinaryExpression",
        "text": "this.x = x",
        "line": 5,
        "character": 2
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "this.x",
        "line": 5,
        "character": 2
      },
      {
        "kind": "ThisKeyword",
        "text": "this",
        "line": 5,
        "character": 2
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: `this` is only supported inside receiver-bound class constructors and instance methods at 62..66
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
