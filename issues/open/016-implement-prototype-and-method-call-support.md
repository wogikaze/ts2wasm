# Implement prototype and method call support

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 016
**Type**: feature
**Area**: runtime/semantics
**Priority**: P1
**Depends on**: 014
**Orchestration class**: implementation-ready

Problem: Prototype chain lookup and method calls are not implemented. Currently diagnosed as `unsupported-method-call` and `unsupported-prototype`.

Scope:

- Implement prototype chain in object model.
- Add `[[Prototype]]` slot to heap object layout.
- Implement method call with correct `this` binding.
- Add fixtures for prototype and method call patterns.
- Verify Node differential test passes.

Acceptance Criteria:

- [ ] Prototype chain lookup works for inherited properties.
- [ ] Method calls use correct receiver (`this` binding).
- [x] Node differential test passes for prototype fixtures.
- [ ] Diagnostics `unsupported-method-call` and `unsupported-prototype` are removed for supported cases.

Progress:

- [x] Added `[[Prototype]]` slot to object layout (OBJECT_HEADER_SIZE: 4 → 8, OBJECT_PROTOTYPE_OFFSET: 4)
- [x] Initialize prototype slot to null in object allocation
- [x] Added prototype test fixture
- [ ] Prototype chain lookup in property_get (requires Object.create or similar to set prototype)
- [ ] Method call with this binding (requires function expression support)

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/prototype.wasm
```
