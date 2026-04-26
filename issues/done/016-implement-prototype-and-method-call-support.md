# Implement prototype and method call support

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Closed**: 2026-04-27
**ID**: 016
**Type**: feature
**Area**: runtime/semantics
**Priority**: P1
**Depends on**: 014
**Orchestration class**: implementation-ready

Problem: Prototype chain lookup and method calls are not implemented. Currently diagnosed as `unsupported-method-call` and `unsupported-prototype`.

Scope:

- Implement prototype chain in object model.
- Add `[[Prototype]]` slot to heap object layout.
- Implement method call with correct `this` binding.
- Add fixtures for prototype and method call patterns.
- Verify Node differential test passes.

Acceptance Criteria:

- [x] Prototype chain lookup works for inherited properties.
- [x] Method calls use correct receiver (`this` binding).
- [x] Node differential test passes for prototype fixtures.
- [x] Diagnostics `unsupported-method-call` and `unsupported-prototype` are removed for supported cases.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/prototype.wasm
```

## Completion evidence

- 2026-04-27: `cargo fmt --all --check` passed.
- 2026-04-27: `cargo test -p ts2wasm-cli --test m2_node_diff -- --exact m3_semantic_fixtures_match_node_output_under_iwasm` passed.
- 2026-04-27: `cargo test -p ts2wasm-cli --test m2_node_diff -- --exact m5_array_object_fixtures_match_node_output_under_iwasm` passed.
- 2026-04-27: `cargo test -p ts2wasm-cli --test m8_oop_classes -- --exact build_smoke_class_extends build_smoke_class_basic build_smoke_class_static build_smoke_class_super build_smoke_class_super_method build_smoke_new_expression` passed.
- 2026-04-27: Full `cargo nextest run` passed with one environment-specific failure in `official_corpora_smoke_gate_finds_reference_shards` (missing `reference/test262` test corpus).
- 2026-04-27: `cargo run -p ts2wasm-cli -- build fixtures/core-semantics/prototype.ts -o <tmp>.wasm` + `iwasm <tmp>.wasm` outputs `11`, `18`, `10`, matching Node output.
- 2026-04-27: `cargo run -p ts2wasm-cli -- build fixtures/classes-and-inheritance/class-super-method.ts -o <tmp>.wasm` + `iwasm <tmp>.wasm` outputs `4`, matching Node output.
