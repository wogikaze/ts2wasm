---
id: 398
title: "Track control-flow-assigned BigInt div/rem locals"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Preserve enough BigInt type information through branch/control-flow assignments for dynamic BigInt `/` and `%` to use the issue-384 cached-decimal runtime helpers.

Problem: Issue 384 implements known-BigInt cached-decimal division/remainder, but locals assigned inside branches, loops, switch cases, or try/catch/finally blocks are still conservatively invalidated before later arithmetic.

## Problem

The BigInt runtime guard and lowering resolver intentionally invalidate locals assigned inside nested control flow so stale signed-i64 helper proofs cannot leak after a branch. That safety rule is still correct for signed-i64-only operators, but it also prevents issue-384 `/` and `%` from recognizing later uses as BigInt when every control-flow assignment remains a BigInt value.

## Desired final state

Dynamic BigInt `/` and `%` over branch/control-flow-assigned locals lower to the cached-decimal runtime helpers when all reachable assignments preserve BigInt values, while mixed Number/BigInt and unknown assignments keep issue-370 diagnostics.

## Scope

In scope:

- [ ] Add a conservative join rule for branch/control-flow-assigned locals that preserves BigInt type, not stale signed-i64 magnitude proofs.
- [ ] Cover at least one `if/else` branch-assigned local with Node/iwasm differential `/` and `%` fixtures outside signed i64.
- [ ] Keep loop/switch/try invalidation conservative unless the join can be proven safely.
- [ ] Preserve issue-370 diagnostics for mixed Number/BigInt arithmetic.

Out of scope:

- Addition, subtraction, multiplication, exponentiation, bitwise, or shift control-flow tracking.
- Division/remainder by zero exception parity; issue 380/370.
- General TypeScript type narrowing.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver_bigint.rs`
- `crates/ir/src/lowered/resolver.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/core-semantics/`
- `current-state.md`

## Acceptance criteria

- [ ] Node/iwasm differential fixture covers BigInt `/` and `%` where both `if` branches assign outside-signed-i64 BigInt locals before the operation.
- [ ] A mixed branch assignment such as one BigInt branch and one Number branch still reports issue-370.
- [ ] Existing issue-384 known-local/literal div/rem fixtures continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
mise run update-issue-index -- --check
mise run check issues
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```
