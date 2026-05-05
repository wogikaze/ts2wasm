---
id: 5022
title: "Implement Array.prototype.every receiver semantics for 2dArrays (audit reopened #5022)"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
updated: 2026-05-05status: open
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

- [ ] Fix Array.prototype.every callback `this` receiver
- [ ] Verify with `2dArrays.ts` fixture

Out of scope:

- Other array iteration methods with the same issue
- General issue-211 resolution

## Affected paths

Expected:

- `crates/runtime-abi/src/`
- `fixtures/`

## Acceptance criteria

- [ ] `2dArrays.ts` compiles and runs without issue-211 diagnostic
- [ ] New fixture test covers every() with `this` receiver

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met, must-reopen.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file; frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5022-implement-array-every-receiver.md` before this move
- `issues/open/5022-implement-array-every-receiver.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
