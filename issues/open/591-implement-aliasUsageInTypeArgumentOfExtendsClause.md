---
id: 591
title: "Implement Aliasusageintypeargumentofextendsclause"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5324]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage aliasUsageInTypeArgumentOfExtendsClause across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `aliasUsageInTypeArgumentOfExtendsClause` with diagnostics: import-export. Fresh triage shows the current failure is the dependency-module `export class` issue-5005 boundary already owned by issue 5324.

Problem: `aliasUsageInTypeArgumentOfExtendsClause` had 1 generated bucket failure and needed smart-triage evidence. No new child is needed because issue 5324 already owns the current blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with issue 5324
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Existing issue 5324 contains the implementation-ready dependency-module `export class` owner
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts`

## Duplicate detection

- `issues/open/126-implement-aliasUsageInTypeArgumentOfExtendsClause.md` - Implement Aliasusageintypeargumentofextendsclause (same reference path, same group key, title overlap)
- `issues/open/505-implement-aliasUsageInTypeArgumentOfExtendsClause.md` - Implement Aliasusageintypeargumentofextendsclause (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasUsageInTypeArgumentOfExtendsClause

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 819,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "export class Model {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 109..115",
  "span_start": 109,
  "span_end": 115,
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
3 | // @Filename: aliasUsageInTypeArgumentOfExtendsClause_backbone.ts
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
    "path": "issues/done/126-implement-aliasUsageInTypeArgumentOfExtendsClause.md",
    "title": "Implement Aliasusageintypeargumentofextendsclause",
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
    "path": "issues/done/505-implement-aliasUsageInTypeArgumentOfExtendsClause.md",
    "title": "Implement Aliasusageintypeargumentofextendsclause",
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
            start: 109,
            end: 115,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 116,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "Model",
        ),
        span: Span {
            start: 122,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 135,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "someData",
        ),
        span: Span {
            start: 142,
            end: 150,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 152,
            end: 158,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 232,
            end: 238,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 239,
            end: 247,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 248,
            end: 249,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 250,
            end: 257,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasUsageInTypeArgumentOfExtendsClause_backbone",
        ),
        span: Span {
            start: 258,
            end: 310,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 310,
            end: 311,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 311,
            end: 312,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 314,
            end: 320,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 321,
            end: 326,
        },
    },
    SpannedToken {
        kind: Ident(
            "VisualizationModel",
        ),
        span: Span {
            start: 327,
            end: 345,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 346,
            end: 353,
        },
    },
    SpannedToken {
        kind: Ident(
            "Backbone",
        ),
        span: Span {
            start: 354,
            end: 362,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 362,
            end: 363,
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 109..115
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 109..115
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 142,
        "length": 8,
        "line": 5,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 239,
        "length": 8,
        "line": 9,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInTypeArgumentOfExtendsClause_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 258,
        "length": 52,
        "line": 9,
        "character": 27
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Backbone'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 478,
        "length": 8,
        "line": 15,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInTypeArgumentOfExtendsClause_backbone' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 497,
        "length": 52,
        "line": 15,
        "character": 27
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './aliasUsageInTypeArgumentOfExtendsClause_moduleA' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 578,
        "length": 51,
        "line": 16,
        "character": 26
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'x' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts",
        "start": 769,
        "length": 1,
        "line": 21,
        "character": 5
      }
    ],
    "hints": [],
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
        "text": "import Backbone = require(\"./aliasUsageInTypeArgumentOfExtendsClause_backbone\");",
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
        "text": "import Backbone = require(\"./aliasUsageInTypeArgumentOfExtendsClause_backbone\");",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasUsageInTypeArgumentOfExtendsClause_moduleA\");",
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
        "kind": "ClassDeclaration",
        "text": "class C<T extends IHasVisualizationModel> {\r\n    x: T;\r\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class D extends C<IHasVisualizationModel> {\r\n    x = moduleA;\r\n}",
        "line": 23,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class Model {\r\n    public someData: string;\r\n}\r\n\r\n// @Filename: aliasUsageInTypeArgumentOfExtendsClause_moduleA.t",
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 109..115
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedModule:1, unsupported_features=import-export:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasUsageInTypeArgumentOfExtendsClause.ts
result: pass; tokens/AST succeed, current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08
```

Current compiler failure:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..51
```

Remaining risks:

- Implementation remains open in `issues/open/5324-support-dependency-export-class-declarations.md`.

## False-done audit

**truly-done** (591)

- Implementation commits: verified via `git log --oneline --all --grep=591`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
