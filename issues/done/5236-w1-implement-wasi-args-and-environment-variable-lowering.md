---
id: 5236
title: "W1: implement WASI args and environment variable lowering"
type: feature
area: wasi
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement WASI args_sizes_get/args_get and environ_sizes_get/environ_get lowering for process.argv and process.env access. Currently these use Node.js host imports. Move to WASI syscalls so standalone WASI programs can access command-line arguments and environment variables without Node.js.

## Problem

W1 (Standalone WASI execution) requires programs to execute without Node.js. Current process.argv and process.env access goes through Node host imports. WASI provides args_get and environ_get syscalls for this purpose.

## Scope

In scope:
- Add RuntimeFn for WASI args_sizes_get/args_get and environ_sizes_get/environ_get
- Lower process.argv and process.env to WASI syscalls
- Add standalone fixtures for argv and env access
- Update capability manifest

Out of scope:
- Mutating process.env at runtime (write-through)
- Full POSIX environ compatibility
- Complex argument parsing

## Affected paths

Expected:
- crates/backend-wasm/src/runtime_fn.rs
- crates/ir/src/builtin_resolver.rs
- crates/cli/tests/

## Acceptance criteria

- [x] process.argv resolves through WASI instead of Node host
- [x] process.env resolves through WASI instead of Node host
- [x] Standalone fixture covers argv[0] and known env variables
- [x] Capability manifest shows WASI arg/env capability

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(argv|env|wasi)'
```



## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
