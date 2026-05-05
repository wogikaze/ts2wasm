---
id: 551
title: "Implement Memberaccessordeclaration (audit reopened #551)"
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

Triage MemberAccessorDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `MemberAccessorDeclaration` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: MemberAccessorDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts
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

- `reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts`

## Duplicate detection

- `issues/done/078-implement-MemberAccessorDeclaration.md` - Implement Memberaccessordeclaration (same reference path, same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)
- `issues/open/465-implement-MemberAccessorDeclaration.md` - Implement Memberaccessordeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class accessor: MemberAccessorDeclaration15

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 63,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Ident(\"a\")) at 49..50",
  "span_start": 49,
  "span_end": 50,
  "line": 3,
  "column": 21,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class C {
3 |    set Foo(public a: number) { }
4 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
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
    "path": "issues/done/078-implement-MemberAccessorDeclaration.md",
    "title": "Implement Memberaccessordeclaration",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/465-implement-MemberAccessorDeclaration.md",
    "title": "Implement Memberaccessordeclaration",
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
// Candidate source class: C
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
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 34,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 38,
            end: 41,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 49,
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
        kind: RightParen,
        span: Span {
            start: 58,
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
            start: 62,
            end: 63,
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
error: [UnsupportedSyntax] expected Comma, got Some(Ident("a")) at 49..50
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Comma, got Some(Ident("a")) at 49..50
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
        "code": 2369,
        "category": "Error",
        "message": "A parameter property is only allowed in a constructor implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts",
        "start": 42,
        "length": 16,
        "line": 3,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/MemberAccessorDeclaration15.ts",
        "start": 49,
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
        "kind": "ClassDeclaration",
        "text": "class C {\r\n   set Foo(public a: number) { }\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n   set Foo(public a: number) { }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n   set Foo(public a: number) { }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "SetAccessor",
        "text": "set Foo(public a: number) { }",
        "line": 3,
        "character": 4
      },
      {
        "kind": "Parameter",
        "text": "public a: number",
        "line": 3,
        "character": 12
      },
      {
        "kind": "Identifier",
        "text": "a",
        "line": 3,
        "character": 19
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Comma, got Some(Ident("a")) at 49..50
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

Superseded by issue #078. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/551-implement-MemberAccessorDeclaration.md` before this move
- `issues/open/551-implement-MemberAccessorDeclaration.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
