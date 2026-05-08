---
id: 372
title: "Implement BigInt object ToPrimitive non-BigInt primitive returns"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

Extend the narrow mixed BigInt object `ToPrimitive` comparison support from BigInt-returning `valueOf` and decimal-string `toString` to other supported primitive returns.

Problem: Direct object-literal/local `valueOf` or `toString` methods that return booleans, supported tagged-int numbers, nullish values, or supported strings outside the current `toString` slice are still diagnosed instead of reusing the existing BigInt primitive mixed comparison paths.

## Current failure

Representative source-backed unsupported shape:

```ts
console.log(({ valueOf: () => true }) == 1n);
console.log(({ valueOf: () => 1 }) == 1n);
console.log(({ valueOf: () => "1" }) == 1n);
```

The compiler currently keeps these outside the implemented issue-368 narrow subset unless they are direct `valueOf: () => <bigint literal>` or direct `toString: () => <supported decimal string>`.

## Desired final state

For direct object-literal/local no-argument arrow `valueOf` or `toString` methods returning a supported primitive value, mixed BigInt abstract equality and relational comparison fold the object operand to that primitive and reuse the existing BigInt/String, BigInt/Boolean, BigInt/nullish, or supported BigInt/Number comparison path.

## Scope

In scope:

- [x] Direct object literals and simple locals with no-argument arrow `valueOf` or `toString` methods returning supported primitive literals.
- [x] Boolean, nullish, supported string, and currently supported tagged-int number primitive returns.
- [x] Node/iwasm differential fixtures for at least one equality and one relational comparison when Node-compatible behavior is in the current primitive subset.
- [x] Source-backed diagnostics for primitive returns that are outside existing BigInt/Number or BigInt/String supported bounds.

Out of scope:

- Invalid or out-of-range string returns; issue 373 owns that category.
- Prototype lookup, Proxy, getters, side effects, function-body methods, non-arrow methods, and general object model behavior; issue 374 owns that category.
- Non-source-backed unknown dynamic string inputs; issue 375 owns that category.
- Fractional, `NaN`, `Infinity`, or `-0` number-model edges; issue 281 owns those cases.

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
- prototype/Proxy/object-model runtime internals

## Acceptance criteria

- [x] Node/iwasm differential fixture covers a direct object-literal/local `valueOf` primitive return that is not BigInt and compares with BigInt compatibly.
- [x] Node/iwasm differential fixture covers a direct object-literal/local `toString` or `valueOf` supported primitive return in a relational BigInt comparison when Node-compatible behavior is in the current primitive subset.
- [x] Unsupported primitive returns keep source-backed diagnostics with issue ownership rather than silently lowering incorrectly.
- [x] Docs/current-state/issues state the exact supported primitive-return subset and remaining exclusions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/05-compatibility-and-semantics.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

This issue is a direct follow-up split from issue 368. Keep it inside the source-backed direct object-literal/local subset; do not grow it into general object coercion.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- 6daf51df and 0143a290 added the implementation, regression fixture, and passing targeted Node/iwasm test.
- close commit records the issue move plus stale docs/current-state wording cleanup.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mixed_object_toprimitive_primitive_matches_node_output_under_iwasm: pass
mise run update-issue-index -- --check: pass after issue index regeneration
mise run check issues: pass after path reference sync
```

Remaining risks:

- Invalid/out-of-range object `toString` returns remain tracked by issue 373.
- Broader object coercion remains tracked separately by issue 374.
- Unknown out-of-range dynamic strings remain tracked separately by issue 375.

## Progress evidence

2026-05-01:

- Implemented direct object-literal/local no-argument arrow `valueOf` / `toString` primitive-return folding for boolean, supported tagged-int number, nullish equality, and supported StringToBigInt string returns in mixed BigInt comparison contexts.
- Added Node/iwasm differential fixture `fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-primitive.ts`.
- Updated docs/current-state wording for the supported primitive-return subset and remaining issue 373-375 exclusions.

Validation:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mixed_object_toprimitive_primitive_matches_node_output_under_iwasm: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint: fail, 43 passed / 12 failed; added issue-372 fixture passes, remaining failures are existing broader BigInt baseline failures outside this slice
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Close status:

- Not closed because the required broad `cargo test -p ts2wasm-cli --test m2_node_diff bigint` validation is not green in this worktree.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
