---
id: 5490
title: "Report array push into never array"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: [3597]
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2345-equivalent diagnostic when an empty array literal is inferred
as `never[]` and a later `push` call passes a value that is not assignable to
`never`.

## Problem

`nonNullFullInference.ts` now build-passes, but TypeScript reports TS2345 at
`arr.push(n)` because `const arr = []` is inferred as `never[]` under the
fixture's compiler settings.

Problem: `arr.push(n)` on an empty-array local inferred as `never[]` is accepted
without the TypeScript argument diagnostic.

## Current failure

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
```

Observed result:

```text
headline: BuildPass
typescript oracle: TS2345 Argument of type 'number' is not assignable to parameter of type 'never'.
location: arr.push(n), line 29 character 18
oracle hint: arr has type never[]
```

Representative source:

```ts
function testNonNullInferenceWithArrays(numbers: number[]) {
    let result;
    const arr = [];

    for (const n of numbers) {
        arr.push(n);
        result = arr;
    }

    result!;
}
```

## Desired final state

The representative fixture no longer silently build-passes when TypeScript
reports TS2345. The compiler emits a source-spanned diagnostic for pushing a
`number` into a `never[]` array local.

## Scope

In scope:

- [ ] Track empty array literal locals that should be treated as `never[]` for
  diagnostic purposes.
- [ ] Report a TS2345-equivalent diagnostic for `arr.push(n)` when `arr` is
  known as `never[]` and `n` is a number.
- [ ] Preserve supported runtime lowering for ordinary mutable arrays that are
  not inferred as `never[]`.
- [ ] Keep non-null assertion erasure (`result!`, `last!`) parsing as-is.

Out of scope:

- Full TypeScript evolving-array inference.
- General overload resolution for all array methods.
- Runtime array push lowering, already implemented separately.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused semantic or CLI fixture

Do not touch:

- `crates/backend-wasm/`
- runtime ABI

## Acceptance criteria

- [ ] `nonNullFullInference.ts` no longer silently build-passes while
  TypeScript reports TS2345 at `arr.push(n)`.
- [ ] The diagnostic is source-spanned at or near the pushed argument `n`.
- [ ] A focused regression covers `const arr = []; arr.push(1);` under the same
  relevant compiler-mode assumptions.
- [ ] Existing supported `arr.push` runtime fixtures for non-`never[]` arrays
  still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(array) or test(push)'
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullFullInference.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
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

Split from `issues/done/3597-implement-nonNullFullInference.md`.

Related but not duplicate:

- Runtime `Array.prototype.push` behavior is covered by existing runtime issues;
  this issue is about TypeScript semantic diagnostic parity.
- Other `never[]` oracle mentions in older generated buckets are not exact
  owners because they either remain parser-blocked or cover different
  assignment/iteration behavior.

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

- Full evolving-array inference may require broader follow-up after this narrow
  diagnostic slice.
