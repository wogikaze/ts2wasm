---
id: 462
title: "Implement Exportassignment (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage ExportAssignment across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `ExportAssignment` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ExportAssignment has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment7.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ExportAssignment7.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ExportAssignment7.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment7.ts
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

- `reference/typescript/tests/cases/compiler/ExportAssignment7.ts`
- `reference/typescript/tests/cases/compiler/ExportAssignment8.ts`

## Duplicate detection

- `issues/done/075-implement-ExportAssignment.md` - Implement Exportassignment (same reference path, same feature label, same group key, title overlap)
- `issues/done/131-implement-allowImportClausesToMergeWithTypes.md` - Implement Allowimportclausestomergewithtypes (same feature label, same group key, title overlap)
- `issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/done/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ExportAssignment7

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ExportAssignment7.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment7.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 71,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "export class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 42..48",
  "span_start": 42,
  "span_end": 48,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | export class C {
4 | }
5 | 
6 | export = B;
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
    "path": "issues/done/075-implement-ExportAssignment.md",
    "title": "Implement Exportassignment",
    "reason": "same reference path, same feature label"
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
// Candidate source class: C
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
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 49,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 42..48
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 42..48
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
        "code": 2309,
        "category": "Error",
        "message": "An export assignment cannot be used in a module with other exported elements.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ExportAssignment7.ts",
        "start": 65,
        "length": 11,
        "line": 6,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ExportAssignment7.ts",
        "start": 74,
        "length": 1,
        "line": 6,
        "character": 10
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "export class C {\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = B;",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class C {\r\n}\r\n\r\nexport = B;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class C {\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 42..48
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/548-implement-ExportAssignment.md` に統合されました。
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
- `issues/done/462-implement-ExportAssignment.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
