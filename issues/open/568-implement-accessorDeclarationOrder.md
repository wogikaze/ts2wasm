---
id: 568
title: "Implement Accessordeclarationorder (audit reopened #568)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage accessorDeclarationOrder across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorDeclarationOrder` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorDeclarationOrder has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts
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

- `reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts`

## Duplicate detection

- `issues/done/098-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same reference path, same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)
- `issues/open/482-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class accessor: accessorDeclarationOrder

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 431,
  "lines": 35,
  "extension": ".ts",
  "first_code_line": "class C1 {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Colon) at 40..41",
  "span_start": 40,
  "span_end": 41,
  "line": 4,
  "column": 10,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: esnext
2 | 
3 | class C1 {
4 |     #name: string;
5 | 
6 |     public get name() {
7 |         return this.#name;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C1",
    "line": 3,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/098-implement-accessorDeclarationOrder.md",
    "title": "Implement Accessordeclarationorder",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/482-implement-accessorDeclarationOrder.md",
    "title": "Implement Accessordeclarationorder",
    "reason": "same reference path, same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: C1
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
            "C1",
        ),
        span: Span {
            start: 26,
            end: 28,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: PrivateIdentifier(
            "name",
        ),
        span: Span {
            start: 35,
            end: 40,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 62,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 66,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 90,
            end: 94,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: PrivateIdentifier(
            "name",
        ),
        span: Span {
            start: 95,
            end: 100,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "private",
        ),
        span: Span {
            start: 113,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 121,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 130,
            end: 134,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 134,
            end: 135,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 40..41
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 40..41
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
        "code": 2564,
        "category": "Error",
        "message": "Property '#name' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 35,
        "length": 5,
        "line": 4,
        "character": 5
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property '#name' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 197,
        "length": 5,
        "line": 16,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 130,
        "length": 4,
        "line": 10,
        "character": 22,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 234,
        "length": 4,
        "line": 18,
        "character": 22,
        "name": "name"
      },
      {
        "kind": "binding",
        "typeText": "C1",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 350,
        "length": 2,
        "line": 27,
        "character": 7,
        "name": "c1"
      },
      {
        "kind": "binding",
        "typeText": "C2",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationOrder.ts",
        "start": 371,
        "length": 2,
        "line": 28,
        "character": 7,
        "name": "c2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C1 {\n    #name: string;\n\n    public get name() {\n        return this.#name;\n    }\n\n    private set name(name: stri",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C2 {\n    #name: string;\n\n    private set name(name: string) {\n        this.#name = name;\n    }\n\n    public get nam",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c1 = new C1();",
        "line": 27,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c2 = new C2();",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c1.name;",
        "line": 32,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c2.name;",
        "line": 35,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C1 {\n    #name: string;\n\n    public get name() {\n        return this.#name;\n    }\n\n    private set name(name: stri",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C1 {\n    #name: string;\n\n    public get name() {\n        return this.#name;\n    }\n\n    private set name(name: stri",
        "line": 3,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "#name: string;",
        "line": 4,
        "character": 5
      },
      {
        "kind": "PrivateIdentifier",
        "text": "#name",
        "line": 4,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 40..41
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

Superseded by issue #098. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/568-implement-accessorDeclarationOrder.md` before this move
- `issues/open/568-implement-accessorDeclarationOrder.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
