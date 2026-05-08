---
id: 637
title: "Implement Anonymousclassdeclarationdoesntprintwithreadonly"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage anonymousClassDeclarationDoesntPrintWithReadonly across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonymousClassDeclarationDoesntPrintWithReadonly` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonymousClassDeclarationDoesntPrintWithReadonly has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts
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

- `reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts`

## Duplicate detection

- `issues/done/179-implement-anonymousClassDeclarationDoesntPrintWithReadonly.md` - Implement Anonymousclassdeclarationdoesntprintwithreadonly (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: anonymousClassDeclarationDoesntPrintWithReadonly

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 176,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "export class X {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported class export; module resolution and loading are not implemented at 64..70",
  "span_start": 64,
  "span_end": 70,
  "line": 4,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @module: commonjs
2 | // @target: es2015
3 | // @declaration: true
4 | export class X {
5 |     constructor(readonly a: number) { }
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
    "path": "issues/done/179-implement-anonymousClassDeclarationDoesntPrintWithReadonly.md",
    "title": "Implement Anonymousclassdeclarationdoesntprintwithreadonly",
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
// Candidate source class: X
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
            start: 64,
            end: 70,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 71,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 86,
            end: 97,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "readonly",
        ),
        span: Span {
            start: 98,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 110,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 128,
            end: 134,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 135,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 155,
            end: 161,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 162,
            end: 167,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 168,
            end: 175,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 176,
            end: 177,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 178,
            end: 179,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 180,
            e
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 64..70
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 64..70
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts",
        "start": 107,
        "length": 1,
        "line": 5,
        "character": 26,
        "name": "a"
      },
      {
        "kind": "function",
        "typeText": "typeof (Anonymous class)",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonymousClassDeclarationDoesntPrintWithReadonly.ts",
        "start": 144,
        "length": 1,
        "line": 8,
        "character": 17,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "export class X {\r\n    constructor(readonly a: number) { }\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function y() {\r\n    return class extends X { }\r\n}",
        "line": 8,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export class X {\r\n    constructor(readonly a: number) { }\r\n}\r\n\r\nexport function y() {\r\n    return class extends X { }\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class X {\r\n    constructor(readonly a: number) { }\r\n}",
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
error: [UnsupportedModule] issue-055: unsupported class export; module resolution and loading are not implemented at 64..70
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
