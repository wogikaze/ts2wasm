---
id: 313
title: "Implement built-in API support"
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

Triage builtin-api feature across 3517 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3517 cases fail with builtin-api diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: builtin-api feature has 3517 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 7034
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js`
- `reference/test262/test/built-ins/AbstractModuleSource/length.js`
- `reference/test262/test/built-ins/AbstractModuleSource/name.js`
- `reference/test262/test/built-ins/AbstractModuleSource/proto.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype/Symbol.toStringTag.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype/constructor.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype/proto.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype.js`
- `reference/test262/test/built-ins/AbstractModuleSource/throw-from-constructor.js`
- `reference/test262/test/built-ins/AggregateError/cause-property.js`
- ... and 3507 more files

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage builtin api: iterator method emulates undefined

- Issue class: `triage-needed`
- Feature label: `builtin-api`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1018,
  "lines": 35,
  "extension": ".js",
  "first_code_line": "esid: sec-%typedarray%.from",
  "test262_metadata": {
    "esid": "sec-%typedarray%.from",
    "description": ">",
    "info": "|",
    "includes": "[testTypedArray.js]",
    "features": "[Symbol.iterator, TypedArray, IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 869..883",
  "span_start": 869,
  "span_end": 883,
  "line": 29,
  "column": 26,
  "feature_label": "builtin-api",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
26 | ---*/
27 | 
28 | var items = {};
29 | items[Symbol.iterator] = $262.IsHTMLDDA;
30 | 
31 | testWithTypedArrayConstructors(function(TypedArray) {
32 |   assert.throws(TypeError, function() {
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "items",
    "line": 28,
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
            start: 828,
            end: 831,
        },
    },
    SpannedToken {
        kind: Ident(
            "items",
        ),
        span: Span {
            start: 832,
            end: 837,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 838,
            end: 839,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 840,
            end: 841,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 841,
            end: 842,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 842,
            end: 843,
        },
    },
    SpannedToken {
        kind: Ident(
            "items",
        ),
        span: Span {
            start: 844,
            end: 849,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 849,
            end: 850,
        },
    },
    SpannedToken {
        kind: Ident(
            "Symbol",
        ),
        span: Span {
            start: 850,
            end: 856,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 856,
            end: 857,
        },
    },
    SpannedToken {
        kind: Ident(
            "iterator",
        ),
        span: Span {
            start: 857,
            end: 865,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 865,
            end: 866,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 867,
            end: 868,
        },
    },
    SpannedToken {
        kind: Ident(
            "$262",
        ),
        span: Span {
            start: 869,
            end: 873,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 873,
            end: 874,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 874,
            end: 883,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 883,
            end: 884,
        },
    },
    SpannedToken {
        kind: Ident(
            "testWithTypedArrayConstructors",
        ),
        span: Span {
            start: 886,
            end: 916,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 916,
            end: 917,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 917,
            end: 925,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 925,
            end: 926,
        },
    },
    SpannedToken {
        kind: Ident(
            "TypedArray",
        ),
        span: Span {
            start: 926,
            end: 936,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 936,
            end: 937,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 938,
            end: 939,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 942,
            end: 948,
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
                start: 840,
                end: 842,
            },
        },
        span: Span {
            start: 828,
            end: 843,
        },
    },
    Expr {
        expr: IndexAssign {
            object: Ident {
                name: "items",
                span: Span {
                    start: 844,
                    end: 849,
                },
            },
            index: Member {
                object: Ident {
                    name: "Symbol",
                    span: Span {
                        start: 850,
                        end: 856,
                    },
                },
                property: "iterator",
                span: Span {
                    start: 850,
                    end: 865,
                },
            },
            value: Member {
                object: Ident {
                    name: "$262",
                    span: Span {
                        start: 869,
                        end: 873,
                    },
                },
                property: "IsHTMLDDA",
                span: Span {
                    start: 869,
                    end: 883,
                },
            },
            span: Span {
                start: 844,
                end: 884,
            },
        },
        span: Span {
            start: 844,
            end: 884,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "testWithTypedArrayConstructors",
                span: Span {
                    start: 886,
                    end: 916,
                },
            },
            args: [
                FunctionExpr {
                    name: "",
                    params: [
                        (
                            "TypedArray",
                            None,
                            false,
                        ),
                    ],
                    body: [
                        Expr {
                            expr: Call {
                                callee: Member {
                                    object: Ident {
                                        name: "assert",
                                        span: Span {
                                            start: 942,
                                            end: 948,
                                        },
                                    },
                                    property: "throws",
                                    span: Span {
                                        start: 942,
                                        end: 955,
                                    },
                                },
                                args: [
                                    Ident {
                                        name: "TypeError",
                                        span: Span {
                                            start: 956,
                                            end: 965,
                                        },
                                    },
                                    FunctionExpr {
                                        name: "",
                                        params: [],
                                        body: [
                                            Expr {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 869..883
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
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 869..883
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
