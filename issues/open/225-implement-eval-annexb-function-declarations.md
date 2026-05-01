---
id: 225
title: "Implement eval and Annex B function declaration semantics"
type: meta
area: frontend/semantics
class: ready
priority: P3
depends_on: []
blocks: [347, 348, 349, 406]
created: 2026-04-28
updated: 2026-05-01
---

## Summary

Implement the direct `eval` and Annex B function-declaration behavior needed by legacy test262 eval-code cases.

Problem: Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not sufficient, the compiler must emit auditable shim JavaScript instead of treating the feature as out of scope.

## Problem

The issue 060 limit-300 test262 classification window found 51 unsupported cases under `annexB/language/eval-code/direct/`. These cases exercise direct `eval` with block-level function declarations and web-compat eval declaration instantiation behavior.

## Desired final state

Direct eval-code cases are implemented with correct ECMAScript semantics through wasm/runtime helpers or emitted shim JavaScript. Until then, reference coverage uses the stable `eval` feature label instead of `unknown-unsupported`.

Accepted decisions:

- JavaScript / TypeScript language features are all support targets.
- Dynamic code evaluation is support-required.
- If wasm/WASI/runtime helpers cannot implement the semantics directly, emit auditable shim JavaScript and record required host capabilities.

## Scope

This meta issue tracks child issues for completing eval and Annex B function declaration semantics.

Child issues:
- [x] Issue 347: Parser and resolver support for direct eval and eval-code scope
- [x] Issue 348: Lowering block-level function declarations in direct eval code
- [x] Issue 349: Runtime helper or shim JavaScript emission for direct eval execution
- [ ] Issue 406: Direct eval Annex B existing binding residuals

Out of scope:

- Broad indirect eval or full host-specific global environment behavior beyond the selected direct-eval slice.
- General function feature work tracked by issue 062.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

This meta issue is complete when all child issues are moved to `done/`.

- [x] Issue 347 closed: parser/resolver direct eval detection and eval-code scope analysis
- [x] Issue 348 closed: IR lowering for block-level function declarations in eval code
- [x] Issue 349 closed: runtime helper or shim JavaScript emission for direct eval execution
- [ ] test262 direct eval Annex B function-declaration cases in the classified window no longer report `eval`.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
```

Not run:

- none

## Breakdown rule

If this issue exceeds one child cycle, split before implementation stalls:

1. overview: select the first direct-eval / Annex B fixture family and expected semantics.
2. file structure: identify parser/resolver/lowering/backend/shim emission ownership.
3. code design: choose wasm/runtime helper vs shim emission for that slice.
4. implementation: land the smallest semantic slice with fixture and reference evidence.

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Split on 2026-04-29:

- issue 302: first implementation-ready slice for
  `func-block-decl-eval-func-init.js` and
  `func-block-decl-eval-func-block-scoping.js`.

Reference-backed affected files in the limit-300 window are under:

- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-*.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-if-decl-else-decl-a-eval-func-*.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-if-decl-else-decl-b-eval-func-*.js`

2026-05-01 parent coverage refresh:

- `mise run reference-coverage -- test262 --limit 300 --no-web-ui` still
  reports `unsupported_features=eval:29` and
  `unsupported_diagcodes=UnsupportedEval:6` in the classified window.
- Direct eval-code cases now include a mix of `BuildPass`, `UnresolvedName`,
  `DuplicateLocal`, and `UnsupportedSyntax: eval`. Issue 406 tracks the next
  concrete direct-eval residual family instead of treating 225 as closable.

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
