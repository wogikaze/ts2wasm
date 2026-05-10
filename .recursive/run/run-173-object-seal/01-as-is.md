# Phase 1: As-Is Analysis (run-173)

## Current State

`Object.seal` is **not registered** in the builtin dispatch table (`program_builtins.rs`).
When the compiler encounters `Object.seal(obj)`, `resolve_method_to_runtime_fn()` returns
`None`, and the lowerer falls through to generic property access. Since `Object` itself
is a compile-time-known global (not a runtime value), the unresolved property access
fails with `[UnresolvedName] unresolved name: 'Object'`.

## Missing: Object.seal catalog entries

Compared to `Object.preventExtensions` (which works), `Object.seal` lacks:

| File | What's missing |
|---|---|
| `crates/ir/src/lowered/program_builtins.rs` | `"seal" => Some("ObjectSeal")` |
| `crates/backend-wasm/src/runtime_fn.rs` | `RuntimeFn::ObjectSeal` variant |
| `crates/backend-wasm/src/runtime_fn_impl.rs` | `RuntimeSpec`, resolver, symbol, `emission_order()`, `all()` |
| `crates/backend-wasm/src/runtime_objects.rs` | `emit_object_seal()` function |
| `crates/backend-wasm/src/runtime_builder.rs` | routing to `emit_object_seal` |
| `fixtures/builtins-and-io/object-seal.ts` | fixture file |
| `crates/cli/tests/m6_builtin_methods.rs` | `build_smoke_object_seal` test |

## Existing Reference: Object.preventExtensions

`Object.preventExtensions` is the closest analog. Its RuntimeSpec shows the pattern:

```
RuntimeSpec {
    deps: &[RuntimeFn::IsObject, RuntimeFn::SetPropertyTag],
    imports: &[],
    capabilities: &[],
    runtime_strings: &[],
    result: RuntimeResult::Value,
}
```

Its emit function sets the `OBJECT_FLAG_SEALED` (non-extensible) bit on the object's
flags field. `Object.seal` should additionally set all own properties as non-configurable.

## Minimal first slice

For `build_smoke` only (no semantic_diff required), the emit function can reuse the
`preventExtensions` approach but also iterate properties to set configurable:false on each.

Scope for this run: add the catalog dispatch + a minimal runtime emit + fixture + test stop.
No runtime correctness beyond build_smoke is claimed.
