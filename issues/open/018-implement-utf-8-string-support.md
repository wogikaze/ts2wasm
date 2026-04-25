# Implement UTF-8 string support

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 018
**Type**: feature
**Area**: runtime/semantics
**Priority**: P1
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: Non-ASCII string literals are intentionally unsupported. UTF-8 support is incomplete. docs/04 specifies UTF-8 decode/encode functions in runtime ABI.

Scope:

- Implement UTF-8 string encoding/decoding in runtime.
- Remove ASCII-only restriction from string literals.
- Implement `utf8_decode` and `utf8_encode` runtime functions.
- Add fixtures for UTF-8 string operations.
- Verify Node differential test passes for UTF-8 strings.

Acceptance Criteria:

- [ ] UTF-8 string literals are parsed and lowered correctly.
- [ ] `utf8_decode` and `utf8_encode` functions are implemented.
- [ ] Node differential test passes for UTF-8 string fixtures.
- [ ] Diagnostic for non-ASCII strings is removed.
- [ ] WASI I/O works with UTF-8 strings.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/basics-utf8/utf8-string.wasm
```
