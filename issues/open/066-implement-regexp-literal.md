---
id: 066
title: "Implement RegExp literal support"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Implement regexp-literal feature to handle 59 failing test cases in reference tests.

## Problem

Reference test results show 59 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

regexp-literal feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for regexp-literal feature
- [ ] Add fixtures for regexp-literal feature behavior
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

- [ ] regexp-literal feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for regexp-literal feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 118
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-not-capturing.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`
- ... and 49 more files

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
