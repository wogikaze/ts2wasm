---
id: 374
title: "Design broader object ToPrimitive for mixed BigInt comparisons"
type: design
area: runtime/semantics
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Define the policy and executable slices for object `ToPrimitive` shapes that require the broader object model before mixed BigInt comparison compatibility can be implemented safely.

Problem: Non-arrow methods, function bodies, prototype lookup, Proxy traps, getters, receiver-sensitive calls, and side-effectful coercion require ordering and object-model guarantees that are outside the direct object-literal/local issue-368 subset.

## Current failure

Representative unsupported shapes:

```ts
let obj = { valueOf() { return 1n; } };
console.log(obj == 1n);

let proto = { valueOf: () => 1n };
let child = Object.create(proto);
console.log(child == 1n);

let side = 0;
console.log(({ valueOf: () => { side = 1; return 1n; } }) == 1n);
```

The issue-368 closure intentionally does not implement these forms because they require broader object/prototype/call/side-effect semantics.

## Desired final state

The project has a documented, source-backed policy for broader object `ToPrimitive` in mixed BigInt comparisons and one or more implementation-ready child issues that split the work by executable object-model slice.

## Scope

In scope:

- [x] Decide which broader `ToPrimitive` shapes are supported, diagnosed, or deferred.
- [x] Define ordering requirements for `valueOf`/`toString`, receiver binding, side effects, and exception behavior.
- [x] Split implementation-ready child issues for safe narrow slices after the policy is accepted.
- [x] Keep current direct object-literal/local issue-368 behavior unchanged.

Out of scope:

- Implementing broad object/prototype/Proxy behavior directly in this design issue.
- BigInt arithmetic or BigInt builtin exception parity.
- Unknown non-source-backed string runtime input; issue 375 owns that category.

## Affected paths

Expected:

- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`
- `issues/done/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md`
- `issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md`
- `issues/done/374-design-broader-object-toprimitive-for-bigint-comparisons.md`
- `issues/done/375-handle-non-source-backed-out-of-range-bigint-string-comparisons.md`
- `crates/cli/tests/` only for diagnostic characterization if needed
- `fixtures/core-semantics/*bigint*` only for diagnostic characterization if needed

Do not touch:

- broad runtime ABI representation
- parser BigInt syntax
- unrelated object-model implementation files before a child issue exists

## Acceptance criteria

- [x] Broader object `ToPrimitive` shapes are classified into supported, diagnostic, and deferred categories with Node evidence for representative cases.
- [x] At least one implementation-ready child issue is created if a safe narrow slice exists.
- [x] Existing direct object-literal/local issue-368 fixtures remain the boundary for implemented behavior until child issues land.
- [x] Docs/current-state/issues no longer point to issue 368 for broad object coercion.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint
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

- [x] created: `issues/open/5130-implement-object-method-toprimitive-for-bigint-comparisons.md`

## Notes

Unblocked on 2026-05-06: dependencies `259` and `261` are in `issues/done/`, and this issue's output is the accepted object-model policy plus implementation-ready child issues. Do not use it to implement broad prototype or Proxy behavior directly.

## Completion evidence

Date: 2026-05-06

Commits:

- child issue: `5130`

Validation result:

```text
Design parent closed by classifying broad object ToPrimitive shapes:
direct source-backed own method syntax with single primitive returns is split to
issue 5130; prototype lookup, accessors/descriptors, Proxy traps,
receiver-sensitive `this`, side effects, throwing coercion, and
Symbol.toPrimitive remain deferred until broader object-model support exists.
```

Remaining risks:

- The final split may depend on object model work outside BigInt comparisons.
