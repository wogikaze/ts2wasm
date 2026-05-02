---
id: 5023
title: "Implement API Sample watcher arrow function return"
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

Implement support for arrow functions with non-single-return-statement bodies in `APISample_watcher.ts`. Currently the runtime (issue-062k) limits arrow functions to a single return statement body.

This is a work order split from the APISample bucket (issue 070) and unknown-unsupported triage.

## Problem

Reference test `APISample_watcher.ts` fails with `UnsupportedSyntax: issue-062k: arrow functions with block bodies or multiple statements are not yet supported`. The watcher file uses arrow functions with block bodies containing multiple statements.

Problem: APISample_watcher fails due to issue-062k arrow function body limitation.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_watcher.ts
```

Failure: issue-062k — arrow function with block body/multiple statements is not supported.

## Scope

In scope:

- [ ] Extend arrow function support to block bodies
- [ ] Verify with `APISample_watcher.ts` fixture

Out of scope:

- General arrow function improvements beyond block body support
- Other APISample files (split separately)

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] `APISample_watcher.ts` compiles without issue-062k diagnostic
- [ ] Existing arrow function fixtures continue to pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```
