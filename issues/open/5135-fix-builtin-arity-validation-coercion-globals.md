---
id: 5135
title: "Fix builtin arity validation for coercion/math globals"
type: bug
area: ir
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Relax the builtin arity check in `validate.rs` from exact-match (`!=`) to minimum-required for global coercion and math builtins. In JavaScript, calling `Boolean()` (0 args) returns `false`, but the compiler currently rejects it with `[ArityMismatch] builtin BooleanCoerce expects 1 argument(s), got 0`.

## Problem

Problem: Builtin arity validation rejects JavaScript global coercion/math calls with optional or missing arguments before runtime coercion can handle them.

The `validate_lowered` function in `crates/ir/src/lowered/validate.rs` (line 364-377) checks builtin call arity with `args.len() != expected`, which is too strict for JavaScript semantics. Builtins should accept fewer arguments (missing args become `undefined`).

Affected builtins (all at `builtin.rs` line 64-69, each with `expected_arity`):
- `BooleanCoerce` (expects 1): `Boolean()` with 0 args returns `false`
- `NumberCoerce` (expects 1): `Number()` with 0 args returns `0`
- `IsNaN` (expects 1): `isNaN()` with 0 args coerces `undefined` to `NaN`
- `IsFinite` (expects 1): `isFinite()` with 0 args returns `false`
- `ParseInt` (expects 2): `parseInt()` with 0-1 args works in JS
- `ParseFloat` (expects 1): `parseFloat()` with 0 args returns `NaN`
- `EncodeURI` (expects 1): `encodeURI()` with 0 args returns `"undefined"`
- `DecodeURI` (expects 1): `decodeURI()` with 0 args decodes `"undefined"`
- `Escape` (expects 1): `escape()` with 0 args returns `"undefined"`
- `Unescape` (expects 1): `unescape()` with 0 args returns `"undefined"`

Existing issue 341c (done) implemented `Boolean(x)` for 1-arg calls but did not cover `Boolean()` with 0 args.

## Current failure

```sh
# Compile Boolean() with 0 args
# Current: error: [ArityMismatch] builtin BooleanCoerce expects 1 argument(s), got 0
# Expected: returns false
```

Representative failing test262 paths:
- `reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js`
- `reference/test262/test/built-ins/Number/S15.7.1.1_A2.js`
- `reference/test262/test/built-ins/isFinite/tonumber-operations.js`
- `reference/test262/test/built-ins/isNaN/tonumber-operations.js`
- `reference/test262/test/built-ins/parseInt/S15.1.2.2_A3.1_T1.js`
- `reference/test262/test/built-ins/parseFloat/S15.1.2.3_A3.1_T1.js`

## Desired final state

Builtin call arity validation uses minimum-required (not exact-match), so `Boolean()`, `Number()`, `isNaN()`, `isFinite()`, `parseInt()`, `parseFloat()` etc. with fewer args than `expected_arity()` compile without `ArityMismatch`. Missing args are handled by the runtime (coerced to `undefined`).

## Scope

In scope:

- [ ] Add `min_arity()` method to `BuiltinId` in `crates/ir/src/builtin.rs`
- [ ] Change `validate.rs` line 366 from `args.len() != expected` to `args.len() < min_arity` for builtins that accept optional arguments
- [ ] Keep `expected_arity()` as-is (used in other contexts like JSON.stringify)
- [ ] Add fixture test for `Boolean()` (0 args) returning `false`
- [ ] Add fixture test for `Number()` (0 args) returning `0`
- [ ] Add fixture test for `isNaN()` (0 args) returning `true` (undefined is NaN)

Out of scope:

- RegExp.prototype.exec/test arity (separate issue 5136)
- String.prototype.match/search arity (separate issue 5136)
- JSON.stringify arity (different check pattern, accepts 1-3)
- User function (non-builtin) arity mismatch (already uses `>=` minimum check)

## Affected paths

Expected:

- `crates/ir/src/builtin.rs`
- `crates/ir/src/lowered/validate.rs`
- `fixtures/`

Do not touch:

- `crates/ir/src/lowered/program_builtins.rs` (RegExp/String arity)
- `crates/ir/src/lowered/resolver_expr.rs` (class-based routing arity)

## Acceptance criteria

- [ ] `Boolean()` (0 args) compiles and returns `false`
- [ ] `Number()` (0 args) compiles and returns `0`
- [ ] `isNaN()` (0 args) compiles and returns `true`
- [ ] `isFinite()` (0 args) compiles and returns `false`
- [ ] `parseInt()` (0 args) compiles and returns `NaN`
- [ ] `parseFloat()` (0 args) compiles and returns `NaN`
- [ ] Existing tests still pass (regression: 1-arg builtin calls unchanged)
- [ ] Focused fixture for `Boolean()` node_diff test passes (Node output matches)

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-triage -- test262 reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js --detail
# Also run for Number, isNaN, isFinite, parseInt, parseFloat paths
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [x] none

## Notes

Blocked until split or narrowed below the issue-readiness L-size threshold. The
current scope covers multiple builtin families and should be broken into smaller
implementation-ready slices before assignment.

The fix pattern:
1. Add `min_arity()` to `BuiltinId` — most global builtins have `min_arity=0` (JS allows calling without args), console log has `min_arity=1` (must have at least 1), etc.
2. Change `validate.rs` builtin branch from `args.len() != expected` to:
   ```rust
   let min_required = builtin.min_arity();
   let expected = builtin.expected_arity();
   if args.len() < min_required || args.len() > expected {
       // error
   }
   ```
   Note: extra args beyond expected should also produce an error or be truncated.
3. The `ParseInt` special case in `resolver_expr.rs` (line 588) injects radix `0` when 1 arg is provided — this still works since it happens before validation.

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
