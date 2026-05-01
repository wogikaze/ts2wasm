---
id: 5005
title: "Meta: TypeScript Compiler Name Resolution Coverage"
type: meta
area: frontend/resolver
class: design-ready
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers TypeScript compiler test cases requiring name resolution and scope analysis (~462 issues). These fail with resolver or name-resolution diagnostics.

## Problem

~462 tsc test cases fail due to missing or incomplete name resolution, scope management, and symbol table logic.

## Scope

In scope:

- Symbol table and scope chain implementation
- Name lookup and resolution algorithms
- Module resolution
- Identifier binding and shadowing

Out of scope:

- Type checking (meta-issue 5002)
- Semantic analysis (meta-issue 5001)

## Affected paths

Expected:

- `crates/frontend/src/resolver/`

## Acceptance criteria

- [ ] All ~462 child issues dependency-linked to this meta

## Validation

```sh
mise run reference-coverage -- tsc --limit 50 --detail
```
