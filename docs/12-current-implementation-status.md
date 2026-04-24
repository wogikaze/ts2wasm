# Current implementation status

Last updated: 2026-04-24

この文書は、現在実装済みの事実だけを記録する。ロードマップ、目標、設計意図は他の docs に置き、この文書では「今できること」と「まだできないこと」を混ぜない。

## Summary

現在の実装段階は M0 の途中である。`docs/11-shared-definitions.md` にある runtime ABI、capability manifest、test status schema の一部を Rust 型と validation として `crates/shared/` に実装した。

まだ TypeScript/JavaScript を読み、IR に変換し、WASM を生成し、`iwasm` で実行する compiler は存在しない。M1 の `console.log("hi")` 実行は未達成である。

## Implemented

| Area | Status | Location |
|---|---|---|
| Rust workspace | implemented | `Cargo.toml` |
| M0 shared crate | implemented | `crates/shared/` |
| Runtime ABI logical definitions | partial | `crates/shared/src/abi.rs` |
| Capability manifest model | partial | `crates/shared/src/capability.rs` |
| Test status model | partial | `crates/shared/src/test_status.rs` |
| Repository agent guidance | implemented | `AGENTS.md`, `.agents/skills/` |

## Verified

| Check | Result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo test` | pass, 8 unit tests |
| `iwasm --version` | pass, `iwasm 2.4.3` |

## Not implemented

| Area | Status |
|---|---|
| TypeScript parser integration | not implemented |
| JavaScript semantic IR | not implemented |
| WASM emitter | not implemented |
| WASM runtime implementation | not implemented |
| CLI | not implemented |
| `console.log("hi")` generated WASI wasm | not implemented |
| Node differential test runner | not implemented |
| Actual capability manifest emission from source analysis | not implemented |
| test262 integration | not implemented |
| performance benchmark harness | not implemented |

## Current M0 gaps

`crates/shared/` is an initial M0 anchor, not a complete implementation of all shared definitions.

Remaining M0 work:

- Keep Rust runtime ABI definitions aligned with `docs/04-compiler-architecture-and-runtime.md`.
- Add serialization format for capability manifests and test records.
- Add schema round-trip tests once serialization exists.
- Decide whether shared definitions are internal Rust-only API or also emitted as JSON schema.

## Next milestone target

The next implementation target is M1: generate a minimal WASI `.wasm` from a single-file TS/JS input such that `console.log("hi")` runs under `iwasm`.
