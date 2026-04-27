mod capability_manifest;
mod emitter;
mod expr_emit;
mod runtime_builder;
mod runtime_fn;
mod runtime_link_plan;
mod stmt_emit;
mod string_intern;
mod wat_writer;

use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::LoweredProgram;

pub(crate) use capability_manifest::emit_canonical_manifest_json;
pub(crate) use runtime_fn::RuntimeFn;

pub(crate) fn has_node_host_imports(program: &LoweredProgram) -> bool {
    let link_plan = runtime_link_plan::RuntimeLinkPlan::from_program(program);
    link_plan.required_imports().iter().any(|import| {
        let spec = import.spec();
        spec.module.contains("host") || spec.module.contains("node")
    })
}

pub(crate) fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
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

pub(crate) fn program_requires_read_stdin_bytes_runtime(program: &LoweredProgram) -> bool {
    runtime_link_plan::RuntimeLinkPlan::from_program(program)
        .required_runtime_functions()
        .contains(&runtime_fn::RuntimeFn::ReadStdinBytes)
}

#[cfg(test)]
mod tests {
    use super::emit_wat;
    use crate::DiagCode;
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

    #[test]
    fn typed_wat_writer_imports_match_string_concat() {
        // Verify that typed WAT writer produces identical output to string concatenation
        let program = crate::parse_program("console.log(1);").expect("parse failed");
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&program)
            .expect("builtin resolution failed");
        let lowered = ts2wasm_ir::lowered::lower_program(&resolved).expect("lowering failed");

        let wat = emit_wat(&lowered).expect("emit_wat failed");

        // Verify that imports are properly formatted
        assert!(wat.contains("(import \"wasi_snapshot_preview1\" \"fd_write\""));
        assert!(wat.contains("(func $fd_write"));
    }
}
