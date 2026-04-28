---
id: 226
title: "Implement TypeScript parameter properties"
type: feature
area: frontend/semantics
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement TypeScript constructor parameter properties so constructor parameters with accessibility or readonly modifiers create and initialize instance properties.

## Problem

The issue 060 tsgo limit-120 classification window found unsupported cases for constructor parameter properties with default values. These cases are currently classified as `parameter-property` instead of `unknown-unsupported`.

## Desired final state

Constructor parameter properties are parsed, represented, lowered, and emitted with TypeScript-compatible runtime behavior, including default values and optional properties.

## Scope

In scope:

- [ ] Parse constructor parameter properties with `public`, `private`, `protected`, and `readonly` modifiers.
- [ ] Preserve default parameter evaluation for parameter properties.
- [ ] Initialize the corresponding instance property during construction.
- [ ] Add regression fixtures for readonly/private/public parameter properties with defaults.

Out of scope:

- [ ] Full TypeScript visibility enforcement beyond diagnostics needed for parameter properties.
- [ ] Declaration emit compatibility.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] The classified tsgo parameter-property cases no longer report `parameter-property`.
- [ ] Constructor parameter properties initialize instance fields with Node-compatible behavior for the supported subset.
- [ ] Unsupported parameter-property forms, if any remain, have precise issue-linked diagnostics.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
```

Not run:

- none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-120 window:

- `reference/typescript-go/testdata/tests/cases/compiler/parameterPropertyWithDefaultValue.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/parameterPropertyWithDefaultValueExtended.ts`

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
