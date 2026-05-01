---
id: 481
title: "Implement Accessordeclarationemitvisibilityerrors"
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

Triage accessorDeclarationEmitVisibilityErrors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorDeclarationEmitVisibilityErrors` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorDeclarationEmitVisibilityErrors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts
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

- `reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts`

## Duplicate detection

- `issues/open/097-implement-accessorDeclarationEmitVisibilityErrors.md` - Implement Accessordeclarationemitvisibilityerrors (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: accessorDeclarationEmitVisibilityErrors

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 108,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "export class Q {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 60..66",
  "span_start": 60,
  "span_end": 66,
  "line": 5,
  "column": 5,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | // @strict: true
3 | // @declaration: true
4 | 
5 | export class Q {
6 |     set bet(arg: DoesNotExist) {}
7 | }
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
    "path": "issues/open/097-implement-accessorDeclarationEmitVisibilityErrors.md",
    "title": "Implement Accessordeclarationemitvisibilityerrors",
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
// Candidate source class: Q
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
            start: 60,
            end: 66,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 67,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "Q",
        ),
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 82,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "bet",
        ),
        span: Span {
            start: 86,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "arg",
        ),
        span: Span {
            start: 90,
            end: 93,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "DoesNotExist",
        ),
        span: Span {
            start: 95,
            end: 107,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 113,
            end: 114,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 60..66
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 60..66
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'DoesNotExist'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts",
        "start": 95,
        "length": 12,
        "line": 6,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "DoesNotExist",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorDeclarationEmitVisibilityErrors.ts",
        "start": 90,
        "length": 3,
        "line": 6,
        "character": 13,
        "name": "arg"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "export class Q {\r\n    set bet(arg: DoesNotExist) {}\r\n}",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class Q {\r\n    set bet(arg: DoesNotExist) {}\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Q {\r\n    set bet(arg: DoesNotExist) {}\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 60..66
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
