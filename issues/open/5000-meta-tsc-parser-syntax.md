---
id: 5000
title: "Meta: TypeScript Compiler Parser Syntax Coverage"
type: meta
area: frontend/syntax
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers all TypeScript compiler test cases that fail with `parser-syntax` diagnostic (1,172 issues). These are individual tsc test cases that require parser-level syntax support.

## Problem

1,172 reference test cases across the TypeScript compiler suite produce `parser-syntax` diagnostic errors. Each is tracked as an individual triage-needed issue. This meta-issue organizes them for coordinated implementation.

## Scope

In scope:

- Parser support for all TypeScript syntax constructs reported as `parser-syntax`
- Individual child issues (IDs 066-4814) each cover one tsc test case or test family

Out of scope:

- Semantic analysis of parsed constructs (covered by meta-issues 5001, 5003)
- Resolver/name-resolution (covered by meta-issue 5005)
- Runtime builtins (covered by meta-issue 5004)

## Affected paths

Expected:

- `crates/frontend/src/parser/`

## Acceptance criteria

- [ ] All 1,172 child issues are blocked on this meta or superseded by implementation
- [ ] Parser-syntax diagnostic count trends toward zero as child issues are resolved

## Validation

```sh
mise run reference-coverage -- tsc --limit 100 --detail
```

## Notes

Child issues in this group have been bulk-updated to `class: blocked` with `depends_on: [5000]`.

## Implementation Plan

See `docs/superpowers/plans/2026-05-02-tsc-parser-syntax.md` for the full wave-based implementation plan.

### Current implementation wave

**Wave 1 (Simple Statement/Expression Fixes)** — in progress:

1. [x] Reserved words as property names (Task 1.1)
2. [x] `for...in` with type annotations (Task 1.2)
3. [x] Index signatures (Task 1.3)
4. [ ] `this`/`super` keyword edge cases (Task 1.4)
