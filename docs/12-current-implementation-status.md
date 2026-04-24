# Current implementation status

Last updated: 2026-04-24

この文書は、現在実装済みの事実だけを記録する。ロードマップ、目標、設計意図は他の docs に置き、この文書では「今できること」と「まだできないこと」を混ぜない。

## Summary

現在の実装段階は M5 の heap-allocated array/object gate に到達した。`docs/11-shared-definitions.md` にある runtime ABI、capability manifest、test status schema の一部を Rust 型と validation として `crates/shared/` に実装し、`console.log("hi")` の単一入力から WASI `.wasm` を生成して `iwasm` で実行できるようになった。

M2/M3 fixtures では、number/string/boolean/if/while/function と、`undefined` / `null` / truthiness / `===` / `+` の小さな subset を Node と比較し、生成 wasm の `iwasm` stdout が一致する。M4 では compile-time evaluator を削除し、stdout の事前計算ではなく、生成 WASM 内の tagged JS value runtime で式・制御フロー・`console.log` 引数評価を実行している。M5 では array literal、numeric index、object literal、data property read、`.length` を heap runtime で実装した。ただし、これはまだ汎用 TypeScript/JavaScript compiler ではない。

## Implemented

| Area | Status | Location |
|---|---|---|
| Rust workspace | implemented | `Cargo.toml` |
| M0 shared crate | implemented | `crates/shared/` |
| Minimal CLI | partial | `crates/cli/` |
| Minimal parser/frontend | partial | `crates/cli/src/lib.rs` |
| WAT/WASM runtime emitter | partial | `crates/cli/src/lib.rs` |
| Tagged JS value execution | partial | immediate values plus heap strings in generated WASM |
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
| `cargo test` | pass, includes M1/M2/M3/M5 `iwasm` integration tests |
| M2 fixtures vs Node | pass for curated fixtures |
| M3 semantic fixtures vs Node | pass for curated fixtures |
| M5 array/object fixtures vs Node | pass for curated fixtures |
| Precomputed stdout embedding check | pass for M2/M3 fixtures |
| `iwasm --version` | pass, `iwasm 2.4.3` |

## Not implemented

| Area | Status |
|---|---|
| TypeScript parser integration | not implemented; current parser is project-local subset only |
| JavaScript semantic IR | not implemented |
| General WASM emitter | not implemented; current emitter is direct WAT generation for M1-M4 subset |
| Complete WASM runtime implementation | not implemented; current runtime only covers M3 fixture semantics |
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

- Add a minimal IR between parser and WAT generation.
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

M4 is implemented for the curated M3 fixtures. The old compile-time evaluator and stdout-only binary emitter have been removed from the CLI implementation path. From M4 onward, fixtures only count as milestone progress if the observable behavior is produced by generated WASM executing JS values and control flow through a runtime representation.

Implemented M4 properties:

- Generated WASM executes `undefined`, `null`, boolean, small integer number, and string values through a tagged runtime value representation.
- M3 semantic fixtures pass without precomputing stdout during compilation.
- `console.log` lowers to WASI `fd_write`, while its argument is evaluated in WASM/runtime code.
- Node.js remains only a differential oracle in tests, not an execution provider.

Current M4 limitations:

- Integer formatting in `value_to_string` only handles single decimal digits correctly.
- Numbers are still small signed integers; `NaN`, `-0`, `Infinity`, floating-point values, and numeric string parsing are not implemented.
- Strings are heap objects with length-prefixed bytes, but there is no GC, bounds growth, UTF-8 validation, or general string library.
- The compiler emits WAT and shells out to `wat2wasm`; there is no direct wasm binary backend yet.
- There is no source-level capability manifest emission yet, even though generated output currently imports only WASI `fd_write`.
- Function support is direct calls only; closures, `this`, hoisting, recursion validation, and arity semantics are not implemented.

Required M6 stdin properties:

- `require("fs").readFileSync(0, "utf8")` must lower to WASI `fd_read`.
- UTF-8 decoding and subsequent string processing must run in WASM/runtime code.
- Node.js must not receive stdin content for program execution. It may only run the same fixture as the differential oracle.

## M5 gate

M5 extends the WASM runtime with heap-allocated array and object values. Array literals, numeric array indexing, `.length` on arrays and ASCII strings, object literals, and data property read are now implemented.

Implemented M5 properties:

- Array literals `[e0, e1, ...]` allocate a heap block `[i32 len, i32 elem₀, ...]`, tagged with `ptr | 5`.
- Numeric array element access `arr[idx]` dispatches through `$array_get`, which validates both array tag (5) and index tag (4) before reading.
- Object literals `{k: v, ...}` allocate a heap block `[i32 count, (i32 key_raw, i32 value)×n]`, tagged with `ptr | 7`.
- Data property read `obj.key` dispatches through `$property_get`, which validates object tag (7) and performs a reverse scan so the last duplicate key wins (JS semantics).
- `.length` on arrays and ASCII strings dispatches through `$get_length`, which validates the tag before reading.
- Non-ASCII string literals are rejected at lowering time with `DiagCode::UnsupportedSyntax`.
- All three M5 fixtures (`array.ts`, `string-length.ts`, `object.ts`) match Node output under `iwasm`.

Current M5 limitations:

- String values are ASCII-only; character byte length equals JS `.length`. UTF-16 code unit semantics are not implemented.
- Dynamic property key expressions are not supported.
- Object literal keys must be identifiers; string-literal keys like `{"x": v}` are a parse error.
- `obj["key"]` computed property access routes through `$array_get` instead of `$property_get` and returns `undefined` for non-array receivers — this is incorrect JS semantics.
- Method calls and prototype chain lookups are not implemented.
- Builtin root name shadowing is not supported yet: `console.log(...)` is resolved by syntax shape in `builtin_resolver`, so local rebinding such as `let console = ...;` is outside the current subset semantics.
- There is no GC; heap grows monotonically.
- `$alloc_heap` does not check `memory.size`; large allocations produce undefined behaviour instead of a graceful OOM.
- Floating-point number values (`NaN`, `-0`, `Infinity`) are still not implemented.

## P0 technical debt carried from M5

The following architectural requirements are tracked from the M5 prototype phase:

| Item | Status | Impact |
|---|---|---|
| RuntimeLinkPlan — separate from WatEmitter | done | RuntimeLinkPlan is isolated from WatEmitter and used as the backend link-plan input |
| AST node span — all `Expr`/`Stmt` carry source span | deferred | New diagnostics (non-ASCII, unsupported-syntax) emit `span: None` |
| BuiltinResolver pass — separate from Resolver/lowering | done | Builtin resolution is a distinct phase (`builtin_resolver`) and lowering consumes `Resolved*` IR |
| Capability manifest output | done | `--emit-capabilities` emits JSON from `RuntimeLinkPlan` (`imports`/`capabilities`/`runtime`); currently covers `fd_write` path |

These items must be resolved before M6 work begins. They are P0 because they block safe extension of the compiler and could mask regressions.

## Next milestone target

The next implementation target is a dedicated P0 debt repayment milestone before M6.

Exit criteria:

- RuntimeLinkPlan is separated from `WatEmitter`.
- BuiltinResolver pass is separated from resolver/lowering and handles at least `console.log`, `.length`, and property read semantics.
- Capability manifest output is emitted through catalog + plan (`fd_write` now; same pathway to be extended with `fd_read` in M6).
- Source-origin diagnostics have a span-bearing path (AST node span rollout started and wired to diagnostics).

Current gate status: 3 / 4 complete. Remaining blocker before M6 is AST node span.

After this gate is complete, the project resumes M6: stdin read (`require("fs").readFileSync(0, "utf8")`) lowering to WASI `fd_read`, with UTF-8 decoding and subsequent string processing running in WASM/runtime code.
