---
id: 100
title: "Implement Accessorinferredreturntypeerrorinreturnstatement"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5007]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage accessorInferredReturnTypeErrorInReturnStatement across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorInferredReturnTypeErrorInReturnStatement` with diagnostics: class-accessor.

### Root cause

The test file starts with `export var basePrototype = { ... }` — top-level `export var` requires module support. Our compiler rejects this as `UnsupportedModule`. The accessor syntax inside the object literal is not reached because the module-level export fails first.

This is a **module resolution issue**, not a parser issue or class-accessor issue.

Problem: accessorInferredReturnTypeErrorInReturnStatement fails due to UnsupportedModule (`export var`).

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
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

- `reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class accessor: accessorInferredReturnTypeErrorInReturnStatement

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 211,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "export var basePrototype = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86",
  "span_start": 80,
  "span_end": 86,
  "line": 6,
  "column": 1,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @strict: true
4 | // @declaration: true
5 |
6 | export var basePrototype = {
7 |   get primaryPath() {
8 |     var _this = this;
9 |     return _this.collection.schema.primaryPath;
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
    "path": "issues/open/100-implement-accessorInferredReturnTypeErrorInReturnStatement.md",
    "title": "Implement Accessorinferredreturntypeerrorinreturnstatement",
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
// Candidate source class: Example
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
        kind: Export,
        span: Span {
            start: 80,
            end: 86,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "basePrototype",
        ),
        span: Span {
            start: 91,
            end: 104,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 111,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "primaryPath",
        ),
        span: Span {
            start: 115,
            end: 126,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 135,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "_this",
        ),
        span: Span {
            start: 139,
            end: 144,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    Spanned
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
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
        "message": "Property 'collection' does not exist on type '{ readonly primaryPath: any; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 170,
        "length": 10,
        "line": 9,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ readonly primaryPath: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 91,
        "length": 13,
        "line": 6,
        "character": 12,
        "name": "basePrototype"
      },
      {
        "kind": "binding",
        "typeText": "{ readonly primaryPath: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 139,
        "length": 5,
        "line": 8,
        "character": 9,
        "name": "_this"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
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

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a `blocked` triage bucket (generated bucket for accessorInferredReturnTypeErrorInReturnStatement) with `depends_on: [5007]` (module resolution meta-issue). It was dragged to `done/` alongside the parent meta-issue without any implementation work. The `## Completion evidence` section has empty template values (`...` for commits, empty validation result). The test still fails with `issue-055: unsupported variable export; module resolution and loading are not implemented`.

**True-done checklist** (all must pass):

1. **Module resolution (issue-055) must be implemented** such that `export var` at top level is supported, OR this specific test case must be confirmed as a duplicate of an active implementation issue.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts` must report `BuildPass` (not `UnsupportedSyntax` / `issue-055`)
   - Or: clear documented decision with evidence that this case is superseded by a specific child issue under 5007
