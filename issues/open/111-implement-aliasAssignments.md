---
id: 111
title: "Implement Aliasassignments"
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

Triage aliasAssignments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasAssignments` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasAssignments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasAssignments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasAssignments.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasAssignments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasAssignments.ts
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

- `reference/typescript/tests/cases/compiler/aliasAssignments.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasAssignments

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasAssignments.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasAssignments.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 316,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "export class someClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 84..90",
  "span_start": 84,
  "span_end": 90,
  "line": 4,
  "column": 4,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: commonjs
3 | // @Filename: aliasAssignments_moduleA.ts
4 | export class someClass {
5 |     public someData: string;
6 | }
7 |
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
    "path": "issues/open/111-implement-aliasAssignments.md",
    "title": "Implement Aliasassignments",
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
// Candidate source class: someClass
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
            start: 84,
            end: 90,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 91,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "someClass",
        ),
        span: Span {
            start: 97,
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
            "public",
        ),
        span: Span {
            start: 114,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "someData",
        ),
        span: Span {
            start: 121,
            end: 129,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 131,
            end: 137,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 182,
            end: 188,
        },
    },
    SpannedToken {
        kind: Ident(
            "moduleA",
        ),
        span: Span {
            start: 189,
            end: 196,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 197,
            end: 19
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
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
        "message": "Property 'someData' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasAssignments.ts",
        "start": 121,
        "length": 8,
        "line": 5,
        "character": 12
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasAssignments_moduleA' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasAssignments.ts",
        "start": 207,
        "length": 28,
        "line": 9,
        "character": 26
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasAssignments.ts",
        "start": 243,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasAssignments.ts",
        "start": 288,
        "length": 1,
        "line": 12,
        "character": 5,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "export class someClass {\r\n    public someData: string;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasAssignments_moduleA\");",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = moduleA;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "x = 1;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y = 1;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "y = moduleA;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class someClass {\r\n    public someData: string;\r\n}\r\n\r\n// @Filename: aliasAssignments_1.ts\r\nimport moduleA = requi",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class someClass {\r\n    public someData: string;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
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
