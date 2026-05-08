---
id: 678
title: "Implement Arrayflatnocrashinference (audit reopened #678)"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage arrayFlatNoCrashInference across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFlatNoCrashInference` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFlatNoCrashInference has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts
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

- `reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage type system: arrayFlatNoCrashInference

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 124,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "function foo<T>(arr: T[], depth: number) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: unknown receiver class for method `flat` at 110..125",
  "span_start": 110,
  "span_end": 125,
  "line": 5,
  "column": 16,
  "feature_label": "type-system",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: true
3 | // @lib: es2020
4 | function foo<T>(arr: T[], depth: number) {
5 |     return arr.flat(depth);
6 | }
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
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: Example
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
        kind: Function,
        span: Span {
            start: 55,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 64,
            end: 67,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 71,
            end: 74,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "depth",
        ),
        span: Span {
            start: 81,
            end: 86,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 88,
            end: 94,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 103,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 110,
            end: 113,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "flat",
        ),
        span: Span {
            start: 114,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "depth",
        ),
        span: Span {
            start: 119,
            end: 124,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Function {
        name: "foo",
        params: [
            (
                "arr",
                None,
                false,
            ),
            (
                "depth",
                None,
                false,
            ),
        ],
        body: [
            Return {
                expr: Call {
                    callee: Member {
                        object: Ident {
                            name: "arr",
                            span: Span {
                                start: 110,
                                end: 113,
                            },
                        },
                        property: "flat",
                        span: Span {
                            start: 110,
                            end: 118,
                        },
                    },
                    args: [
                        Ident {
                            name: "depth",
                            span: Span {
                                start: 119,
                                end: 124,
                            },
                        },
                    ],
                    span: Span {
                        start: 110,
                        end: 125,
                    },
                },
                span: Span {
                    start: 103,
                    end: 126,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 55,
            end: 126,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `flat` at 110..125
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
        "kind": "function",
        "typeText": "FlatArray<T, 0 | 1 | -1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20>[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts",
        "start": 64,
        "length": 3,
        "line": 4,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "T[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts",
        "start": 71,
        "length": 3,
        "line": 4,
        "character": 17,
        "name": "arr"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFlatNoCrashInference.ts",
        "start": 81,
        "length": 5,
        "line": 4,
        "character": 27,
        "name": "depth"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo<T>(arr: T[], depth: number) {\r\n    return arr.flat(depth);\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo<T>(arr: T[], depth: number) {\r\n    return arr.flat(depth);\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo<T>(arr: T[], depth: number) {\r\n    return arr.flat(depth);\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    return arr.flat(depth);\r\n}",
        "line": 4,
        "character": 42
      },
      {
        "kind": "ReturnStatement",
        "text": "return arr.flat(depth);",
        "line": 5,
        "character": 5
      },
      {
        "kind": "CallExpression",
        "text": "arr.flat(depth)",
        "line": 5,
        "character": 12
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "arr.flat",
        "line": 5,
        "character": 12
      },
      {
        "kind": "Identifier",
        "text": "arr",
        "line": 5,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `flat` at 110..125
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

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/678-implement-arrayFlatNoCrashInference.md` before this move
- `issues/open/678-implement-arrayFlatNoCrashInference.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
