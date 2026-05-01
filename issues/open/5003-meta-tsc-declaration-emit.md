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

## Scope

In scope:

- Declaration emit for TypeScript syntax constructs
- `.d.ts` file generation

Out of scope:

- Runtime code generation
- Parser support (meta-issue 5000)

## Affected paths

Expected:

- `crates/frontend/src/emit/`

## Acceptance criteria

- [ ] All 104 child issues are dependency-linked to this meta

## Validation

```sh
mise run reference-coverage -- tsc --limit 50 --detail
```
