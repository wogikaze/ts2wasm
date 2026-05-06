---
id: 5229
title: "W0: implement user/runtime string origin tracking"
type: feature
area: backend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement `StringOrigin` tracking to distinguish user literal strings from runtime-required strings. `docs/12-coding-standard.md §11` requires origin tracking so that wasm data segments contain only necessary strings, but the current codebase has no `StringOrigin` type.

## Problem

`docs/12-coding-standard.md §1.5` identifies unconditional runtime string interning as a past bug:

```
console.log なしでも undefined/null/true/false/newline が data segment に入る
```

And `§11` requires:

```rust
enum StringOrigin {
    UserLiteral,
    Runtime(RuntimeFn),
}
```

Current state:
- The `StringOrigin` enum defined in docs does not exist in code
- Some runtime strings may still be unconditionally interned
- No way to audit which strings come from user source vs runtime dependencies

Problem: Without origin tracking, wasm binary size includes unnecessary runtime strings, and there is no audit trail for data segment content.

## Desired final state

- `StringOrigin` enum exists in `crates/runtime-abi/src/` or `crates/backend-wasm/src/`
- All interned strings carry origin metadata
- Runtime strings are only interned when their owning `RuntimeFn` is in the link plan
- A validation test verifies: no console.log → no runtime strings for Log/Write in data segment

## Scope

In scope:

- [ ] Define `StringOrigin` enum in the runtime-abi or backend crate
- [ ] Add origin field to string interning data structures
- [ ] Thread origin from `RuntimeFn::spec().runtime_strings` through `RuntimeLinkPlan`
- [ ] Add test: "no console.log → no Log/Write runtime strings in data segment"
- [ ] Add test: "console.log present → only Log/Write runtime strings interned"
- [ ] `docs/12-coding-standard.md` §11 update to reflect implementation
- [ ] `current-state.md` update

Out of scope:

- Changing the runtime string set or adding/removing specific strings
- WASM binary size optimization (that's the downstream effect)
- Any behavioral change

## Affected paths

Expected:

- `crates/backend-wasm/src/` or `crates/runtime-abi/src/` — `StringOrigin` type
- String interning logic in backend
- Linker or runtime-link-plan tests
- `docs/12-coding-standard.md`
- `current-state.md`

Do not touch:

- `crates/frontend/`, `crates/ir/`, `crates/compiler/`
- Any fixture, test, or coverage data

## Acceptance criteria

- [ ] `StringOrigin` enum defined with `UserLiteral` and `Runtime(RuntimeFn)` variants
- [ ] All interned strings tracked by origin
- [ ] Only `RuntimeFn`-required strings appear in data segment when that function is in the link plan
- [ ] Test: "no console.log → zero Log/Write runtime strings in data segment"
- [ ] Test: "console.log present → Log/Write runtime strings present in data segment"
- [ ] `cargo test` and `cargo nextest run` all pass
- [ ] `docs/12-coding-standard.md §19.4` (RuntimeFn catalog checklist) includes StringOrigin requirement

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
# Verify runtime strings only present when needed
grep "RuntimeString::Runtime" crates/ --type rust
grep "RuntimeString::UserLiteral" crates/ --type rust
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/12-coding-standard.md §11` and `§19.4`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none
