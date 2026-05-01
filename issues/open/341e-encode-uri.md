---
id: 341e
title: "Implement encodeURI, decodeURI, escape, unescape"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement `encodeURI`, `decodeURI`, `escape`, and `unescape` global functions.

## Problem

Legacy URI encoding/decoding functions are used in some test262 harness and test cases.

## Desired final state

`encodeURI(str)` and `decodeURI(str)` perform basic URI encoding/decoding. `escape(str)` and `unescape(str)` provide legacy escape/unescape behavior.

## Scope

- [x] `encodeURI(str)` — percent-encode special characters
- [x] `decodeURI(str)` — percent-decode
- [x] `escape(str)` — legacy %-encoding
- [x] `unescape(str)` — legacy %-decoding

## Affected paths

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`
