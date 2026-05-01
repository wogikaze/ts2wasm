---
id: 632
title: "Implement Amdmodulebundlenoduplicatedeclarationemitcomments"
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

Triage amdModuleBundleNoDuplicateDeclarationEmitComments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `amdModuleBundleNoDuplicateDeclarationEmitComments` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdModuleBundleNoDuplicateDeclarationEmitComments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts
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

- `reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts`

## Duplicate detection

- `issues/open/174-implement-amdModuleBundleNoDuplicateDeclarationEmitComments.md` - Implement Amdmodulebundlenoduplicatedeclarationemitcomments (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: amdModuleBundleNoDuplicateDeclarationEmitComments

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleBundleNoDuplicateDeclarationEmitComments.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 265,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "export class Foo {}"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 158..164",
  "span_start": 158,
  "span_end": 164,
  "line": 7,
  "column": 7,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 4 | // @outFile: ./out.js
 5 | // @filename: file1.ts
 6 | /// <amd-module name="mynamespace::SomeModuleA" />
 7 | export class Foo {}
 8 | // @filename: file2.ts
 9 | /// <amd-module name="mynamespace::SomeModuleB" />
10 | export class Bar {}
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
    "path": "issues/open/174-implement-amdModuleBundleNoDuplicateDeclarationEmitComments.md",
    "title": "Implement Amdmodulebundlenoduplicatedeclarationemitcomments",
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
// Candidate source class: Foo
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Export,
        span: Span {
            start: 158,
            end: 164,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 165,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 171,
            end: 174,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 175,
            end: 176,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 176,
            end: 177,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 255,
            end: 261,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 262,
            end: 267,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 268,
            end: 271,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 272,
            end: 273,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 273,
            end: 274,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 158..164
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 158..164
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "export class Foo {}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Bar {}",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class Foo {}\r\n// @filename: file2.ts\r\n/// <amd-module name=\"mynamespace::SomeModuleB\" />\r\nexport class Bar {}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Foo {}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 7,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 158..164
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
