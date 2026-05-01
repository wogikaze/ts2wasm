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

## Scope

In scope:

- Block scoping semantics
- Lexical scope chain resolution
- Variable hoisting and temporal dead zone

Out of scope:

- General name resolution (meta-issue 5005)
- Type checking

## Affected paths

Expected:

- `crates/frontend/src/resolver/`

## Acceptance criteria

- [ ] All 32 child issues dependency-linked to this meta

## Validation

```sh
mise run reference-coverage -- tsc --limit 20 --detail
```
