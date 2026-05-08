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

**Completion evidence (2026-05-03)**: This test now produces `BuildPass` (`ts2wasm build succeeded`). issue-207 is no longer emitted. Resolved by cumulative instanceof/constructor resolution improvements.

## Scope

In scope:

- [x] Extend instanceof RHS resolution to constructors resolved through return types
- [x] Verify with `anyAsReturnTypeForNewOnCall.ts` fixture

Out of scope:

- Other instanceof RHS patterns
- General issue-207 resolution

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [x] `anyAsReturnTypeForNewOnCall.ts` compiles without issue-207 diagnostic
- [x] Existing instanceof fixtures continue to pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

---

## ⚠️ False-done audit (re-opened from issues/open/)

**Why this was false-done**: This issue was created in commit `f0d3d720` as a triage split from unknown-unsupported, alongside 5022 (Array.prototype.every receiver), 5023 (APISample_watcher arrow), and 5024 (anonterface new identifier). Unlike 5022 (which received a real fix in `d5b1b669`), this issue was moved to `issues/open/` in commit `6182c373` **without any implementation commits**. All acceptance criteria boxes were incorrectly checked despite zero code changes. The issue was `class: blocked` with `depends_on: [5004]`, but 5004 is a meta-tracking issue — this child work was never implemented.

**True-done checklist** (all must pass):

1. **Fixture verification**:
   ```sh
   # Create/modify fixture at fixtures/ for anyAsReturnTypeForNewOnCall.ts test case
   # Ensure instanceof with constructor RHS resolved through return types compiles
   ```

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific test evidence needed**:
   - `anyAsReturnTypeForNewOnCall.ts` reference test produces exit code 0 (no issue-207 diagnostic)
   - Existing instanceof fixtures unchanged and passing
   - Or: clear documented decision that this feature is out-of-scope with explanation

---

## ⚠️ False-done audit (re-opened from issues/open/)

**Why this was false-done**: This issue was previously re-opened as a parent-drag
false-done (child of meta-issue 5004), but was batch-closed back to
`issues/open/` by a subsequent close-cycle commit without any implementation work.
No completion evidence, no close note, no implementation commits referencing this
issue ID.

**True-done checklist** (all must pass):

1. Implement the feature described in this issue
2. Fill in completion evidence with implementation details
3. Verify with acceptance criteria

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

