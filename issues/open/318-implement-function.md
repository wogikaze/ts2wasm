---
id: 318
title: "Implement function support"
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

Triage function feature across 515 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 515 cases fail with function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: function feature has 515 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js --detail
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
mise run reference-coverage -- test262 --limit 1030
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
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

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-params.js`
- `reference/test262/test/built-ins/Function/15.3.2.1-10-6gs.js`
- `reference/test262/test/built-ins/Function/15.3.2.1-11-1-s.js`
- `reference/test262/test/built-ins/Function/15.3.2.1-11-1.js`
- `reference/test262/test/built-ins/Function/15.3.2.1-11-2-s.js`
- ... and 505 more files

## Duplicate detection

- `issues/open/017b-implement-gc-strategy.md` - issues/open/017b-implement-gc-strategy.md (same feature label, same group key)
- `issues/open/021-implement-full-wasm-backend.md` - issues/open/021-implement-full-wasm-backend.md (same feature label, same group key)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/open/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)
- `issues/open/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same feature label, same group key, title overlap)
- `issues/open/079-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)
- `issues/open/089-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage function: createdynfn html close comment body

- Issue class: `triage-needed`
- Feature label: `function`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 721,
  "lines": 20,
  "extension": ".js",
  "first_code_line": "esid: sec-createdynamicfunction",
  "test262_metadata": {
    "esid": "sec-createdynamicfunction",
    "description": ">",
    "info": "|",
    "19.2.1.1.1 Runtime Semantics": "CreateDynamicFunction(constructor, newTarget, kind, args)"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 702..719",
  "span_start": 702,
  "span_end": 719,
  "line": 20,
  "column": 2,
  "feature_label": "function",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
17 |     ...
18 | ---*/
19 | 
20 | Function("\n-->");
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
    "path": "issues/open/225-implement-eval-annexb-function-declarations.md",
    "title": "Implement eval and Annex B function declaration semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/036-implement-arrow-function.md",
    "title": "Implement arrow function",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/052g-implement-json-stringify-function-replacer-callbacks.md",
    "title": "Implement JSON.stringify function replacer callbacks",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/057-implement-function-resolution.md",
    "title": "Implement function resolution for function calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062-implement-function.md",
    "title": "Implement function support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062a-split-function-epic-into-callable-child-issues.md",
    "title": "Split function epic into callable child issues",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062b-dynamic-function-constructor-diagnostics.md",
    "title": "Own dynamic Function constructor diagnostics",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062c-ordinary-function-declarations-and-calls.md",
    "title": "Implement ordinary function declarations and direct calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062d-function-this-and-arguments.md",
    "title": "Implement function this and arguments semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062e-function-closures.md",
    "title": "Implement function closures",
    "reason": "same feature label, title overlap"
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "Function",
        ),
        span: Span {
            start: 702,
            end: 710,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 710,
            end: 711,
        },
    },
    SpannedToken {
        kind: String(
            "\n-->",
        ),
        span: Span {
            start: 711,
            end: 718,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 718,
            end: 719,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 719,
            end: 720,
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
    Expr {
        expr: Call {
            callee: Ident {
                name: "Function",
                span: Span {
                    start: 702,
                    end: 710,
                },
            },
            args: [
                String {
                    value: "\n-->",
                    span: Span {
                        start: 711,
                        end: 718,
                    },
                },
            ],
            span: Span {
                start: 702,
                end: 719,
            },
        },
        span: Span {
            start: 702,
            end: 720,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 702..719
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
error: [UnsupportedSyntax] issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 702..719
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
