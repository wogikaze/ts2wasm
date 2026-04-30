---
id: 320
title: "Implement legacy-global-builtin support"
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

Triage legacy-global-builtin feature across 35 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 35 cases fail with legacy-global-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: legacy-global-builtin feature has 35 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/argument_bigint.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/argument_bigint.js --detail
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
mise run reference-coverage -- test262 --limit 70
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/argument_bigint.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/argument_bigint.js
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

- `reference/test262/test/annexB/built-ins/escape/argument_bigint.js`
- `reference/test262/test/annexB/built-ins/escape/argument_types.js`
- `reference/test262/test/annexB/built-ins/escape/empty-string.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above.js`
- `reference/test262/test/annexB/built-ins/escape/escape-below.js`
- `reference/test262/test/annexB/built-ins/escape/length.js`
- `reference/test262/test/annexB/built-ins/escape/name.js`
- `reference/test262/test/annexB/built-ins/escape/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/escape/prop-desc.js`
- ... and 25 more files

## Duplicate detection

- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same reference path)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage function resolution: argument bigint

- Issue class: `triage-needed`
- Feature label: `function-resolution`
- Diagnostic: `UnresolvedFunction` / `resolver-symbol`
- Path: `reference/test262/test/annexB/built-ins/escape/argument_bigint.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/argument_bigint.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 371,
  "lines": 16,
  "extension": ".js",
  "first_code_line": "esid: sec-escape-string",
  "test262_metadata": {
    "esid": "sec-escape-string",
    "description": "Input is a BigInt",
    "info": "|",
    "features": "[BigInt]"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedFunction",
  "message": "unresolved function: `escape`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "function-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// Copyright (C) 2020 Qu Xing. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
esid: sec-escape-string
description: Input is a BigInt
info: |
    B.2.1.1 escape ( string )
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
    "path": "issues/open/318-implement-function.md",
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
    "path": "issues/done/063-implement-function-resolution.md",
    "title": "Implement function resolution",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 298,
            end: 304,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 304,
            end: 305,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 305,
            end: 314,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 314,
            end: 315,
        },
    },
    SpannedToken {
        kind: Ident(
            "escape",
        ),
        span: Span {
            start: 315,
            end: 321,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 321,
            end: 322,
        },
    },
    SpannedToken {
        kind: BigIntLiteral(
            "1n",
        ),
        span: Span {
            start: 322,
            end: 324,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 324,
            end: 325,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 325,
            end: 326,
        },
    },
    SpannedToken {
        kind: String(
            "1",
        ),
        span: Span {
            start: 327,
            end: 330,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 330,
            end: 331,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 331,
            end: 332,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 334,
            end: 340,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 340,
            end: 341,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 341,
            end: 350,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 350,
            end: 351,
        },
    },
    SpannedToken {
        kind: Ident(
            "escape",
        ),
        span: Span {
            start: 351,
            end: 357,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 357,
            end: 358,
        },
    },
    SpannedToken {
        kind: Minus,
        span: Span {
            start: 358,
            end: 359,
        },
    },
    SpannedToken {
        kind: BigIntLiteral(
            "1n",
        ),
        span: Span {
            start: 359,
            end: 361,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 361,
            end: 362,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 362,
            end: 363,
        },
    },
    SpannedToken {
        kind: String(
            "-1",
        ),
        span: Span {
            start: 364,
            end: 368,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 368,
            end: 369,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 369,
            end: 370,
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 298,
                        end: 304,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 298,
                    end: 314,
                },
            },
            args: [
                Call {
                    callee: Ident {
                        name: "escape",
                        span: Span {
                            start: 315,
                            end: 321,
                        },
                    },
                    args: [
                        BigInt {
                            raw: "1n",
                            span: Span {
                                start: 322,
                                end: 324,
                            },
                        },
                    ],
                    span: Span {
                        start: 315,
                        end: 325,
                    },
                },
                String {
                    value: "1",
                    span: Span {
                        start: 327,
                        end: 330,
                    },
                },
            ],
            span: Span {
                start: 298,
                end: 331,
            },
        },
        span: Span {
            start: 298,
            end: 332,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 334,
                        end: 340,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 334,
                    end: 350,
                },
            },
            args: [
                Call {
                    callee: Ident {
                        name: "escape",
                        span: Span {
                            start: 351,
                            end: 357,
                        },
                    },
                    args: [
                        Unary {
                            op: Negate,
                            expr: BigInt {
                                raw: "1n",
                                span: Span {
                                    start: 359,
                                    end: 361,
                                },
                            },
                            span: Span {
                                start: 358,
                                end: 361,
                            },
                        },
                    ],
                    span: Span {
                        start: 351,
                        end: 362,
                    },
                },
                String {
                    value: "-1",
                    span: Span {
                        start: 364,
                        end: 368,
                    },
                },
            ],
            span: Span {
                start: 334,
                end: 369,
            },
        },
        span: Span {
            start: 334,
            end: 370,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedFunction] unresolved function: `escape`
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
