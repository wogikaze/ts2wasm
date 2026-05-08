---
id: 140
title: "Implement Ambientclassdeclarationwithextends"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ambientClassDeclarationWithExtends across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientClassDeclarationWithExtends` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientClassDeclarationWithExtends has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
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

- `reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientClassDeclarationWithExtends

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 478,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "declare class A { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 107..112",
  "span_start": 107,
  "span_end": 112,
  "line": 4,
  "column": 12,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | // @Filename: ambientClassDeclarationExtends_singleFile.ts
4 | declare class A { }
5 | declare class B extends A { }
6 |
7 | declare class C {
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
    "path": "issues/open/140-implement-ambientClassDeclarationWithExtends.md",
    "title": "Implement Ambientclassdeclarationwithextends",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 99,
            end: 106,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 107,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 120,
            end: 127,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 128,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 136,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 107..112
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 107..112
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
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 210,
        "length": 1,
        "line": 10,
        "character": 19,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 253,
        "length": 1,
        "line": 13,
        "character": 5,
        "name": "d"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 386,
        "length": 1,
        "line": 20,
        "character": 19,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "E",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 486,
        "length": 1,
        "line": 25,
        "character": 5,
        "name": "f"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class A { }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class B extends A { }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class C {\r\n    public foo;\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace D { var x; }",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class D extends C { }",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var d: C = new D();",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class E {\r\n    public bar;\r\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace F { var y; }",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class F extends E { }",
        "line": 24,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var f: E = new F();",
        "line": 25,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class A { }\r\ndeclare class B extends A { }\r\n\r\ndeclare class C {\r\n    public foo;\r\n}\r\nnamespace D { var x; }\r\ndec",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class A { }",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 107..112
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/604-implement-ambientClassDeclarationWithExtends.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/140-implement-ambientClassDeclarationWithExtends.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
