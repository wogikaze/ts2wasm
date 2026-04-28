---
id: 062f
title: "Implement function object metadata"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["062c"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Function object metadata such as `name`, `length`, and prototype
properties is observable but should not be mixed into callable execution support.

## Summary

Implement the smallest supported function object metadata slice after ordinary
function declarations and direct calls exist.

## Scope

In scope:

- [ ] `fn.name` for supported named functions.
- [ ] `fn.length` for supported fixed-arity functions.
- [ ] Diagnostics or explicit follow-up issues for unsupported prototype and descriptor behavior.
- [ ] Node/iwasm differential fixtures for the supported metadata subset.

Out of scope:

- Dynamic Function constructor metadata.
- `bind`, `call`, `apply`, and prototype mutation behavior.
- Property descriptor compatibility.
- Async/generator/class constructor metadata.

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

- [ ] Supported named function `name` reads match Node.
- [ ] Supported fixed-arity function `length` reads match Node.
- [ ] Unsupported metadata/prototype forms remain issue-linked diagnostics or follow-up issues.
- [ ] Callable execution fixtures from issue 062c continue to pass.

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
