---
id: 673
title: "Implement Arrayevery"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayEvery across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayEvery` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayEvery has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayEvery.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayEvery.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayEvery.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayEvery.ts
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

- `reference/typescript/tests/cases/compiler/arrayEvery.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage class: arrayEvery

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayEvery.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayEvery.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 180,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "const foo: (number | string)[] = ['aaa'];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: unknown receiver class for method `every` at 141..160",
  "span_start": 141,
  "span_end": 160,
  "line": 6,
  "column": 10,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | 
4 | const isString = (x: unknown): x is string => typeof x === 'string';
5 | 
6 | if (foo.every(isString)) {
7 |   foo[0].slice(0);
8 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "foo",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "isString",
    "line": 4,
    "column": 1,
    "initializer": "(x: unknown): x is string => typeof x === 'string'"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/255-implement-private-class-element-runtime-semantics.md",
    "title": "Implement private class element runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/312-triage-test262-blocked-p0-window.md",
    "title": "Triage test262 blocked P0 window",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/421-implement-class.md",
    "title": "Implement class syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/045-implement-class-syntax.md",
    "title": "Implement class declaration and expression",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/248-implement-private-class-element-parser.md",
    "title": "Implement private class element parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/249-implement-class-static-block-parser.md",
    "title": "Implement class static block parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/254-implement-class-static-block-runtime-semantics.md",
    "title": "Implement class static block runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/289-resolve-callcount-binding-in-class-destructuring.md",
    "title": "Resolve callCount binding in class destructuring tests",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/292-resolve-initcount-binding-in-class-destructuring.md",
    "title": "Resolve initCount binding in class destructuring defaults",
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
        kind: Const,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 26,
            end: 29,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 32,
            end: 38,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 41,
            end: 47,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: String(
            "aaa",
        ),
        span: Span {
            start: 54,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 65,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "isString",
        ),
        span: Span {
            start: 71,
            end: 79,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "unknown",
        ),
        span: Span {
            start: 86,
            end: 93,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "is",
        ),
        span: Span {
            start: 98
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "foo",
        expr: Array {
            elements: [
                Present(
                    String {
                        value: "aaa",
                        span: Span {
                            start: 54,
                            end: 59,
                        },
                    },
                ),
            ],
            span: Span {
                start: 53,
                end: 60,
            },
        },
        span: Span {
            start: 20,
            end: 61,
        },
    },
    Let {
        name: "isString",
        expr: ArrowFn {
            params: [
                "x",
            ],
            body: Binary {
                left: TypeOf {
                    expr: Ident {
                        name: "x",
                        span: Span {
                            start: 118,
                            end: 119,
                        },
                    },
                    span: Span {
                        start: 111,
                        end: 119,
                    },
                },
                op: StrictEqual,
                right: String {
                    value: "string",
                    span: Span {
                        start: 124,
                        end: 132,
                    },
                },
                span: Span {
                    start: 111,
                    end: 132,
                },
            },
            span: Span {
                start: 82,
                end: 132,
            },
        },
        span: Span {
            start: 65,
            end: 133,
        },
    },
    If {
        condition: Call {
            callee: Member {
                object: Ident {
                    name: "foo",
                    span: Span {
                        start: 141,
                        end: 144,
                    },
                },
                property: "every",
                span: Span {
                    start: 141,
                    end: 150,
                },
            },
            args: [
                Ident {
                    name: "isString",
                    span: Span {
                        start: 151,
                        end: 159,
                    },
                },
            ],
            span: Span {
                start: 141,
                end: 160,
            },
        },
        then_body: [
            Expr {
                expr: Call {
                    callee: Member {
                        object: Index {
                            object: Ident {
                                name: "foo",
                                span: Span {
                                    start: 167,
                                    end: 170,
                                },
                            },
                            index: Number {
                                value: 0,
                                span: Span {
                                    start: 171,
                                    end: 172,
                                },
                            },
                            span: Span {
                                start: 167,
                                end: 173,
                            },
                        },
                        property: "slice",
                        span: Span {
                            start: 167,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `every` at 141..160
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
        "typeText": "(string | number)[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayEvery.ts",
        "start": 26,
        "length": 3,
        "line": 2,
        "character": 7,
        "name": "foo"
      },
      {
        "kind": "binding",
        "typeText": "(x: unknown) => x is string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayEvery.ts",
        "start": 71,
        "length": 8,
        "line": 4,
        "character": 7,
        "name": "isString"
      },
      {
        "kind": "parameter",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayEvery.ts",
        "start": 83,
        "length": 1,
        "line": 4,
        "character": 19,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const foo: (number | string)[] = ['aaa'];",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const isString = (x: unknown): x is string => typeof x === 'string';",
        "line": 4,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (foo.every(isString)) {\r\n  foo[0].slice(0);\r\n}",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const foo: (number | string)[] = ['aaa'];\r\n\r\nconst isString = (x: unknown): x is string => typeof x === 'string';\r\n\r\nif ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (foo.every(isString)) {\r\n  foo[0].slice(0);\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "foo.every(isString)",
        "line": 6,
        "character": 5
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "foo.every",
        "line": 6,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 6,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `every` at 141..160
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
