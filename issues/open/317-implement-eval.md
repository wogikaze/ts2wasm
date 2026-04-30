---
id: 317
title: "Implement eval support"
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

Triage eval feature across 469 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 469 cases fail with eval diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: eval feature has 469 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js --detail
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
mise run reference-coverage -- test262 --limit 938
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js
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

- `reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-param.js`
- ... and 459 more files

## Duplicate detection

- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/128-implement-aliasUsedAsNameValue.md` - Implement Aliasusedasnamevalue (same feature label, same group key, title overlap)
- `issues/open/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/open/225-implement-eval-annexb-function-declarations.md` - Implement eval and Annex B function declaration semantics (same feature label, same group key, title overlap)
- `issues/open/281-implement-bigint-number-edge-equality-comparison.md` - Implement BigInt/Number edge equality and comparison (same feature label, same group key, title overlap)
- `issues/open/308-implement-abc451-depth9-gc-cadence-policy.md` - Implement ABC451 depth-9 GC cadence policy (same feature label, same group key, title overlap)
- `issues/open/309-reduce-abc451-depth9-live-allocation-shape.md` - Reduce ABC451 depth-9 live allocation shape (same feature label, same group key)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/062-implement-function.md` - Implement function support (same feature label, same group key, title overlap)
- `issues/done/062b-dynamic-function-constructor-diagnostics.md` - Own dynamic Function constructor diagnostics (same feature label, same group key)

## Smart triage

### Smart triage: Triage name resolution: block decl nostrict

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 583,
  "lines": 27,
  "extension": ".js",
  "first_code_line": "esid: sec-web-compat-evaldeclarationinstantiation",
  "test262_metadata": {
    "esid": "sec-web-compat-evaldeclarationinstantiation",
    "description": ">",
    "info": "|",
    "flags": "[noStrict]"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `f`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
esid: sec-web-compat-evaldeclarationinstantiation
description: >
    AnnexB extension not honored in strict mode, Block statement
    in eval code containing a function declaration
info: |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "err",
    "line": 17,
    "column": 1
  },
  {
    "kind": "function",
    "name": "f",
    "line": 19,
    "column": 9,
    "params": ""
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
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/302-implement-direct-eval-block-function-declaration-slice.md",
    "title": "Implement direct eval block function declaration slice",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Automatic repair sketch:

```rust
// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});
```

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
            start: 454,
            end: 457,
        },
    },
    SpannedToken {
        kind: Ident(
            "err",
        ),
        span: Span {
            start: 458,
            end: 461,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 461,
            end: 462,
        },
    },
    SpannedToken {
        kind: Ident(
            "eval",
        ),
        span: Span {
            start: 464,
            end: 468,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 468,
            end: 469,
        },
    },
    SpannedToken {
        kind: String(
            "{ function f() {} }",
        ),
        span: Span {
            start: 469,
            end: 490,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 490,
            end: 491,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 491,
            end: 492,
        },
    },
    SpannedToken {
        kind: Try,
        span: Span {
            start: 494,
            end: 497,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 498,
            end: 499,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 502,
            end: 503,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 503,
            end: 504,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 505,
            end: 506,
        },
    },
    SpannedToken {
        kind: Catch,
        span: Span {
            start: 507,
            end: 512,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 513,
            end: 514,
        },
    },
    SpannedToken {
        kind: Ident(
            "exception",
        ),
        span: Span {
            start: 514,
            end: 523,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 523,
            end: 524,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 525,
            end: 526,
        },
    },
    SpannedToken {
        kind: Ident(
            "err",
        ),
        span: Span {
            start: 529,
            end: 532,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 533,
            end: 534,
        },
    },
    SpannedToken {
        kind: Ident(
            "exception",
        ),
        span: Span {
            start: 535,
            end: 544,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 544,
            end: 545,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 546,
            end: 547,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 549,
            end: 555,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 555,
            end: 556,
        },
    },
    SpannedToken {
        kind: Ident(
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "err",
        expr: Undefined {
            span: Span {
                start: 458,
                end: 461,
            },
        },
        span: Span {
            start: 454,
            end: 462,
        },
    },
    Function {
        name: "f",
        params: [],
        body: [],
        span: Span {
            start: 464,
            end: 491,
        },
    },
    TryCatch {
        try_block: [
            Expr {
                expr: Ident {
                    name: "f",
                    span: Span {
                        start: 502,
                        end: 503,
                    },
                },
                span: Span {
                    start: 502,
                    end: 504,
                },
            },
        ],
        catch_param: Some(
            "exception",
        ),
        catch_block: Some(
            [
                Assign {
                    name: "err",
                    expr: Ident {
                        name: "exception",
                        span: Span {
                            start: 535,
                            end: 544,
                        },
                    },
                    span: Span {
                        start: 529,
                        end: 545,
                    },
                },
            ],
        ),
        finally_block: None,
        span: Span {
            start: 494,
            end: 545,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 549,
                        end: 555,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 549,
                    end: 565,
                },
            },
            args: [
                Ident {
                    name: "err",
                    span: Span {
                        start: 566,
                        end: 569,
                    },
                },
                Undefined {
                    span: Span {
                        start: 571,
                        end: 580,
                    },
                },
            ],
            span: Span {
                start: 549,
                end: 581,
            },
        },
        span: Span {
            start: 549,
            end: 582,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `f`
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
