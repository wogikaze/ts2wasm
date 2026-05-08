---
id: 659
title: "Implement Argumentsusedinclassfieldinitializerorstaticinitializationblock"
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

Triage argumentsUsedInClassFieldInitializerOrStaticInitializationBlock across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsUsedInClassFieldInitializerOrStaticInitializationBlock` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsUsedInClassFieldInitializerOrStaticInitializationBlock has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts
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

- `reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage class: argumentsUsedInClassFieldInitializerOrStaticInitializationBlock

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1523,
  "lines": 113,
  "extension": ".ts",
  "first_code_line": "function A() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 43, end: 48 } }) at 49..50",
  "span_start": 49,
  "span_end": 50,
  "line": 3,
  "column": 16,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | function A() {
3 |   return class T {
4 |      a = arguments
5 |   }
6 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "A",
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
    "path": "issues/open/045-implement-class-syntax.md",
    "title": "Implement class declaration and expression",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/248-implement-private-class-element-parser.md",
    "title": "Implement private class element parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/249-implement-class-static-block-parser.md",
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
// Candidate source class: T
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
            start: 19,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 36,
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
            "T",
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
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 62,
            end: 71,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 79,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "A1",
        ),
        span: Span {
            start: 88,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 97,
            end: 103,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 104,
            end: 107,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 108,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 43, end: 48 } }) at 49..50
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 43, end: 48 } }) at 49..50
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
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 62,
        "length": 9,
        "line": 4,
        "character": 10
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 127,
        "length": 9,
        "line": 10,
        "character": 10
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 192,
        "length": 9,
        "line": 16,
        "character": 15
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 264,
        "length": 9,
        "line": 22,
        "character": 15
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 408,
        "length": 9,
        "line": 34,
        "character": 16
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 499,
        "length": 9,
        "line": 41,
        "character": 7
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 568,
        "length": 9,
        "line": 43,
        "character": 16
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 1035,
        "length": 9,
        "line": 76,
        "character": 7
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 1088,
        "length": 9,
        "line": 78,
        "character": 9
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 1312,
        "length": 9,
        "line": 97,
        "character": 26
      },
      {
        "code": 2815,
        "category": "Error",
        "message": "'arguments' cannot be referenced in property initializers or class static initialization blocks.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 1401,
        "length": 9,
        "line": 103,
        "character": 15
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "typeof T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 28,
        "length": 1,
        "line": 2,
        "character": 10,
        "name": "A"
      },
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 88,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "A1"
      },
      {
        "kind": "function",
        "typeText": "typeof T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 153,
        "length": 1,
        "line": 14,
        "character": 10,
        "name": "B"
      },
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 220,
        "length": 2,
        "line": 20,
        "character": 10,
        "name": "B1"
      },
      {
        "kind": "function",
        "typeText": "typeof T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 292,
        "length": 1,
        "line": 26,
        "character": 10,
        "name": "C"
      },
      {
        "kind": "function",
        "typeText": "typeof T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 368,
        "length": 1,
        "line": 32,
        "character": 10,
        "name": "D"
      },
      {
        "kind": "function",
        "typeText": "typeof T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 451,
        "length": 2,
        "line": 38,
        "character": 10,
        "name": "D1"
      },
      {
        "kind": "binding",
        "typeText": "() => any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 541,
        "length": 1,
        "line": 42,
        "character": 13,
        "name": "b"
      },
      {
        "kind": "function",
        "typeText": "IArguments",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInClassFieldInitializerOrStaticInitializationBlock.ts",
        "start": 623,
        "length": 1,
        "line": 46,
        "character": 16,
        "name": "f"
      },
      {
        "kind": "function",
        "typeText": "typeof (Anonymous class)",
        "file": "/
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Class, span: Span { start: 43, end: 48 } }) at 49..50
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
