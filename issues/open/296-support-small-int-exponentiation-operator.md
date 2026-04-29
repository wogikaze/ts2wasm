---
id: 296
title: "Support small-int exponentiation operator"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the numeric `**` operator for the current non-negative small-integer
runtime subset needed by ABC451 D.

This is a work order, not a design document and not a progress log.

## Problem

Problem: `fixtures/atcoder/abc451-d-concat-power2.ts` now advances past issue
295 map callbacks and stops at the powers-of-two loop because `BinaryOp::Power`
is still rejected during lowering.

## Current failure

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-parent-295.wasm --host-deny
```

Current result:

```text
error: [UnsupportedSyntax] binary operator Power not yet supported
```

The failing source shape is:

```ts
for (let i = 0; 2 ** i <= 1000000000; i++) {
    powersOfTwo.push(2 ** i);
}
```

## Desired final state

Small integer exponentiation such as `2 ** i`, `3 ** 4`, and `5 ** 0` lowers
through the normal pipeline and matches Node output under `iwasm` for
non-negative integer exponents within the current tagged small-int range.

## Scope

In scope:

- [ ] Add lowering/backend support for `BinaryOp::Power` over the current
  integer-backed number subset.
- [ ] Cover dynamic right-hand exponents such as `2 ** i` where `i` is a loop
  counter.
- [ ] Keep unsupported BigInt exponentiation and out-of-range numeric behavior
  issue-linked or trapped rather than silently miscompiled.
- [ ] Record the next ABC451 blocker after `**`, if the fixture advances.

Out of scope:

- Fractional, `NaN`, `Infinity`, and negative-exponent number semantics.
- BigInt exponentiation; issue 260 owns BigInt arithmetic policy.
- Full ECMAScript `Math.pow` compatibility.

## Affected paths

Expected:

- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/types.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- BigInt parser syntax
- broad number-model redesign
- problem-specific source rewrite hooks

## Acceptance criteria

- [ ] Focused fixture for small-int `**` matches Node output under `iwasm`.
- [ ] Focused fixture covers dynamic exponent `2 ** i`.
- [ ] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past the
  `binary operator Power not yet supported` blocker.
- [ ] BigInt exponentiation remains issue-260-linked or is explicitly split.
- [ ] No code path detects the ABC451 source text or substitutes another
  program.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli <new focused test name>
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-power.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a small runtime helper or existing integer arithmetic path. Do not widen
the runtime number model in this issue.

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
