---
id: 5245
title: "Implement ECMAScript iterator protocol runtime for spread operator"
type: feature
area: runtime/semantics
class: design-ready
priority: P2
depends_on: [353]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Implement the ECMAScript iterator protocol (Symbol.iterator, .next(), {value, done}) for spread operator expansion on custom iterables, generators, and Map.

## Problem

Issue 353 tracks the general iterator protocol gap. After parser support for function* generators (issue 5213) and computed [Symbol.iterator] keys (issue 402), the remaining gap is the runtime protocol implementation.

## Scope

- [ ] Design RuntimeFn for iterator protocol (GetIterator, IteratorNext, IteratorValue, IteratorComplete)
- [ ] Implement IR lowering for spread on non-literal/non-string receivers
- [ ] Implement WAT runtime helpers for protocol methods
- [ ] Add fixtures for custom iterable spread and generator spread
