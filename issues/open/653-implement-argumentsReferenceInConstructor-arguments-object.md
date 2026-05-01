---
id: 653
title: "Implement Argumentsreferenceinconstructor Arguments Object"
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

Triage argumentsReferenceInConstructor-arguments-object across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail in directory `argumentsReferenceInConstructor-arguments-object` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsReferenceInConstructor-arguments-object has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts --detail
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts
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

- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts`
- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor6_Js.ts`
- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor7_Js.ts`

## Duplicate detection

- `issues/open/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, title overlap)

## Smart triage

### Smart triage: Triage arguments object: argumentsReferenceInConstructor5 Js

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 348,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "const bar = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 142..147",
  "span_start": 142,
  "span_end": 147,
  "line": 11,
  "column": 1,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 8 |  arguments: {}
 9 | }
10 | 
11 | class A {
12 |  /**
13 |   * Constructor
14 |   *
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "bar",
    "line": 7,
    "column": 1,
    "initializer": "{"
  }
]
```

Duplicate candidates:

```json
[
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
        kind: Const,
        span: Span {
            start: 110,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 116,
            end: 119,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 125,
            end: 134,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 142,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 212,
            end: 223,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 224,
            end: 227,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 231,
            end: 232,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 232,
            end: 233,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 234,
            end: 235,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 268,
            end: 272,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 272,
            end: 273,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 273,
            end: 276,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 277,
            end: 278,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 279,
            end: 282,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 282,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 142..147
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 142..147
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
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts",
        "start": 273,
        "length": 3,
        "line": 21,
        "character": 8
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'bar' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts",
        "start": 322,
        "length": 3,
        "line": 26,
        "character": 8
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ arguments: {}; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts",
        "start": 116,
        "length": 3,
        "line": 7,
        "character": 7,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts",
        "start": 224,
        "length": 3,
        "line": 17,
        "character": 14,
        "name": "foo"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const bar = {\n\targuments: {}\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A {\n\t/**\n\t * Constructor\n\t *\n\t * @param {object} [foo={}]\n\t */\n\tconstructor(foo = {}) {\n\t\t/**\n\t\t * @type object\n\t\t",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const bar = {\n\targuments: {}\n}\n\nclass A {\n\t/**\n\t * Constructor\n\t *\n\t * @param {object} [foo={}]\n\t */\n\tconstructor(foo = ",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A {\n\t/**\n\t * Constructor\n\t *\n\t * @param {object} [foo={}]\n\t */\n\tconstructor(foo = {}) {\n\t\t/**\n\t\t * @type object\n\t\t",
        "line": 11,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 142..147
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
