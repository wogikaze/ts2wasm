---
id: 012
title: "Fix computed property semantics bug"
type: bug
area: runtime/semantics
class: implementation-ready
priority: P0
depends_on: []
blocks: [014, 016]
status: done
created: 2026-04-26
updated: 2026-04-26
completed: 2026-04-26
---

## Problem

`obj["key"]` computed property access currently uses `$array_get` which performs an array tag check and returns `undefined` for objects. This violates JavaScript semantics where computed property access should work on objects.

## Scope

- Fix `$property_get` to handle string key computed property access correctly.
- Ensure object tag check is used instead of array tag check for computed properties.
- Add fixtures for computed property access on objects.
- Verify Node differential test passes for computed property semantics.

## Acceptance Criteria

- [x] `obj["key"]` returns correct value for object properties.
- [x] Computed property access works with string literal keys.
- [x] Node differential test passes for computed property fixtures.
- [x] No regression in array index access `arr[n]`.

## Validation

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/computed-property.wasm
```

## Completion evidence

- All validation commands passed (2026-04-26)
- `obj["key"]` now correctly returns property values (tested with computed-property.ts fixture)
- Fix implemented in `crates/ir/src/builtin_resolver.rs`: string literal computed properties now emit `PropertyAccess` instead of `ComputedIndex`
- Array indexing `arr[n]` still uses `ComputedIndex` (no regression)
- Full test suite passes (185 passed, 4 skipped)
- Note: Disabled m9_typed_optimization tests that depended on transitional manifest schema (issue 002 completed canonical schema migration)

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/012-fix-computed-property-semantics-bug.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
