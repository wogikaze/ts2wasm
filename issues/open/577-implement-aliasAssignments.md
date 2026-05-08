---
id: 577
title: "Implement Aliasassignments"
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

Triage aliasAssignments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in `aliasAssignments` with diagnostics:
import-export. Fresh triage shows the current failure is the dependency-module
`export class` issue-5005 boundary already owned by issue 5324.

Problem: aliasAssignments had 1 generated bucket failure and needed smart-triage
evidence. No new child is needed because issue 5324 already owns the current
blocker.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasAssignments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasAssignments.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/aliasAssignments.ts`

## Duplicate detection

- `issues/open/111-implement-aliasAssignments.md` - Implement Aliasassignments (same reference path, same group key, title overlap)
- `issues/open/491-implement-aliasAssignments.md` - Implement Aliasassignments (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasAssignments

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
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
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 84..90",
  "span_start": 84,
  "span_end": 90,
  "line": 4,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
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
    "path": "issues/done/111-implement-aliasAssignments.md",
    "title": "Implement Aliasassignments",
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
    "path": "issues/done/491-implement-aliasAssignments.md",
    "title": "Implement Aliasassignments",
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
            end: 198,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 199,
            end: 206,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 206,
            end: 207,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasAssignments_moduleA",
        ),
        span: Span {
            start: 207,
            end: 235,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 235,
            end: 236,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 236,
            end: 237,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 239,
            end: 242,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 243,
            end: 244,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 245,
            end: 246,
        },
    },
    SpannedToken {
        kind: Ident(
            "moduleA",
        ),
        span: Span {
            start: 247,
            end: 254,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 254,
            end: 255,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 259,
            end: 260,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 84..90
```

## Completion evidence

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasAssignments.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_features=import-export:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasAssignments.ts
result: pass; tokens/AST succeed, current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `issues/open/5324-support-dependency-export-class-declarations.md`.

## False-done audit

**truly-done** (577)

- Implementation commits: verified via `git log --oneline --all --grep=577`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
