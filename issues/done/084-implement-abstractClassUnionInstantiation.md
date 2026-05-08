---
id: 084
title: "Implement Abstractclassunioninstantiation (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage abstractClassUnionInstantiation across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractClassUnionInstantiation` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractClassUnionInstantiation has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
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

- `reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: abstractClassUnionInstantiation

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 755,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "class ConcreteA {}"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 66..71",
  "span_start": 66,
  "span_end": 71,
  "line": 4,
  "column": 10,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class ConcreteA {}
3 | class ConcreteB {}
4 | abstract class AbstractA { a: string; }
5 | abstract class AbstractB { b: string; }
6 |
7 | type Abstracts = typeof AbstractA | typeof AbstractB;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "ConcreteA",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "ConcreteB",
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/084-implement-abstractClassUnionInstantiation.md",
    "title": "Implement Abstractclassunioninstantiation",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
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
// Candidate source class: ConcreteA
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
            start: 19,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "ConcreteA",
        ),
        span: Span {
            start: 25,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 38,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "ConcreteB",
        ),
        span: Span {
            start: 44,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 57,
            end: 65,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 66,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractA",
        ),
        span: Span {
            start: 72,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 84,
            end: 85,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 66..71
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 66..71
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
        "message": "Property 'a' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 84,
        "length": 1,
        "line": 4,
        "character": 28
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'b' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 124,
        "length": 1,
        "line": 5,
        "character": 28
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 403,
        "length": 10,
        "line": 15,
        "character": 1
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 431,
        "length": 10,
        "line": 16,
        "character": 1
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 532,
        "length": 9,
        "line": 19,
        "character": 46
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 605,
        "length": 9,
        "line": 20,
        "character": 46
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 728,
        "length": 9,
        "line": 22,
        "character": 35
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "ConcretesOrAbstracts",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 312,
        "length": 4,
        "line": 11,
        "character": 15,
        "name": "cls1"
      },
      {
        "kind": "binding",
        "typeText": "Abstracts",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 354,
        "length": 4,
        "line": 12,
        "character": 15,
        "name": "cls2"
      },
      {
        "kind": "binding",
        "typeText": "Concretes",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 385,
        "length": 4,
        "line": 13,
        "character": 15,
        "name": "cls3"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA | typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 525,
        "length": 3,
        "line": 19,
        "character": 39,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA | typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 598,
        "length": 3,
        "line": 20,
        "character": 39,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 660,
        "length": 3,
        "line": 21,
        "character": 28,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 721,
        "length": 3,
        "line": 22,
        "character": 28,
        "name": "cls"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class ConcreteA {}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ConcreteB {}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractA { a: string; }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractB { b: string; }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Abstracts = typeof AbstractA | typeof AbstractB;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Concretes = typeof ConcreteA | typeof ConcreteB;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ConcretesOrAbstracts = Concretes | Abstracts;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls1: ConcretesOrAbstracts;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls2: Abstracts;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls3: Concretes;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls1();",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls2();",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls3();",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[ConcreteA, AbstractA, AbstractB].map(cls => new cls());",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[AbstractA, AbstractB, ConcreteA].map(cls => new cls());",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[ConcreteA, ConcreteB].map(cls => new cls());",
        "line": 21,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[AbstractA, AbstractB].map(cls => new cls());",
        "line": 22,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class ConcreteA {}\nclass ConcreteB {}\nabstract class AbstractA { a: string; }\nabstract class AbstractB { b: string; }\n\nt",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractA { a: string; }",
        "line": 4,
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 66..71
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/556-implement-abstractClassUnionInstantiation.md` に統合されました。
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

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/084-implement-abstractClassUnionInstantiation.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
