---
id: 214
title: "Replace string method placeholders"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Replace placeholder implementations of `String.prototype.trim`, `toUpperCase`, and `toLowerCase` with semantic runtime behavior.

## Problem

Issue 042 records that several string methods currently return the original string as placeholders. Those methods need dedicated semantic implementation and differential fixtures.

## Desired final state

Supported string methods transform strings according to ECMAScript semantics for the project's supported string subset.

## Scope

In scope:

- [ ] Implement `String.prototype.trim` for supported whitespace/code unit cases.
- [ ] Implement `String.prototype.toUpperCase` and `String.prototype.toLowerCase` for the supported ASCII/Unicode subset chosen by the issue.
- [ ] Add Node differential fixtures for changed and unchanged strings.
- [ ] Track any Unicode parity gaps explicitly if the first implementation is ASCII-only.

Out of scope:

- Full `Intl` or locale-sensitive case mapping.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] The listed methods no longer return the input unchanged for cases that require transformation.
- [ ] Node differential fixtures prove trim and case conversion for supported strings.
- [ ] Any unsupported Unicode behavior is issue-linked and not counted as semantic pass.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(string)'
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

Created from issue 203 audit of `issues/done/042-implement-string-methods.md`.

## Completion evidence

Fill only when moving to `done/`.
