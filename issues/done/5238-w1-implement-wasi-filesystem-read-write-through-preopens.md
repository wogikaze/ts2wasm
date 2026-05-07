---
id: 5238
title: "W1: implement WASI filesystem read/write through preopens"
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

Implement WASI path_open/fd_read/fd_write for filesystem access through preopened directories. Currently fs.readFileSync and fs.writeFileSync use Node.js host imports. Move to WASI preopens so file I/O works in standalone WASI mode when preopened directories are provided.

## Problem

W1 (Standalone WASI execution) requires that file I/O works through WASI preopens instead of Node.js fs module. WASI provides path_open to open files under preopened directories, followed by fd_read/fd_write for data access.

## Scope

In scope:
- Add RuntimeFn for WASI path_open/fd_close/fd_read/fd_write for file paths
- Lower fs.readFileSync and fs.writeFileSync to WASI preopen path
- Ensure preopen-based file access is conditional on capabilities
- Add standalone fixture for file read/write
- Update capability manifest

Out of scope:
- Recursive directory creation
- File metadata (stat, permissions)
- Symlink handling
- Network filesystem (NFS, etc.)

## Affected paths

Expected:
- crates/backend-wasm/src/runtime_fn.rs
- crates/ir/src/builtin_resolver.rs
- crates/cli/tests/

## Acceptance criteria

- [x] fs.readFileSync through WASI preopen path for existing fixture files
- [x] fs.writeFileSync through WASI preopen path
- [x] Capability manifest shows wasi.filesystem entries
- [x] Standalone fixture reads and writes a file through preopens

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(filesystem|wasi)'
```



## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
