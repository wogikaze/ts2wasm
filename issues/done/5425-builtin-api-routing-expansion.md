---
id: 5425
title: "W4: Expand builtin API routing for test262-encountered builtins"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Add IR-level routing and RuntimeFn entries for the most commonly used test262 builtin APIs. After the 5419 metadata fix, builtin-api unsupported is 6,870 at full corpus — missing runtime function dispatch for common builtins.

## Problem

6,870 test262 files hit builtin-api unsupported at full corpus. These files compile past the parser and name resolution but fail at the IR lowering stage because the builtin method is not routed.

Problem: 6,870 builtin-api unsupported at full corpus.

## Likely missing routing

Based on feature breakdown (object-builtin: 813, array-builtin: 1,035, string-builtin: 393, date: 290, function: 416):
- Object methods: Object.preventExtensions, Object.isSealed, Object.isFrozen, Object.isExtensible, Object.getPrototypeOf, Object.setPrototypeOf
- Array methods: Array.prototype.reduce, Array.prototype.reduceRight (not yet listed in build_smoke)
- Date methods: Date.prototype.getUTCFullYear, getUTCMonth, getUTCDate, getTimezoneOffset (methods exist but routing may be incomplete)
- Function methods: Function.prototype.bind, Function.prototype.call, Function.prototype.apply
- String methods: More complete match/split/replace routing

## Desired final state

- Top 10 missing builtin API routings added
- builtin-api unsupported count reduced at full corpus

## Scope

In scope:

- [x] Add RuntimeFn variants for missing builtins in runtime_fn.rs
- [x] Add RuntimeSpec entries in runtime_fn_impl.rs
- [x] Add IR routing entries in program_builtins.rs
- [x] Build_smoke fixtures for each new routing
- [x] Verify with full corpus run

Out of scope:

- WAT runtime implementation for these builtins (separate issues if missing)
- Semantic tests (build_smoke only for routing verification)
- Intl, WeakMap, WeakSet, Atomics (deferred)

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs` — add routing entries
- `crates/backend-wasm/src/runtime_fn.rs` — add enum variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — add RuntimeSpec entries

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/src/name_resolver.rs` — name resolver out of scope

## Validation

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262
```

## False-done audit

**truly-done** (5425)

- Implementation commits: verified via `git log --oneline --all --grep=5425`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
