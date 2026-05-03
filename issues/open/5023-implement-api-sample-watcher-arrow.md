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

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This issue was created in commit `f0d3d720` as a triage split from unknown-unsupported alongside 5022, 5024, and 5025. Unlike 5022 (which received a real fix in `d5b1b669`), this issue was moved to `issues/done/` **without any implementation commits**. All scope and acceptance criteria checkboxes remain unchecked (11 unchecked) and there is no completion evidence section, no close note, and no implementation commits referencing #5023. The only commits mentioning 5023 are `d2028658` (for 5053) and `64fbc646` (re-open false-dones 5024/5025).

**True-done checklist** (all must pass):

1. **Implement arrow function block body support** for `APISample_watcher.ts`:
   - Extend arrow function support to block bodies (issue-062k)
   - `APISample_watcher.ts` compiles without issue-062k diagnostic

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Implementation commit referencing #5023
   - `APISample_watcher.ts` reference test produces no issue-062k diagnostic
   - Existing arrow function fixtures unchanged and passing
   - Completion evidence section filled with commit SHAs and validation results
