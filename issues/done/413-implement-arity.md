---
id: 413
title: "Implement arity support"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage arity feature across 34 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 34 cases fail with arity diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arity feature has 34 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- test262 --limit 68
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js --detail
mise run reference-triage -- test262 reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

### Triage findings

**Root cause**: `validate.rs` line 364-377 checks builtin arity with `args.len() != expected`, which is too strict. JavaScript allows calling any function with fewer arguments (missing args become `undefined`). The same pattern exists in `program_builtins.rs` for RegExp/String prototype method arity checks.

**Duplicate detection results**:
- `issues/done/287-fix-arguments-object-arity-mismatch.md` — NOT a match. Issue 287 was about `arguments` object arity mismatch for **user-defined functions** (`function 5 expects at least 3 argument(s), got 2`). Issue 413 is about **builtin** arity mismatch. Different fix location (validate.rs builtin branch vs. user function min_required_params).
- `issues/done/341c-boolean-global.md` — Partial overlap. Implements `Boolean(x)` for 1-arg calls, but does NOT cover `Boolean()` with 0 args. The 0-arg case is covered by child issue 5135.
- `issues/open/2460-implement-functionParameterArityMismatch.md` — NOT a match. This is about TypeScript compiler's `functionParameterArityMismatch` diagnostic (a TS type-checking error), blocked on issue 5005. Different domain and fix location.

**Child issues created**:
- `issues/done/5135-fix-builtin-arity-validation-coercion-globals.md` — Fix validate.rs + builtin.rs for Boolean, Number, isNaN, isFinite, parseInt, parseFloat, encodeURI, decodeURI, escape, unescape (~20-25 test cases)
- `issues/open/5136-fix-arity-validation-regexp-string-prototype.md` — Fix program_builtins.rs + resolver_expr.rs for RegExp.prototype.exec/test and String.prototype.match/search (~10 test cases)

## Affected test files

- `reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js`
- `reference/test262/test/built-ins/Number/S15.7.1.1_A2.js`
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A12.js`
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A1_T16.js`
- `reference/test262/test/built-ins/RegExp/prototype/test/S15.10.6.3_A1_T16.js`
- `reference/test262/test/built-ins/isFinite/tonumber-operations.js`
- `reference/test262/test/built-ins/isNaN/tonumber-operations.js`
- `reference/test262/test/built-ins/parseInt/S15.1.2.2_A3.1_T1.js`
- `reference/test262/test/built-ins/parseInt/S15.1.2.2_A3.1_T2.js`
- `reference/test262/test/built-ins/parseInt/S15.1.2.2_A3.1_T3.js`
- ... and 24 more files

## Duplicate detection

- `issues/open/021-implement-full-wasm-backend.md` - issues/open/021-implement-full-wasm-backend.md (same feature label, same group key)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/open/300-support-abc451-large-integer-number-boundary.md` - Support ABC451 large integer number boundary (same feature label, same group key, title overlap)
- `issues/done/370-implement-bigint-arithmetic-exception-parity.md` - Implement BigInt arithmetic RangeError and TypeError parity (same feature label, same group key, title overlap)
- `issues/done/374-design-broader-object-toprimitive-for-bigint-comparisons.md` - Design broader object ToPrimitive for mixed BigInt comparisons (same feature label, same group key)
- `issues/done/407-map-spread-key-preserving-iterator-storage.md` - Implement key-preserving Map entry storage for spread iteration (same feature label, same group key, title overlap)
- `issues/done/021a-implement-wasm-encoder-hello-binary-mvp.md` - Implement wasm-encoder hello binary MVP (same feature label, same group key, title overlap)
- `issues/done/033-implement-switch-statement.md` - Implement switch statement (same feature label, same group key, title overlap)
- `issues/done/049-implement-map-set.md` - Implement Map and Set (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage arity: S15.6.1.1 A2

- Issue class: `triage-needed`
- Feature label: `arity`
- Diagnostic: `ArityMismatch` / `compiler-diagnostic`
- Path: `reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 443,
  "lines": 10,
  "extension": ".js",
  "first_code_line": "info: Boolean() returns false",
  "test262_metadata": {
    "info": "Boolean() returns false",
    "esid": "sec-terms-and-definitions-boolean-value",
    "description": "Call Boolean() and check result"
  }
}
```

Failure location:

```json
{
  "code": "ArityMismatch",
  "message": "builtin BooleanCoerce expects 1 argument(s), got 0",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "arity",
  "error_type": "compiler-diagnostic"
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
[
  {
    "state": "done",
    "path": "issues/done/287-fix-arguments-object-arity-mismatch.md",
    "title": "Fix arguments-object arity mismatch bucket",
    "reason": "same feature label, title overlap"
  }
]
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
error: [ArityMismatch] builtin BooleanCoerce expects 1 argument(s), got 0
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
        "message": "File '/tmp/tmpgadeunvj/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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

- `...` (this issue is a triage spike, not an implementation issue; child issues 5135 and 5136 contain the implementation commits)

Validation result:

```text
command: Triage analysis of 34 arity-related test262 failures
result: PASS — split into 2 implementation-ready child issues (5135, 5136)
date: 2026-05-06

Child issues:
- issues/done/5135-fix-builtin-arity-validation-coercion-globals.md (coercion/math globals)
- issues/open/5136-fix-arity-validation-regexp-string-prototype.md (RegExp/String prototype methods)

Duplicate check:
- issues/done/287-fix-arguments-object-arity-mismatch.md → NOT a match (user-function arity)
- issues/done/341c-boolean-global.md → Partial overlap (Boolean(1) implemented, Boolean() not covered)
- issues/open/2460-implement-functionParameterArityMismatch.md → NOT a match (TypeScript diagnostic)
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

