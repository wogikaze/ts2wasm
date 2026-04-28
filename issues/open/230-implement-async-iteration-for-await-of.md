---
id: 230
title: "Implement async iteration and for-await-of"
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

Implement or explicitly diagnose async iteration and `for await...of` semantics.

## Problem

The issue 060 test262 limit-1250 classification window found an unsupported case under `annexB/language/statements/for-await-of/`. The reference metadata identifies `async-iteration`, and the case uses `for await (var x of iter)` with `Symbol.asyncIterator` and Annex B `IsHTMLDDA` behavior. It is now classified as `async-iteration` instead of `unknown-unsupported`.

## Desired final state

Async iteration constructs are accepted and lowered when supported, or rejected with precise diagnostics when the required runtime semantics are not yet implemented.

## Scope

In scope:

- [ ] Decide the first supported slice for `for await...of`.
- [ ] Add parser and semantic diagnostics for unsupported async iteration forms.
- [ ] Track `Symbol.asyncIterator` and async iterator close behavior needed by reference coverage.
- [ ] Add regression coverage for the selected async iteration behavior or diagnostics.

Out of scope:

- [ ] Full Promise implementation unless required by the selected slice.
- [ ] Broad async function lowering beyond async iteration diagnostics.
- [ ] Annex B `IsHTMLDDA` behavior outside the classified async iteration case.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] The classified test262 async-iteration case no longer reports `async-iteration` for the selected implementation slice, or reports a more precise issue-linked diagnostic.
- [ ] `for await...of` behavior or unsupported diagnostics have regression coverage.
- [ ] Existing async/function/parser fixtures still pass.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/language/statements/for-await-of --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected file in the limit-1250 window:

- `reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js`

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
