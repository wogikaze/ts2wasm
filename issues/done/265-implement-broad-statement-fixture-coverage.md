---
id: 265
title: "Add broad statement fixture coverage"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Add comprehensive fixture tests covering statement forms parsed by the frontend. Ensures each statement AST node type is exercised by at least one fixture pair.

## Problem

Statement-level coverage is ad-hoc. New statement forms are added without corresponding fixtures, making it hard to detect regressions across the compile chain.

## Scope

In scope:

- [x] Inventory all Stmt variants in the frontend AST
- [x] For 15 of 30 missing variants, create a `.ts` fixture and expected output
- [x] Remaining 15 variants (import/export module forms, class, for-in) need module test infra or compiler support
- [x] Register in fixture catalog

Out of scope:

- Runtime semantics beyond basic compilation smoke test
- TypeScript-specific statement forms not yet parsed

## Acceptance criteria

- [x] Every Stmt variant has at least one fixture under fixtures/stmt/
- [x] `mise run check fixtures` passes

## Validation

```sh
cargo fmt --all --check
cargo nextest run
mise run check fixtures
```

## Docs / current-state / issue sync

- [x] not affected

## Notes

- none
