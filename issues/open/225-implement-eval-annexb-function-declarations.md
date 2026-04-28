---
id: 225
title: "Implement eval and Annex B function declaration semantics"
type: feature
area: frontend/semantics
class: design-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement the direct `eval` and Annex B function-declaration behavior needed by legacy test262 eval-code cases.

## Problem

The issue 060 limit-300 test262 classification window found 51 unsupported cases under `annexB/language/eval-code/direct/`. These cases exercise direct `eval` with block-level function declarations and web-compat eval declaration instantiation behavior.

## Desired final state

Direct eval-code cases are either implemented with correct ECMAScript semantics or rejected with a precise issue-linked diagnostic. Until then, reference coverage uses the stable `eval` feature label instead of `unknown-unsupported`.

## Scope

In scope:

- [ ] Decide the supported subset and diagnostics for direct `eval`.
- [ ] Implement or explicitly diagnose Annex B eval declaration instantiation cases involving block-level function declarations.
- [ ] Add fixtures for direct eval and block-level function declaration behavior.
- [ ] Preserve existing function, scope, and name-resolution behavior for non-eval code.

Out of scope:

- [ ] Implementing indirect eval or host-specific global environment behavior unless required by the selected direct-eval subset.
- [ ] General function feature work tracked by issue 062.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] test262 direct eval Annex B function-declaration cases in the classified window no longer report `eval`.
- [ ] Unsupported direct-eval forms, if any remain, have precise issue-linked diagnostics.
- [ ] Regression fixtures cover direct eval, block-level function declarations inside eval code, and existing non-eval function behavior.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 300
```

Not run:

- none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-300 window are under:

- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-*.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-if-decl-else-decl-a-eval-func-*.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-if-decl-else-decl-b-eval-func-*.js`

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
