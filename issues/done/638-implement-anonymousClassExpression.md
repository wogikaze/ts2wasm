---
id: 638
title: "Implement Anonymousclassexpression"
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

Triage anonymousClassExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonymousClassExpression` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonymousClassExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts
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

- `reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts`

## Duplicate detection

- `issues/done/180-implement-anonymousClassExpression.md` - Implement Anonymousclassexpression (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class: anonymousClassExpression1

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 78,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "function f() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 54, end: 59 } }) at 60..61",
  "span_start": 60,
  "span_end": 61,
  "line": 3,
  "column": 27,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | function f() {
3 |     return typeof class {} === "function";
4 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f",
    "line": 2,
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
    "path": "issues/done/180-implement-anonymousClassExpression.md",
    "title": "Implement Anonymousclassexpression",
    "reason": "same reference path, same feature label"
  },
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 40,
            end: 46,
        },
    },
    SpannedToken {
        kind: TypeOf,
        span: Span {
            start: 47,
            end: 53,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 54,
            end: 59,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: StrictEqual,
        span: Span {
            start: 63,
            end: 66,
        },
    },
    SpannedToken {
        kind: String(
            "function",
        ),
        span: Span {
            start: 67,
            end: 77,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 54, end: 59 } }) at 60..61
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 54, end: 59 } }) at 60..61
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
        "kind": "function",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousClassExpression1.ts",
        "start": 29,
        "length": 1,
        "line": 2,
        "character": 10,
        "name": "f"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    return typeof class {} === \"function\";\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function f() {\r\n    return typeof class {} === \"function\";\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    return typeof class {} === \"function\";\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    return typeof class {} === \"function\";\r\n}",
        "line": 2,
        "character": 14
      },
      {
        "kind": "ReturnStatement",
        "text": "return typeof class {} === \"function\";",
        "line": 3,
        "character": 5
      },
      {
        "kind": "BinaryExpression",
        "text": "typeof class {} === \"function\"",
        "line": 3,
        "character": 12
      },
      {
        "kind": "TypeOfExpression",
        "text": "typeof class {}",
        "line": 3,
        "character": 12
      },
      {
        "kind": "ClassExpression",
        "text": "class {}",
        "line": 3,
        "character": 19
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 54, end: 59 } }) at 60..61
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

## Status

Superseded by issue #180. Test now build_pass after anonymous class expression fix.
