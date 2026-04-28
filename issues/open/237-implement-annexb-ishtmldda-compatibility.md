---
id: 237
title: "Implement Annex B IsHTMLDDA compatibility"
type: feature
area: runtime/semantics
class: design-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement or explicitly diagnose ECMAScript Annex B `[[IsHTMLDDA]]` compatibility behavior used by test262 emulates-undefined cases.

## Problem

Issue 228 removed the `logical-assignment` parser/operator blocker from the limit-750 Annex B logical assignment files, but those reference cases still depend on test262/browser compatibility objects such as `$262.IsHTMLDDA` and `assert`. The current build stops earlier with `UnresolvedName` on `$262`, so full browser-compatible `document.all`-style semantics are not implemented.

## Desired final state

Annex B `[[IsHTMLDDA]]` values have a clear support policy. Supported cases match ECMAScript observable behavior for truthiness, nullish checks, and equality where in scope; unsupported host/browser compatibility forms produce precise issue-linked diagnostics.

## Scope

In scope:

- [ ] Decide whether `[[IsHTMLDDA]]` is modeled directly, shimmed for reference tests, or rejected as unsupported browser compatibility.
- [ ] Account for logical assignment cases under `reference/test262/test/annexB/language/expressions/logical-assignment/`.
- [ ] Account for other known emulates-undefined cases such as `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`.
- [ ] Add regression fixtures or reference classification evidence for the selected policy.

Out of scope:

- General test262 harness implementation beyond the minimum needed to classify these cases.
- Logical assignment operator parsing and ordinary short-circuit semantics, completed by issue 228.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] The three Annex B logical-assignment emulates-undefined cases have a stable support or unsupported classification that no longer depends on incidental unresolved `$262` handling.
- [ ] Unsupported `[[IsHTMLDDA]]` forms, if any remain, report a precise issue-linked diagnostic.
- [ ] Regression fixtures or reference coverage evidence cover `&&=`, `||=`, `??=`, truthiness, and nullish behavior for the selected policy.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/language/expressions/logical-assignment/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated when behavior changes

Follow-up issues:

- [ ] none

## Notes

Split from issue 228 close audit after the reference files stopped reporting the `logical-assignment` feature label and instead failed earlier on test262 harness name resolution.

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
