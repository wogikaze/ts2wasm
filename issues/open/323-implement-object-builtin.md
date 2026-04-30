---
id: 323
title: "Implement object-builtin support"
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

Triage object-builtin feature across 1215 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1215 cases fail with object-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: object-builtin feature has 1215 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 2430
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A1_T1.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A1_T2.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A1_T3.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A1_T4.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A1_T5.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A2_T1.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A2_T10.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A2_T11.js`
- `reference/test262/test/built-ins/Object/S15.2.1.1_A2_T12.js`
- ... and 1205 more files

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (same reference path, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/237-implement-annexb-ishtmldda-compatibility.md` - Implement Annex B IsHTMLDDA compatibility (same reference path, title overlap)

## Smart triage

### Smart triage: Triage object builtin: emulates undefined

- Issue class: `triage-needed`
- Feature label: `object-builtin`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 835,
  "lines": 26,
  "extension": ".js",
  "first_code_line": "esid: sec-object.is",
  "test262_metadata": {
    "esid": "sec-object.is",
    "description": ">",
    "info": "|",
    "features": "[IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 443..457",
  "span_start": 443,
  "span_end": 457,
  "line": 18,
  "column": 17,
  "feature_label": "object-builtin",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 | features: [IsHTMLDDA]
16 | ---*/
17 | 
18 | var IsHTMLDDA = $262.IsHTMLDDA;
19 | 
20 | assert.sameValue(Object.is(IsHTMLDDA, undefined), false, "SameValue with `undefined`");
21 | assert.sameValue(Object.is(undefined, IsHTMLDDA), false, "SameValue with `undefined`");
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "IsHTMLDDA",
    "line": 18,
    "column": 1,
    "initializer": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same reference path"
  },
  {
    "state": "done",
    "path": "issues/done/237-implement-annexb-ishtmldda-compatibility.md",
    "title": "Implement Annex B IsHTMLDDA compatibility",
    "reason": "same reference path"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Classify this as runtime/API work unless the parser fails before builtin resolution.

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
            start: 427,
            end: 430,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 431,
            end: 440,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 441,
            end: 442,
        },
    },
    SpannedToken {
        kind: Ident(
            "$262",
        ),
        span: Span {
            start: 443,
            end: 447,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 447,
            end: 448,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 448,
            end: 457,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 457,
            end: 458,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 460,
            end: 466,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 466,
            end: 467,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 467,
            end: 476,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 476,
            end: 477,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 477,
            end: 483,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 483,
            end: 484,
        },
    },
    SpannedToken {
        kind: Ident(
            "is",
        ),
        span: Span {
            start: 484,
            end: 486,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 486,
            end: 487,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 487,
            end: 496,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 496,
            end: 497,
        },
    },
    SpannedToken {
        kind: Undefined,
        span: Span {
            start: 498,
            end: 507,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 507,
            end: 508,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 508,
            end: 509,
        },
    },
    SpannedToken {
        kind: False,
        span: Span {
            start: 510,
            end: 515,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 515,
            end: 516,
        },
    },
    SpannedToken {
        kind: String(
            "SameValue with `undefined`",
        ),
        span: Span {
            start: 517,
            end: 545,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 545,
            end: 546,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 546,
            end: 547,
        },
    },
    SpannedTok
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
                    start: 443,
                    end: 447,
                },
            },
            property: "IsHTMLDDA",
            span: Span {
                start: 443,
                end: 457,
            },
        },
        span: Span {
            start: 427,
            end: 458,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 460,
                        end: 466,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 460,
                    end: 476,
                },
            },
            args: [
                Call {
                    callee: Member {
                        object: Ident {
                            name: "Object",
                            span: Span {
                                start: 477,
                                end: 483,
                            },
                        },
                        property: "is",
                        span: Span {
                            start: 477,
                            end: 486,
                        },
                    },
                    args: [
                        Ident {
                            name: "IsHTMLDDA",
                            span: Span {
                                start: 487,
                                end: 496,
                            },
                        },
                        Undefined {
                            span: Span {
                                start: 498,
                                end: 507,
                            },
                        },
                    ],
                    span: Span {
                        start: 477,
                        end: 508,
                    },
                },
                Bool {
                    value: false,
                    span: Span {
                        start: 510,
                        end: 515,
                    },
                },
                String {
                    value: "SameValue with `undefined`",
                    span: Span {
                        start: 517,
                        end: 545,
                    },
                },
            ],
            span: Span {
                start: 460,
                end: 546,
            },
        },
        span: Span {
            start: 460,
            end: 547,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 548,
                        end: 554,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 548,
                    end: 564,
                },
            },
            args: [
                Call {
                    callee: Member {
                        object: Ident {
                            name: "Object",
                            span: Span {
                                start: 565,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 443..457
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
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 443..457
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
