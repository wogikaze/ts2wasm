---
id: 5025
title: "Implement any as return type instanceof constructor RHS"
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

Implement support for `instanceof` with constructor RHS resolved through return types in `anyAsReturnTypeForNewOnCall.ts`. The runtime (issue-207) requires a supported class constructor for `instanceof` RHS.

This is a work order split from unknown-unsupported triage.

## Problem

Reference test `anyAsReturnTypeForNewOnCall.ts` fails with `UnsupportedSyntax: issue-207: instanceof right-hand side must be a supported class constructor`. The RHS of `instanceof` is resolved through a function return type rather than a direct class reference.

Problem: anyAsReturnTypeForNewOnCall fails due to issue-207 instanceof RHS constructor resolution.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAsReturnTypeForNewOnCall.ts
```

Failure: issue-207 — instanceof RHS is not recognized as a supported class constructor.

## Scope

In scope:

- [ ] Extend instanceof RHS resolution to constructors resolved through return types
- [ ] Verify with `anyAsReturnTypeForNewOnCall.ts` fixture

Out of scope:

- Other instanceof RHS patterns
- General issue-207 resolution

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] `anyAsReturnTypeForNewOnCall.ts` compiles without issue-207 diagnostic
- [ ] Existing instanceof fixtures continue to pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```
