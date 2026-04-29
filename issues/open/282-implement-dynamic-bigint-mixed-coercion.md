---
id: 282
title: "Implement dynamic BigInt mixed coercion"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement dynamic mixed BigInt abstract equality and relational comparison coercion beyond issue 261's static literal folds.

Problem: issue 261 deliberately handles statically visible literal BigInt/String, BigInt/Boolean, BigInt/tagged-int Number, and BigInt/nullish abstract equality in the resolver. Runtime-only mixed values currently trap rather than producing a silent incorrect boolean, and relational mixed primitive comparison remains unsupported.

## Current failure

```sh
tmp=/tmp/ts2wasm-282-dynamic-bigint-coercion.ts
printf 'let a = 1n; let box = { x: a }; console.log(box.x == "1"); console.log(box.x < "2");\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-282-dynamic-bigint-coercion.wasm
iwasm /tmp/ts2wasm-282-dynamic-bigint-coercion.wasm
```

Current result: runtime-only mixed BigInt comparisons trap, while statically visible unsupported mixed relational comparisons report issue-linked diagnostics.

## Desired final state

Dynamic BigInt/String, BigInt/Boolean, BigInt/nullish, and supported object `ToPrimitive` equality/comparison cases match Node within the current value model. Unsupported cases produce source diagnostics or intentional runtime traps with issue ownership.

## Scope

In scope:

- [ ] Implement dynamic StringToBigInt parsing for abstract equality where the current runtime string model can preserve Node-compatible behavior.
- [ ] Implement dynamic Boolean-to-Number-to-BigInt-equivalent abstract equality boundaries.
- [ ] Implement mixed BigInt/String and BigInt/Boolean relational comparison for supported primitive values.
- [ ] Track or implement object `ToPrimitive` interactions consistently with the current object model.

Out of scope:

- BigInt/Number edge cases; issue 281 owns number-model-sensitive comparisons.
- BigInt arithmetic; issue 260 owns arithmetic.
- BigInt builtin dynamic inputs; issue 280 owns builtin calls.
- Parser BigInt syntax.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- parser BigInt syntax
- broad runtime ABI representation unless a compile error proves it is required

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover runtime BigInt/String abstract equality for supported StringToBigInt inputs and invalid strings.
- [ ] Node/iwasm differential fixtures cover runtime BigInt/Boolean and BigInt/nullish abstract equality.
- [ ] Node/iwasm differential fixtures cover supported mixed BigInt/String and BigInt/Boolean relational comparisons.
- [ ] Object `ToPrimitive` behavior is either implemented for supported objects or explicitly split with source-backed diagnostics.
- [ ] Docs/current-state/issues state dynamic mixed BigInt coercion limits.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(bigint) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli bigint
```

## Notes

Split from issue 261 on 2026-04-29 because the existing runtime-only mixed fixtures intentionally trap instead of returning a wrong boolean:

- `fixtures/core-semantics/bigint-runtime-mixed-abstract-equality-trap.ts`
- `fixtures/core-semantics/bigint-runtime-mixed-relational-trap.ts`

Progress on 2026-04-29:

- Runtime-only BigInt/Boolean and BigInt/nullish abstract equality now has Node/iwasm differential coverage in `fixtures/core-semantics/bigint-runtime-mixed-boolean-nullish-abstract-equality.ts`.
- BigInt/String abstract equality and mixed BigInt relational comparison remain intentionally outside this slice; the existing runtime-only String and relational trap fixtures continue to own those blockers.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run for this follow-up; issue is open
```

Remaining risks:

- Runtime string parsing and object `ToPrimitive` may require broader runtime helper work.
