---
id: 067
title: "Investigate and classify unknown-unsupported cases"
type: feature
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement unknown-unsupported feature to handle 223 failing test cases in reference tests.

## Problem

Reference test results show 223 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

unknown-unsupported feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for unknown-unsupported feature
- [ ] Add fixtures for unknown-unsupported feature behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] unknown-unsupported feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for unknown-unsupported feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 446
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

## Affected test files

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/B.2.3.7.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/B.2.3.8.js`
- `reference/test262/test/annexB/built-ins/String/prototype/link/B.2.3.10.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/surrogate-pairs.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above.js`
- `reference/test262/test/annexB/built-ins/escape/escape-below.js`
- `reference/test262/test/annexB/built-ins/unescape/four-ignore-bad-u.js`
- `reference/test262/test/annexB/built-ins/unescape/four.js`
- ... and 213 more files

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
