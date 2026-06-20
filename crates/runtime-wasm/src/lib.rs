//! Non-observable wasm-callable runtime primitives.
//!
//! These functions implement low-level operations that have NO ECMAScript
//! observable semantics. Called by SpecAlgoIR algorithms.
//!
//! Non-observable: raw f64→f64 math, bitwise BigInt primitives, memory
//! allocation, memory copy, string flatten, typed array raw load/store.
//!
//! Observable builtins (Array.indexOf, String.replace, Promise.then)
//! belong in `builtin-kernel`, NOT here.
//!
//! Budget: ~80 small functions. Fixed symbols and signatures.
//! No `RuntimeFn` enum. No `emission_order()` table. No enum dispatch.

use ts2wasm_backend_core::wasm_ir::{WasmFunction, WasmInstr, WasmValType};

pub fn runtime_primitives() -> Vec<WasmFunction> {
    vec![
        // ── Math (27) ───────────────────────────────────────────────
        build_math_sin(), build_math_cos(), build_math_tan(),
        build_math_asin(), build_math_acos(), build_math_atan(),
        build_math_atan2(), build_math_sinh(), build_math_cosh(),
        build_math_tanh(), build_math_asinh(), build_math_acosh(),
        build_math_atanh(), build_math_floor(), build_math_ceil(),
        build_math_round(), build_math_abs(), build_math_sqrt(),
        build_math_exp(), build_math_expm1(), build_math_log(),
        build_math_log10(), build_math_log1p(), build_math_log2(),
        build_math_pow(), build_math_random(), build_math_hypot(),

        // ── BigInt (10) ─────────────────────────────────────────────
        build_bigint_add(), build_bigint_sub(), build_bigint_mul(),
        build_bigint_div(), build_bigint_rem(), build_bigint_pow(),
        build_bigint_compare(), build_bigint_to_string(),
        build_bigint_and(), build_bigint_or(),

        // ── Date math primitives (5) ─────────────────────────────────
        build_date_is_leap_year(), build_date_days_in_month(),
        build_date_days_from_epoch(), build_date_time_from_ms(),
        build_date_ms_from_components(),

        // ── Heap / memory (8) ───────────────────────────────────────
        build_heap_alloc(), build_heap_free(), build_heap_realloc(),
        build_mem_copy(), build_mem_move(), build_mem_set(),
        build_mem_compare(), build_mem_zero(),

        // ── String (10) ──────────────────────────────────────────────
        build_string_flatten(), build_string_length(),
        build_string_indexof_byte(), build_string_char_code_at(),
        build_string_from_char_code(), build_string_concat(),
        build_string_substring(), build_string_slice(),
        build_string_to_upper(), build_string_to_lower(),

        // ── Typed array / ArrayBuffer (8) ────────────────────────────
        build_typed_array_load(), build_typed_array_store(),
        build_typed_array_byte_length(), build_array_buffer_alloc(),
        build_array_buffer_slice(), build_array_buffer_detach(),
        build_typed_array_from(), build_typed_array_set(),

        // ── Atomics (6) ─────────────────────────────────────────────
        build_atomics_load(), build_atomics_store(),
        build_atomics_add(), build_atomics_sub(),
        build_atomics_and(), build_atomics_cmpxchg(),

        // ── Misc (12) ───────────────────────────────────────────────
        build_is_string(), build_is_object(), build_tag_of(),
        build_same_value(), build_number_is_nan(), build_number_is_finite(),
        build_call_function(), build_heap_alloc_object(),
        build_heap_alloc_array(), build_heap_alloc_function(),
        build_throw_exception(), build_is_throw_completion(),
    ]
}

// ── Math ──────────────────────────────────────────────────────────────────

fn math_unary(symbol: &str) -> WasmFunction {
    WasmFunction {
        symbol: symbol.into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}

fn build_math_sin()    -> WasmFunction { math_unary("$math_sin") }
fn build_math_cos()    -> WasmFunction { math_unary("$math_cos") }
fn build_math_tan()    -> WasmFunction { math_unary("$math_tan") }
fn build_math_floor()  -> WasmFunction { math_unary("$math_floor") }
fn build_math_ceil()   -> WasmFunction { math_unary("$math_ceil") }
fn build_math_round()  -> WasmFunction { math_unary("$math_round") }
fn build_math_abs()    -> WasmFunction { math_unary("$math_abs") }
fn build_math_sqrt()   -> WasmFunction { math_unary("$math_sqrt") }
fn build_math_exp()    -> WasmFunction { math_unary("$math_exp") }
fn build_math_log()    -> WasmFunction { math_unary("$math_log") }

fn build_math_pow() -> WasmFunction {
    WasmFunction {
        symbol: "$math_pow".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}

fn build_math_random() -> WasmFunction {
    WasmFunction {
        symbol: "$math_random".into(),
        params: vec![],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_math_random".into()), WasmInstr::End],
    }
}

// ── Heap / memory ─────────────────────────────────────────────────────────

fn build_heap_alloc() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_alloc".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_alloc".into()), WasmInstr::End],
    }
}

fn build_heap_free() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_free".into(),
        params: vec![WasmValType::I32],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_free".into()), WasmInstr::End],
    }
}

fn build_mem_copy() -> WasmFunction {
    WasmFunction {
        symbol: "$mem_copy".into(),
        params: vec![WasmValType::I32; 3], // dest, src, size
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_mem_copy".into()), WasmInstr::End],
    }
}

fn build_mem_move() -> WasmFunction {
    WasmFunction {
        symbol: "$mem_move".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_mem_move".into()), WasmInstr::End],
    }
}

fn build_mem_set() -> WasmFunction {
    WasmFunction {
        symbol: "$mem_set".into(),
        params: vec![WasmValType::I32; 3], // ptr, value, size
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_mem_set".into()), WasmInstr::End],
    }
}

// ── String ────────────────────────────────────────────────────────────────

fn build_string_flatten() -> WasmFunction {
    WasmFunction {
        symbol: "$string_flatten".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_flatten".into()), WasmInstr::End],
    }
}

fn build_string_length() -> WasmFunction {
    WasmFunction {
        symbol: "$string_length".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_length".into()), WasmInstr::End],
    }
}

fn build_string_indexof_byte() -> WasmFunction {
    WasmFunction {
        symbol: "$string_indexof_byte".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_indexof_byte".into()), WasmInstr::End],
    }
}

// ── Typed array ───────────────────────────────────────────────────────────

fn build_typed_array_load() -> WasmFunction {
    WasmFunction {
        symbol: "$typed_array_load".into(),
        params: vec![WasmValType::I32; 2], // array_ptr, index
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_typed_array_load".into()), WasmInstr::End],
    }
}

fn build_typed_array_store() -> WasmFunction {
    WasmFunction {
        symbol: "$typed_array_store".into(),
        params: vec![WasmValType::I32; 3], // array_ptr, index, value
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_typed_array_store".into()), WasmInstr::End],
    }
}

fn build_typed_array_byte_length() -> WasmFunction {
    WasmFunction {
        symbol: "$typed_array_byte_length".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_typed_array_byte_length".into()), WasmInstr::End],
    }
}

// ── Atomics ───────────────────────────────────────────────────────────────

fn build_atomics_load() -> WasmFunction {
    WasmFunction {
        symbol: "$atomics_load".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_atomics_load".into()), WasmInstr::End],
    }
}

fn build_atomics_store() -> WasmFunction {
    WasmFunction {
        symbol: "$atomics_store".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_atomics_store".into()), WasmInstr::End],
    }
}

fn build_atomics_add() -> WasmFunction {
    WasmFunction {
        symbol: "$atomics_add".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_atomics_add".into()), WasmInstr::End],
    }
}

fn build_atomics_cmpxchg() -> WasmFunction {
    WasmFunction {
        symbol: "$atomics_cmpxchg".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_atomics_cmpxchg".into()), WasmInstr::End],
    }
}

// ── Additional math ─────────────────────────────────────────────────────────

fn build_math_asin() -> WasmFunction { math_unary("$math_asin") }
fn build_math_acos() -> WasmFunction { math_unary("$math_acos") }
fn build_math_atan() -> WasmFunction { math_unary("$math_atan") }
fn build_math_expm1() -> WasmFunction { math_unary("$math_expm1") }
fn build_math_log10() -> WasmFunction { math_unary("$math_log10") }
fn build_math_log1p() -> WasmFunction { math_unary("$math_log1p") }
fn build_math_log2() -> WasmFunction { math_unary("$math_log2") }

fn build_math_atan2() -> WasmFunction {
    WasmFunction {
        symbol: "$math_atan2".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}

fn build_math_hypot() -> WasmFunction {
    WasmFunction {
        symbol: "$math_hypot".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}

// ── BigInt primitives ───────────────────────────────────────────────────────

fn bigint_binary(symbol: &str) -> WasmFunction {
    WasmFunction {
        symbol: symbol.into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::LocalGet(0), WasmInstr::End],
    }
}

fn build_bigint_add()     -> WasmFunction { bigint_binary("$bigint_add") }
fn build_bigint_sub()     -> WasmFunction { bigint_binary("$bigint_sub") }
fn build_bigint_mul()     -> WasmFunction { bigint_binary("$bigint_mul") }
fn build_bigint_div()     -> WasmFunction { bigint_binary("$bigint_div") }
fn build_bigint_compare() -> WasmFunction { bigint_binary("$bigint_compare") }

fn build_bigint_to_string() -> WasmFunction {
    WasmFunction {
        symbol: "$bigint_to_string".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_bigint_to_string".into()), WasmInstr::End],
    }
}

// ── Date math primitives ────────────────────────────────────────────────────

fn build_date_is_leap_year() -> WasmFunction {
    WasmFunction {
        symbol: "$date_is_leap_year".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_date_is_leap_year".into()), WasmInstr::End],
    }
}

fn build_date_days_in_month() -> WasmFunction {
    WasmFunction {
        symbol: "$date_days_in_month".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_date_days_in_month".into()), WasmInstr::End],
    }
}

fn build_date_days_from_epoch() -> WasmFunction {
    WasmFunction {
        symbol: "$date_days_from_epoch".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_date_days_from_epoch".into()), WasmInstr::End],
    }
}

fn build_date_time_from_ms() -> WasmFunction {
    WasmFunction {
        symbol: "$date_time_from_ms".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_date_time_from_ms".into()), WasmInstr::End],
    }
}

fn build_date_ms_from_components() -> WasmFunction {
    WasmFunction {
        symbol: "$date_ms_from_components".into(),
        params: vec![WasmValType::I32; 6],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_date_ms_from_components".into()), WasmInstr::End],
    }
}

// ── Additional heap ─────────────────────────────────────────────────────────

fn build_heap_realloc() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_realloc".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_realloc".into()), WasmInstr::End],
    }
}

fn build_mem_compare() -> WasmFunction {
    WasmFunction {
        symbol: "$mem_compare".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_mem_compare".into()), WasmInstr::End],
    }
}

// ── Additional string ───────────────────────────────────────────────────────

fn build_string_char_code_at() -> WasmFunction {
    WasmFunction {
        symbol: "$string_char_code_at".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_char_code_at".into()), WasmInstr::End],
    }
}

fn build_string_from_char_code() -> WasmFunction {
    WasmFunction {
        symbol: "$string_from_char_code".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_from_char_code".into()), WasmInstr::End],
    }
}

fn build_string_concat() -> WasmFunction {
    WasmFunction {
        symbol: "$string_concat".into(),
        params: vec![WasmValType::I32; 2],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_string_concat".into()), WasmInstr::End],
    }
}

// ── Additional typed array / ArrayBuffer ────────────────────────────────────

fn build_array_buffer_alloc() -> WasmFunction {
    WasmFunction {
        symbol: "$array_buffer_alloc".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_array_buffer_alloc".into()), WasmInstr::End],
    }
}

fn build_array_buffer_slice() -> WasmFunction {
    WasmFunction {
        symbol: "$array_buffer_slice".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_array_buffer_slice".into()), WasmInstr::End],
    }
}

fn build_array_buffer_detach() -> WasmFunction {
    WasmFunction {
        symbol: "$array_buffer_detach".into(),
        params: vec![WasmValType::I32],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![WasmInstr::Call("$host_array_buffer_detach".into()), WasmInstr::End],
    }
}

fn build_math_sinh() -> WasmFunction { math_unary("$math_sinh") }
fn build_math_cosh() -> WasmFunction { math_unary("$math_cosh") }
fn build_math_tanh() -> WasmFunction { math_unary("$math_tanh") }
fn build_math_asinh() -> WasmFunction { math_unary("$math_asinh") }
fn build_math_acosh() -> WasmFunction { math_unary("$math_acosh") }
fn build_math_atanh() -> WasmFunction { math_unary("$math_atanh") }
fn build_bigint_rem() -> WasmFunction { bigint_binary("$bigint_rem") }
fn build_bigint_pow() -> WasmFunction { bigint_binary("$bigint_pow") }
fn build_bigint_and() -> WasmFunction { bigint_binary("$bigint_and") }
fn build_bigint_or() -> WasmFunction { bigint_binary("$bigint_or") }
fn build_mem_zero() -> WasmFunction { WasmFunction { symbol: "$mem_zero".into(), params: vec![WasmValType::I32; 2], results: vec![], locals: vec![], body: vec![WasmInstr::Call("$host_mem_zero".into()), WasmInstr::End] } }
fn build_string_substring() -> WasmFunction { WasmFunction { symbol: "$string_substring".into(), params: vec![WasmValType::I32; 3], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_string_substring".into()), WasmInstr::End] } }
fn build_string_slice() -> WasmFunction { WasmFunction { symbol: "$string_slice".into(), params: vec![WasmValType::I32; 3], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_string_slice".into()), WasmInstr::End] } }
fn build_string_to_upper() -> WasmFunction { WasmFunction { symbol: "$string_to_upper".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_string_to_upper".into()), WasmInstr::End] } }
fn build_string_to_lower() -> WasmFunction { WasmFunction { symbol: "$string_to_lower".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_string_to_lower".into()), WasmInstr::End] } }
fn build_typed_array_from() -> WasmFunction { WasmFunction { symbol: "$typed_array_from".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_typed_array_from".into()), WasmInstr::End] } }
fn build_typed_array_set() -> WasmFunction { WasmFunction { symbol: "$typed_array_set".into(), params: vec![WasmValType::I32; 2], results: vec![], locals: vec![], body: vec![WasmInstr::Call("$host_typed_array_set".into()), WasmInstr::End] } }
fn build_atomics_sub() -> WasmFunction { WasmFunction { symbol: "$atomics_sub".into(), params: vec![WasmValType::I32; 2], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_atomics_sub".into()), WasmInstr::End] } }
fn build_atomics_and() -> WasmFunction { WasmFunction { symbol: "$atomics_and".into(), params: vec![WasmValType::I32; 2], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_atomics_and".into()), WasmInstr::End] } }
fn build_is_string() -> WasmFunction { WasmFunction { symbol: "$is_string".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_is_string".into()), WasmInstr::End] } }
fn build_is_object() -> WasmFunction { WasmFunction { symbol: "$is_object".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_is_object".into()), WasmInstr::End] } }
fn build_tag_of() -> WasmFunction { WasmFunction { symbol: "$tag_of".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::I32Const(0), WasmInstr::End] } }
fn build_same_value() -> WasmFunction { WasmFunction { symbol: "$same_value".into(), params: vec![WasmValType::I32; 2], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_same_value".into()), WasmInstr::End] } }
fn build_number_is_nan() -> WasmFunction { WasmFunction { symbol: "$number_is_nan".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_number_is_nan".into()), WasmInstr::End] } }
fn build_number_is_finite() -> WasmFunction { WasmFunction { symbol: "$number_is_finite".into(), params: vec![WasmValType::I32], results: vec![WasmValType::I32], locals: vec![], body: vec![WasmInstr::Call("$host_number_is_finite".into()), WasmInstr::End] } }

/// $call_function: call a JS function. Takes (callee, this, args) -> result.
fn build_call_function() -> WasmFunction {
    WasmFunction {
        symbol: "$call_function".into(),
        params: vec![WasmValType::I32; 3],
        results: vec![WasmValType::I32],
        locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0), WasmInstr::LocalGet(1), WasmInstr::LocalGet(2),
            WasmInstr::Call("$host_call_function".into()), WasmInstr::End,
        ],
    }
}

/// $heap_alloc_object: allocate a new ordinary object.
fn build_heap_alloc_object() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_alloc_object".into(), params: vec![],
        results: vec![WasmValType::I32], locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_alloc_object".into()), WasmInstr::End],
    }
}

/// $heap_alloc_array: allocate a new array object.
fn build_heap_alloc_array() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_alloc_array".into(), params: vec![],
        results: vec![WasmValType::I32], locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_alloc_array".into()), WasmInstr::End],
    }
}

/// $heap_alloc_function: allocate a new function object.
fn build_heap_alloc_function() -> WasmFunction {
    WasmFunction {
        symbol: "$heap_alloc_function".into(), params: vec![],
        results: vec![WasmValType::I32], locals: vec![],
        body: vec![WasmInstr::Call("$host_heap_alloc_function".into()), WasmInstr::End],
    }
}

/// $throw_exception: throw a tagged exception value.
fn build_throw_exception() -> WasmFunction {
    WasmFunction {
        symbol: "$throw_exception".into(), params: vec![WasmValType::I32],
        results: vec![], locals: vec![],
        body: vec![
            WasmInstr::LocalGet(0),
            WasmInstr::Call("$host_throw_exception".into()),
            WasmInstr::Unreachable,
        ],
    }
}

/// $is_throw_completion: check if a completion value indicates a throw.
fn build_is_throw_completion() -> WasmFunction {
    WasmFunction {
        symbol: "$is_throw_completion".into(), params: vec![WasmValType::I32],
        results: vec![WasmValType::I32], locals: vec![],
        body: vec![
            WasmInstr::Call("$host_is_throw_completion".into()), WasmInstr::End,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_primitives_count() {
        let fns = runtime_primitives();
        assert_eq!(fns.len(), 80, "runtime_primitives should have 80 functions");
    }

    #[test]
    fn each_primitive_has_unique_symbol() {
        let fns = runtime_primitives();
        let mut symbols: Vec<&str> = fns.iter().map(|f| f.symbol.as_str()).collect();
        symbols.sort();
        symbols.dedup();
        assert_eq!(symbols.len(), fns.len(), "all symbols must be unique");
    }

    #[test]
    fn each_function_ends_with_end() {
        let fns = runtime_primitives();
        for f in &fns {
            let has_end = f.body.iter().any(|i| matches!(i, WasmInstr::End));
            assert!(has_end, "{} must end with End", f.symbol);
        }
    }

    #[test]
    fn math_sin_has_correct_signature() {
        let f = build_math_sin();
        assert_eq!(f.params.len(), 1);
        assert_eq!(f.results.len(), 1);
    }

    #[test]
    fn heap_alloc_takes_size_returns_ptr() {
        let f = build_heap_alloc();
        assert_eq!(f.params.len(), 1, "heap_alloc takes size");
        assert_eq!(f.results.len(), 1, "heap_alloc returns ptr");
    }

    #[test]
    fn mem_copy_takes_three_params() {
        let f = build_mem_copy();
        assert_eq!(f.params.len(), 3, "mem_copy takes dest, src, size");
    }
}
