---
id: 341c
title: "Implement Boolean global"
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

Implement the `Boolean()` global function as a coercion/conversion callable.

## Problem

`Boolean(x)` is not recognized, causing test262 failures when boolean coercion is used in harness or test setup.

## Desired final state

`Boolean(x)` returns `true` for truthy values and `false` for falsy values.

## Scope

- [x] Add `Boolean` to `allowed_globals` in name resolver (already present)
- [x] `Boolean(x)` call recognized as a BuiltinCall or RuntimeCall
- [x] Returns boolean value (TRUE/FALSE tag)

## Affected paths

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`

## Acceptance criteria

- [x] `Boolean(1)` returns `true`
- [x] `Boolean(0)` returns `false`
- [x] `Boolean("")` returns `false`
- [x] `Boolean("x")` returns `true`
- [x] `Boolean(undefined)` returns `false`
- [x] `Boolean(null)` returns `false`

## Completion evidence

- All acceptance criteria verified: `Boolean(1)→true`, `Boolean(0)→false`, `Boolean("")→false`, `Boolean("x")→true`, `Boolean(undefined)→false`, `Boolean(null)→false`
- Node differential matches Node output for all inputs
- Implemented as `$boolean_coerce` WAT function in `runtime_builtins_host.rs`, dispatched via `RuntimeFn::BooleanCoerce`
- `Boolean` was already in `allowed_globals` (name_resolver.rs); added support in `builtin_resolver.rs` and lowered resolver

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/341c-boolean-global.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
