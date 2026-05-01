---
id: 501
title: "Implement Aliasusageingenericfunction"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage aliasUsageInGenericFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasUsageInGenericFunction` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasUsageInGenericFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts
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

- `reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts`

## Duplicate detection

- `issues/open/122-implement-aliasUsageInGenericFunction.md` - Implement Aliasusageingenericfunction (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasUsageInGenericFunction

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 787,
  "lines": 24,
  "extension": ".ts",
  "first_code_line": "export class Model {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 97..103",
  "span_start": 97,
  "span_end": 103,
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
3 | // @Filename: aliasUsageInGenericFunction_backbone.ts
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
    "path": "issues/open/122-implement-aliasUsageInGenericFunction.md",
    "title": "Implement Aliasusageingenericfunction",
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
            start: 97,
            end: 103,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 104,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
            start: 110,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 123,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "someData",
        ),
        span: Span {
            start: 130,
            end: 138,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 140,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 208,
            end: 214,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 215,
            end: 223,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 224,
            end: 225,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 226,
            end: 233,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 233,
            end: 234,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasUsageInGenericFunction_backbone",
        ),
        span: Span {
            start: 234,
            end: 274,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 274,
            end: 275,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 275,
            end: 276,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 278,
            end: 284,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 285,
            end: 290,
        },
    },
    SpannedToken {
        kind: Ident(
            "VisualizationModel",
        ),
        span: Span {
            start: 291,
            end: 309,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 310,
            end: 317,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 318,
            end: 326,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 326,
            end: 327,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
            star
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 97..103
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 97..103
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 130,
        "length": 8,
        "line": 5,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 215,
        "length": 8,
        "line": 9,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInGenericFunction_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 234,
        "length": 40,
        "line": 9,
        "character": 27
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 430,
        "length": 8,
        "line": 15,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInGenericFunction_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 449,
        "length": 40,
        "line": 15,
        "character": 27
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInGenericFunction_moduleA' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 518,
        "length": 39,
        "line": 16,
        "character": 26
      },
      {
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type 'null' to type 'IHasVisualizationModel' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 777,
        "length": 28,
        "line": 24,
        "character": 19
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 657,
        "length": 3,
        "line": 20,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 702,
        "length": 1,
        "line": 20,
        "character": 55,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "{ a: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 733,
        "length": 1,
        "line": 23,
        "character": 5,
        "name": "r"
      },
      {
        "kind": "binding",
        "typeText": "{ a: IHasVisualizationModel; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInGenericFunction.ts",
        "start": 763,
        "length": 2,
        "line": 24,
        "character": 5,
        "name": "r2"
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
        "text": "import Backbone = require(\"./aliasUsageInGenericFunction_backbone\");",
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
        "text": "import Backbone = require(\"./aliasUsageInGenericFunction_backbone\");",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasUsageInGenericFunction_moduleA\");",
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
        "kind": "FunctionDeclaration",
        "text": "function foo<T extends { a: IHasVisualizationModel }>(x: T) {\r\n    return x;\r\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r = foo({ a: moduleA });",
        "line": 23,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r2 = foo({ a: <IHasVisualizationModel>null });",
        "line": 24,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class Model {\r\n    public someData: string;\r\n}\r\n\r\n// @Filename: aliasUsageInGenericFunction_moduleA.ts\r\nimport Ba",
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 97..103
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
