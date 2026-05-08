---
id: 5413
title: "W1: Implement WASI proc_exit and stdin edge cases"
type: feature
area: wasi
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement WASI `proc_exit` for clean program termination and add test coverage for stdin edge cases (empty stdin, large stdin, pipe stdin).

## Problem

- WASI `proc_exit` is not implemented — when a program finishes, it traps instead of exiting cleanly with an exit code.
- Stdin edge cases (empty input, large input, piped input) lack test coverage.

Problem: WASI proc_exit not wired; stdin edge cases untested.

## Current failure

```sh
# A program that calls process.exit(0) or reaches normal end traps
ts2wasm build fixtures/basics-hello/hello.ts
iwasm output.wasm
# traps instead of clean exit
```

## Desired final state

- WASI `proc_exit` is imported and called at program termination with the correct exit code.
- Standalone fixture tests cover: empty stdin, 1MB+ stdin, piped stdin.
- `m_standalone_wasi.rs` has new test functions for these cases.

## Scope

In scope:

- [ ] Wire `wasi_snapshot_preview1.proc_exit` in the WASM emitter
- [ ] Emit `proc_exit(0)` at the end of the program's `_start` function for normal termination
- [ ] Emit `proc_exit(code)` on uncaught exception/trap
- [ ] Add test fixture for empty stdin (`echo -n "" | ts2wasm build ...`)
- [ ] Add test fixture for large stdin (10KB+ piped input)
- [ ] Add test fixture for stdin pipe from another process
- [ ] Add test functions in `m_standalone_wasi.rs`

Out of scope:

- WASI args_get/sizes_get (separate issue if needed)
- WASI environ_get/sizes_get (separate issue)
- Windows-specific WASI behavior
- Browser WASI polyfill

## Affected paths

Expected:

- `crates/backend-wasm/src/emitter.rs` — add proc_exit import + call in _start
- `crates/cli/tests/m_standalone_wasi.rs` — add tests for proc_exit and stdin
- `crates/cli/tests/m1_iwasm.rs` — add basic exit code test
- `fixtures/basics-hello/exit-code.ts` — fixture for exit code
- `fixtures/basics-hello/stdin-*.ts` — fixtures for stdin edge cases

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
- `crates/backend-wasm/src/runtime_*.rs` — runtime functions out of scope

## Acceptance criteria

- [ ] `proc_exit` import appears in emitted WAT for programs that need it
- [ ] Normal program termination calls `proc_exit(0)` instead of trapping
- [ ] Uncaught exception calls `proc_exit(1)` (non-zero)
- [ ] Empty stdin produces empty output without hanging
- [ ] Large stdin (10KB+) is fully read without truncation

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m_standalone_wasi
cargo nextest run -- m1_iwasm
```

Impacted commands:

```sh
echo -n "" | ts2wasm build fixtures/basics-hello/stdin-empty.ts && iwasm output.wasm
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

- WASI Preview 1: `wasi_snapshot_preview1.proc_exit(rval: i32) -> !`
- The import declaration goes in the WAT `(import "wasi_snapshot_preview1" "proc_exit" (func ...))`
- Look at existing WASI import pattern in emitter.rs for reference (fd_write, fd_read, random_get, clock_time_get)
- In WAT: `call $wasi_proc_exit` followed by `unreachable` (since proc_exit never returns)
