---
id: 586
title: "Implement Aliasusageinfunctionexpression"
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

Triage aliasUsageInFunctionExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasUsageInFunctionExpression` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasUsageInFunctionExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts
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

- `reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts`

## Duplicate detection

- `issues/done/121-implement-aliasUsageInFunctionExpression.md` - Implement Aliasusageinfunctionexpression (same reference path, same group key, title overlap)
- `issues/done/500-implement-aliasUsageInFunctionExpression.md` - Implement Aliasusageinfunctionexpression (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasUsageInFunctionExpression

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 708,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "export class Model {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 100..106",
  "span_start": 100,
  "span_end": 106,
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
3 | // @Filename: aliasUsageInFunctionExpression_backbone.ts
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
    "path": "issues/done/121-implement-aliasUsageInFunctionExpression.md",
    "title": "Implement Aliasusageinfunctionexpression",
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
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/500-implement-aliasUsageInFunctionExpression.md",
    "title": "Implement Aliasusageinfunctionexpression",
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
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
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
            start: 100,
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
            "Model",
        ),
        span: Span {
            start: 113,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 126,
            end: 132,
        },
    },
    SpannedToken {
        kind: Ident(
            "someData",
        ),
        span: Span {
            start: 133,
            end: 141,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 143,
            end: 149,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 214,
            end: 220,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 221,
            end: 229,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 232,
            end: 239,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 239,
            end: 240,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasUsageInFunctionExpression_backbone",
        ),
        span: Span {
            start: 240,
            end: 283,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 283,
            end: 284,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 284,
            end: 285,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 287,
            end: 293,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 294,
            end: 299,
        },
    },
    SpannedToken {
        kind: Ident(
            "VisualizationModel",
        ),
        span: Span {
            start: 300,
            end: 318,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 319,
            end: 326,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 327,
            end: 335,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 335,
            end: 336,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 100..106
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 100..106
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 133,
        "length": 8,
        "line": 5,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 221,
        "length": 8,
        "line": 9,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInFunctionExpression_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 240,
        "length": 43,
        "line": 9,
        "character": 27
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 442,
        "length": 8,
        "line": 15,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInFunctionExpression_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 461,
        "length": 43,
        "line": 15,
        "character": 27
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInFunctionExpression_moduleA' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 533,
        "length": 42,
        "line": 16,
        "character": 26
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "(x: IHasVisualizationModel) => IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 670,
        "length": 1,
        "line": 20,
        "character": 5,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 675,
        "length": 1,
        "line": 20,
        "character": 10,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "IHasVisualizationModel",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInFunctionExpression.ts",
        "start": 714,
        "length": 1,
        "line": 21,
        "character": 6,
        "name": "x"
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
        "text": "import Backbone = require(\"./aliasUsageInFunctionExpression_backbone\");",
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
        "text": "import Backbone = require(\"./aliasUsageInFunctionExpression_backbone\");",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasUsageInFunctionExpression_moduleA\");",
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
        "text": "var f = (x: IHasVisualizationModel) => x;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f = (x) => moduleA;",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class Model {\r\n    public someData: string;\r\n}\r\n\r\n// @Filename: aliasUsageInFunctionExpression_moduleA.ts\r\nimport",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Model {\r\n    public someData: string;\r\n}",
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 100..106
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
