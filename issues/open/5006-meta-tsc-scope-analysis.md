---
id: 5006
title: "Meta: TypeScript Compiler Scope Analysis Coverage"
type: meta
area: frontend/resolver
class: design-ready
priority: P2
depends_on: [5005]
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers TypeScript compiler test cases for scope analysis (~32 issues). Scope analysis is a subset of name resolution focusing on block scoping, hoisting, and lexical scope chains.

## Problem

~32 tsc test cases fail due to scope analysis gaps. These are a subset of the broader name resolution work.

Problem: scope-analysis failures currently need child issue classification by block scope, hoisting, lexical lookup, or temporal-dead-zone behavior.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 20 --detail` reports scope-analysis gaps, but the meta issue lacks an actionable dependency cleanup contract.

## Scope

In scope:

- [ ] Review child issues currently labeled or dependency-linked as scope-analysis.
- [ ] Keep block scoping, lexical scope chain, hoisting, and temporal-dead-zone children under `5006`.
- [ ] Move general resolver or type-system children to `5005` or `5002`.

Out of scope:

- General name resolution (meta-issue 5005)
- Type checking

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Scope-analysis child issues are dependency-linked to `5006` only when lexical scope or hoisting behavior is the primary blocker.
- [ ] General resolver and type-system children are linked to the correct meta issue.
- [ ] `issues/index.md` is regenerated after dependency or class edits.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 20 --detail
```

Not run:

- none
