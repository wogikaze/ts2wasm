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
completed: 2026-04-29
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

- [x] Named function declaration with a block body.
- [x] Direct local call by identifier.
- [x] Positional parameter binding for fixed arity.
- [x] `return` from the function body.
- [x] Node/iwasm differential fixtures for basic calls.

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

- [x] A named function can be declared and called from top-level code.
- [x] Fixed positional arguments are bound in declaration order.
- [x] Return values match Node for the supported scalar subset.
- [x] Unsupported function forms continue to emit issue-linked diagnostics.

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

Commits:

- `446224c` issue-062c: add ordinary function direct-call coverage

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(function) or test(node_diff)'
result: passed (22 tests)
date: 2026-04-29

command: cargo nextest run
result: passed (418 tests, 4 skipped)
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 94 --detail
result: passed (executed=94; unsupported=94; build_pass=0; semantic_pass=0)
date: 2026-04-29
```

Remaining risks:

- none

Evidence notes:

- `fixtures/core-semantics/ordinary-function-direct-call.ts` verifies top-level named function declarations, direct identifier calls, declaration-order positional binding, and scalar return values against Node/iwasm output.
- `fixtures/core-semantics/nested-function-declaration-unsupported.ts` verifies nested function declarations remain out of scope with an `issue-062c` diagnostic.
- Existing `fixtures/primitives-control-flow/function.ts` continues to verify the basic `add(2, 3)` direct-call path.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/062c-ordinary-function-declarations-and-calls.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
