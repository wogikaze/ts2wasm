# Current implementation status

Last updated: 2026-04-24

この文書は、現在実装済みの事実だけを記録する。ロードマップ、目標、設計意図は他の docs に置き、この文書では「今できること」と「まだできないこと」を混ぜない。

## Summary

現在の実装段階は M1 の最小縦切りに到達した直後である。`docs/11-shared-definitions.md` にある runtime ABI、capability manifest、test status schema の一部を Rust 型と validation として `crates/shared/` に実装し、`console.log("hi")` の単一入力から WASI `.wasm` を生成して `iwasm` で実行できるようになった。

ただし、これは汎用 TypeScript/JavaScript compiler ではない。現在の CLI は `console.log("literal")` だけを認識し、IR や runtime ABI call を経由せず、WASI `fd_write` を呼ぶ最小 `.wasm` を直接生成する。

## Implemented

| Area | Status | Location |
|---|---|---|
| Rust workspace | implemented | `Cargo.toml` |
| M0 shared crate | implemented | `crates/shared/` |
| Minimal CLI | partial | `crates/cli/` |
| Runtime ABI logical definitions | partial | `crates/shared/src/abi.rs` |
| Capability manifest model | partial | `crates/shared/src/capability.rs` |
| Test status model | partial | `crates/shared/src/test_status.rs` |
| `console.log("hi")` to WASI wasm | implemented for string literal only | `crates/cli/`, `fixtures/m1/hello.ts` |
| Repository agent guidance | implemented | `AGENTS.md`, `.agents/skills/` |

## Verified

| Check | Result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo test` | pass, includes M1 `iwasm` integration test |
| `iwasm --version` | pass, `iwasm 2.4.3` |

## Not implemented

| Area | Status |
|---|---|
| TypeScript parser integration | not implemented |
| JavaScript semantic IR | not implemented |
| General WASM emitter | not implemented |
| WASM runtime implementation | not implemented |
| General CLI | not implemented |
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

## Current M1 gaps

The current M1 path proves that a generated WASI `.wasm` can run under `iwasm`, but it bypasses several intended architecture layers.

Remaining M1 work:

- Replace the ad-hoc `console.log("literal")` recognizer with the first parser/frontend boundary.
- Introduce a minimal IR node for stdout/log output.
- Emit a capability manifest for `wasi.stdout`.
- Keep the `iwasm` integration test as the milestone gate.

## Next milestone target

The next implementation target is to harden M1 from a direct emitter demo into the intended `source -> minimal IR -> WASI wasm -> iwasm` path.
