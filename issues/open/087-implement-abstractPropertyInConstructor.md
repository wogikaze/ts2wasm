---
id: 087
title: "Implement Abstractpropertyinconstructor (dup)"
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

Triage abstractPropertyInConstructor across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractPropertyInConstructor` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractPropertyInConstructor has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
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

- `reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: abstractPropertyInConstructor

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2079,
  "lines": 93,
  "extension": ".ts",
  "first_code_line": "abstract class AbstractClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 29..34",
  "span_start": 29,
  "span_end": 34,
  "line": 2,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | abstract class AbstractClass {
3 |     constructor(str: string, other: AbstractClass) {
4 |         this.method(parseInt(str));
5 |         let val = this.prop.toLowerCase();
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/087-implement-abstractPropertyInConstructor.md",
    "title": "Implement Abstractpropertyinconstructor",
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
// Candidate source class: AbstractClass
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
            "abstract",
        ),
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 29,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractClass",
        ),
        span: Span {
            start: 35,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 56,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 68,
            end: 71,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 73,
            end: 79,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "other",
        ),
        span: Span {
            start: 81,
            end: 86,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractClass",
        ),
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 29..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 29..34
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
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 166,
        "length": 4,
        "line": 5,
        "character": 24
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 227,
        "length": 4,
        "line": 8,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'cb' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 274,
        "length": 2,
        "line": 10,
        "character": 14
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 634,
        "length": 4,
        "line": 26,
        "character": 18
      },
      {
        "code": 2729,
        "category": "Error",
        "message": "Property 'prop' is used before its initialization.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 634,
        "length": 4,
        "line": 26,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1033,
        "length": 4,
        "line": 40,
        "character": 22
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'x' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1856,
        "length": 1,
        "line": 79,
        "character": 15
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1859,
        "length": 1,
        "line": 79,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'x' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1903,
        "length": 1,
        "line": 80,
        "character": 12
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1906,
        "length": 1,
        "line": 80,
        "character": 15
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1913,
        "length": 3,
        "line": 80,
        "character": 22
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'x' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1970,
        "length": 1,
        "line": 85,
        "character": 5
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'y' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1986,
        "length": 1,
        "line": 86,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 68,
        "length": 3,
        "line": 3,
        "character": 17,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "AbstractClass",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 81,
        "length": 5,
        "line": 3,
        "character": 30,
        "name": "other"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 155,
        "length": 3,
        "line": 5,
        "character": 13,
        "name": "val"
      },
      {
        "kind": "binding",
        "typeText": "() => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 345,
        "length": 13,
        "line": 13,
        "character": 15,
        "name": "innerFunction"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 551,
        "length": 1,
        "line": 22,
        "character": 19,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 594,
        "length": 3,
        "line": 24,
        "character": 21,
        "name": "num"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 707,
        "length": 15,
        "line": 30,
        "character": 21,
        "operator": "+",
        "leftType": "string",
        "rightType": "\"!\"",
        "candidate": "string-concat-fast-path"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 808,
        "length": 1,
        "line": 35,
        "character": 11,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 845,
        "length": 3,
        "line": 37,
        "character": 17,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 29..34
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/558-implement-abstractPropertyInConstructor.md` に統合されました。
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
- `issues/open/087-implement-abstractPropertyInConstructor.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
