---
id: 062c
title: "Implement ordinary function declarations and direct calls"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Ordinary function declarations and direct calls are a separate callable
surface from dynamic Function constructors, closures, receiver binding, and
function object metadata.

## Summary

Implement the smallest direct-call slice for named function declarations:
parsing/resolution/lowering/backend execution for local calls with positional
arguments and return values.

## Scope

In scope:

- [ ] Named function declaration with a block body.
- [ ] Direct local call by identifier.
- [ ] Positional parameter binding for fixed arity.
- [ ] `return` from the function body.
- [ ] Node/iwasm differential fixtures for basic calls.

Out of scope:

- Dynamic `Function(...)` / `new Function(...)`.
- `this` binding and `arguments`.
- Closures over outer locals.
- Function object metadata such as `name`, `length`, or prototype behavior.
- Overloads, generators, async functions, and TypeScript type-only signatures.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/` unless ABI changes are explicitly required and reviewed.

## Acceptance criteria

- [ ] A named function can be declared and called from top-level code.
- [ ] Fixed positional arguments are bound in declaration order.
- [ ] Return values match Node for the supported scalar subset.
- [ ] Unsupported function forms continue to emit issue-linked diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(function) or test(node_diff)'
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

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
