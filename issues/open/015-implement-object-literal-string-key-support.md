# Implement object literal string key support

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 015
**Type**: feature
**Area**: parser/semantics
**Priority**: P1
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: Object literal with string literal keys `{"x": v}` is not implemented. Currently only identifier keys `{x: v}` are supported.

Scope:

- Extend parser to accept string literal keys in object literals.
- Lower string literal keys to runtime property keys.
- Add fixtures for string key object literals.
- Verify Node differential test passes.

Acceptance Criteria:

- [ ] Parser accepts `{"key": value}` syntax.
- [ ] String literal keys are correctly lowered to runtime properties.
- [ ] Node differential test passes for string key fixtures.
- [ ] No parse error for valid string key object literals.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/string-key-literal.wasm
```
