---
id: 210
title: "Implement arrow function closure and lexical this semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [211]
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Complete arrow function runtime semantics by lowering closures and preserving lexical `this`.

## Problem

Issue 036 completed arrow syntax but recorded placeholder emission. Arrow functions must not count as semantic compatibility until closure capture and lexical `this` behavior are differentially verified.

## Desired final state

Arrow functions execute with captured lexical variables and lexical `this`, matching Node.js behavior for the supported subset.

## Scope

In scope:

- [x] Lower arrow functions to callable closure values instead of placeholder `undefined`.
- [x] Capture referenced lexical variables needed by the arrow body.
- [x] Preserve lexical `this` rather than binding a new receiver at call time.
- [x] Add Node differential fixtures for expression body, block body, captured variable, and lexical `this`.

Out of scope:

- Async arrow functions.

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

- [x] Arrow functions no longer lower or emit to a placeholder value.
- [x] Closure capture works for at least supported local bindings.
- [x] Lexical `this` matches Node.js in differential fixtures.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(arrow|closure|this)'
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

Created from issue 203 audit of `issues/done/036-implement-arrow-function.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- close commit for arrow closure and lexical-this semantics

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run arrow_function_fixtures_match_node_output_under_iwasm this_receiver_method_fixtures_match_node_output_under_iwasm this_receiver_method_unsupported_forms_report_issue_211 --no-tests warn
result: pass (3 passed)
date: 2026-04-28

command: cargo nextest run -E 'test(arrow|closure|this)'
result: no tests selected (nextest treats `arrow|closure|this` literally in this filter position; exact semantic tests above were run)
date: 2026-04-28

command: cargo nextest run
result: pass (245 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Escaping function values remain tied to issue 221 call-frame roots; this issue closes local arrow binding calls, captured locals, and lexical `this` differential coverage for the supported subset.
