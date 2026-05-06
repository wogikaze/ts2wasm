---
id: 5235
title: "W1: implement WASI stdin/fd_read lowering for input"
type: feature
area: wasi
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement WASI fd_read lowering for stdin input. Currently fs.readFileSync(0, "utf8") uses a Node.js host import. Move stdin input to WASI fd_read so standalone WASI programs can read stdin without Node.js.

## Problem

W1 (Standalone WASI execution) requires that programs execute without Node.js host imports. Current stdin code path (fs.readFileSync) goes through Node host, violating standalone requirements.

The WASI fd_read syscall provides standard input reading. The compiler should lower stdin read operations to WASI fd_read instead of the Node.js host import path.

## Scope

In scope:
- Add RuntimeFn for WASI fd_read
- Lower stdin read calls to WASI fd_read in the RuntimeLinkPlan
- Ensure fd_read import is conditional (only when program reads stdin)
- Add standalone fixture that reads stdin and confirms WASI-only execution
- Update capability manifest for fd_read

Out of scope:
- Network socket I/O
- File path-based fd_read (WASI preopens)
- stderr fd_read

## Affected paths

Expected:
- crates/backend-wasm/src/runtime_fn.rs
- crates/backend-wasm/src/runtime_link_plan.rs
- crates/ir/src/builtin_resolver.rs
- crates/cli/tests/

## Acceptance criteria

- [ ] WASI fd_read is emitted instead of Node host import for stdin
- [ ] fd_read import is conditional (absent when program doesn't read stdin)
- [ ] Standalone fixture reads stdin and confirms WASI-only execution
- [ ] Capability manifest shows wasi.stdin instead of node host import

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(stdin|wasi)'
```

