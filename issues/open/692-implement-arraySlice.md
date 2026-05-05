---
id: 692
title: "Implement Arrayslice (audit reopened #692)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage arraySlice across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arraySlice` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arraySlice has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySlice.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arraySlice.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arraySlice.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySlice.ts
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

- `reference/typescript/tests/cases/compiler/arraySlice.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage class: arraySlice

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arraySlice.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySlice.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 67,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var arr: string[] | number[];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: unknown receiver class for method `splice` at 49..65",
  "span_start": 49,
  "span_end": 65,
  "line": 3,
  "column": 1,
  "feature_label": "class",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var arr: string[] | number[];
3 | arr.splice(1, 1);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "arr",
    "line": 2,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
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
    "path": "issues/done/421-implement-class.md",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 23,
            end: 26,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 39,
            end: 45,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 49,
            end: 52,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "splice",
        ),
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
]
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "arr",
        expr: Undefined {
            span: Span {
                start: 23,
                end: 26,
            },
        },
        span: Span {
            start: 19,
            end: 48,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "arr",
                    span: Span {
                        start: 49,
                        end: 52,
                    },
                },
                property: "splice",
                span: Span {
                    start: 49,
                    end: 59,
                },
            },
            args: [
                Number {
                    value: 1,
                    span: Span {
                        start: 60,
                        end: 61,
                    },
                },
                Number {
                    value: 1,
                    span: Span {
                        start: 63,
                        end: 64,
                    },
                },
            ],
            span: Span {
                start: 49,
                end: 65,
            },
        },
        span: Span {
            start: 49,
            end: 66,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `splice` at 49..65
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'arr' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySlice.ts",
        "start": 49,
        "length": 3,
        "line": 3,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "string[] | number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySlice.ts",
        "start": 23,
        "length": 3,
        "line": 2,
        "character": 5,
        "name": "arr"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var arr: string[] | number[];",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "arr.splice(1, 1);",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var arr: string[] | number[];\narr.splice(1, 1);\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "arr.splice(1, 1);",
        "line": 3,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "arr.splice(1, 1)",
        "line": 3,
        "character": 1
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "arr.splice",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "arr",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `splice` at 49..65
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
- `issues/open/692-implement-arraySlice.md` before this move
- `issues/open/692-implement-arraySlice.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
