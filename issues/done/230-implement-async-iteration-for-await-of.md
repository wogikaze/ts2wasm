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
completed: 2026-04-28
status: done
---

## Summary

Implement or explicitly diagnose async iteration and `for await...of` semantics.

## Problem

The issue 060 test262 limit-1250 classification window found an unsupported case under `annexB/language/statements/for-await-of/`. The reference metadata identifies `async-iteration`, and the case uses `for await (var x of iter)` with `Symbol.asyncIterator` and Annex B `IsHTMLDDA` behavior. It is now classified as `async-iteration` instead of `unknown-unsupported`.

## Desired final state

Async iteration constructs are accepted and lowered when supported, or rejected with precise diagnostics when the required runtime semantics are not yet implemented.

## Scope

In scope:

- [x] Decide the first supported slice for `for await...of`.
- [x] Add parser and semantic diagnostics for unsupported async iteration forms.
- [x] Track `Symbol.asyncIterator` and async iterator close behavior needed by reference coverage.
- [x] Add regression coverage for the selected async iteration behavior or diagnostics.

Out of scope:

- [x] Full Promise implementation unless required by the selected slice.
- [x] Broad async function lowering beyond async iteration diagnostics.
- [x] Annex B `IsHTMLDDA` behavior outside the classified async iteration case.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] The classified test262 async-iteration case no longer reports `async-iteration` for the selected implementation slice, or reports a more precise issue-linked diagnostic.
- [x] `for await...of` behavior or unsupported diagnostics have regression coverage.
- [x] Existing async/function/parser fixtures still pass.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/statements/for-await-of --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected by this closure audit

Follow-up issues:

- [x] none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected file in the limit-1250 window:

- `reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js`

Progress on 2026-04-28:

- Added parser/frontend unsupported diagnostics for direct `for await...of` and the `async function` wrapper that currently gates the reference-backed for-await-of case.
- Added regression coverage in frontend parser tests and CLI fixture diagnostics.
- Reference shard remains `UnsupportedSyntax: async-iteration`; raw compiler stderr for the reference file now reports `issue-230` instead of the previous generic `unsupported expression: Async`.

## Completion evidence

Commits:

- `6aa3283` (`issue-230: close async iteration diagnostics`)

Validation result:

```text
command: cargo test -p ts2wasm-frontend issue_linked_diagnostic
result: passed; 4 parser diagnostic tests passed including `rejects_for_await_of_with_issue_linked_diagnostic` and `rejects_async_function_with_issue_linked_diagnostic`
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test m2_node_diff for_await_of_unsupported_reports_issue_230
result: passed; CLI fixture diagnostic coverage reports `issue-230`
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build ./reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js -o /tmp/ts2wasm-issue230-ref.wasm
result: failed as expected with `[UnsupportedSyntax] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 432..446`
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/statements/for-await-of --detail
result: passed; executed=1, unsupported=1, unsupported_features=async-iteration:1; raw compiler diagnostic is the precise issue-linked `issue-230` unsupported diagnostic above
date: 2026-04-28

command: cargo fmt --all --check
result: passed
date: 2026-04-28

command: cargo nextest run
result: passed; 304 tests passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
