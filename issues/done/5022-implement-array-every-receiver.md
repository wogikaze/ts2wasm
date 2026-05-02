---
id: 5022
title: "Implement Array.prototype.every receiver semantics for 2dArrays"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
updated: 2026-05-03
---

## Summary

Implement Array.prototype.every receiver semantics to resolve the issue-211 method receiver problem in `2dArrays.ts`. The runtime fails to resolve the receiver for `arr.every(...)` when the callback accesses `this` to reference the array.

This is a work order for the residual compatibility gap, split from unknown-unsupported triage.

## Problem

Reference test `2dArrays.ts` fails with `UnsupportedSyntax: issue-211: function-valued local calls such as extracted method`. The `every()` callback uses `this` to reference the enclosing array, but the runtime cannot resolve the method receiver in this pattern.

Problem: 2dArrays fails due to issue-211 Array.prototype.every method receiver.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
```

Failure: issue-211 — method `every` receiver is not resolved, causing UnsupportedSyntax diagnostic.

## Scope

In scope:

- [x] Fix Array.prototype.every callback `this` receiver
- [x] Verify with `2dArrays.ts` fixture

Out of scope:

- Other array iteration methods with the same issue
- General issue-211 resolution

## Affected paths

Expected:

- `crates/runtime-abi/src/`
- `fixtures/`

## Acceptance criteria

- [x] `2dArrays.ts` compiles and runs without issue-211 diagnostic
- [x] New fixture test covers every() with `this` receiver

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```
