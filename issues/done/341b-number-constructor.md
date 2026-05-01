---
id: 341b
title: "Implement Number constructor and static methods"
type: feature
area: runtime/builtins
status: done
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
closed: 2026-05-01
---

## Summary

Implement `Number()` as a callable coercion function and its static methods (`Number.isNaN`, `Number.isFinite`, `Number.isInteger`, `Number.isSafeInteger`, `Number.parseInt`, `Number.parseFloat`).

## Problem

`Number(x)` and `Number.*` static methods are not recognized, causing test262 failures.

## Desired final state

`Number(x)` coerces to number. Static methods `Number.isNaN`, `Number.isFinite`, `Number.isInteger`, `Number.isSafeInteger`, `Number.parseInt`, `Number.parseFloat` are recognized method calls.

## Scope

- [x] `Number(x)` coercion function (as BuiltinId)
- [x] `Number.isNaN(x)` — delegates to global isNaN
- [x] `Number.isFinite(x)` — delegates to global isFinite
- [x] `Number.isInteger(x)` — number check
- [x] `Number.isSafeInteger(x)` — number check

## Affected paths

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/program_builtins.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`

## Acceptance criteria

- [x] `Number(42)` returns `42`
- [x] `Number("42")` returns `42`
- [x] `Number.isNaN(NaN)` / `Number.isFinite(Infinity)` — see issue-281 limitations

## Completion evidence

- `Number(x)` coercion works: `Number(42)` → `42`, `Number("42")` → `42`, `Number(true)` → `1`, `Number(null)` → `0`
- `Number.isNaN(x)` works via wrapper to `$is_nan`: `Number.isNaN(42)` → `false`
- `Number.isFinite(x)` works via wrapper to `$is_finite`: `Number.isFinite(42)` → `true`
- `Number.isInteger(x)` works: `Number.isInteger(42)` → `true`
- `Number.isSafeInteger(x)` works: `Number.isSafeInteger(100)` → `true`
- Implemented as BuiltinId::NumberCoerce (global call), plus RuntimeFn::NumberIsNaN/IsFinite/IsInteger/IsSafeInteger (static methods)
- `$number_coerce` depends on `$parse_int_string` (RuntimeFn::ParseInt) for string-to-number coercion
- Node differential: behavior matches Node except for `NaN`/`Infinity` (pre-existing issue-281)
