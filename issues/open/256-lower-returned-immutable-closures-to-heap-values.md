---
id: 256
title: "Lower returned immutable closures to heap closure values"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the IR/lowering part of the heap closure ABI defined in
`docs/14-runtime-abi.md`.

Problem: Returned nested closures are currently rejected with `issue-062e`
because lowering only creates an opaque `ArrowFn` token for direct local-call
devirtualization.

## Current failure

`fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts`
currently builds as an unsupported diagnostic fixture. Returning the nested
function would lose captured locals because captures are hidden call arguments,
not heap environment slots.

## Desired final state

Lowering represents an escaping immutable closure as a heap-closure creation
expression carrying a stable `FuncId` and ordered capture list. Calls through a
known heap-closure local are represented explicitly instead of falling through to
the generic `issue-211` function-valued local diagnostic.

## Scope

In scope:

- [ ] Add lowered IR nodes or call kinds for heap closure creation and supported
      heap closure calls.
- [ ] Detect returned nested ordinary functions with immutable captures and
      lower them to the new heap closure value representation.
- [ ] Preserve the existing non-escaping direct-call closure path where it is
      already sufficient.
- [ ] Keep mutable captured bindings rejected with an issue-linked diagnostic.

Out of scope:

- Backend WAT emission for closure allocation or dispatch.
- GC mark/sweep changes.
- Function metadata (`name`, `length`, `prototype`) behavior.
- `eval`, dynamic `Function`, generator, or async closure semantics.

## Affected paths

Expected:

- `crates/ir/src/`
- `fixtures/core-semantics/`
- `crates/cli/tests/`

Do not touch:

- `crates/runtime-abi/`
- Issue 062f function metadata behavior.
- Issue 225 eval behavior.

## Acceptance criteria

- [ ] `ts2wasm_ir::lowered` can represent `return inner;` for an immutable
      captured nested function as heap closure creation.
- [ ] A new or updated lowering regression asserts the closure carries the
      expected `FuncId` and capture `LocalId` order.
- [ ] `ordinary-function-closure-mutation-unsupported.ts` still reports an
      issue-linked mutable-environment diagnostic.
- [ ] Backend validation remains honest: unsupported heap-closure IR is either
      rejected by backend validation until issue 257 lands, or covered by
      explicit backend support.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` if supported behavior changes

Follow-up issues:

- [ ] none

## Notes

Use the closure ABI from `docs/14-runtime-abi.md`: closure values are
`OBJECT_TAG` heap objects with a closure sentinel payload and immutable capture
slots.

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
