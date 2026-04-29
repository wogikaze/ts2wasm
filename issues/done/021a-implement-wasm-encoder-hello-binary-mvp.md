---
id: 021a
title: "Implement wasm-encoder hello binary MVP"
type: feature
area: backend
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: The wasm backend needs a first direct binary emission slice for a WASI stdout fixture; WAT and wasm binary are equivalent backend outputs, and WASI `fd_write` support is an accepted prerequisite.

## Summary

Add the smallest direct wasm binary emission path that can produce a runnable WASI stdout module for one existing fixture without replacing the WAT path.

ADR input:

- WAT and wasm binary are equivalent backend outputs; neither changes observable semantics.
- WASI is an accepted target prerequisite.
- `fd_write` support is part of this slice if required for the selected stdout fixture.

## Scope

In scope:

- [x] Add a binary-emission entry point using the existing runtime ABI and linker plan.
- [x] Emit the WASI imports/sections needed for stdout through `fd_write`.
- [x] Support one known stdout fixture that already works through the WAT path.
- [x] Add a parity test proving the direct wasm output matches the WAT path output for that fixture.

Out of scope:

- Full backend replacement.
- Wasm GC.
- Broad fixture coverage.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/src/`
- `crates/cli/tests/`

Do not touch:

- `crates/frontend/src/`
- `crates/runtime-abi/src/`

## Acceptance criteria

- [x] A direct `.wasm` binary can be emitted for the selected fixture.
- [x] The emitted binary imports/uses WASI `fd_write` consistently with the manifest/runtime link plan.
- [x] The emitted binary runs under `iwasm` and prints the same stdout as the WAT path.
- [x] Node/iwasm or WAT/binary parity is covered by a focused regression test.
- [x] Issue 021 remains open as the backend epic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli --test m2_node_diff
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/ts2wasm-021a-hello.wasm
iwasm /tmp/ts2wasm-021a-hello.wasm
```

Not run:

- none

## Completion evidence

Completed on 2026-04-29 by child-021a.

Commits:

- `09af1cf` issue-021a: add hello wasm binary MVP
- `47412e6` issue-021a: close wasm binary MVP

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-backend-wasm
result: pass (21 tests)
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff
result: pass (62 tests)
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/ts2wasm-021a-hello.wasm
result: pass
date: 2026-04-29

command: iwasm /tmp/ts2wasm-021a-hello.wasm
result: pass; stdout `hi`
date: 2026-04-29

command: cargo nextest run
result: pass (417 tests, 4 skipped)
date: 2026-04-29

command: mise run check issues
result: fail; pre-existing missing reports/runs references in issues/open/052-implement-json.md and issues/done/228-implement-logical-assignment-operators.md, unrelated to issue 021a files
date: 2026-04-29
```

Remaining risks:

- `ts2wasm build` still uses the existing WAT-to-`wat2wasm` pipeline because `crates/compiler/src/` was outside this child assignment. The direct binary emitter is exposed by `ts2wasm-backend-wasm::emit_wasm_binary_mvp` and covered by backend WAT/direct parity tests for `fixtures/basics-hello/hello.ts`.
