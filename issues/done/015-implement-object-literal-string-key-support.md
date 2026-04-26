# Implement object literal string key support

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
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

- [x] Parser accepts `{"key": value}` syntax.
- [x] String literal keys are correctly lowered to runtime properties.
- [x] Node differential test passes for string key fixtures.
- [x] No parse error for valid string key object literals.

Close:

- Date: 2026-04-26
- Evidence:
  - Added parse_object_key() method to accept both Token::Ident and Token::String as object keys
  - Updated object literal parsing to use parse_object_key() instead of expect_ident()
  - Test fixture fixtures/arrays-objects/string-key-literal.ts builds and runs correctly
  - iwasm output matches Node.js output ("Alice" and "30")
  - All tests pass: cargo nextest run (185 passed, 4 skipped)
  - Format check passes: cargo fmt --all --check

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/string-key-literal.wasm
```
