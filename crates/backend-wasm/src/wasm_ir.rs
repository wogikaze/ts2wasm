// Re-export core types from ts2wasm-backend-core for backward compatibility.
// New code should import from `ts2wasm_backend_core` directly.

pub use ts2wasm_backend_core::wasm_ir::{
    WasmDataSegment, WasmExport, WasmExportKind, WasmFunction, WasmGlobal, WasmImport, WasmInstr,
    WasmMemory, WasmModule, WasmValType,
};

use super::runtime_fn::{HostImportSpec, RuntimeFn, RuntimeSignature};

/// Create a WasmImport from a HostImportSpec.
pub fn wasm_import_from_host_spec(spec: &HostImportSpec) -> WasmImport {
    WasmImport {
        module: spec.module.to_owned(),
        name: spec.name.to_owned(),
        func_symbol: spec.wat_symbol.to_owned(),
        params: parse_catalog_type_list(spec.params),
        results: parse_catalog_type_list(spec.result),
    }
}

/// Parse a runtime-catalog-style type spec ("param i32 i32", "result i32").
fn parse_catalog_type_list(raw: &str) -> Vec<WasmValType> {
    raw.split_whitespace()
        .skip(1)
        .filter_map(|s| match s {
            "i32" => Some(WasmValType::I32),
            "i64" => Some(WasmValType::I64),
            _ => None,
        })
        .collect()
}

/// Validate that a runtime call's expected stack effect matches the
/// function's declared signature.
///
/// # Panics
///
/// Panics if `expected_params` or `expected_results` differ from
/// `runtime_fn.stack_effect()`.
pub fn check_runtime_signature(
    runtime_fn: RuntimeFn,
    expected_params: usize,
    expected_results: usize,
) {
    let sig = runtime_fn.stack_effect();
    assert_eq!(
        sig.params, expected_params,
        "RuntimeFn {:?}: expected {} params, declared {}",
        runtime_fn, expected_params, sig.params,
    );
    assert_eq!(
        sig.results, expected_results,
        "RuntimeFn {:?}: expected {} results, declared {}",
        runtime_fn, expected_results, sig.results,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_fn::{HostAbi, RuntimeFn};

    #[test]
    fn maps_host_import_spec_to_wasm_import() {
        let spec = HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_write",
            wat_symbol: "$fd_write",
            abi: HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32",
            result: "result i32",
        };
        let imp = wasm_import_from_host_spec(&spec);
        assert_eq!(imp.module, "wasi_snapshot_preview1");
        assert_eq!(imp.name, "fd_write");
        assert_eq!(imp.func_symbol, "$fd_write");
        assert_eq!(imp.params, vec![WasmValType::I32; 4]);
        assert_eq!(imp.results, vec![WasmValType::I32]);
    }

    #[test]
    fn runtime_signature_stack_effect_basics() {
        assert_eq!(
            RuntimeFn::TruthyBool.stack_effect(),
            RuntimeSignature {
                params: 1,
                results: 1
            },
        );
        assert_eq!(
            RuntimeFn::PrivateBrandTypeError.stack_effect(),
            RuntimeSignature {
                params: 0,
                results: 1
            },
        );
        assert_eq!(
            RuntimeFn::ArrayGet.stack_effect(),
            RuntimeSignature {
                params: 2,
                results: 1
            },
        );
        assert_eq!(
            RuntimeFn::PropertyGet.stack_effect(),
            RuntimeSignature {
                params: 3,
                results: 1
            },
        );
        assert_eq!(
            RuntimeFn::PropertySet.stack_effect(),
            RuntimeSignature {
                params: 4,
                results: 1
            },
        );
    }
}
