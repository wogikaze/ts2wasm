---
id: 065
title: "Implement parser syntax extensions"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement parser-syntax feature to handle 52 failing test cases in reference tests.

Problem: This duplicates the parser syntax epic in issue 059 and should not compete as a separate Ready item.

Queue design note:

- Do not select this issue directly.
- Merge useful affected-test evidence into issue 059 child slices, then close this issue as superseded when references are preserved.

## Problem

Reference test results show 52 cases fail with parser-syntax diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

parser-syntax feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for parser-syntax feature
- [ ] Add fixtures for parser-syntax feature behavior
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

- [ ] parser-syntax feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for parser-syntax feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 104
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

- `reference/test262/test/annexB/built-ins/String/prototype/big/B.2.3.3.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/B.2.3.4.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/B.2.3.5.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/B.2.3.6.js`
- `reference/test262/test/annexB/built-ins/String/prototype/italics/B.2.3.9.js`
- `reference/test262/test/annexB/built-ins/String/prototype/small/B.2.3.11.js`
- `reference/test262/test/annexB/built-ins/String/prototype/strike/B.2.3.12.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/B.2.3.13.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/start-and-length-as-numbers.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/start-negative.js`
- ... and 42 more files

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
