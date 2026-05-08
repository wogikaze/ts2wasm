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
completed: 2026-04-28
---

## Summary

Implement TypeScript constructor parameter properties so constructor parameters with accessibility or readonly modifiers create and initialize instance properties.

## Problem

The issue 060 tsgo limit-120 classification window found unsupported cases for constructor parameter properties with default values. These cases are currently classified as `parameter-property` instead of `unknown-unsupported`.

## Desired final state

Constructor parameter properties are parsed, represented, lowered, and emitted with TypeScript-compatible runtime behavior, including default values and optional properties.

## Scope

In scope:

- [x] Parse constructor parameter properties with `public`, `private`, `protected`, and `readonly` modifiers.
- [x] Preserve default parameter evaluation for parameter properties.
- [x] Initialize the corresponding instance property during construction.
- [x] Add regression fixtures for readonly/private/public parameter properties with defaults.

Out of scope:

- Full TypeScript visibility enforcement beyond diagnostics needed for parameter properties.
- Declaration emit compatibility.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] The classified tsgo parameter-property cases no longer report `parameter-property`.
- [x] Constructor parameter properties initialize instance fields with Node-compatible behavior for the supported subset.
- [x] Unsupported parameter-property forms, if any remain, have precise issue-linked diagnostics.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- tsgo --limit 120
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

- close commit on `agent/226-parameter-properties-20260428T014053Z`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -E 'test(parameter) or test(class)'
result: pass (20 passed)
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- tsgo --limit 120
result: pass; parameter-property no longer appears; build_pass 9, unsupported 111
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- tsgo --path-filter parameterPropertyWithDefaultValue --limit 120 --detail
result: pass; parameterPropertyWithDefaultValue.ts and parameterPropertyWithDefaultValueExtended.ts are build_pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass (270 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Full Date runtime semantics remain tracked outside this issue; this issue only requires the parameter-property default-expression build path used by the tsgo reference cases.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/226-implement-parameter-properties.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
