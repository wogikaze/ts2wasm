---
id: 083
title: "Implement Abstractclassinlocalscopeisabstract"
type: spike
area: frontend/resolver
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage abstractClassInLocalScopeIsAbstract across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractClassInLocalScopeIsAbstract` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractClassInLocalScopeIsAbstract has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts
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

- `reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage scope analysis: abstractClassInLocalScopeIsAbstract

- Issue class: `triage-needed`
- Feature label: `scope-analysis`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 108,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "(() => {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 43..48",
  "span_start": 43,
  "span_end": 48,
  "line": 3,
  "column": 16,
  "feature_label": "scope-analysis",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | (() => {
3 |     abstract class A {}
4 |     class B extends A {}
5 |     new A();
6 |     new B();
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
    "path": "issues/open/083-implement-abstractClassInLocalScopeIsAbstract.md",
    "title": "Implement Abstractclassinlocalscopeisabstract",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: A
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
        kind: LeftParen,
        span: Span {
            start: 20,
            end: 21,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 21,
            end: 22,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 22,
            end: 23,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 24,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 34,
            end: 42,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 43,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 59,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 67,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 43..48
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 43..48
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
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassInLocalScopeIsAbstract.ts",
        "start": 85,
        "length": 7,
        "line": 5,
        "character": 5
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n})()",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n})()\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n})()",
        "line": 2,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n})()",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n})",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n}",
        "line": 2,
        "character": 2
      },
      {
        "kind": "Block",
        "text": "{\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new A();\r\n    new B();\r\n}",
        "line": 2,
        "character": 8
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class A {}",
        "line": 3,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 43..48
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
