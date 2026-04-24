# Current implementation status

Last updated: 2026-04-24

この文書は、現在実装済みの事実だけを記録する。ロードマップ、目標、設計意図は他の docs に置き、この文書では「今できること」と「まだできないこと」を混ぜない。

## Summary

現在の実装段階は M3 の最小 semantic fixture gate に到達した直後である。`docs/11-shared-definitions.md` にある runtime ABI、capability manifest、test status schema の一部を Rust 型と validation として `crates/shared/` に実装し、`console.log("hi")` の単一入力から WASI `.wasm` を生成して `iwasm` で実行できるようになった。

M2/M3 fixtures では、number/string/boolean/if/while/function と、`undefined` / `null` / truthiness / `===` / `+` の小さな subset を Node と比較し、生成 wasm の `iwasm` stdout が一致する。ただし、これは汎用 TypeScript/JavaScript compiler ではない。現在の CLI は限定 parser と compile-time evaluator で stdout を先に求め、その stdout を WASI `fd_write` する最小 `.wasm` に埋め込む。JS 意味論を WASM 上の runtime と IR で実行しているわけではない。

## Implemented

| Area | Status | Location |
|---|---|---|
| Rust workspace | implemented | `Cargo.toml` |
| M0 shared crate | implemented | `crates/shared/` |
| Minimal CLI | partial | `crates/cli/` |
| Minimal parser/frontend | partial | `crates/cli/src/lib.rs` |
| Compile-time evaluator for M2 fixtures | partial | `crates/cli/src/lib.rs` |
| Runtime ABI logical definitions | partial | `crates/shared/src/abi.rs` |
| Capability manifest model | partial | `crates/shared/src/capability.rs` |
| Test status model | partial | `crates/shared/src/test_status.rs` |
| `console.log("hi")` to WASI wasm | implemented for string literal only | `crates/cli/`, `fixtures/m1/hello.ts` |
| M2 fixture comparison | implemented for small curated fixtures | `fixtures/m2/`, `crates/cli/tests/m2_node_diff.rs` |
| M3 semantic fixture comparison | implemented for small curated fixtures | `fixtures/m3/`, `crates/cli/tests/m2_node_diff.rs` |
| Repository agent guidance | implemented | `AGENTS.md`, `.agents/skills/` |

## Verified

| Check | Result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo test` | pass, includes M1/M2/M3 `iwasm` integration tests |
| M2 fixtures vs Node | pass for curated fixtures |
| M3 semantic fixtures vs Node | pass for curated fixtures |
| `iwasm --version` | pass, `iwasm 2.4.3` |

## Not implemented

| Area | Status |
|---|---|
| TypeScript parser integration | not implemented; current parser is project-local subset only |
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

## Current M2 gaps

The current M2 gate proves that curated fixtures can match Node stdout, but the implementation is intentionally narrow.

Remaining M2 work:

- Replace compile-time stdout evaluation with a minimal IR that executes program behavior in generated wasm.
- Expand expression support beyond `+`, `-`, and `<`.
- Add `const` support or explicitly classify it as unsupported in test records.
- Add machine-readable test records for each fixture result.

## Current M3 gaps

The current M3 gate covers only a small integer/string/boolean/null/undefined subset.

Remaining M3 work:

- Add `NaN`, `-0`, `Infinity`, and floating-point number fixtures.
- Add `==` only after explicit abstract equality semantics are defined.
- Add machine-readable semantic test records.

## Runtime execution gate

M4 is the point where compile-time stdout evaluation must stop being the implementation path. From M4 onward, fixtures only count as milestone progress if the observable behavior is produced by generated WASM executing JS values and control flow through a runtime representation.

Required M4 properties:

- Generated WASM must execute at least `undefined`, `null`, boolean, integer number, and string values through a runtime value representation.
- M3 semantic fixtures must pass without precomputing stdout during compilation.
- `console.log` may still lower to WASI `fd_write`, but its argument must be evaluated in WASM/runtime code.
- Node.js must remain only a differential oracle, not an execution provider.

Required M6 stdin properties:

- `require("fs").readFileSync(0, "utf8")` must lower to WASI `fd_read`.
- UTF-8 decoding and subsequent string processing must run in WASM/runtime code.
- Node.js must not receive stdin content for program execution. It may only run the same fixture as the differential oracle.

## Next milestone target

The next implementation target is M4: replace `source -> compile-time evaluator -> stdout wasm` with `source -> minimal IR -> WASM runtime JS values -> iwasm`, while preserving the Node differential fixture gate.
