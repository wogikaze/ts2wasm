# Implement full wasm backend

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 021
**Type**: feature
**Area**: backend
**Priority**: P2
**Depends on**: 008, 020
**Orchestration class**: blocked

Problem: Full wasm backend is not implemented. Current implementation is WAT-centric. docs/04 specifies initial linear memory backend with future Wasm GC support.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Execute it through child slices such as issue 021a, each with one observable backend behavior and parity validation.
- Move this issue back to an active class only when all child slices needed for the current backend milestone are closed.

Scope:

- Implement direct wasm binary emission (not just WAT).
- Use wasm-tools or similar for binary generation.
- Maintain compatibility with existing runtime ABI.
- Add typed WAT writer skeleton (issue 008) as foundation.
- Consider future Wasm GC backend path.

Acceptance Criteria:

- [ ] Direct wasm binary emission is implemented.
- [ ] Generated wasm is functionally equivalent to WAT path.
- [ ] Runtime ABI compatibility is maintained.
- [ ] Typed WAT writer skeleton is used as foundation.
- [ ] Node differential test passes for wasm backend fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm
iwasm /tmp/hello.wasm
```
