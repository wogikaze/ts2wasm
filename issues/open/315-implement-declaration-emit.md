---
id: 315
title: "Implement declaration-emit support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Triage declaration-emit feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with declaration-emit diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: declaration-emit feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js --detail
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
mise run reference-coverage -- test262 --limit 10
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js
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

- `reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js`
- `reference/test262/test/annexB/language/statements/try/catch-redeclared-for-of-var.js`
- `reference/test262/test/annexB/language/statements/try/catch-redeclared-for-var.js`
- `reference/test262/test/annexB/language/statements/try/catch-redeclared-var-statement-captured.js`
- `reference/test262/test/annexB/language/statements/try/catch-redeclared-var-statement.js`

## Duplicate detection

- `issues/open/096-implement-accessorDeclarationEmitJs.md` - Implement Accessordeclarationemitjs (same feature label, same group key, title overlap)
- `issues/open/097-implement-accessorDeclarationEmitVisibilityErrors.md` - Implement Accessordeclarationemitvisibilityerrors (same feature label, same group key, title overlap)
- `issues/open/141-implement-ambientClassDeclaredBeforeBase.md` - Implement Ambientclassdeclaredbeforebase (same feature label, same group key, title overlap)
- `issues/open/170-implement-amdDeclarationEmitNoExtraDeclare.md` - Implement Amddeclarationemitnoextradeclare (same feature label, same group key, title overlap)
- `issues/open/173-implement-amdLikeInputDeclarationEmit.md` - Implement Amdlikeinputdeclarationemit (same feature label, same group key, title overlap)
- `issues/open/174-implement-amdModuleBundleNoDuplicateDeclarationEmitComments.md` - Implement Amdmodulebundlenoduplicatedeclarationemitcomments (same feature label, same group key, title overlap)
- `issues/open/177-implement-anonClassDeclarationEmitIsAnon.md` - Implement Anonclassdeclarationemitisanon (same feature label, same group key, title overlap)
- `issues/open/184-implement-anyDeclare.md` - Implement Anydeclare (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)

## Smart triage

### Smart triage: Triage declaration emit: catch redeclared for in var

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `Unknown` / `unknown`
- Path: `reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/try/catch-redeclared-for-in-var.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 828,
  "lines": 27,
  "extension": ".js",
  "first_code_line": "esid: sec-variablestatements-in-catch-blocks",
  "test262_metadata": {
    "esid": "sec-variablestatements-in-catch-blocks",
    "es6id": "B.3.5",
    "description": "Re-declaration of catch parameter (for-in statement)",
    "info": "|",
    "CatchParameter": "BindingIdentifier."
  }
}
```

Failure location:

```json
{
  "code": "Unknown",
  "message": "",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "declaration-emit",
  "error_type": "unknown"
}
```

Source context:

```text
// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
esid: sec-variablestatements-in-catch-blocks
es6id: B.3.5
description: Re-declaration of catch parameter (for-in statement)
info: |
    It is a Syntax Error if any element of the BoundNames of CatchParameter
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "before",
    "line": 13,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "err",
    "line": 19,
    "column": 8
  }
]
```

Duplicate candidates:

```json
[]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 474,
            end: 477,
        },
    },
    SpannedToken {
        kind: Ident(
            "before",
        ),
        span: Span {
            start: 478,
            end: 484,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 484,
            end: 485,
        },
    },
    SpannedToken {
        kind: Ident(
            "during",
        ),
        span: Span {
            start: 486,
            end: 492,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 492,
            end: 493,
        },
    },
    SpannedToken {
        kind: Ident(
            "after",
        ),
        span: Span {
            start: 494,
            end: 499,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 499,
            end: 500,
        },
    },
    SpannedToken {
        kind: Try,
        span: Span {
            start: 502,
            end: 505,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 506,
            end: 507,
        },
    },
    SpannedToken {
        kind: Throw,
        span: Span {
            start: 510,
            end: 515,
        },
    },
    SpannedToken {
        kind: String(
            "exception",
        ),
        span: Span {
            start: 516,
            end: 527,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 527,
            end: 528,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 529,
            end: 530,
        },
    },
    SpannedToken {
        kind: Catch,
        span: Span {
            start: 531,
            end: 536,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 537,
            end: 538,
        },
    },
    SpannedToken {
        kind: Ident(
            "err",
        ),
        span: Span {
            start: 538,
            end: 541,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 541,
            end: 542,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 543,
            end: 544,
        },
    },
    SpannedToken {
        kind: Ident(
            "before",
        ),
        span: Span {
            start: 547,
            end: 553,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 554,
            end: 555,
        },
    },
    SpannedToken {
        kind: Ident(
            "err",
        ),
        span: Span {
            start: 556,
            end: 559,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 559,
            end: 560,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 563,
            end: 566,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 567,
            end: 568,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 568,
            end: 571,
        },
    },
    SpannedToken {
        kind: Ident(
            "err",
        ),
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "before",
        expr: Undefined {
            span: Span {
                start: 478,
                end: 484,
            },
        },
        span: Span {
            start: 474,
            end: 500,
        },
    },
    Let {
        name: "during",
        expr: Undefined {
            span: Span {
                start: 486,
                end: 492,
            },
        },
        span: Span {
            start: 474,
            end: 492,
        },
    },
    Let {
        name: "after",
        expr: Undefined {
            span: Span {
                start: 494,
                end: 499,
            },
        },
        span: Span {
            start: 474,
            end: 499,
        },
    },
    TryCatch {
        try_block: [
            Throw {
                expr: String {
                    value: "exception",
                    span: Span {
                        start: 516,
                        end: 527,
                    },
                },
                span: Span {
                    start: 510,
                    end: 528,
                },
            },
        ],
        catch_param: Some(
            "err",
        ),
        catch_block: Some(
            [
                Assign {
                    name: "before",
                    expr: Ident {
                        name: "err",
                        span: Span {
                            start: 556,
                            end: 559,
                        },
                    },
                    span: Span {
                        start: 547,
                        end: 560,
                    },
                },
                ForIn {
                    var: "err",
                    iter: Object {
                        props: [
                            (
                                "propertyName",
                                Null {
                                    span: Span {
                                        start: 595,
                                        end: 599,
                                    },
                                },
                            ),
                        ],
                        span: Span {
                            start: 579,
                            end: 601,
                        },
                    },
                    body: [
                        Assign {
                            name: "during",
                            expr: Ident {
                                name: "err",
                                span: Span {
                                    start: 618,
                                    end: 621,
                                },
                            },
                            span: Span {
                                start: 609,
                                end: 622,
                            },
                        },
                    ],
                    span: Span {
                        start: 563,
                        end: 622,
                    },
                },
                Assign {
                    name: "after",
                    expr: Ident {
                        name: "err",
                        span: Span {
                            start: 637,
                            end: 640,
                        },
                    },
                    span: Span {
```

#### resolved

- ok: `True`
- truncated: `False`

```text
== resolved ==
[
    Let(
        "before",
        Undefined,
    ),
    Let(
        "during",
        Undefined,
    ),
    Let(
        "after",
        Undefined,
    ),
    TryCatch {
        try_block: [
            Throw(
                String(
                    "exception",
                ),
            ),
        ],
        catch_param: Some(
            "err",
        ),
        catch_block: Some(
            [
                Assign(
                    "before",
                    Ident(
                        "err",
                    ),
                ),
                ForIn {
                    var: "err",
                    iter: Object(
                        [
                            (
                                "propertyName",
                                Null,
                            ),
                        ],
                    ),
                    body: [
                        Assign(
                            "during",
                            Ident(
                                "err",
                            ),
                        ),
                    ],
                },
                Assign(
                    "after",
                    Ident(
                        "err",
                    ),
                ),
            ],
        ),
        finally_block: None,
    },
    If {
        condition: Binary {
            left: Ident(
                "before",
            ),
            op: StrictNotEqual,
            right: String(
                "exception",
            ),
        },
        then_body: [
            Expr(
                BuiltinCall {
                    builtin: ConsoleLog,
                    args: [
                        String(
                            "__TS2WASM_TEST262_ASSERT_FAIL__",
                        ),
                    ],
                },
            ),
        ],
        else_body: [],
    },
    If {
        condition: Binary {
            left: Ident(
                "during",
            ),
            op: StrictNotEqual,
            right: String(
                "propertyName",
            ),
        },
        then_body: [
            Expr(
                BuiltinCall {
                    builtin: ConsoleLog,
                    args: [
                        String(
                            "__TS2WASM_TEST262_ASSERT_FAIL__",
                        ),
                    ],
                },
            ),
        ],
        else_body: [],
    },
    If {
        condition: Binary {
            left: Ident(
                "after",
            ),
            op: StrictNotEqual,
            right: String(
                "propertyName",
            ),
        },
        then_body: [
            Expr(
                BuiltinCall {
                    builtin: ConsoleLog,
                    args: [
                        String(
                            "__TS2WASM_TEST262_ASSERT_FAIL__",
                        ),
                    ],
                },
            ),
        ],
        else_body: [],
    },
]
```

TypeScript/JavaScript oracle:

```json
{
  "ok": false,
  "returncode": 2,
  "typescript": {
    "ok": false,
    "error": "failed to load TypeScript compiler API: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/scripts/check/typescript-oracle.js",
    "diagnostics": [],
    "hints": []
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
