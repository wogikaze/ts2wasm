---
id: 5130
title: "Implement own method ToPrimitive for mixed BigInt comparisons"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: []
blocks: []
status: done
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Extend the current mixed BigInt object `ToPrimitive` slice from direct
no-argument arrow properties to own method syntax with a statically visible
single primitive `return`.

Problem: `{ valueOf() { return 1n; } } == 1n` is Node-compatible and
source-backed, but the current implementation diagnoses it under issue 374
because only arrow-valued own properties are folded.

## Current failure

Representative Node behavior:

```sh
node -e 'let obj={valueOf(){return 1n}}; console.log(obj == 1n)'
```

Expected Node stdout:

```text
true
```

Current project boundary:

- `fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-unsupported.ts`
- diagnostic substring: `issue-374: object ToPrimitive for mixed BigInt comparison is limited to direct no-argument arrow valueOf/toString methods returning supported primitive literals`

## Desired final state

Direct object literals and direct local object values with own no-argument
`valueOf` or `toString` method syntax are folded when the method body is exactly
a single `return` of a supported primitive literal. The ordinary `valueOf`
before `toString` ordering is preserved for equality and relational BigInt
comparison coercion.

## Scope

In scope:

- [x] Own object-literal method syntax: `{ valueOf() { return 1n; } } == 1n`.
- [x] Own object-literal/local `toString() { return "1"; }` for supported
      StringToBigInt strings.
- [x] `valueOf` before `toString` ordering when `valueOf` returns a
      non-primitive object and `toString` returns a supported primitive.
- [x] Source-spanned issue diagnostics for method bodies outside the single
      primitive-return subset.

Out of scope:

- Prototype lookup or inherited `valueOf` / `toString`.
- Getters, setters, Proxy traps, `Symbol.toPrimitive`, and descriptors.
- Receiver-sensitive method bodies that read `this`.
- Side effects, mutation ordering, or exception-producing coercion.
- Unknown non-source-backed out-of-range dynamic strings; issue 375 owns that
  category.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver_bigint.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/core-semantics/*bigint*toprimitive*`

Do not touch:

- broad object prototype/runtime ABI files
- parser BigInt syntax
- BigInt arithmetic helpers unrelated to comparison coercion

## Acceptance criteria

- [x] A bigint runtime mixed object ToPrimitive method fixture is added under
      `fixtures/core-semantics/` and matches Node/iwasm output for own `valueOf() { return 1n; }`
      and `toString() { return "1"; }`.
- [x] A fixture covers `valueOf` returning a non-primitive object followed by
      `toString` returning a supported primitive, preserving Node ordering.
- [x] Unsupported method bodies that read `this`, mutate state, throw, or return
      unsupported dynamic values still produce issue-linked diagnostics.
- [x] Existing arrow-property fixtures remain passing.
- [x] Docs/current-state/issues are synchronized if the boundary changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mixed_object_toprimitive
cargo nextest run -E 'test(bigint_runtime_mixed_object_toprimitive)'
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

- [x] none

## Notes

Reuse the existing issue-374 guard as the boundary. This slice may inspect
source-backed method bodies, but it must not introduce prototype lookup,
receiver-sensitive `this`, or side-effect ordering semantics.

Issue 374 is the completed design parent for this implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending closure commit

Validation result:

```text
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mixed_object_toprimitive -- --nocapture
=> pass (3 tests passed)

cargo nextest run -E 'test(bigint_runtime_mixed_object_toprimitive)'
=> pass (3 tests passed)

cargo test -p ts2wasm-cli --test m2_node_diff bigint -- --nocapture
=> pass (62 tests passed)

cargo fmt --all --check
=> pass

date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

