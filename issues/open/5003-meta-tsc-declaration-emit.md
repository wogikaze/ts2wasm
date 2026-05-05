---
id: 5003
title: "Meta: TypeScript Compiler Declaration Emit Coverage"
type: meta
area: frontend/syntax
class: design-ready
priority: P2
depends_on: [5000, 5001]
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers TypeScript compiler test cases for declaration emit (~104 issues). Primarily `.d.ts` generation and declaration output.

## Problem

~104 tsc test cases fail due to declaration emit (`.d.ts` generation) missing or incorrect.

Problem: declaration-emit failures currently need child issue classification by emitted declaration shape before implementers can safely change frontend emit contracts.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 50 --detail` reports declaration-emit gaps, but this meta issue does not yet define the child issue review contract.

## Scope

In scope:

- [ ] Review declaration-emit child issues for the emitted `.d.ts` construct or diagnostic they cover.
- [ ] Keep declaration output, visibility, and `.d.ts` generation children under `5003`.
- [ ] Move parser, runtime emit, name-resolution, or type-system children to the correct meta issue.

Out of scope:

- Runtime code generation
- Parser support (meta-issue 5000)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Declaration-emit child issues are dependency-linked to `5003` only when the observable output is `.d.ts` or declaration diagnostics.
- [ ] Non-declaration children are linked to the correct narrower meta issue.
- [ ] `issues/index.md` is regenerated after dependency or class edits.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 50 --detail
```

Not run:

- none
