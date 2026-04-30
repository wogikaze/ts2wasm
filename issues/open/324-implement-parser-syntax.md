---
id: 324
title: "Implement parser syntax extensions"
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

Triage parser-syntax feature across 319 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 319 cases fail with parser-syntax diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: parser-syntax feature has 319 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 638
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js
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

- `reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-as-for-in-lhs.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-as-for-of-lhs.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-postfix-update.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-prefix-update.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/cover-callexpression-and-asyncarrowhead.js`
- `reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js`
- `reference/test262/test/annexB/language/expressions/yield/star-iterable-throw-emulates-undefined-throws-when-called.js`
- ... and 309 more files

## Duplicate detection

- `issues/open/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/open/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)
- `issues/open/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/open/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same feature label, same group key, title overlap)
- `issues/open/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same feature label, same group key, title overlap)
- `issues/open/079-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)
- `issues/open/081-implement-TransportStream.md` - Implement Transportstream (same feature label, same group key, title overlap)
- `issues/open/084-implement-abstractClassUnionInstantiation.md` - Implement Abstractclassunioninstantiation (same feature label, same group key, title overlap)
- `issues/open/086-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: array pattern emulates undefined

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/array-pattern-emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1292,
  "lines": 44,
  "extension": ".js",
  "first_code_line": "esid: sec-destructuring-binding-patterns-runtime-semantics-bindinginitialization",
  "test262_metadata": {
    "esid": "sec-destructuring-binding-patterns-runtime-semantics-bindinginitialization",
    "description": ">",
    "info": "|",
    "BindingPattern": "ArrayBindingPattern",
    "Runtime Semantics": "IteratorBindingInitialization",
    "SingleNameBinding": "BindingIdentifier Initializer[opt]",
    "features": "[destructuring-binding, IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 979..993",
  "span_start": 979,
  "span_end": 993,
  "line": 28,
  "column": 17,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
25 | features: [destructuring-binding, IsHTMLDDA]
26 | ---*/
27 | 
28 | var IsHTMLDDA = $262.IsHTMLDDA;
29 | var initCount = 0;
30 | var counter = function() {
31 |   initCount += 1;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "IsHTMLDDA",
    "line": 28,
    "column": 1,
    "initializer": ""
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
            start: 963,
            end: 966,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 967,
            end: 976,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 977,
            end: 978,
        },
    },
    SpannedToken {
        kind: Ident(
            "$262",
        ),
        span: Span {
            start: 979,
            end: 983,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 983,
            end: 984,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 984,
            end: 993,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 993,
            end: 994,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 995,
            end: 998,
        },
    },
    SpannedToken {
        kind: Ident(
            "initCount",
        ),
        span: Span {
            start: 999,
            end: 1008,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 1009,
            end: 1010,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 1011,
            end: 1012,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 1012,
            end: 1013,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 1014,
            end: 1017,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 1018,
            end: 1025,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 1026,
            end: 1027,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 1028,
            end: 1036,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 1036,
            end: 1037,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 1037,
            end: 1038,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 1039,
            end: 1040,
        },
    },
    SpannedToken {
        kind: Ident(
            "initCount",
        ),
        span: Span {
            start: 1043,
            end: 1052,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 1053,
            end: 1055,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 1056,
            end: 1057,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 1057,
            end: 1058,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 1059,
            end: 1060,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 1060,
            end: 1061,
        },
    },
    SpannedToken {
        kind
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "IsHTMLDDA",
        expr: Member {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 979,
                    end: 983,
                },
            },
            property: "IsHTMLDDA",
            span: Span {
                start: 979,
                end: 993,
            },
        },
        span: Span {
            start: 963,
            end: 994,
        },
    },
    Let {
        name: "initCount",
        expr: Number {
            value: 0,
            span: Span {
                start: 1011,
                end: 1012,
            },
        },
        span: Span {
            start: 995,
            end: 1013,
        },
    },
    Let {
        name: "counter",
        expr: FunctionExpr {
            name: "",
            params: [],
            body: [
                Assign {
                    name: "initCount",
                    expr: Binary {
                        left: Ident {
                            name: "initCount",
                            span: Span {
                                start: 1043,
                                end: 1052,
                            },
                        },
                        op: Add,
                        right: Number {
                            value: 1,
                            span: Span {
                                start: 1056,
                                end: 1057,
                            },
                        },
                        span: Span {
                            start: 1043,
                            end: 1057,
                        },
                    },
                    span: Span {
                        start: 1043,
                        end: 1058,
                    },
                },
            ],
            span: Span {
                start: 1028,
                end: 1058,
            },
        },
        span: Span {
            start: 1014,
            end: 1061,
        },
    },
    Let {
        name: "x",
        expr: Undefined {
            span: Span {
                start: 1067,
                end: 1068,
            },
        },
        span: Span {
            start: 1063,
            end: 1069,
        },
    },
    Expr {
        expr: Assign {
            name: "[x = counter()]",
            expr: Array {
                elements: [
                    Ident {
                        name: "IsHTMLDDA",
                        span: Span {
                            start: 1090,
                            end: 1099,
                        },
                    },
                ],
                span: Span {
                    start: 1089,
                    end: 1100,
                },
            },
            span: Span {
                start: 1071,
                end: 1101,
            },
        },
        span: Span {
            start: 1071,
            end: 1102,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 1104,
                        end: 1110,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 1104,
                    end: 1120,
                },
            },
            args: [
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 979..993
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
  },
  "ast_error": "node:internal/modules/cjs/loader:1423\n  throw err;\n  ^\n\nError: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/[eval]\n    at Module._resolveFilename (node:internal/modules/cjs/loader:1420:15)\n    at defaultResolveImpl (node:internal/modules/cjs/loader:1058:19)\n    at resolveForCJSWithHooks (node:internal/modules/cjs/loader:1063:22)\n    at Module._load (node:internal/modules/cjs/loader:1226:37)\n    at TracingChannel.traceSync (node:diagnostics_channel:328:14)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:245:24)\n    at Module.require (node:internal/modules/cjs/loader:1503:12)\n    at require (node:internal/modules/helpers:152:16)\n    at [eval]:3:12\n    at runScriptInThisContext (node:internal/vm:219:10) {\n  code: 'MODULE_NOT_FOUND',\n  requireStack: [ '/home/wogikaze/ts2wasm/[eval]' ]\n}\n\nNode.js v25.2.1\n"
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 979..993
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
