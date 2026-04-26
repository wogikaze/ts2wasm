# Implement dynamic property key support

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 014
**Type**: feature
**Area**: runtime/semantics
**Priority**: P1
**Depends on**: 012
**Orchestration class**: implementation-ready

Problem: Dynamic property keys (e.g., `obj[variable]`) are not implemented. Currently diagnosed as `unsupported-dynamic-property`.

Scope:

- Implement lowering for dynamic property access with non-literal keys.
- Extend `$property_get` and `$property_set` to handle runtime string keys.
- Add fixtures for dynamic property access patterns.
- Ensure Node differential test passes.

Acceptance Criteria:

- [x] Dynamic property access `obj[key]` works with string variables.
- [x] Dynamic property set `obj[key] = value` works correctly.
- [x] Node differential test passes for dynamic property fixtures.
- [x] Diagnostic `unsupported-dynamic-property` is removed for supported cases.

Evidence:

- `fixtures/arrays-objects/dynamic-property.ts` compiles and runs correctly
- All 6 differential tests pass (m2_node_diff test suite)
- No `unsupported-dynamic-property` diagnostic exists (never implemented)
- Array indexing fixed to use `ArrayGet` instead of `PropertyGetDynamic`

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/dynamic-property.wasm
```
