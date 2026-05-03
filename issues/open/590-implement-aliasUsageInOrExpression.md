---
id: 590
title: "Implement Aliasusageinorexpression"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage aliasUsageInOrExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasUsageInOrExpression` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasUsageInOrExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts
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

- `reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts`

## Duplicate detection

- `issues/open/125-implement-aliasUsageInOrExpression.md` - Implement Aliasusageinorexpression (same reference path, same group key, title overlap)
- `issues/open/504-implement-aliasUsageInOrExpression.md` - Implement Aliasusageinorexpression (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasUsageInOrExpression

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 960,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "export class Model {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 94..100",
  "span_start": 94,
  "span_end": 100,
  "line": 4,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | // @Filename: aliasUsageInOrExpression_backbone.ts
4 | export class Model {
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
    "path": "issues/open/125-implement-aliasUsageInOrExpression.md",
    "title": "Implement Aliasusageinorexpression",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/504-implement-aliasUsageInOrExpression.md",
    "title": "Implement Aliasusageinorexpression",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: Model
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
            start: 94,
            end: 100,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 101,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
            start: 107,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 120,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "someData",
        ),
        span: Span {
            start: 127,
            end: 135,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 137,
            end: 143,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 202,
            end: 208,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 209,
            end: 217,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 218,
            end: 219,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 220,
            end: 227,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 227,
            end: 228,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasUsageInOrExpression_backbone",
        ),
        span: Span {
            start: 228,
            end: 265,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 265,
            end: 266,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 266,
            end: 267,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 269,
            end: 275,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 276,
            end: 281,
        },
    },
    SpannedToken {
        kind: Ident(
            "VisualizationModel",
        ),
        span: Span {
            start: 282,
            end: 300,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 301,
            end: 308,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 309,
            end: 317,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 317,
            end: 318,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
            start:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 94..100
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 94..100
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 127,
        "length": 8,
        "line": 5,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 209,
        "length": 8,
        "line": 9,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInOrExpression_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 228,
        "length": 37,
        "line": 9,
        "character": 27
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 418,
        "length": 8,
        "line": 15,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInOrExpression_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 437,
        "length": 37,
        "line": 15,
        "character": 27
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInOrExpression_moduleA' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 503,
        "length": 36,
        "line": 16,
        "character": 26
      },
      {
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type 'null' to type '{ x: IHasVisualizationModel; }' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 829,
        "length": 35,
        "line": 24,
        "character": 40
      },
      {
        "code": 2873,
        "category": "Error",
        "message": "This kind of expression is always falsy.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 829,
        "length": 35,
        "line": 24,
        "character": 40
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '{ x: any; } | null' is not assignable to type '{ x: IHasVisualizationModel; }'.\n  Type 'null' is not assignable to type '{ x: IHasVisualizationModel; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 889,
        "length": 1,
        "line": 25,
        "character": 5
      },
      {
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type 'null' to type '{ x: IHasVisualizationModel; }' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 924,
        "length": 35,
        "line": 25,
        "character": 40
      },
      {
        "code": 2873,
        "category": "Error",
        "message": "This kind of expression is always falsy.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 924,
        "length": 35,
        "line": 25,
        "character": 40
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 642,
        "length": 1,
        "line": 20,
        "character": 13,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 674,
        "length": 2,
        "line": 21,
        "character": 5,
        "name": "d1"
      },
      {
        "kind": "binding",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 698,
        "length": 2,
        "line": 22,
        "character": 5,
        "name": "d2"
      },
      {
        "kind": "binding",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 746,
        "length": 2,
        "line": 23,
        "character": 5,
        "name": "d2"
      },
      {
        "kind": "binding",
        "typeText": "{ x: IHasVisualizationModel; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 794,
        "length": 1,
        "line": 24,
        "character": 5,
        "name": "e"
      },
      {
        "kind": "binding",
        "typeText": "{ x: IHasVisualizationModel; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInOrExpression.ts",
        "start": 889,
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
        "text": "export class Model {\r\n    public someData: string;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import Backbone = require(\"./aliasUsageInOrExpression_backbone\");",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class VisualizationModel extends Backbone.Model {\r\n    // interesting stuff here\r\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import Backbone = require(\"./aliasUsageInOrExpression_backbone\");",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasUsageInOrExpression_moduleA\");",
        "line": 16,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface IHasVisualizationModel {\r\n    VisualizationModel: typeof Backbone.Model;\r\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var i: IHasVisualizationModel;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var d1 = i || moduleA;",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var d2: IHasVisualizationModel = i || moduleA;",
        "li
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 94..100
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
