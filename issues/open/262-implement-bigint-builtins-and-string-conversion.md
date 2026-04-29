---
id: 262
title: "Implement BigInt builtins and string conversion"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: [259]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the BigInt builtin surface that depends on runtime BigInt values: `BigInt(...)`, string conversion, and selected static BigInt functions.

Problem: After BigInt values exist, builtin and conversion behavior still needs explicit runtime helpers and diagnostics so `BigInt` is not treated as an ordinary unresolved identifier or generic object.

## Current failure

```sh
tmp=/tmp/ts2wasm-262-bigint-builtins.ts
printf 'console.log(BigInt("10")); console.log(String(10n)); console.log(BigInt.asIntN(8, 255n));\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-262-bigint-builtins.wasm
```

Current result: unsupported BigInt runtime/builtin diagnostics.

## Desired final state

Supported BigInt builtins are resolved by `BuiltinResolver`, lowered through explicit runtime helpers, and covered by Node differential fixtures. Unsupported builtin forms produce issue-linked diagnostics rather than generic unsupported builtin messages.

## Scope

In scope:

- [ ] Implement `BigInt(value)` for supported string, boolean, integer number, and BigInt inputs.
- [ ] Implement `String(bigint)` / runtime `ToString` for BigInt without `n` suffix.
- [ ] Implement `BigInt.asIntN` and `BigInt.asUintN` for supported integer bit widths, or split them into narrower follow-up issues if needed.
- [ ] Add issue-linked diagnostics for unsupported BigInt builtin/coercion forms.

Out of scope:

- BigInt literal allocation; issue 259.
- BigInt arithmetic; issue 260.
- BigInt equality/comparison; issue 261.
- Full object `ToPrimitive` if the broader object model cannot support it yet.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- unrelated builtin families
- parser BigInt literal syntax

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover `BigInt(...)`, `String(bigint)`, template/string interpolation involving BigInt, and selected `BigInt.asIntN` / `BigInt.asUintN` cases or a split follow-up if those are not in this slice.
- [ ] Unsupported BigInt builtin and conversion forms produce source diagnostics with issue 262 or a narrower follow-up issue ID.
- [ ] Runtime linker structure tests cover selected BigInt builtin helpers and avoid new host imports.
- [ ] Docs/current-state/issues state the supported builtin subset and remaining limits.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli bigint
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] create narrower builtin follow-up if `asIntN` / `asUintN` cannot fit safely

## Notes

`BigInt` is not a constructor with `new`; `new BigInt(...)` must remain an error-compatible path.

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
