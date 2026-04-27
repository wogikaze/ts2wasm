---
id: 212
title: "Implement rest parameter argument collection"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement runtime collection of remaining arguments into the rest-parameter array.

## Problem

Issue 038 parses rest parameters but records that lowering creates an empty-array placeholder. Rest parameters need call-site argument collection before they count as semantic compatibility.

## Desired final state

`function f(a, ...rest) {}` receives all extra call arguments in `rest` as a dense array matching Node.js behavior for supported calls.

## Scope

In scope:

- [x] Pass argument-count and argument values through call lowering/emission as needed.
- [x] Populate rest arrays with arguments after the last named parameter.
- [x] Add Node differential fixtures for zero, one, and multiple rest arguments.
- [x] Preserve behavior of ordinary fixed parameters.

Out of scope:

- `arguments` object semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] Rest parameters no longer always produce an empty array.
- [x] Rest arrays preserve argument order and length.
- [x] Node differential fixtures cover zero/extra argument cases.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(rest)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/038-implement-rest-parameters.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Rest parameter calls now pass a dense rest array built from extra call-site arguments.
- Regression coverage: `fixtures/core-semantics/rest-params-zero.ts`,
  `fixtures/core-semantics/rest-params-one.ts`, and
  `fixtures/core-semantics/rest-params-multiple.ts`.

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -E 'test(rest)'
result: PASS (1 passed)
date: 2026-04-28

command: cargo nextest run
result: PASS (240 passed, 4 skipped)
date: 2026-04-28

command: scripts/manager check-issue-health
result: PASS
date: 2026-04-28
```

Remaining risks:

- none
