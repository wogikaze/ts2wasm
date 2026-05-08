---
id: 5219
title: "Support explicit this-parameter function expression lowering"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support the runtime/lowering shape exposed after parsing TypeScript
`this` parameters in function expressions: a function expression with an
explicit erased `this: T` parameter that reads `this` in its body.

## Problem

After issue 5218 parses `function (this: any) { ... }`, the representative
`binaryArithmeticControlFlowGraphNotTooLarge.ts` advances into lowering and
fails at the generic nested-function closure runtime guard.

Problem: function expressions that declare an erased TypeScript `this` parameter and read `this` in the body currently fail with `issue-062e: nested function closures with this or arguments are not supported`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Current diagnostic after issue 5218:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice
```

Representative source:

```ts
const foo = function (this: any) {
    var a, b, c, d, ab, bc, cd, da, blocks = this.blocks;
};
```

## Desired final state

The compiler handles the explicit-`this` function-expression shape without the
generic closure guard, either by lowering a supported receiver/closure contract
or by emitting a more specific source-backed diagnostic before runtime lowering.

## Scope

In scope:

- [x] Decide the narrow contract for explicit `this` parameters in function expressions
- [x] Implement the chosen lowering or source-backed diagnostic for `this` reads in this shape
- [x] Add a focused fixture/test covering `function (this: any) { return this.value; }`
- [x] Re-run the representative triage and confirm it advances past the current `issue-062e` guard

Out of scope:

- General heap closure environments for arbitrary `this`/`arguments`
- Full JavaScript dynamic `this` binding semantics
- Bitwise/control-flow lowering later in `binaryArithmeticControlFlowGraphNotTooLarge.ts`

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated numeric literal parsing

## Acceptance criteria

- [x] A focused explicit-`this` function-expression test no longer reports the generic `issue-062e` closure guard
- [x] The representative reference triage no longer stops at `issue-062e: nested function ... closures with this or arguments`
- [x] Existing closure and implicit-this diagnostics still pass
- [x] Issue index and readiness checks pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli this_parameter
cargo nextest run -p ts2wasm-cli -E 'test(closure)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
python scripts/manager.py check issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split while closing issue 5218. Existing broad buckets 597 and 445 mention
`issue-062e`, but this issue records the exact post-parser reference window and
the explicit TypeScript `this` parameter shape.

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


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
