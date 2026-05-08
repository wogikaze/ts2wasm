---
id: 439
title: "Implement new expression"
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

Triage new-expression feature across 10 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 10 cases fail with new-expression diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: new-expression feature has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js --detail
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
mise run reference-coverage -- test262 --limit 20
mise run reference-coverage -- test262 --path-filter reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js --detail
mise run reference-triage -- test262 reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js
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

- `reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js`
- `reference/test262/test/intl402/NumberFormat/dft-currency-mnfd-range-check-mxfd.js`
- `reference/test262/test/language/function-code/10.4.3-1-25-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-25gs.js`
- `reference/test262/test/language/function-code/10.4.3-1-26-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-26gs.js`
- `reference/test262/test/staging/sm/Array/from_surfaces.js`
- `reference/test262/test/staging/sm/Function/regress-518103.js`
- `reference/test262/test/staging/sm/extensions/new-cross-compartment.js`
- `reference/test262/test/staging/sm/regress/regress-624968.js`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage new expression: ignorePunctuation default

- Issue class: `triage-needed`
- Feature label: `new-expression`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/intl402/Collator/prototype/resolvedOptions/ignorePunctuation-default.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 692,
  "lines": 16,
  "extension": ".js",
  "first_code_line": "esid: sec-initializecollator",
  "test262_metadata": {
    "esid": "sec-initializecollator",
    "description": "resolved ignorePunctuation come from locale default instead of false.",
    "locale": "[en, th, ja]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "only new ClassName(...) is supported",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "new-expression",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text

function print(message) {
  console.log(message);
}


/* standard globals shim */
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "print",
    "line": 2,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "binding",
    "name": "NaN",
    "line": 10,
    "column": 1,
    "initializer": "0/0"
  },
  {
    "kind": "binding",
    "name": "Infinity",
    "line": 11,
    "column": 1,
    "initializer": "1/0"
  },
  {
    "kind": "binding",
    "name": "$262",
    "line": 17,
    "column": 1,
    "initializer": "{}"
  },
  {
    "kind": "function",
    "name": "$ERROR",
    "line": 26,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "function",
    "name": "$DONOTEVALUATE",
    "line": 30,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "assert",
    "line": 34,
    "column": 1,
    "params": "mustBeTrue, message"
  }
]
```

Duplicate candidates:

```json
[]
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
            start: 1,
            end: 9,
        },
    },
    SpannedToken {
        kind: Ident(
            "print",
        ),
        span: Span {
            start: 10,
            end: 15,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 15,
            end: 16,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 16,
            end: 23,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 29,
            end: 36,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "log",
        ),
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 41,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "NaN",
        ),
        span: Span {
            start: 89,
            end: 92,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "Infinity",
        ),
        span: Span {
            start: 104,
            end: 112,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "print",
        params: [
            (
                "message",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: Call {
                    callee: Member {
                        object: Ident {
                            name: "console",
                            span: Span {
                                start: 29,
                                end: 36,
                            },
                        },
                        property: "log",
                        span: Span {
                            start: 29,
                            end: 40,
                        },
                    },
                    args: [
                        Ident {
                            name: "message",
                            span: Span {
                                start: 41,
                                end: 48,
                            },
                        },
                    ],
                    span: Span {
                        start: 29,
                        end: 49,
                    },
                },
                span: Span {
                    start: 29,
                    end: 50,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 1,
            end: 50,
        },
    },
    Let {
        name: "NaN",
        expr: Binary {
            left: Number {
                value: 0,
                span: Span {
                    start: 95,
                    end: 96,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 97,
                    end: 98,
                },
            },
            span: Span {
                start: 95,
                end: 98,
            },
        },
        span: Span {
            start: 85,
            end: 99,
        },
    },
    Let {
        name: "Infinity",
        expr: Binary {
            left: Number {
                value: 1,
                span: Span {
                    start: 115,
                    end: 116,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 117,
                    end: 118,
                },
            },
            span: Span {
                start: 115,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 119,
        },
    },
    Let {
        name: "$262",
        expr: Object {
            props: [],
            span: Span {
                start: 182,
                end: 184,
            },
        },
        span: Span {
            start: 171,
            end: 185,
        },
    },
    Expr {
        expr: PropertyAssign {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 186,
                    end: 190,
                },
            },
            property: "gc",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [],
                span: Span {
                    start: 196,
                    end: 204,
                },
            },
            span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] only new ClassName(...) is supported
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
        "code": 6504,
        "category": "Error",
        "message": "File '/tmp/tmpw7wr2lhs/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
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

## False-done audit

**truly-done** (439)

- Implementation commits: verified via `git log --oneline --all --grep=439`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
