---
id: 296
title: "Support small-int exponentiation operator"
type: feature
area: runtime/semantics
class: done
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

- [x] Add lowering/backend support for `BinaryOp::Power` over the current
  integer-backed number subset.
- [x] Cover dynamic right-hand exponents such as `2 ** i` where `i` is a loop
  counter.
- [x] Keep unsupported BigInt exponentiation and out-of-range numeric behavior
  issue-linked or trapped rather than silently miscompiled.
- [x] Record the next ABC451 blocker after `**`, if the fixture advances.

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
- `issues/done/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- BigInt parser syntax
- broad number-model redesign
- problem-specific source rewrite hooks

## Acceptance criteria

- [x] Focused fixture for small-int `**` matches Node output under `iwasm`.
- [x] Focused fixture covers dynamic exponent `2 ** i`.
- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past the
  `binary operator Power not yet supported` blocker.
- [x] BigInt exponentiation remains issue-260-linked or is explicitly split.
- [x] No code path detects the ABC451 source text or substitutes another
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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/done/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a small runtime helper or existing integer arithmetic path. Do not widen
the runtime number model in this issue.

Progress on 2026-04-29:

- Added `BinaryOp::Power` lowering to `LoweredBinaryOp::Power` and linked it to
  the existing tagged-number `$math_pow` runtime helper.
- Added focused Node/iwasm differential coverage for constants and dynamic
  loop-counter exponents in
  `fixtures/core-semantics/small-int-exponentiation.ts`.
- Added a BigInt exponentiation unsupported fixture proving the current path
  remains issue-260-linked instead of using the number helper.
- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` advances past
  `binary operator Power not yet supported` and now reaches:
  `error: [UnsupportedSyntax] issue-211: unknown receiver class for method
  map at 970..996`.

## Completion evidence

Commits:

- child branch final commit: small-int exponentiation operator slice.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli small_int_exponentiation_fixture_matches_node_output_under_iwasm bigint_exponentiation_reports_issue_260
result: pass, 2 passed
date: 2026-04-29

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-power-child.wasm --host-deny
result: advanced past Power; next blocker is issue-211 unknown receiver class for method `map` at 970..996
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass after restoring ignored local artifact `artifacts/coverage/results/test262-results.jsonl` in this worktree
date: 2026-04-29

command: mise run check
result: pass
date: 2026-04-29

command: cargo nextest run
result: pass, 569 passed, 4 skipped
date: 2026-04-29
```

Remaining risks:

- Out-of-scope fractional, negative-exponent, `NaN`, `Infinity`, BigInt, and
  full `Math.pow` compatibility semantics remain tracked outside this issue.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/296-support-small-int-exponentiation-operator.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
