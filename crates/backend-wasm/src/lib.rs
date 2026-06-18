mod capability_manifest;
mod native_lowered;
mod native_runtime_embed;
mod runtime;
mod runtime_fn;
pub mod runtime_link_plan;
pub mod spec_emit;
mod wasm_binary;
pub mod wasm_ir;

pub use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{BuiltinErrorConstructor, LoweredProgram, MirProgram, Validated};
use ts2wasm_shared::abi::{ABI_CUSTOM_SECTION_NAME, AbiMetadata};

pub use runtime_fn::{RuntimeFn, runtime_fn_from_name};

pub(crate) fn builtin_error_prototype_global(constructor: BuiltinErrorConstructor) -> &'static str {
    match constructor {
        BuiltinErrorConstructor::Error => "error_proto_error",
        BuiltinErrorConstructor::EvalError => "error_proto_eval_error",
        BuiltinErrorConstructor::RangeError => "error_proto_range_error",
        BuiltinErrorConstructor::ReferenceError => "error_proto_reference_error",
        BuiltinErrorConstructor::SyntaxError => "error_proto_syntax_error",
        BuiltinErrorConstructor::TypeError => "error_proto_type_error",
        BuiltinErrorConstructor::URIError => "error_proto_uri_error",
        BuiltinErrorConstructor::AggregateError => "error_proto_aggregate_error",
    }
}
pub use runtime_link_plan::{
    LinkPlanSnapshot, ValidatedRuntimeLinkPlan, build_runtime_link_plan,
    build_validated_runtime_link_plan, emit_link_plan_snapshot_json,
};

// Re-export binary helpers used by the compiler pipeline.
pub use wasm_binary::append_custom_section;

pub fn append_abi_custom_section(wasm_bytes: &[u8], abi_metadata: &AbiMetadata) -> Vec<u8> {
    append_custom_section(
        wasm_bytes,
        ABI_CUSTOM_SECTION_NAME,
        &abi_metadata.to_custom_section_payload(),
    )
}

pub fn emit_canonical_manifest_json(plan: &ValidatedRuntimeLinkPlan) -> String {
    capability_manifest::emit_canonical_manifest_json(plan.as_ref())
}

pub fn has_node_host_imports(program: &LoweredProgram) -> bool {
    let link_plan = runtime_link_plan::build_runtime_link_plan(program);
    link_plan_has_node_host_imports(&link_plan)
}

fn link_plan_has_node_host_imports(plan: &runtime_link_plan::RuntimeLinkPlan) -> bool {
    plan.required_imports()
        .iter()
        .any(|import| matches!(import.spec().abi, runtime_fn::HostAbi::NodeShim))
}

pub fn emit_mir_wasm_binary(program: &Validated<MirProgram>) -> Result<Vec<u8>, Diagnostic> {
    let lowered = LoweredProgram::from(program.program());
    let (validated, _) = Validated::new(lowered)?;
    emit_wasm_binary(&validated)
}

pub fn emit_wasm_module_native(
    program: &Validated<LoweredProgram>,
) -> Result<wasm_ir::WasmModule, Diagnostic> {
    native_lowered::emit_wasm_module_native(program)
}

pub fn emit_wasm_module_native_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<wasm_ir::WasmModule, Diagnostic> {
    native_lowered::emit_wasm_module_native_with_abi(program, abi_metadata)
}

pub fn emit_wasm_binary_native(program: &Validated<LoweredProgram>) -> Result<Vec<u8>, Diagnostic> {
    native_lowered::emit_wasm_binary_native(program)
}

pub fn emit_wasm_binary_native_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<Vec<u8>, Diagnostic> {
    native_lowered::emit_wasm_binary_native_with_abi(program, abi_metadata)
}

/// Emit a validated lowered program to WASM binary.
///
/// This is the build-facing entry point: it uses the native `WasmModule`
/// backend and reports unsupported native shapes instead of accepting a WAT
/// conversion fallback as build success.
pub fn emit_wasm_binary(program: &Validated<LoweredProgram>) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_binary_native(program)
}

pub fn emit_wasm_binary_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_binary_native_with_abi(program, abi_metadata)
}

pub fn program_requires_read_stdin_bytes_runtime(program: &LoweredProgram) -> bool {
    runtime_link_plan::build_runtime_link_plan(program)
        .required_runtime_functions()
        .contains(&runtime_fn::RuntimeFn::ReadStdinBytes)
}

pub(crate) fn align_to(value: u32, alignment: u32) -> Option<u32> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return None;
    }
    value
        .checked_add(alignment - 1)
        .map(|aligned| aligned & !(alignment - 1))
}

mod wasm_encoder_backend;

pub use wasm_encoder_backend::{WasmEncoderBackendExt, emit_wasm_module_binary};

#[cfg(test)]
mod tests {}
