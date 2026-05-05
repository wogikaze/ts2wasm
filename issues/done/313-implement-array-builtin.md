---
id: 313
title: "Implement array-builtin support"
type: feature
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-05-02
---

## Summary

Implement Array builtin method support for runtime (test262 coverage). Consolidated from generated fixture-bucket issues.

## Problem

Reference test results show 31 cases fail with array-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

This issue tracks Array builtin API implementation. Generated child fixture-bucket issues (3130-3139) have been consolidated back into this parent and archived (now in `issues/open/`).

In scope:
- [x] Implement Array.prototype.indexOf, Array.prototype.lastIndexOf, Array.prototype.every, etc.
- [x] Add Node/iwasm differential fixture coverage for supported Array methods
- [x] Reduce test262 `array-builtin` unsupported count

Out of scope:
- String builtins (tracked by issue 314)
- Object builtins (tracked by issue 342)
- JsxFactory/keepImportsInDts/keyRemappingKeyofResult (residual fixture buckets, originally misclassified under 313 prefix)

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- test262 --limit 62
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js --detail
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js
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

- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-10.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-11.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-15.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-16.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-17.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-18.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-19.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-20.js`
- `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-21.js`
- ... and 21 more files

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage backend io: 15.4.4.14 5 1

- Issue class: `triage-needed`
- Feature label: `backend-io`
- Diagnostic: `BackendIo` / `backend-io`
- Path: `reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 400,
  "lines": 12,
  "extension": ".js",
  "first_code_line": "esid: sec-array.prototype.indexof",
  "test262_metadata": {
    "esid": "sec-array.prototype.indexof",
    "description": "Array.prototype.indexOf when fromIndex is string"
  }
}
```

Failure location:

```json
{
  "code": "BackendIo",
  "message": "wat2wasm failed",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "backend-io",
  "error_type": "backend-io"
}
```

Source context:

```text
// Copyright (c) 2012 Ecma International.  All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-array.prototype.indexof
description: Array.prototype.indexOf when fromIndex is string
---*/
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
    "line": 9,
    "column": 1,
    "initializer": "[1, 2, 1, 2, 1, 2]"
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
            start: 245,
            end: 248,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 249,
            end: 250,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 251,
            end: 252,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 253,
            end: 254,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 254,
            end: 255,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 255,
            end: 256,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 258,
            end: 259,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 260,
            end: 261,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 261,
            end: 262,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 263,
            end: 264,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 264,
            end: 265,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 266,
            end: 267,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 267,
            end: 268,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 269,
            end: 270,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 270,
            end: 271,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 271,
            end: 272,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 274,
            end: 280,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 280,
            end: 281,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 281,
            end: 290,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 290,
            end: 291,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 291,
            end: 292,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 292,
            end: 293,
        },
    },
    SpannedToken {
        kind: Ident(
            "indexOf",
        ),
        span: Span {
            start: 293,
            end: 300,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 300,
            end: 301,
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
        name: "a",
        expr: Array {
            elements: [
                Number {
                    value: 1,
                    span: Span {
                        start: 254,
                        end: 255,
                    },
                },
                Number {
                    value: 2,
                    span: Span {
                        start: 257,
                        end: 258,
                    },
                },
                Number {
                    value: 1,
                    span: Span {
                        start: 260,
                        end: 261,
                    },
                },
                Number {
                    value: 2,
                    span: Span {
                        start: 263,
                        end: 264,
                    },
                },
                Number {
                    value: 1,
                    span: Span {
                        start: 266,
                        end: 267,
                    },
                },
                Number {
                    value: 2,
                    span: Span {
                        start: 269,
                        end: 270,
                    },
                },
            ],
            span: Span {
                start: 253,
                end: 271,
            },
        },
        span: Span {
            start: 245,
            end: 272,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 274,
                        end: 280,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 274,
                    end: 290,
                },
            },
            args: [
                Call {
                    callee: Member {
                        object: Ident {
                            name: "a",
                            span: Span {
                                start: 291,
                                end: 292,
                            },
                        },
                        property: "indexOf",
                        span: Span {
                            start: 291,
                            end: 300,
                        },
                    },
                    args: [
                        Number {
                            value: 2,
                            span: Span {
                                start: 301,
                                end: 302,
                            },
                        },
                        String {
                            value: "2",
                            span: Span {
                                start: 304,
                                end: 307,
                            },
                        },
                    ],
                    span: Span {
                        start: 291,
                        end: 308,
                    },
                },
                Number {
                    value: 3,
                    span: Span {
                        start: 310,
                        end: 311,
                    },
                },
                String {
                    value: "\"2\" resolves to 2",
                    span: Span
```

#### resolved

- ok: `True`
- truncated: `False`

```text
== resolved ==
[
    Let(
        "a",
        Array(
            [
                Number(
                    1,
                ),
                Number(
                    2,
                ),
                Number(
                    1,
                ),
                Number(
                    2,
                ),
                Number(
                    1,
                ),
                Number(
                    2,
                ),
            ],
        ),
    ),
    If {
        condition: Binary {
            left: MethodCall {
                object: Ident(
                    "a",
                ),
                method: "indexOf",
                args: [
                    Number(
                        2,
                    ),
                    String(
                        "2",
                    ),
                ],
                span: Span {
                    start: 291,
                    end: 308,
                },
            },
            op: StrictNotEqual,
            right: Number(
                3,
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
            left: MethodCall {
                object: Ident(
                    "a",
                ),
                method: "indexOf",
                args: [
                    Number(
                        2,
                    ),
                    String(
                        "one",
                    ),
                ],
                span: Span {
                    start: 352,
                    end: 371,
                },
            },
            op: StrictNotEqual,
            right: Number(
                1,
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

#### wat

- ok: `True`
- truncated: `True`

```text
== wat ==
(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 2 185)
  (global $heap (mut i32) (i32.const 2048))
  (global $alloc_bytes_since_last_gc (mut i32) (i32.const 0))
  (global $gc_free_list (mut i32) (i32.const 0))
  (global $gc_free_list_max_body_size (mut i32) (i32.const 0))
  (global $gc_free_list_second_max_body_size (mut i32) (i32.const 0))
  (global $gc_root_base (mut i32) (i32.const 0))
  (global $gc_root_count (mut i32) (i32.const 0))
  (global $gc_call_frame_base (mut i32) (i32.const 0))
  (global $gc_call_frame_top (mut i32) (i32.const 0))
  (global $gc_call_frame_limit (mut i32) (i32.const 0))
  (global $gc_call_frame_current (mut i32) (i32.const 0))
  (data (i32.const 256) "\01\00\00\00\0a")
  (data (i32.const 264) "\05\00\00\00false")
  (data (i32.const 280) "\04\00\00\00null")
  (data (i32.const 288) "\04\00\00\00true")
  (data (i32.const 296) "\09\00\00\00undefined")
  (data (i32.const 312) "\01\00\00\002")
  (data (i32.const 320) "\1f\00\00\00__TS2WASM_TEST262_ASSERT_FAIL__")
  (data (i32.const 360) "\03\00\00\00one")

  (func $write (param $ptr i32) (param $len i32)
    (i32.store (i32.const 8) (local.get $ptr))
    (i32.store (i32.const 12) (local.get $len))
    (drop (call $fd_write (i32.const 1) (i32.const 8) (i32.const 1) (i32.const 0))))

  (func $copy (param $src i32) (param $dst i32) (param $len i32)
    (local $i i32)
    (block $exit
      (loop $loop
        (br_if $exit (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store8
          (i32.add (local.get $dst) (local.get $i))
          (i32.load8_u (i32.add (local.get $src) (local.get $i))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop))))

  (func $value_to_string_into (param $v i32) (param $ptr i32) (result i32)
    (local $obj i32)
    (local $len i32)
    (local $n i32)
    (local $abs i32)
    (local $start i32)
    (local $i i32)
    (local $j i32)
    (local $tmp i32)
    (local $digit i32)
    (if (i32.eq (local.get $v) (i32.const 0))
      (then
        (call $copy (i32.const 300) (local.get $ptr) (i32.const 9))
        (return (i32.const 9))))
    (if (i32.eq (local.get $v) (i32.const 1))
      (then
        (call $copy (i32.const 284) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (local.get $v) (i32.const 2))
      (then
        (call $copy (i32.const 268) (local.get $ptr) (i32.const 5))
        (return (i32.const 5))))
    (if (i32.eq (local.get $v) (i32.const 3))
      (then
        (call $copy (i32.const 292) (local.get $ptr) (i32.const 4))
        (return (i32.const 4))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 6))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (local.set $len (i32.load (local.get $obj)))
        (call $copy (i32.add (local.get $obj) (i32.const 4)) (local.get $ptr) (local.get $len))
        (return (local.get $len))))
    (if (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const 7))
      (then
        (local.set $obj (i32.and (local.get $v) (i32.const -8)))
        (if (i32.eq
              (i32.and
                (i32.load
                  (i32.add
                    (i32.sub (local.get $obj) (i32.const 16))
                    (i32.const 0)))
                (i32.const 28))
              (i32.const 16))
          (then
            (local.set $len (i32.load (i32.add (local.get $obj) (i32
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
        "message": "File '/home/wogikaze/wgkz/ts2wasm/reference/test262/test/built-ins/Array/prototype/indexOf/15.4.4.14-5-1.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```

Stack trace:

```text
/tmp/ts2wasm-23313-0.wat:1104:7: error: type mismatch at end of function, expected [] but got [i32, i32]
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
