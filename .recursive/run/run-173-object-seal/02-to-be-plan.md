# Phase 2: TO-BE Plan (run-173)

## Changes

### 1. `crates/ir/src/lowered/program_builtins.rs` (line ~46)

Add `"seal" => Some("ObjectSeal".to_owned())` after `"is"`.

### 2. `crates/backend-wasm/src/runtime_fn.rs` (after ObjectFreeze)

Add `/// Object.seal(obj) — sets SEALED flag + makes all props non-configurable\nObjectSeal,`

### 3. `crates/backend-wasm/src/runtime_fn_impl.rs`

- Add `"ObjectSeal" => Some(RuntimeFn::ObjectSeal)` in the resolver table
- Add `Self::ObjectSeal => "object_seal"` in symbol
- Add `Self::ObjectSeal` in `emission_order()` and `all()`
- Add `RuntimeSpec` for ObjectSeal

### 4. `crates/backend-wasm/src/runtime_objects.rs`

Add `emit_object_seal()` function that:
- Validates object tag
- Sets `OBJECT_FLAG_SEALED` bit (same as preventExtensions)
- Writes `OBJECT_FLAG_NON_CONFIGURABLE` bit per property entry

### 5. `crates/backend-wasm/src/runtime_builder.rs`

Add `RuntimeFn::ObjectSeal => self.emit_object_seal(wat)` routing.

### 6. `fixtures/builtins-and-io/object-seal.ts`

Create fixture: `Object.seal(obj); console.log(Object.isSealed(obj)); console.log(obj.x);`

### 7. `crates/cli/tests/m6_builtin_methods.rs`

Add `build_smoke_object_seal` test.

## Non-goals

- No `Object.seal` runtime correctness beyond build_smoke.
- No property-level configurable flag enforcement in the runtime.
