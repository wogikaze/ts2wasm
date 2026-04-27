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
---

## Summary

Implement runtime collection of remaining arguments into the rest-parameter array.

## Problem

Issue 038 parses rest parameters but records that lowering creates an empty-array placeholder. Rest parameters need call-site argument collection before they count as semantic compatibility.

## Desired final state

`function f(a, ...rest) {}` receives all extra call arguments in `rest` as a dense array matching Node.js behavior for supported calls.

## Scope

In scope:

- [ ] Pass argument-count and argument values through call lowering/emission as needed.
- [ ] Populate rest arrays with arguments after the last named parameter.
- [ ] Add Node differential fixtures for zero, one, and multiple rest arguments.
- [ ] Preserve behavior of ordinary fixed parameters.

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

- [ ] Rest parameters no longer always produce an empty array.
- [ ] Rest arrays preserve argument order and length.
- [ ] Node differential fixtures cover zero/extra argument cases.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

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

- [ ] update `docs/language-reference/javascript-features.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/038-implement-rest-parameters.md`.

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
