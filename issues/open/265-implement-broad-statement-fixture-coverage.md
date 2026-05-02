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

- [ ] Inventory all Stmt variants in the frontend AST
- [ ] For each missing variant, create a `.ts` fixture and expected output
- [ ] Register in fixture catalog

Out of scope:

- Runtime semantics beyond basic compilation smoke test
- TypeScript-specific statement forms not yet parsed

## Acceptance criteria

- [ ] Every Stmt variant has at least one fixture under fixtures/stmt/
- [ ] `mise run check fixtures` passes

## Validation

```sh
cargo fmt --all --check
cargo nextest run
mise run check fixtures
```

## Docs / current-state / issue sync

- [ ] not affected

## Notes

- none
