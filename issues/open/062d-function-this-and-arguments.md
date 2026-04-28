---
id: 062d
title: "Implement function this and arguments semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: ["062c"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Function receiver binding and the `arguments` object have observable
JavaScript semantics that should not be bundled with ordinary direct-call
support.

## Summary

Implement the callable slice for non-arrow function `this` binding and the
`arguments` object after ordinary function declarations/direct calls exist.

## Scope

In scope:

- [ ] Receiver binding for supported function calls.
- [ ] Top-level or unsupported receiver forms diagnosed when not implemented.
- [ ] Basic `arguments.length` and indexed argument reads.
- [ ] Node/iwasm differential fixtures for supported receiver and arguments cases.

Out of scope:

- Arrow lexical `this` beyond already validated arrow-function behavior.
- Escaping function values and closure environments.
- Full exotic arguments object aliasing semantics.
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

- [ ] Supported function calls bind `this` consistently with Node for the selected fixtures.
- [ ] Supported `arguments` reads match Node for zero, one, and multiple arguments.
- [ ] Unsupported receiver/arguments forms produce issue-linked diagnostics.
- [ ] The ordinary direct-call fixtures from issue 062c continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(function) or test(arguments) or test(node_diff)'
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
