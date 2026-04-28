---
id: 062b
title: "Own dynamic Function constructor diagnostics"
type: feature
area: frontend/semantics
class: verification-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Dynamic `Function(...)` and `new Function(...)` behave like runtime
code evaluation. They must not silently become wasm semantics without an
explicit dynamic evaluation policy.

## Summary

Keep dynamic Function constructor support diagnostic-only, preserve issue-linked
unsupported evidence, and own the Annex B dynamic Function cases formerly listed
by issue 063.

## Scope

In scope:

- [ ] Verify `Function(...)` reports an issue-linked unsupported diagnostic.
- [ ] Verify `new Function(...)` reports the same policy-backed diagnostic.
- [ ] Preserve Annex B HTML-comment parameter/body cases from issue 063 as owned
  reference evidence.
- [ ] Add or update regression fixtures only for diagnostic behavior.

Out of scope:

- Implementing runtime code evaluation.
- Implementing `eval`.
- Adding parser support unrelated to dynamic Function constructor diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `issues/open/062-implement-function.md`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Direct and constructed dynamic Function forms produce stable unsupported diagnostics.
- [ ] Diagnostic text links the behavior to dynamic runtime code evaluation policy.
- [ ] Issue 063 affected tests are referenced here or in regression evidence.
- [ ] No dynamic evaluation runtime semantics are implemented in this issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(function_constructor) or test(unsupported)'
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Affected test files from superseded issue 063

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`

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
