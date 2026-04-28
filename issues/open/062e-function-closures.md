---
id: 062e
title: "Implement function closures"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: ["062c"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Captured lexical environments require different resolver, lowering,
runtime root, and backend behavior than direct local function calls.

## Summary

Implement closure capture for ordinary functions as a focused slice after basic
function declarations and direct calls are available.

## Scope

In scope:

- [ ] Capturing immutable outer locals used by a returned or nested function.
- [ ] Calling a closure after the declaring scope has returned.
- [ ] GC/rooting evidence for captured values when allocation pressure is involved.
- [ ] Node/iwasm differential fixtures for basic closure capture.

Out of scope:

- Dynamic Function constructor behavior.
- Full environment mutation semantics beyond the selected fixtures.
- Generator/async closure semantics.
- Function object metadata.

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

- [ ] A nested function can capture an outer local and return the captured value.
- [ ] A returned closure keeps captured values live across the selected fixture.
- [ ] Closure allocation/rooting behavior is covered by a regression fixture.
- [ ] Unsupported escaping/mutation forms produce issue-linked diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
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
