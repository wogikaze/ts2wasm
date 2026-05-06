---
id: 5220
title: "Preserve ambient function parameters for arity"
type: feature
area: frontend/ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Preserve enough parameter metadata from erased `declare function` declarations
for TypeScript-style call arity validation.

## Problem

After issue 5151's ASI blocker is gone,
`badInferenceLowerPriorityThanGoodInference.ts` reaches arity validation and
reports that `canYouInferThis(...)` expected 0 arguments even though the
ambient declaration has one parameter.

Problem: ambient function declarations are currently emitted with empty parameter lists, so calls to declared functions can receive false TS2554 arity diagnostics.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
```

Current diagnostic after issue 5151:

```text
ArityMismatch: TS2554: Expected 0 arguments, but got 1. at 180..248
```

Representative source:

```ts
declare function canYouInferThis<A>(fn: () => Foo<A>): A;

const result = canYouInferThis(() => ({
    a: { BLAH: 33 },
    b: x => { }
}))
```

## Desired final state

Erased ambient function declarations still register their runtime-unavailable
name and parameter arity metadata so TypeScript call validation can accept
valid calls before later type-inference work.

## Scope

In scope:

- [ ] Parse ambient function parameter names or placeholders into the emitted metadata statement
- [ ] Preserve generic/type annotation erasure while keeping arity count
- [ ] Add focused arity validation coverage for a one-parameter `declare function`
- [ ] Re-run the representative triage and confirm it advances past the current TS2554 false positive

Out of scope:

- Full TypeScript generic inference
- Type compatibility for `Foo<A>` and arrow return object inference
- Runtime implementation of ambient functions

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/ir/src/semantic.rs`
- `crates/cli/tests/`

Do not touch:

- backend lowering for ambient functions

## Acceptance criteria

- [ ] `declare function canYouInferThis(fn: () => number): number; canYouInferThis(() => 1);` no longer reports expected-0 arity
- [ ] Ambient function declarations remain erased from runtime lowering
- [ ] `badInferenceLowerPriorityThanGoodInference.ts` no longer stops at `TS2554: Expected 0 arguments, but got 1`
- [ ] Issue index and readiness checks pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli ambient
cargo nextest run -p ts2wasm-cli typescript_semantics_
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
python scripts/manager.py check issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split while closing issue 5151. This is the metadata/arity slice only; it does
not make ambient functions callable at runtime.

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
