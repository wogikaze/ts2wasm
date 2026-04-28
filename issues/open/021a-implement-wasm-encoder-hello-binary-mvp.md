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
---

Problem: The wasm backend epic is too broad to select directly; the first closeable slice is a minimal direct binary emission path for an existing hello-style fixture.

## Summary

Add the smallest direct wasm binary emission path that can produce a runnable module for one existing stdout fixture without replacing the WAT path.

## Scope

In scope:

- [ ] Add a binary-emission entry point using the existing runtime ABI and linker plan.
- [ ] Support one known fixture that already works through the WAT path.
- [ ] Add a parity test proving the direct wasm output matches the WAT path output for that fixture.

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

- [ ] A direct `.wasm` binary can be emitted for the selected fixture.
- [ ] The emitted binary runs under `iwasm`.
- [ ] Node/iwasm or WAT/binary parity is covered by a focused regression test.
- [ ] Issue 021 remains open as the backend epic.

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

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
