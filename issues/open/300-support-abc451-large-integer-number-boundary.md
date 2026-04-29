---
id: 300
title: "Support ABC451 large integer number boundary"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the smallest architecture-preserving number representation or lowering
path needed for the ABC451 D fixture after issue 299.

Problem: `fixtures/atcoder/abc451-d-concat-power2.ts` now reaches a large
ordinary number literal, but the current tagged small-int wire representation
rejects `1000000000` before wasm generation.

## Current failure

Reproduction:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number.wasm --host-deny
```

Current result:

```text
error: [NumberOutOfRange] number literal 1000000000 is out of small-int tagged range (-268435456..=268435455)
```

The failure occurs at the loop condition in the ABC451 fixture:

```ts
for (let i = 0; 2 ** i <= 1000000000; i++) {
```

## Desired final state

The ABC451 fixture advances beyond the `1000000000` literal without weakening
range validation or silently wrapping tagged small-int values.

For this slice, it is acceptable to implement either:

- a real runtime/ABI representation path for large integer-valued `number`
  values used by the fixture; or
- a proven, narrowly documented lowering path that preserves Node-compatible
  observable output for the fixture's integer-only arithmetic and comparisons.

## Scope

In scope:

- [ ] Represent or lower ordinary `number` values needed by ABC451 up to at
      least `1_000_000_000`.
- [ ] Preserve correct behavior for `2 ** i <= 1000000000`, `String(n)`,
      numeric sort comparator values, `Set<number>`, and the official sample
      output path through `819264512`.
- [ ] Keep existing small-int behavior unchanged for values already
      representable by `ValueTag`.
- [ ] Add focused regression coverage for the new large-integer number path.

Out of scope:

- Full IEEE-754 `number` semantics for fractional values, `NaN`, `Infinity`,
  and signed zero.
- BigInt runtime value support.
- Reclassifying BigInt/Number mixed comparison work owned by issue 281.
- Source-text recognition or replacement of the ABC451 program.

## Affected paths

Expected:

- `crates/runtime-abi/src/value.rs`
- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`
- `issues/open/300-support-abc451-large-integer-number-boundary.md`

Do not touch:

- problem-specific source rewrite hooks
- BigInt runtime representation, unless only adding explicit non-overlap notes

## Acceptance criteria

- [ ] `fixtures/atcoder/abc451-d-concat-power2.ts` builds past the current
      `NumberOutOfRange` diagnostic for `1000000000`.
- [ ] A focused regression fixture proves the supported large integer number
      path matches Node under `iwasm`.
- [ ] Existing small-int tests still pass.
- [ ] Unsupported number forms outside this slice still produce explicit
      diagnostics or traps instead of silent miscompilation.
- [ ] Issue 294 is updated with the new next blocker or closed if the official
      sample outputs are fully verified.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-large-number.wasm
printf '69\n' | iwasm /tmp/abc451-d-large-number.wasm
printf '1099898\n' | iwasm /tmp/abc451-d-large-number.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if the runtime ABI representation changes
- [ ] updated: `docs/05-compatibility-and-semantics.md` if supported number
      semantics change

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root) if the supported subset changes

Follow-up issues:

- [ ] none
- [ ] created/updated if the slice proves a broader number-model design is
      required before implementation.

## Notes

Do not remove the `NumberOutOfRange` validator without replacing it with an
equivalent guard for unsupported values. The close condition is observable
Node-compatible behavior for the large integer subset, not merely accepting the
literal.

## Progress evidence

2026-04-30 child `019dda13-74bf-7ec2-9146-e75ae64c098c`:

- Implemented a narrow integer-only heap-number path for ordinary `number`
  values outside the tagged small-int payload range.
- Added `fixtures/core-semantics/large-integer-number-boundary.ts` covering
  `2 ** i <= 1000000000`, `String`/unary-plus round trip through large integer
  strings, `Set<number>` duplicate handling, and numeric sort values including
  `819264512`.
- Verified the reduced regression manually with Node/iwasm matching output:

```text
536870912
536870912
819264512
```

- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` now builds past the
  previous `NumberOutOfRange` diagnostic:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number-child.wasm --host-deny
```

- The ABC451 sample execution is not yet done: all three issue validation
  inputs currently trap under `iwasm` with `Exception: out of bounds memory
  access`. The issue remains open until the official sample path is safe and
  Node-compatible.

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
