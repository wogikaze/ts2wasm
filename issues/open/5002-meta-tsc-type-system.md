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

## Scope

In scope:

- Type inference and type relationship algorithms
- Conditional types, mapped types, generic constraints
- Type-level operations and type alias resolution

Out of scope:

- Basic semantic analysis (meta-issue 5001)
- Name resolution (meta-issue 5005)

## Affected paths

Expected:

- `crates/frontend/src/check/`
- `crates/frontend/src/types/`

## Acceptance criteria

- [ ] All 244 child issues are dependency-linked to this meta

## Validation

```sh
mise run reference-coverage -- tsc --limit 50 --detail
```
