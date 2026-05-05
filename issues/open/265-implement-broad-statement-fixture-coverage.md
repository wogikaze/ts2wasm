---
id: 265
title: "Add broad statement fixture coverage (audit reopened #265)"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-05status: open
---

## Summary

Add comprehensive fixture tests covering statement forms parsed by the frontend. Ensures each statement AST node type is exercised by at least one fixture pair.

## Problem

Statement-level coverage is ad-hoc. New statement forms are added without corresponding fixtures, making it hard to detect regressions across the compile chain.

## Scope

In scope:

- [ ] Inventory all Stmt variants in the frontend AST
- [ ] For 15 of 30 missing variants, create a `.ts` fixture and expected output
- [ ] Remaining 15 variants (import/export module forms, class, for-in) need module test infra or compiler support
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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/265-implement-broad-statement-fixture-coverage.md` before this move
- `issues/open/265-implement-broad-statement-fixture-coverage.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
