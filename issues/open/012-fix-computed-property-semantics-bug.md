# Fix computed property semantics bug

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 012
**Type**: bug
**Area**: runtime/semantics
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: `obj["key"]` computed property access currently uses `$array_get` which performs an array tag check and returns `undefined` for objects. This violates JavaScript semantics where computed property access should work on objects.

Scope:

- Fix `$property_get` to handle string key computed property access correctly.
- Ensure object tag check is used instead of array tag check for computed properties.
- Add fixtures for computed property access on objects.
- Verify Node differential test passes for computed property semantics.

Acceptance Criteria:

- [ ] `obj["key"]` returns correct value for object properties.
- [ ] Computed property access works with string literal keys.
- [ ] Node differential test passes for computed property fixtures.
- [ ] No regression in array index access `arr[n]`.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/computed-property.wasm
```
