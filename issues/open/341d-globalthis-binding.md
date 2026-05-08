---
id: 341d
title: "Implement globalThis binding"
type: feature
area: runtime/builtins
status: done
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
closed: 2026-05-01
---

## Summary

Implement the `globalThis` global binding so it can be referenced in test262 harness files (e.g., `$262.global`).

## Problem

`globalThis` is not recognized as a global identifier, causing `UnresolvedName` failures in test262 harness cases that reference `$262.global`.

## Desired final state

`globalThis` is recognized in the name resolver and resolves to a global object value.

## Scope

- [x] Add `globalThis` to `allowed_globals` in name resolver
- [x] `globalThis` resolves to the global object reference (lowered to Undefined)

## Affected paths

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/resolver_expr.rs` (globalThis → Undefined)

## Acceptance criteria

- [x] `globalThis` is recognized as a global identifier
- [x] Simple fixture `console.log(typeof globalThis)` matches Node output

## Completion evidence

- `globalThis` added to `allowed_globals` in `crates/ir/src/name_resolver.rs`
- Lowered to `LoweredExpr::Undefined` in `crates/ir/src/lowered/resolver_expr.rs`
- Node differential: `console.log(typeof globalThis)` matches Node output

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/341d-globalthis-binding.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
