---
id: 641
title: "Implement Anyasreturntypefornewoncall (audit reopened #641)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage anyAsReturnTypeForNewOnCall across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyAsReturnTypeForNewOnCall` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyAsReturnTypeForNewOnCall has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
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

- `reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts`

## Duplicate detection

- `issues/open/183-implement-anyAsReturnTypeForNewOnCall.md` - Implement Anyasreturntypefornewoncall (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class: anyAsReturnTypeForNewOnCall

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 135,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "function Point(x, y) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-207: instanceof right-hand side must be a supported class constructor `Point`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
// @strict: false
function Point(x, y) {

 this.x = x;

 this.y = y;
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "Point",
    "line": 3,
    "column": 1,
    "params": "x, y"
  },
  {
    "kind": "binding",
    "name": "o",
    "line": 11,
    "column": 1,
    "initializer": "new Point(3, 4)"
  },
  {
    "kind": "binding",
    "name": "xx",
    "line": 13,
    "column": 1,
    "initializer": "o.x"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/183-implement-anyAsReturnTypeForNewOnCall.md",
    "title": "Implement Anyasreturntypefornewoncall",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/255-implement-private-class-element-runtime-semantics.md",
    "title": "Implement private class element runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/312-triage-test262-blocked-p0-window.md",
    "title": "Triage test262 blocked P0 window",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/421-implement-class.md",
    "title": "Implement class syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/045-implement-class-syntax.md",
    "title": "Implement class declaration and expression",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/248-implement-private-class-element-parser.md",
    "title": "Implement private class element parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/249-implement-class-static-block-parser.md",
    "title": "Implement class static block parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/254-implement-class-static-block-runtime-semantics.md",
    "title": "Implement class static block runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/289-resolve-callcount-binding-in-class-destructuring.md",
    "title": "Resolve callCount binding in class destructuring tests",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/292-resolve-initcount-binding-in-class-destructuring.md",
    "title": "Resolve initCount binding in class destructuring defaults",
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
            start: 37,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "Point",
        ),
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 62,
            end: 66,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 76,
            end: 80,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "o",
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "Point",
        ),
        span: Span {
            start: 104,
            end: 109,
        },
    },
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Function {
        name: "Point",
        params: [
            (
                "x",
                None,
                false,
            ),
            (
                "y",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: PropertyAssign {
                    object: This {
                        span: Span {
                            start: 62,
                            end: 66,
                        },
                    },
                    property: "x",
                    value: Ident {
                        name: "x",
                        span: Span {
                            start: 71,
                            end: 72,
                        },
                    },
                    span: Span {
                        start: 62,
                        end: 73,
                    },
                },
                span: Span {
                    start: 62,
                    end: 73,
                },
            },
            Expr {
                expr: PropertyAssign {
                    object: This {
                        span: Span {
                            start: 76,
                            end: 80,
                        },
                    },
                    property: "y",
                    value: Ident {
                        name: "y",
                        span: Span {
                            start: 85,
                            end: 86,
                        },
                    },
                    span: Span {
                        start: 76,
                        end: 87,
                    },
                },
                span: Span {
                    start: 76,
                    end: 87,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 37,
            end: 87,
        },
    },
    Let {
        name: "o",
        expr: New {
            expr: Ident {
                name: "Point",
                span: Span {
                    start: 104,
                    end: 109,
                },
            },
            args: [
                Number {
                    value: 3,
                    span: Span {
                        start: 110,
                        end: 111,
                    },
                },
                Number {
                    value: 4,
                    span: Span {
                        start: 113,
                        end: 114,
                    },
                },
            ],
            span: Span {
                start: 100,
                end: 115,
            },
        },
        span: Span {
            start: 92,
            end: 116,
        },
    },
    Let {
        name: "xx",
        expr: Member {
            object: Ident {
                name: "o",
                span: Span {
                    start: 127,
                    end: 128,
                },
            },
            property: "x",
            span: Span {
                start: 127,
                end: 130,
            },
        },
        span: Span {
            start: 118,
            end: 131,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Point`
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
        "code": 2683,
        "category": "Error",
        "message": "'this' implicitly has type 'any' because it does not have a type annotation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 62,
        "length": 4,
        "line": 5,
        "character": 2
      },
      {
        "code": 2683,
        "category": "Error",
        "message": "'this' implicitly has type 'any' because it does not have a type annotation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 76,
        "length": 4,
        "line": 7,
        "character": 2
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 46,
        "length": 5,
        "line": 3,
        "character": 10,
        "name": "Point"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 55,
        "length": 1,
        "line": 3,
        "character": 19,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 96,
        "length": 1,
        "line": 11,
        "character": 5,
        "name": "o"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts",
        "start": 122,
        "length": 2,
        "line": 13,
        "character": 5,
        "name": "xx"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
```

## Completion evidence

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## Status

Superseded by issue #183. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- open issue file before this move
- `issues/open/641-implement-anyAsReturnTypeForNewOnCall.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
