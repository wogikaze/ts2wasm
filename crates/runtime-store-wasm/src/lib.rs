//! PropertyStore wasm-callable storage primitives.
//!
//! These functions read/write `runtime-core`'s Object layout in linear memory.
//! Storage primitives ONLY — no prototype chain, no accessor dispatch,
//! no descriptor validation, no Proxy, no ToPropertyKey.
//!
//! Layout:
//!   Object header (16 bytes):
//!     [0]: shape_id (i32)
//!     [4]: elements_kind (i32)
//!     [8]: prototype_ptr (i32) — tagged value for [[Prototype]]
//!    [12]: property_count (i32) — number of inline properties
//!   Inline properties start at offset 16:
//!     Each entry: [key_ptr:4][value_ptr:4] = 8 bytes

use ts2wasm_backend_core::wasm_ir::{WasmFunction, WasmInstr, WasmValType};
use ts2wasm_runtime_abi::value::ValueTag;

// Object header offsets
const SHAPE_ID_OFFSET: i32 = 0;
const ELEMENTS_KIND_OFFSET: i32 = 4;
const PROTOTYPE_OFFSET: i32 = 8;
const PROPERTY_COUNT_OFFSET: i32 = 12;
const INLINE_PROPS_OFFSET: i32 = 16;
const PROP_ENTRY_SIZE: i32 = 8;  // key(4) + value(4)

pub fn property_store_functions() -> Vec<WasmFunction> {
    vec![
        build_own_property_lookup(),
        build_own_property_insert(),
        build_own_property_update(),
        build_own_property_delete(),
        build_own_property_keys_raw(),
        build_get_prototype_slot(),
        build_set_prototype_slot(),
        build_is_extensible_bit(),
        build_prevent_extensions_bit(),
    ]
}

/// $own_property_lookup: (obj_ptr: i32, key_ptr: i32) -> desc_ptr_or_zero: i32
///
/// Iterates inline properties, comparing each key_ptr. Returns pointer to
/// the property entry (which IS the descriptor) if found, or 0 if not found.
fn build_own_property_lookup() -> WasmFunction {
    let count_local = 0usize;
    let i_local = 1usize;
    let entry_ptr_local = 2usize;

    WasmFunction {
        symbol: "$own_property_lookup".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32; 3], // count, i, entry_ptr
        body: vec![
            // count = *(obj_ptr + PROPERTY_COUNT_OFFSET)
            WasmInstr::LocalGet(0),           // obj_ptr
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalSet(count_local),

            // for i = 0; i < count; i++
            // entry_ptr = obj_ptr + INLINE_PROPS_OFFSET + i * PROP_ENTRY_SIZE
            WasmInstr::I32Const(0),
            WasmInstr::LocalSet(i_local),

            // loop header
            WasmInstr::Loop("lookup_loop".into()),
            // if i >= count: break (return 0 — not found)
            WasmInstr::LocalGet(i_local),
            WasmInstr::LocalGet(count_local),
            WasmInstr::I32GeS,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            WasmInstr::I32Const(0),
            WasmInstr::Return,
            WasmInstr::End,

            // entry_ptr = obj_ptr + INLINE_PROPS_OFFSET + i * PROP_ENTRY_SIZE
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(i_local),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::LocalSet(entry_ptr_local),

            // if *(entry_ptr) == key_ptr: found, return entry_ptr
            WasmInstr::LocalGet(entry_ptr_local),
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalGet(1),  // key_ptr arg
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            WasmInstr::LocalGet(entry_ptr_local),
            WasmInstr::Return,
            WasmInstr::End,

            // i++
            WasmInstr::LocalGet(i_local),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::LocalSet(i_local),
            WasmInstr::Br("lookup_loop".into()),
            WasmInstr::End,
        ],
    }
}

/// $own_property_insert: (obj_ptr: i32, key_ptr: i32, desc_ptr: i32)
///
/// Appends a new property entry at inline_properties[property_count].
/// Caller must validate that the property does not exist and object is extensible.
fn build_own_property_insert() -> WasmFunction {
    WasmFunction {
        symbol: "$own_property_insert".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![],
        locals: vec![WasmValType::I32; 1], // count
        body: vec![
            // count = *(obj_ptr + PROPERTY_COUNT_OFFSET)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalSet(0),

            // entry_ptr = obj_ptr + INLINE_PROPS_OFFSET + count * PROP_ENTRY_SIZE
            // *(entry_ptr + 0) = key_ptr
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(0),  // count
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),  // key_ptr arg
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },

            // *(entry_ptr + 4) = desc_ptr
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::I32Const(4),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(2),  // desc_ptr arg
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },

            // property_count++
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },

            WasmInstr::End,
        ],
    }
}

/// $own_property_update: (obj_ptr: i32, key_ptr: i32, desc_ptr: i32)
///
/// Updates the value of an existing property entry.
/// Caller must validate that the property exists and writable/configurable.
fn build_own_property_update() -> WasmFunction {
    WasmFunction {
        symbol: "$own_property_update".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![],
        locals: vec![WasmValType::I32; 2], // count, i
        body: vec![
            // count = *(obj_ptr + PROPERTY_COUNT_OFFSET)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalSet(0),

            // for i = 0; i < count; i++
            WasmInstr::I32Const(0),
            WasmInstr::LocalSet(1),
            WasmInstr::Loop("update_loop".into()),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(0),
            WasmInstr::I32GeS,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            WasmInstr::Return,  // not found → return (no update needed)
            WasmInstr::End,

            // entry_ptr = obj_ptr + INLINE_PROPS_OFFSET + i * PROP_ENTRY_SIZE
            // if *(entry_ptr) == key_ptr: update value at *(entry_ptr + 4)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,

            // check key match
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalGet(1), // key_ptr arg
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },

            // update value: *(entry_ptr + 4) = desc_ptr (arg 2)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::I32Const(4),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(2),
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },

            WasmInstr::Return,  // done → return
            WasmInstr::End,

            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::LocalSet(1),
            WasmInstr::Br("update_loop".into()),
            WasmInstr::End,
        ],
    }
}

/// $own_property_delete: (obj_ptr: i32, key_ptr: i32) -> success: i32
fn build_own_property_delete() -> WasmFunction {
    WasmFunction {
        symbol: "$own_property_delete".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32; 3], // count, i, entry_ptr
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalSet(0),

            WasmInstr::I32Const(0),
            WasmInstr::LocalSet(1),
            WasmInstr::Loop("delete_loop".into()),
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(0),
            WasmInstr::I32GeS,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            WasmInstr::I32Const(0),  // return 0 (not found)
            WasmInstr::Return,
            WasmInstr::End,

            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::LocalSet(2),

            WasmInstr::LocalGet(2),
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalGet(1),
            WasmInstr::I32Eq,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            // Found: shift remaining properties down
            // (simplified: just decrement count and return 1)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(1),
            WasmInstr::I32Sub,
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },
            WasmInstr::I32Const(1),
            WasmInstr::Return,
            WasmInstr::End,

            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::LocalSet(1),
            WasmInstr::Br("delete_loop".into()),
            WasmInstr::End,
        ],
    }
}

/// $own_property_keys_raw: (obj_ptr: i32) -> keys_array_ptr: i32
/// Returns a heap-allocated array of key pointers, or null (0) if empty.
fn build_own_property_keys_raw() -> WasmFunction {
    WasmFunction {
        symbol: "$own_property_keys_raw".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![WasmValType::I32; 3], // count, i, arr_ptr
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROPERTY_COUNT_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::LocalTee(0),

            // if count == 0: return null
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            WasmInstr::I32Const(ValueTag::NULL),
            WasmInstr::Return,
            WasmInstr::End,

            // Allocate array: count * 4 bytes for pointers
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(4),
            WasmInstr::I32Mul,
            WasmInstr::Call("$heap_alloc".into()),
            WasmInstr::LocalSet(1),

            // Copy key pointers
            WasmInstr::I32Const(0),
            WasmInstr::LocalSet(2),
            WasmInstr::Loop("keys_loop".into()),
            WasmInstr::LocalGet(2),
            WasmInstr::LocalGet(0),
            WasmInstr::I32GeS,
            WasmInstr::If { result_ty: ts2wasm_backend_core::wasm_ir::WasmBlockType::Empty },
            // Return array pointer with ARRAY tag
            WasmInstr::LocalGet(1),
            WasmInstr::I32Const(ValueTag::ARRAY as i32),
            WasmInstr::I32Or,
            WasmInstr::Return,
            WasmInstr::End,

            // arr_ptr[2*4] = inline_props[2].key
            WasmInstr::LocalGet(1),
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(4),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(INLINE_PROPS_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(PROP_ENTRY_SIZE),
            WasmInstr::I32Mul,
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },

            WasmInstr::LocalGet(2),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::LocalSet(2),
            WasmInstr::Br("keys_loop".into()),
            WasmInstr::End,
        ],
    }
}

/// $get_prototype_slot: (obj_ptr: i32) -> proto_tagged: i32
fn build_get_prototype_slot() -> WasmFunction {
    WasmFunction {
        symbol: "$get_prototype_slot".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Return *(obj_ptr + PROTOTYPE_OFFSET)
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROTOTYPE_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            WasmInstr::End,
        ],
    }
}

/// $set_prototype_slot: (obj_ptr: i32, proto_ptr: i32) -> success: i32
fn build_set_prototype_slot() -> WasmFunction {
    WasmFunction {
        symbol: "$set_prototype_slot".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // *(obj_ptr + PROTOTYPE_OFFSET) = proto_ptr
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(PROTOTYPE_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::LocalGet(1),
            WasmInstr::I32Store { offset: 0u32, align: 2u32 },
            WasmInstr::I32Const(1),  // success
            WasmInstr::End,
        ],
    }
}

/// $is_extensible_bit: (obj_ptr: i32) -> 1 if extensible, 0 if not
fn build_is_extensible_bit() -> WasmFunction {
    WasmFunction {
        symbol: "$is_extensible_bit".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // Read shape_id from header
            WasmInstr::LocalGet(0),
            WasmInstr::I32Const(SHAPE_ID_OFFSET),
            WasmInstr::I32Add,
            WasmInstr::I32Load { offset: 0u32, align: 2u32 },
            // STUB(scaffold): shape table not yet accessible from wasm
            // For now, return 1 (extensible)
            WasmInstr::Drop,
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

/// $prevent_extensions_bit: (obj_ptr: i32) -> success: i32
fn build_prevent_extensions_bit() -> WasmFunction {
    WasmFunction {
        symbol: "$prevent_extensions_bit".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            // STUB(scaffold): need to update shape's capability_flags
            // through shape table. For now, return 1 (no-op).
            WasmInstr::I32Const(1),
            WasmInstr::End,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_store_has_9_functions() {
        let fns = property_store_functions();
        assert_eq!(fns.len(), 9);
    }

    #[test]
    fn each_function_has_unique_symbol() {
        let fns = property_store_functions();
        let mut symbols: Vec<&str> = fns.iter().map(|f| f.symbol.as_str()).collect();
        symbols.sort();
        symbols.dedup();
        assert_eq!(symbols.len(), fns.len());
    }

    #[test]
    fn lookup_has_correct_signature() {
        let f = build_own_property_lookup();
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.results.len(), 1);
    }

    #[test]
    fn insert_has_correct_signature() {
        let f = build_own_property_insert();
        assert_eq!(f.params.len(), 3);
        assert_eq!(f.results.len(), 0);
    }

    #[test]
    fn get_prototype_has_correct_signature() {
        let f = build_get_prototype_slot();
        assert_eq!(f.params.len(), 1);
        assert_eq!(f.results.len(), 1);
    }
}
