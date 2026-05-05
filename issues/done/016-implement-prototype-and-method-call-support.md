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

Progress:

- [x] Added `[[Prototype]]` slot to object layout (OBJECT_HEADER_SIZE: 4 → 8, OBJECT_PROTOTYPE_OFFSET: 4)
- [x] Initialize prototype slot to null in object allocation
- [x] Added prototype test fixture
- [x] Prototype chain lookup in property_get (recursive call to $property_get on prototype)
- [x] Method call with this binding (class methods already pass `this` as first argument)
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/016-implement-prototype-and-method-call-support.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
