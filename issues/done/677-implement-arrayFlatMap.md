---
id: 677
title: "Implement Arrayflatmap"
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

Triage arrayFlatMap across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFlatMap` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFlatMap has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatMap.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFlatMap.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFlatMap.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatMap.ts
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

- `reference/typescript/tests/cases/compiler/arrayFlatMap.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage class: arrayFlatMap

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayFlatMap.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatMap.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 229,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "const array: number[] = [];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: unknown receiver class for method `flatMap` at 118..164",
  "span_start": 118,
  "span_end": 164,
  "line": 6,
  "column": 6,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | 
4 | const array: number[] = [];
5 | const readonlyArray: ReadonlyArray<number> = [];
6 | array.flatMap((): ReadonlyArray<number> => []); // ok
7 | readonlyArray.flatMap((): ReadonlyArray<number> => []); // ok
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "array",
    "line": 4,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "readonlyArray",
    "line": 5,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/255-implement-private-class-element-runtime-semantics.md",
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
            start: 39,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "array",
        ),
        span: Span {
            start: 45,
            end: 50,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 52,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 58,
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
        kind: Equal,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 68,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "readonlyArray",
        ),
        span: Span {
            start: 74,
            end: 87,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "ReadonlyArray",
        ),
        span: Span {
            start: 89,
            end: 102,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 103,
            end: 109,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "array",
        ),
        span: Span {
            start: 118,
            end: 123,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "flatMap",
        ),
        span: Span {
            start: 124,
            end: 131,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 132,
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "array",
        expr: Array {
            elements: [],
            span: Span {
                start: 63,
                end: 65,
            },
        },
        span: Span {
            start: 39,
            end: 66,
        },
    },
    Let {
        name: "readonlyArray",
        expr: Array {
            elements: [],
            span: Span {
                start: 113,
                end: 115,
            },
        },
        span: Span {
            start: 68,
            end: 116,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "array",
                    span: Span {
                        start: 118,
                        end: 123,
                    },
                },
                property: "flatMap",
                span: Span {
                    start: 118,
                    end: 131,
                },
            },
            args: [
                ArrowFn {
                    params: [],
                    body: Array {
                        elements: [],
                        span: Span {
                            start: 161,
                            end: 163,
                        },
                    },
                    span: Span {
                        start: 132,
                        end: 163,
                    },
                },
            ],
            span: Span {
                start: 118,
                end: 164,
            },
        },
        span: Span {
            start: 118,
            end: 165,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "readonlyArray",
                    span: Span {
                        start: 173,
                        end: 186,
                    },
                },
                property: "flatMap",
                span: Span {
                    start: 173,
                    end: 194,
                },
            },
            args: [
                ArrowFn {
                    params: [],
                    body: Array {
                        elements: [],
                        span: Span {
                            start: 224,
                            end: 226,
                        },
                    },
                    span: Span {
                        start: 195,
                        end: 226,
                    },
                },
            ],
            span: Span {
                start: 173,
                end: 227,
            },
        },
        span: Span {
            start: 173,
            end: 228,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `flatMap` at 118..164
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
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFlatMap.ts",
        "start": 45,
        "length": 5,
        "line": 4,
        "character": 7,
        "name": "array"
      },
      {
        "kind": "binding",
        "typeText": "readonly number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFlatMap.ts",
        "start": 74,
        "length": 13,
        "line": 5,
        "character": 7,
        "name": "readonlyArray"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const array: number[] = [];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const readonlyArray: ReadonlyArray<number> = [];",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "array.flatMap((): ReadonlyArray<number> => []);",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "readonlyArray.flatMap((): ReadonlyArray<number> => []);",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const array: number[] = [];\r\nconst readonlyArray: ReadonlyArray<number> = [];\r\narray.flatMap((): ReadonlyArray<number> =",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "array.flatMap((): ReadonlyArray<number> => []);",
        "line": 6,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "array.flatMap((): ReadonlyArray<number> => [])",
        "line": 6,
        "character": 1
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "array.flatMap",
        "line": 6,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "array",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `flatMap` at 118..164
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
