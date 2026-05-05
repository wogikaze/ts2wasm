---
id: 5002
title: "Meta: TypeScript Compiler Type System Coverage"
type: meta
area: frontend/semantics
class: design-ready
priority: P1
depends_on: [5000, 5005]
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers TypeScript compiler test cases specifically for type-system semantics (~244 issues). Requires type inference, conditional types, mapped types, and generic type operations.

## Problem

~244 tsc test cases fail with type-system related diagnostics. These require implementing type inference, type relationships, and type-level computations.

Problem: type-system reference failures currently need dependency and scope cleanup so each child issue is a concrete type inference, relationship, or type-level computation slice.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 50 --detail` exposes type-system buckets, but child issues still need review against `5001` and `5005` before implementation order is clear.

## Scope

In scope:

- [ ] Review child issues currently labeled type-system.
- [ ] Keep only type inference, type relationship, conditional type, mapped type, generic constraint, and type-level computation children under `5002`.
- [ ] Move parser, declaration-emit, name-resolution, or broad semantic children to their narrower meta dependencies.

Out of scope:

- Basic semantic analysis (meta-issue 5001)
- Name resolution (meta-issue 5005)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Type-system child issues are dependency-linked to `5002` only when they require type-level implementation work.
- [ ] Non-type-system children are linked to the correct narrower meta issue.
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
