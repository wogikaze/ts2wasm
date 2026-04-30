---
id: 312
title: "Implement array-builtin support"
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

Triage array-builtin feature across 3082 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3082 cases fail with array-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: array-builtin feature has 3082 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 6164
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`
- `reference/test262/test/built-ins/Array/15.4.5-1.js`
- `reference/test262/test/built-ins/Array/15.4.5.1-5-1.js`
- `reference/test262/test/built-ins/Array/15.4.5.1-5-2.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T2.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T3.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.2_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.3_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A2.1_T1.js`
- ... and 3072 more files

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage array builtin: iterator method emulates undefined

- Issue class: `triage-needed`
- Feature label: `array-builtin`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 812,
  "lines": 29,
  "extension": ".js",
  "first_code_line": "esid: sec-array.from",
  "test262_metadata": {
    "esid": "sec-array.from",
    "description": ">",
    "info": "|",
    "features": "[Symbol.iterator, IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 732..746",
  "span_start": 732,
  "span_end": 746,
  "line": 25,
  "column": 26,
  "feature_label": "array-builtin",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
22 | ---*/
23 | 
24 | var items = {};
25 | items[Symbol.iterator] = $262.IsHTMLDDA;
26 | 
27 | assert.throws(TypeError, function() {
28 |   Array.from(items);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "items",
    "line": 24,
    "column": 1,
    "initializer": "{}"
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
            start: 691,
            end: 694,
        },
    },
    SpannedToken {
        kind: Ident(
            "items",
        ),
        span: Span {
            start: 695,
            end: 700,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 701,
            end: 702,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 703,
            end: 704,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 704,
            end: 705,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 705,
            end: 706,
        },
    },
    SpannedToken {
        kind: Ident(
            "items",
        ),
        span: Span {
            start: 707,
            end: 712,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 712,
            end: 713,
        },
    },
    SpannedToken {
        kind: Ident(
            "Symbol",
        ),
        span: Span {
            start: 713,
            end: 719,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 719,
            end: 720,
        },
    },
    SpannedToken {
        kind: Ident(
            "iterator",
        ),
        span: Span {
            start: 720,
            end: 728,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 728,
            end: 729,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 730,
            end: 731,
        },
    },
    SpannedToken {
        kind: Ident(
            "$262",
        ),
        span: Span {
            start: 732,
            end: 736,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 736,
            end: 737,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 737,
            end: 746,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 746,
            end: 747,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 749,
            end: 755,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 755,
            end: 756,
        },
    },
    SpannedToken {
        kind: Ident(
            "throws",
        ),
        span: Span {
            start: 756,
            end: 762,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 762,
            end: 763,
        },
    },
    SpannedToken {
        kind: Ident(
            "TypeError",
        ),
        span: Span {
            start: 763,
            end: 772,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 772,
            end: 773,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 774,
            end: 782,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 782,
            end: 783,
        },
    },
    SpannedToken {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "items",
        expr: Object {
            props: [],
            span: Span {
                start: 703,
                end: 705,
            },
        },
        span: Span {
            start: 691,
            end: 706,
        },
    },
    Expr {
        expr: IndexAssign {
            object: Ident {
                name: "items",
                span: Span {
                    start: 707,
                    end: 712,
                },
            },
            index: Member {
                object: Ident {
                    name: "Symbol",
                    span: Span {
                        start: 713,
                        end: 719,
                    },
                },
                property: "iterator",
                span: Span {
                    start: 713,
                    end: 728,
                },
            },
            value: Member {
                object: Ident {
                    name: "$262",
                    span: Span {
                        start: 732,
                        end: 736,
                    },
                },
                property: "IsHTMLDDA",
                span: Span {
                    start: 732,
                    end: 746,
                },
            },
            span: Span {
                start: 707,
                end: 747,
            },
        },
        span: Span {
            start: 707,
            end: 747,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 749,
                        end: 755,
                    },
                },
                property: "throws",
                span: Span {
                    start: 749,
                    end: 762,
                },
            },
            args: [
                Ident {
                    name: "TypeError",
                    span: Span {
                        start: 763,
                        end: 772,
                    },
                },
                FunctionExpr {
                    name: "",
                    params: [],
                    body: [
                        Expr {
                            expr: Call {
                                callee: Member {
                                    object: Ident {
                                        name: "Array",
                                        span: Span {
                                            start: 789,
                                            end: 794,
                                        },
                                    },
                                    property: "from",
                                    span: Span {
                                        start: 789,
                                        end: 799,
                                    },
                                },
                                args: [
                                    Ident {
                                        name: "items",
                                        span: Span {
                                            start: 800,
                                            end: 805,
                                        },
                                    },
                                ],
                                span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 732..746
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
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 732..746
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
