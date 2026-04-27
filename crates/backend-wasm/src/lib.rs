mod capability_manifest;
mod emitter;
mod expr_emit;
mod runtime_arrays_objects;
mod runtime_builder;
mod runtime_builtins_host;
mod runtime_collections;
mod runtime_core;
mod runtime_fn;
mod runtime_link_plan;
mod runtime_strings;
mod stmt_emit;
mod string_intern;
mod wat_writer;

pub use ts2wasm_frontend::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::LoweredProgram;

pub(crate) use runtime_fn::RuntimeFn;

pub fn emit_canonical_manifest_json(program: &LoweredProgram) -> String {
    capability_manifest::emit_canonical_manifest_json(program)
}

pub fn has_node_host_imports(program: &LoweredProgram) -> bool {
    let link_plan = runtime_link_plan::RuntimeLinkPlan::from_program(program);
    link_plan.required_imports().iter().any(|import| {
        let spec = import.spec();
        spec.module.contains("host") || spec.module.contains("node")
    })
}

pub fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
    if let Err(errors) = ts2wasm_ir::lowered::validate_lowered(program) {
        let first = errors.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        });
        return Err(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "refusing to emit WAT from invalid lowered IR: [{:?}] {}",
                first.code, first.message
            ),
            span: first.span,
        });
    }
    emitter::emit_wat(program)
}

pub fn program_requires_read_stdin_bytes_runtime(program: &LoweredProgram) -> bool {
    runtime_link_plan::RuntimeLinkPlan::from_program(program)
        .required_runtime_functions()
        .contains(&runtime_fn::RuntimeFn::ReadStdinBytes)
}

pub(crate) fn align_to(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}

pub(crate) fn wat_bytes(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        match byte {
            b'\n' => out.push_str("\\0a"),
            b'\r' => out.push_str("\\0d"),
            b'\t' => out.push_str("\\09"),
            b'"' => out.push_str("\\22"),
            b'\\' => out.push_str("\\5c"),
            0x20..=0x7e => out.push(*byte as char),
            _ => out.push_str(&format!("\\{byte:02x}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::emit_wat;
    use ts2wasm_frontend::DiagCode;
    use ts2wasm_ir::lowered::{LoweredExpr, LoweredProgram, LoweredStmt};

    #[test]
    fn emit_wat_rejects_residual_method_call_before_emission() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::MethodCall {
                object: Box::new(LoweredExpr::Undefined),
                method: "trim".to_owned(),
            })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let err = emit_wat(&program).expect_err("emit_wat must reject residual MethodCall");
        assert_eq!(err.code, DiagCode::InvariantViolation);
        assert!(err.message.contains("MethodCall"));
    }
}
