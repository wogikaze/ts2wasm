---
id: 302
title: "Implement direct eval block function declaration slice"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement the first narrow issue-225 slice for direct `eval` with one Annex B
block-level function declaration inside eval code.

Problem: Direct eval block-function reference cases still report unsupported
feature label `eval` instead of exercising implemented semantics.

## Problem

Direct `eval` needs special handling because evaluated source can introduce or
update bindings in the caller's variable environment. The first supported slice
should choose the smallest Annex B block-function behavior before adding the
if/else and early-error variants.

## Current failure

Reference-backed failures from issue 225:

```sh
mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-init.js --detail
mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
```

Current status: both cases are in issue 225's classified direct-eval Annex B
window and are expected to report unsupported feature label `eval` before this
slice is implemented.

## Desired final state

The compiler implements the narrow direct-call `eval("<static source>")` form
for the selected block-function declaration cases, either through explicit
frontend/IR lowering or through an auditable shim path with manifest/link-plan
coverage if a shim is required.

## Scope

In scope:

- [ ] Direct `eval` calls where the callee is the unshadowed identifier `eval`.
- [ ] Static string-literal eval source for a single block containing a function declaration, matching `func-block-decl-eval-func-init.js`.
- [ ] The companion block-scoping behavior from `func-block-decl-eval-func-block-scoping.js`.
- [ ] Regression fixture covering direct eval block-level function declaration behavior.
- [ ] Existing non-eval ordinary function declaration/call fixture remains passing.

Out of scope:

- Indirect eval, shadowed eval, non-string eval input, and dynamic eval source.
- The `func-if-decl-else-decl-a-*` and `func-if-decl-else-decl-b-*` families.
- The `skip-early-err-*`, `existing-var-*`, `existing-fn-*`, and `existing-block-fn-*` variants.
- Issue 300 runtime number work and BigInt/runtime-number representation changes.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/` only for eval/shim-specific support
- `crates/cli/src/` only for eval/shim/manifest support
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- `docs/`
- issue 300 files or large-number runtime work
- BigInt runtime/ABI files
- class-method mutable cell work tracked by issue 301

## Acceptance criteria

- [ ] `func-block-decl-eval-func-init.js` no longer reports unsupported feature label `eval`.
- [ ] `func-block-decl-eval-func-block-scoping.js` no longer reports unsupported feature label `eval`.
- [ ] A new fixture under `fixtures/core-semantics/` exercises direct eval with a block-level function declaration and passes Node/iwasm differential validation.
- [ ] A non-eval ordinary function fixture remains in the focused validation set to prove existing function behavior is preserved.
- [ ] If shim JavaScript is emitted, its host capability is represented in link planning and manifest output.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli direct_eval_block_function
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-init.js --detail
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
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

This is split from issue 225 because the parent covers multiple direct-eval
families and existing-binding/early-error variants. Keep this slice limited to
the first block-function declaration behavior.

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
