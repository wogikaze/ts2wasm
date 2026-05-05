---
id: 264
title: "Add broad expression fixture coverage (audit reopened #264)"
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

Add comprehensive fixture tests covering expression forms parsed by the frontend. Ensures each expression AST node type is exercised by at least one fixture pair.

## Problem

Expression-level coverage is ad-hoc. New expression forms are added without corresponding fixtures, making it hard to detect regressions across the compile chain.

## Scope

In scope:

- [ ] Inventory all Expr variants in the frontend AST
- [ ] For each missing variant, create a `.ts` fixture and expected output
- [ ] Register in fixture catalog

Out of scope:

- Runtime semantics beyond basic compilation smoke test
- TypeScript-specific expression forms not yet parsed

## Acceptance criteria

- [ ] Every Expr variant has at least one fixture under fixtures/expr/
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
- `issues/open/264-implement-broad-expression-fixture-coverage.md` before this move
- `issues/open/264-implement-broad-expression-fixture-coverage.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
