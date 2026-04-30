---
id: 375
title: "Handle non-source-backed out-of-range BigInt/String comparisons"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261, 282]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Prevent unknown runtime BigInt/String comparison inputs outside the current signed-i32 StringToBigInt helper boundary from returning silently incorrect booleans.

Problem: Source-backed local and object-property out-of-range strings are diagnosed, but unknown non-source-backed dynamic strings can still reach runtime comparison paths where the current small-int StringToBigInt helper boundary is insufficient for full BigInt comparison semantics.

## Current failure

Representative runtime-only shape:

```ts
declare function getString(): string;
let s = getString();
console.log(1n == s);
console.log(1n < s);
```

A concrete reproducer should use an existing supported dynamic source that is not literal-derived, then feed an out-of-range decimal string to BigInt/String abstract equality or relational comparison.

## Desired final state

Unknown non-source-backed BigInt/String comparison input either compares compatibly for out-of-range StringToBigInt values or emits an explicit issue-owned runtime abort/trap marker before trapping. It must not return an incorrect normal boolean.

## Scope

In scope:

- [ ] Runtime-only dynamic strings where the compiler cannot source-prove the StringToBigInt value.
- [ ] Abstract equality and relational BigInt/String comparisons.
- [ ] Explicit runtime marker/trap or compatible comparison behavior for out-of-range values.
- [ ] Node/iwasm differential coverage for supported runtime values and trap/diagnostic coverage for out-of-range unknown values.

Out of scope:

- Source-backed direct object `toString` string returns; issue 373 owns that category.
- General `BigInt(...)` exception parity; issue 333 owns builtin invalid-string exceptions.
- Full multi-limb BigInt arithmetic; issue 369 owns arithmetic representation expansion.
- Broad object `ToPrimitive`; issue 374 owns object-model-dependent coercion.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/` if runtime marker/helper behavior is required
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- parser BigInt syntax
- unrelated BigInt arithmetic helpers unless required by the comparison boundary
- broad runtime ABI representation unless a compile error proves it is required

## Acceptance criteria

- [ ] A non-source-backed unknown dynamic string comparison reproducer is added or documented.
- [ ] Out-of-range unknown dynamic BigInt/String comparison cannot silently return an incorrect normal boolean.
- [ ] Supported in-range unknown dynamic BigInt/String comparisons continue to match Node under iwasm.
- [ ] Docs/current-state/issues state the runtime-only unknown out-of-range boundary.

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

- [ ] updated: `docs/05-compatibility-and-semantics.md`
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

This issue is a direct follow-up split from issue 368. It is intentionally separate from source-backed object `toString` returns so runtime-only safety can be validated directly.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Compatible comparison for all out-of-range values may require broader BigInt representation work; a marker/trap is acceptable if documented and tested.
