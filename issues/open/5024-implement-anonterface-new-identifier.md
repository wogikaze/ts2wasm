---
id: 5024
title: "Implement anonymous interface new expression identifier"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

Implement support for `new` expressions with non-identifier class references in `anonterface.ts`. The runtime (issue-062) requires a class name identifier for `new` expressions, rejecting anonymous interface patterns.

This is a work order split from unknown-unsupported triage.

## Problem

Reference test `anonterface.ts` fails with `UnsupportedSyntax: issue-062: constructors/resolutions requiring a class-name identifier are not supported`. The test uses `new` with a type reference that is not a simple class name identifier.

Problem: anonterface fails due to issue-062 new expression identifier requirement.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
```

Failure: issue-062 — `new` expression requires a class-name identifier but receives a non-identifier type reference.

## Scope

In scope:

- [ ] Extend `new` expression resolution to non-identifier class references
- [ ] Verify with `anonterface.ts` fixture

Out of scope:

- Other issue-062 sub-cases
- General constructor resolution improvements

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] `anonterface.ts` compiles without issue-062 diagnostic
- [ ] Existing new expression fixtures continue to pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```
