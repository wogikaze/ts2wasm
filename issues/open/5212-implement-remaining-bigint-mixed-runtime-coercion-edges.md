---
id: 5212
title: "Implement remaining BigInt mixed runtime coercion edges"
type: feature
area: runtime/semantics
class: blocked
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement the remaining dynamic BigInt mixed coercion edges that were split out of issue 282 after the primitive runtime and source-backed diagnostic slices were covered.

Problem: Compatible object `ToPrimitive` for mixed BigInt comparisons and non-source-backed unknown out-of-range BigInt/String runtime parsing still lack a precise lowering/runtime implementation.

## Current failure

Object `ToPrimitive` is source-diagnosed by issue 282 fixtures instead of lowered:

```sh
cargo test -p ts2wasm-cli bigint_runtime_mixed_object_toprimitive_reports_issue_282
```

Representative unsupported fixtures:

```text
fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-unsupported.ts
fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-string-unsupported.ts
```

Unknown dynamic strings that are not literal-derived locals or object properties are not source-backed by the current signed-i32 guard. The existing issue-282 guard covers literal-derived local/object-property out-of-range values, but a broader runtime path must avoid silent wrong booleans when `StringToBigInt` produces a value outside the current small-int comparison helper boundary.

## Desired final state

Mixed BigInt comparisons either:

- perform compatible `ToPrimitive` for the supported object-literal/local subset, then reuse the existing BigInt/String, BigInt/Boolean, BigInt/nullish, or BigInt/BigInt comparison paths;
- reject unsupported object coercion with source-backed diagnostics before lowering;
- handle unknown out-of-range dynamic BigInt/String parsing without returning a silently incorrect boolean.

## Scope

In scope:

- [ ] Implement a narrow object `ToPrimitive` comparison subset for object literals/locals with direct `valueOf` or `toString` methods returning supported primitive values.
- [ ] Add Node/iwasm differential coverage for at least one compatible object `ToPrimitive` BigInt comparison case.
- [ ] Preserve source-backed diagnostics for unsupported object coercion shapes.
- [ ] Add a precise guard or runtime behavior for non-source-backed unknown out-of-range dynamic BigInt/String comparison input.

Out of scope:

- BigInt arithmetic; issue 260 owns arithmetic.
- BigInt/Number `NaN`, `Infinity`, fractional, and broader number-model edges; issue 281 owns those cases.
- BigInt builtin dynamic invalid-string exception parity; issue 333 owns `BigInt(...)` inputs.
- General object/prototype/Proxy coercion semantics beyond the explicitly supported object-literal/local subset.

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
- unrelated object model or prototype behavior

## Acceptance criteria

- [ ] Node/iwasm differential fixture covers an object `valueOf` or `toString` mixed BigInt comparison that currently reports the issue-282 object `ToPrimitive` diagnostic.
- [ ] Unsupported object `ToPrimitive` shapes keep source-backed diagnostics with issue ownership.
- [ ] Unknown non-source-backed out-of-range dynamic BigInt/String comparison cannot silently return an incorrect normal boolean.
- [ ] Docs/current-state/issues state the remaining limits and the supported subset.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli bigint
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

Issue 282 closed the implemented primitive mixed BigInt coercion slice and retained source-backed diagnostics for object `ToPrimitive` boundaries. This issue owns changing those diagnostics into compatible behavior for a deliberately narrow object subset and handling unknown out-of-range runtime strings without silent boolean corruption.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Object coercion may need broader object-model support if direct object-literal/local method extraction is insufficient.
