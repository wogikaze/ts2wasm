---
id: 106
title: "Implement Accessors"
type: spike
area: reference
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage accessors across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `accessors` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessors has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts --detail
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

- unrelated runtime/backend code unless the triage report proves the failure is not parser/frontend

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
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

- `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts`
- `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_inference.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class accessor: accessors spec section 4.5 error cases

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 562,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "class LanguageSpec_section_4_5_error_cases {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"set\")) at 77..80",
  "span_start": 77,
  "span_end": 80,
  "line": 3,
  "column": 14,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class LanguageSpec_section_4_5_error_cases {
3 |     public set AnnotatedSetter_SetterFirst(a: number) { }
4 |     public get AnnotatedSetter_SetterFirst() { return ""; }
5 |
6 |     public get AnnotatedSetter_SetterLast() { return ""; }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "LanguageSpec_section_4_5_error_cases",
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
    "path": "issues/open/106-implement-accessors.md",
    "title": "Implement Accessors",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/060-investigate-unknown-unsupported-cases.md",
    "title": "Investigate and classify unknown-unsupported diagnostic cases",
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
// Candidate source class: LanguageSpec_section_4_5_error_cases
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
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "LanguageSpec_section_4_5_error_cases",
        ),
        span: Span {
            start: 26,
            end: 62,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 70,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 77,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "AnnotatedSetter_SetterFirst",
        ),
        span: Span {
            start: 81,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 112,
            end: 118,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Right
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 172,
        "length": 6,
        "line": 4,
        "character": 48
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 234,
        "length": 6,
        "line": 6,
        "character": 47
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 428,
        "length": 4,
        "line": 10,
        "character": 52
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 493,
        "length": 4,
        "line": 12,
        "character": 51
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 109,
        "length": 1,
        "line": 3,
        "character": 44,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 290,
        "length": 1,
        "line": 7,
        "character": 43,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 420,
        "length": 4,
        "line": 10,
        "character": 44,
        "name": "aStr"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 485,
        "length": 4,
        "line": 12,
        "character": 43,
        "name": "aStr"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "SetAccessor",
        "text": "public set AnnotatedSetter_SetterFirst(a: number) { }",
        "line": 3,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
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
