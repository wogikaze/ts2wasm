---
id: 094
title: "Implement Accessoraccidentalcalldiagnostic"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage accessorAccidentalCallDiagnostic across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorAccidentalCallDiagnostic` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorAccidentalCallDiagnostic has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
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

- `reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class accessor: accessorAccidentalCallDiagnostic

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 204,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "class Test24554 {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"property\")) at 107..115",
  "span_start": 107,
  "span_end": 115,
  "line": 5,
  "column": 9,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 |
3 | // https://github.com/microsoft/TypeScript/issues/24554
4 | class Test24554 {
5 |     get property(): number { return 1; }
6 | }
7 | function test24554(x: Test24554) {
8 |     return x.property();
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Test24554",
    "line": 4,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/094-implement-accessorAccidentalCallDiagnostic.md",
    "title": "Implement Accessoraccidentalcalldiagnostic",
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
// Candidate source class: Test24554
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
        kind: Class,
        span: Span {
            start: 81,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "Test24554",
        ),
        span: Span {
            start: 87,
            end: 96,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 103,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "property",
        ),
        span: Span {
            start: 107,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 119,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 128,
            end: 134,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 136,
            end: 137,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("property")) at 107..115
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("property")) at 107..115
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
        "code": 6234,
        "category": "Error",
        "message": "This expression is not callable because it is a 'get' accessor. Did you mean to use it without '()'?\n  Type 'Number' has no call signatures.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts",
        "start": 190,
        "length": 8,
        "line": 8,
        "character": 14
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts",
        "start": 151,
        "length": 9,
        "line": 7,
        "character": 10,
        "name": "test24554"
      },
      {
        "kind": "parameter",
        "typeText": "Test24554",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts",
        "start": 161,
        "length": 1,
        "line": 7,
        "character": 20,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Test24554 {\n    get property(): number { return 1; }\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test24554(x: Test24554) {\n    return x.property();\n}",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Test24554 {\n    get property(): number { return 1; }\n}\nfunction test24554(x: Test24554) {\n    return x.property();",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Test24554 {\n    get property(): number { return 1; }\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "GetAccessor",
        "text": "get property(): number { return 1; }",
        "line": 5,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "property",
        "line": 5,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("property")) at 107..115
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
