---
id: 573
title: "Implement Accessorwithoutbody"
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

Triage accessorWithoutBody across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `accessorWithoutBody` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorWithoutBody has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts
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

- `reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts`
- `reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts`

## Duplicate detection

- `issues/open/105-implement-accessorWithoutBody.md` - Implement Accessorwithoutbody (same reference path, same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)
- `issues/open/487-implement-accessorWithoutBody.md` - Implement Accessorwithoutbody (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class accessor: accessorWithoutBody2

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 64,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var v = { set foo(a) }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(Ident(\"foo\")) at 58..61",
  "span_start": 58,
  "span_end": 61,
  "line": 3,
  "column": 17,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @strict: false
2 | // @target: ES5, ES2015
3 | var v = { set foo(a) }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "v",
    "line": 3,
    "column": 1,
    "initializer": "{ set fo"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/105-implement-accessorWithoutBody.md",
    "title": "Implement Accessorwithoutbody",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/487-implement-accessorWithoutBody.md",
    "title": "Implement Accessorwithoutbody",
    "reason": "same reference path, same feature label"
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 44,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 54,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 58,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
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
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts",
        "start": 63,
        "length": 1,
        "line": 3,
        "character": 20
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ foo: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts",
        "start": 62,
        "length": 1,
        "line": 3,
        "character": 19,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var v = { set foo(a) }",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var v = { set foo(a) }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var v = { set foo(a) }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var v = { set foo(a) }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "v = { set foo(a) }",
        "line": 3,
        "character": 5
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{ set foo(a) }",
        "line": 3,
        "character": 9
      },
      {
        "kind": "SetAccessor",
        "text": "set foo(a)",
        "line": 3,
        "character": 11
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 3,
        "character": 15
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
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
