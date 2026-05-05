---
id: 5136
title: "Fix arity validation for RegExp/String prototype methods"
type: bug
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Relax the arity checks for RegExp.prototype.exec/test and String.prototype.match/search in `program_builtins.rs` to accept fewer arguments. These methods currently require exactly 1 argument, but JavaScript allows calling them with 0 args (missing arg becomes `undefined`).

## Problem

Three functions in `crates/ir/src/lowered/program_builtins.rs` check `args.len() != 1` for RegExp and String prototype methods, producing `ArityMismatch` when called with 0 arguments:

- `regexp_test_runtime` (line 649): `args.len() != 1` for `RegExp.prototype.test`
- `regexp_exec_runtime` (line 730): `args.len() != 1` for `RegExp.prototype.exec`
- `regexp_string_match_runtime` (line 685): `args.len() != 1` for `String.prototype.match` and `String.prototype.search`

Additionally, in `crates/ir/src/lowered/resolver_expr.rs` (line 1679), the class-based routing path for `RegExp` also checks `args.len() != 1`.

Representative failing test262 paths:
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A12.js`
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A1_T16.js`
- `reference/test262/test/built-ins/RegExp/prototype/test/S15.10.6.3_A1_T16.js`

## Current failure

```sh
# Current: error: [ArityMismatch] RegExp.prototype.exec expects 1 argument, got 0
# Expected: exec/undefined returns null
```

## Desired final state

`RegExp.prototype.exec(string?)`, `RegExp.prototype.test(string?)`, `String.prototype.match(regexp?)`, and `String.prototype.search(regexp?)` accept 0 arguments. The ArityMismatch check is removed or relaxed to a minimum-only check.

## Scope

In scope:

- [ ] Remove or relax `args.len() != 1` check in `regexp_test_runtime` (program_builtins.rs)
- [ ] Remove or relax `args.len() != 1` check in `regexp_exec_runtime` (program_builtins.rs)
- [ ] Remove or relax `args.len() != 1` check in `regexp_string_match_runtime` (program_builtins.rs)
- [ ] Relax `args.len() != 1` check for RegExp class-based routing in `resolver_expr.rs` (line 1679)
- [ ] Add fixture test for `RegExp.prototype.test()` (0 args) returning `false`

Out of scope:

- Global builtins Boolean/Number/isNaN/isFinite arity (separate issue 5135)
- Full RegExp/undefined semantics (the 0-arg case should not crash, but exact output matching with Node is a separate concern)
- JSON.stringify arity (different pattern, accepts 1-3 args)

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `fixtures/`

Do not touch:

- `crates/ir/src/builtin.rs` (handled by issue 5135)
- `crates/ir/src/lowered/validate.rs` (handled by issue 5135)

## Acceptance criteria

- [ ] `RegExp.prototype.exec()` (0 args) compiles without ArityMismatch
- [ ] `RegExp.prototype.test()` (0 args) compiles without ArityMismatch
- [ ] `String.prototype.match()` (0 args) compiles without ArityMismatch
- [ ] `String.prototype.search()` (0 args) compiles without ArityMismatch
- [ ] Existing tests still pass (regression: 1-arg calls unchanged)

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-triage -- test262 reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A12.js
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A12.js --detail
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

The fix pattern for each check:

1. `program_builtins.rs`: Change `if args.len() != 1` to `if args.len() > 1` (allow 0-1 args, reject 2+). Or simply remove the check and let the runtime handle undefined.

2. `resolver_expr.rs` line 1679: Change `if class_name == "RegExp" && args.len() != 1` to `if class_name == "RegExp" && args.len() > 1`.

The runtime WAT functions for exec/test may crash when receiving `undefined` as the search string, but the immediate goal is to not produce a compile-time `ArityMismatch` diagnostic. Runtime robustness for undefined input is a downstream concern.

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
