mod binary_mvp;
mod capability_manifest;
mod emitter;
mod expr_emit;
mod mir_emit;
mod native_lowered;
mod runtime;
mod runtime_arrays;
mod runtime_async;
mod runtime_atomics;
mod runtime_builder;
mod runtime_builtins_host;
mod runtime_collections;
mod runtime_core;
mod runtime_dates;
mod runtime_dispatch_array;
mod runtime_dispatch_bigint;
mod runtime_dispatch_collections;
mod runtime_dispatch_core;
mod runtime_dispatch_date;
mod runtime_dispatch_host;
mod runtime_dispatch_object;
mod runtime_dispatch_string;
mod runtime_fn;
pub mod runtime_link_plan;
mod runtime_objects;
mod runtime_promise;
mod runtime_regexp;
mod runtime_strings;
mod stmt_emit;
mod string_intern;
mod wasm_binary;
pub mod wasm_ir;
pub mod wat_writer;

pub use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{LoweredProgram, MirProgram, Validated};
use ts2wasm_shared::abi::{ABI_CUSTOM_SECTION_NAME, AbiMetadata};

pub use runtime_fn::{RuntimeFn, runtime_fn_from_name};
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
    link_plan.required_imports().iter().any(|import| {
        let spec = import.spec();
        spec.module.contains("host") || spec.module.contains("node")
    })
}

pub fn emit_wat(program: &Validated<LoweredProgram>) -> Result<String, Diagnostic> {
    // Validated guarantees no fatal InvariantViolation errors.
    // Non-fatal diagnostics (UnsupportedModule etc.) produce valid WAT.
    emitter::emit_wat(program.as_ref())
}

pub fn emit_mir_wat(program: &Validated<MirProgram>) -> Result<String, Diagnostic> {
    // Feature-gated MIR emission path. Native MIR subset emission is attempted
    // first; unsupported MIR remains on the explicit compatibility fallback.
    mir_emit::emit_mir_wat_validated(program)
}

pub fn emit_mir_wasm_binary(program: &Validated<MirProgram>) -> Result<Vec<u8>, Diagnostic> {
    let wat = emit_mir_wat(program)?;
    wat::parse_str(&wat).map_err(|err| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("MIR WAT-to-binary conversion failed: {err}"),
        span: None,
        phase: None,
    })
}

pub fn emit_wasm_binary_mvp(program: &Validated<LoweredProgram>) -> Result<Vec<u8>, Diagnostic> {
    // Validated guarantees no fatal InvariantViolation errors.
    binary_mvp::emit_wasm_binary_mvp(program.as_ref())
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

/// Debug-only WAT fallback for tests and dump-style probes that intentionally
/// inspect legacy WAT coverage while native binary coverage grows.
pub fn emit_wasm_binary_with_wat_debug_fallback(
    program: &Validated<LoweredProgram>,
) -> Result<Vec<u8>, Diagnostic> {
    match emit_wasm_binary_native(program) {
        Ok(bytes) => return Ok(bytes),
        Err(err)
            if matches!(
                err.code,
                DiagCode::UnsupportedSyntax | DiagCode::UnsupportedModule
            ) => {}
        Err(err) => return Err(err),
    }

    let wat = emit_wat(program)?;
    wat::parse_str(&wat).map_err(|err| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("WAT-to-binary conversion failed: {err}"),
        span: None,
        phase: None,
    })
}

pub fn emit_wasm_binary_with_abi(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<Vec<u8>, Diagnostic> {
    emit_wasm_binary_native_with_abi(program, abi_metadata)
}

/// Debug-only WAT fallback variant that appends ABI metadata after legacy WAT
/// conversion. Build paths should use `emit_wasm_binary_with_abi`.
pub fn emit_wasm_binary_with_abi_wat_debug_fallback(
    program: &Validated<LoweredProgram>,
    abi_metadata: &AbiMetadata,
) -> Result<Vec<u8>, Diagnostic> {
    match emit_wasm_binary_native_with_abi(program, abi_metadata) {
        Ok(bytes) => return Ok(bytes),
        Err(err)
            if matches!(
                err.code,
                DiagCode::UnsupportedSyntax | DiagCode::UnsupportedModule
            ) => {}
        Err(err) => return Err(err),
    }

    emit_wasm_binary_with_wat_debug_fallback(program)
        .map(|bytes| append_abi_custom_section(&bytes, abi_metadata))
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
    use super::runtime_link_plan::build_validated_runtime_link_plan;
    use super::{
        emit_canonical_manifest_json, emit_wasm_binary, emit_wasm_binary_mvp,
        emit_wasm_binary_native, emit_wasm_binary_native_with_abi, emit_wasm_binary_with_abi,
        emit_wasm_binary_with_abi_wat_debug_fallback, emit_wasm_binary_with_wat_debug_fallback,
        emit_wasm_module_binary, emit_wasm_module_native, emit_wasm_module_native_with_abi,
        emit_wat, emitter::LocalFrame,
    };
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use ts2wasm_diagnostic::DiagCode;
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::{
        ClassPrototypeRef, ClosureRepresentation, FuncId, FunctionCallKind, LocalId,
        LoweredBinaryOp, LoweredExpr, LoweredFunction, LoweredLogicalAssignOp, LoweredProgram,
        LoweredStmt, LoweredUnaryOp, ModuleInfo, ModuleLoadKind, RuntimeFn, Validated,
    };
    use ts2wasm_runtime_abi::{Layout, ValueTag};
    use ts2wasm_shared::abi::{ABI_CUSTOM_SECTION_NAME, AbiMetadata};
    use ts2wasm_shared::test_helpers::unique_temp_dir;
    use ts2wasm_source::Span;

    fn wat_words(wat: &str) -> String {
        wat.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn wasm_custom_section_payload<'a>(
        wasm_bytes: &'a [u8],
        section_name: &str,
    ) -> Option<&'a [u8]> {
        let mut offset = 8;
        while offset < wasm_bytes.len() {
            let section_id = wasm_bytes[offset];
            offset += 1;
            let (payload_len, len_size) = read_leb128_u32(&wasm_bytes[offset..]);
            offset += len_size;
            let section_end = offset + payload_len as usize;
            if section_end > wasm_bytes.len() {
                return None;
            }
            if section_id == 0 {
                let (name_len, name_len_size) = read_leb128_u32(&wasm_bytes[offset..]);
                let name_start = offset + name_len_size;
                let name_end = name_start + name_len as usize;
                if name_end <= section_end
                    && &wasm_bytes[name_start..name_end] == section_name.as_bytes()
                {
                    return Some(&wasm_bytes[name_end..section_end]);
                }
            }
            offset = section_end;
        }
        None
    }

    fn read_leb128_u32(bytes: &[u8]) -> (u32, usize) {
        let mut result = 0u32;
        let mut shift = 0;
        for (i, byte) in bytes.iter().enumerate() {
            result |= ((byte & 0x7f) as u32) << shift;
            if byte & 0x80 == 0 {
                return (result, i + 1);
            }
            shift += 7;
        }
        (result, bytes.len())
    }

    #[test]
    fn emit_wat_rejects_residual_method_call_before_emission() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::MethodCall {
                    object: Box::new(LoweredExpr::Undefined(Span::generated("test"))),
                    method: "trim".to_owned(),
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let err = Validated::new(program).expect_err("Validated must reject residual MethodCall");
        assert_eq!(err.code, DiagCode::InvariantViolation);
        assert!(err.message.contains("MethodCall"));
    }

    #[test]
    fn emit_wat_rejects_residual_this_before_emission() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::This(Span::generated("test")),
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let err = Validated::new(program).expect_err("Validated must reject residual this");
        assert_eq!(err.code, DiagCode::InvariantViolation);
        assert!(err.message.contains("issue-211: residual `this`"));
    }

    #[test]
    fn direct_wasm_binary_mvp_rejects_non_hello_shape() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::String("hi".to_owned(), Span::generated("test")),
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let err = emit_wasm_binary_mvp(&v).expect_err("non-console.log shape is out of MVP");
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("direct wasm binary MVP"));
    }

    #[test]
    fn direct_wasm_binary_mvp_number_literal() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::Number(42, Span::generated("test"))],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (v, _) = Validated::new(program).expect("should validate");
        let direct_wasm =
            emit_wasm_binary_mvp(&v).expect("number literal should emit direct wasm binary");
        let wat = emit_wat(&v).expect("should emit WAT");
        let temp_dir = unique_temp_dir("direct-wasm-binary-mvp-number");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let direct_path = temp_dir.join("direct.wasm");
        let wat_wasm_path = temp_dir.join("wat.wasm");
        fs::write(&direct_path, &direct_wasm).expect("direct wasm should be written");
        let wat_path = temp_dir.join("out.wat");
        fs::write(&wat_path, &wat).expect("wat should be written");
        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wat_wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );
        let direct_out = run_iwasm(&direct_path);
        let wat_out = run_iwasm(&wat_wasm_path);
        assert_eq!(direct_out, "42\n");
        assert_eq!(direct_out, wat_out);
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn direct_wasm_binary_mvp_local_variable() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Number(42, Span::generated("test")),
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(0), Span::generated("test"))],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };
        let (v, _) = Validated::new(program).expect("should validate");
        let direct_wasm =
            emit_wasm_binary_mvp(&v).expect("local variable should emit direct wasm binary");
        let wat = emit_wat(&v).expect("should emit WAT");
        let temp_dir = unique_temp_dir("direct-wasm-binary-mvp-local");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let direct_path = temp_dir.join("direct.wasm");
        let wat_wasm_path = temp_dir.join("wat.wasm");
        fs::write(&direct_path, &direct_wasm).expect("direct wasm should be written");
        let wat_path = temp_dir.join("out.wat");
        fs::write(&wat_path, &wat).expect("wat should be written");
        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wat_wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );
        let direct_out = run_iwasm(&direct_path);
        let wat_out = run_iwasm(&wat_wasm_path);
        // WAT emitter produces runtime output; direct binary produces compile-time string.
        // When Let is const-initialized, both should match.
        assert_eq!(direct_out, "42\n");
        assert_eq!(direct_out, wat_out);
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn direct_wasm_binary_mvp_binary_expression() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Number(10, Span::generated("test"))),
                        op: LoweredBinaryOp::Add,
                        right: Box::new(LoweredExpr::Number(32, Span::generated("test"))),
                        span: Span::generated("test"),
                    }],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (v, _) = Validated::new(program).expect("should validate");
        let direct_wasm =
            emit_wasm_binary_mvp(&v).expect("binary expression should emit direct wasm binary");
        let wat = emit_wat(&v).expect("should emit WAT");
        let temp_dir = unique_temp_dir("direct-wasm-binary-mvp-binary");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let direct_path = temp_dir.join("direct.wasm");
        let wat_wasm_path = temp_dir.join("wat.wasm");
        fs::write(&direct_path, &direct_wasm).expect("direct wasm should be written");
        let wat_path = temp_dir.join("out.wat");
        fs::write(&wat_path, &wat).expect("wat should be written");
        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wat_wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );
        let direct_out = run_iwasm(&direct_path);
        let wat_out = run_iwasm(&wat_wasm_path);
        assert_eq!(direct_out, "42\n");
        assert_eq!(direct_out, wat_out);
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn direct_wasm_binary_mvp_multiple_statements() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::String(
                            "hello".to_owned(),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Number(42, Span::generated("test"))],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (v, _) = Validated::new(program).expect("should validate");
        let direct_wasm =
            emit_wasm_binary_mvp(&v).expect("multiple statements should emit direct wasm binary");
        let wat = emit_wat(&v).expect("should emit WAT");
        let temp_dir = unique_temp_dir("direct-wasm-binary-mvp-multi");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let direct_path = temp_dir.join("direct.wasm");
        let wat_wasm_path = temp_dir.join("wat.wasm");
        fs::write(&direct_path, &direct_wasm).expect("direct wasm should be written");
        let wat_path = temp_dir.join("out.wat");
        fs::write(&wat_path, &wat).expect("wat should be written");
        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wat_wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );
        let direct_out = run_iwasm(&direct_path);
        let wat_out = run_iwasm(&wat_wasm_path);
        assert_eq!(direct_out, "hello\n42\n");
        assert_eq!(direct_out, wat_out);
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_module_builds_typed_wasm_module_sections() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::String(
                        "hello".to_owned(),
                        Span::generated("test"),
                    )],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let module = emit_wasm_module_native(&v).expect("native module should emit");

        assert!(
            module
                .imports
                .iter()
                .any(|import| import.name == "fd_write"),
            "native module should import fd_write"
        );
        assert!(
            module.memory.is_some(),
            "native module should declare memory"
        );
        assert!(
            module.exports.iter().any(|export| export.name == "_start"),
            "native module should export _start"
        );
        assert!(
            module
                .data_segments
                .iter()
                .any(|segment| segment.data == b"hello"),
            "native module should carry string data"
        );
        assert!(
            module
                .functions
                .iter()
                .any(|function| function.symbol == "$native_write_buf"),
            "native module should build runtime helpers as typed functions"
        );
        assert!(!emit_wasm_module_binary(&module).is_empty());
    }

    #[test]
    fn native_lowered_module_with_abi_carries_custom_section() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::String(
                        "hello".to_owned(),
                        Span::generated("test"),
                    )],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let abi_metadata = AbiMetadata::default();
        let expected_payload = abi_metadata.to_custom_section_payload();
        let (v, _) = Validated::new(program).expect("should validate");
        let module =
            emit_wasm_module_native_with_abi(&v, &abi_metadata).expect("native module should emit");

        assert!(
            module
                .custom_sections
                .iter()
                .any(|section| section.name == ABI_CUSTOM_SECTION_NAME
                    && section.payload == expected_payload),
            "native module should carry ABI custom section"
        );

        let wasm = emit_wasm_module_binary(&module);
        assert_eq!(
            wasm_custom_section_payload(&wasm, ABI_CUSTOM_SECTION_NAME),
            Some(expected_payload.as_slice())
        );
    }

    #[test]
    fn native_lowered_binary_runs_locals_arithmetic_if_while_and_function_call() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Number(0, span), span),
                LoweredStmt::Let(LocalId(1), LoweredExpr::Number(0, span), span),
                LoweredStmt::While {
                    condition: LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(LoweredExpr::Number(3, span)),
                        span,
                    },
                    body: vec![
                        LoweredStmt::Assign(
                            LocalId(0),
                            LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                op: LoweredBinaryOp::Add,
                                right: Box::new(LoweredExpr::Number(1, span)),
                                span,
                            },
                            span,
                        ),
                        LoweredStmt::If {
                            condition: LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                op: LoweredBinaryOp::StrictEqual,
                                right: Box::new(LoweredExpr::Number(2, span)),
                                span,
                            },
                            then_body: vec![LoweredStmt::Assign(
                                LocalId(1),
                                LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(1), span)),
                                    op: LoweredBinaryOp::Add,
                                    right: Box::new(LoweredExpr::Call {
                                        kind: FunctionCallKind::User(FuncId(0)),
                                        args: vec![
                                            LoweredExpr::Local(LocalId(0), span),
                                            LoweredExpr::Number(3, span),
                                        ],
                                        span,
                                    }),
                                    span,
                                },
                                span,
                            )],
                            else_body: vec![LoweredStmt::Assign(
                                LocalId(1),
                                LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(1), span)),
                                    op: LoweredBinaryOp::Add,
                                    right: Box::new(LoweredExpr::Number(1, span)),
                                    span,
                                },
                                span,
                            )],
                            span,
                        },
                    ],
                    span,
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0), LocalId(1)],
                uses_receiver: false,
                min_required_params: 2,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        op: LoweredBinaryOp::Add,
                        right: Box::new(LoweredExpr::Local(LocalId(1), span)),
                        span,
                    },
                    span,
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-control-flow");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "7\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_direct_function_binding_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(0),
                        captures: vec![],
                        representation: ClosureRepresentation::DirectLocalToken,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Call {
                            kind: FunctionCallKind::User(FuncId(0)),
                            args: vec![],
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: Some("f".to_owned()),
                locals: vec![],
                body: vec![LoweredStmt::Return(LoweredExpr::Number(42, span), span)],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native function binding should emit");
        let temp_dir = unique_temp_dir("native-lowered-function-binding");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "42\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_i32_writer_handles_multi_digit_and_negative_values() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Number(12345, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Number(-42, span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-i32-writer");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "12345\n-42\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_for_break_continue_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Number(0, span), span),
                LoweredStmt::Let(LocalId(1), LoweredExpr::Number(0, span), span),
                LoweredStmt::For {
                    init: Some(Box::new(LoweredStmt::Assign(
                        LocalId(0),
                        LoweredExpr::Number(0, span),
                        span,
                    ))),
                    condition: Some(LoweredExpr::Binary {
                        left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        op: LoweredBinaryOp::Less,
                        right: Box::new(LoweredExpr::Number(6, span)),
                        span,
                    }),
                    update: Some(LoweredExpr::Assign {
                        local: LocalId(0),
                        expr: Box::new(LoweredExpr::Binary {
                            left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(LoweredExpr::Number(1, span)),
                            span,
                        }),
                        span,
                    }),
                    body: vec![
                        LoweredStmt::If {
                            condition: LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                op: LoweredBinaryOp::StrictEqual,
                                right: Box::new(LoweredExpr::Number(2, span)),
                                span,
                            },
                            then_body: vec![LoweredStmt::Continue { label: None, span }],
                            else_body: vec![],
                            span,
                        },
                        LoweredStmt::If {
                            condition: LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                op: LoweredBinaryOp::StrictEqual,
                                right: Box::new(LoweredExpr::Number(5, span)),
                                span,
                            },
                            then_body: vec![LoweredStmt::Break { label: None, span }],
                            else_body: vec![],
                            span,
                        },
                        LoweredStmt::Assign(
                            LocalId(1),
                            LoweredExpr::Binary {
                                left: Box::new(LoweredExpr::Local(LocalId(1), span)),
                                op: LoweredBinaryOp::Add,
                                right: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                span,
                            },
                            span,
                        ),
                    ],
                    span,
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native for binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-for-break-continue");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "8\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_unary_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Unary {
                        op: LoweredUnaryOp::Negate,
                        expr: Box::new(LoweredExpr::Number(5, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(
                    LocalId(1),
                    LoweredExpr::Unary {
                        op: LoweredUnaryOp::Not,
                        expr: Box::new(LoweredExpr::Bool(true, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Unary {
                            op: LoweredUnaryOp::Plus,
                            expr: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native unary binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-numeric-unary");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "-5\nfalse\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_logical_assign_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Number(0, span), span),
                LoweredStmt::Expr(
                    LoweredExpr::LogicalAssign {
                        local: LocalId(0),
                        op: LoweredLogicalAssignOp::Or,
                        expr: Box::new(LoweredExpr::Number(42, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(0), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(LocalId(1), LoweredExpr::Number(1, span), span),
                LoweredStmt::Expr(
                    LoweredExpr::LogicalAssign {
                        local: LocalId(1),
                        op: LoweredLogicalAssignOp::And,
                        expr: Box::new(LoweredExpr::Number(99, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(LocalId(2), LoweredExpr::Null(span), span),
                LoweredStmt::Expr(
                    LoweredExpr::LogicalAssign {
                        local: LocalId(2),
                        op: LoweredLogicalAssignOp::Nullish,
                        expr: Box::new(LoweredExpr::Number(7, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(2), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native logical assign binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-logical-assign");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "42\n99\n7\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_object_and_array_reads_run_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![
                            ("a".to_owned(), LoweredExpr::Number(1, span)),
                            ("b".to_owned(), LoweredExpr::Number(2, span)),
                        ],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Let(
                    LocalId(1),
                    LoweredExpr::ArrayNew {
                        elements: vec![LoweredExpr::Number(3, span), LoweredExpr::Number(4, span)],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "a".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::OptionalPropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "b".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGetDynamic {
                            obj: Box::new(LoweredExpr::Local(LocalId(1), span)),
                            key: Box::new(LoweredExpr::Number(0, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm =
            emit_wasm_binary_native(&v).expect("native static object and array binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-object-array");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "1\n2\n3\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_array_index_assign_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ArrayNew {
                        elements: vec![
                            LoweredExpr::Number(10, span),
                            LoweredExpr::Number(20, span),
                            LoweredExpr::Number(30, span),
                        ],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::PropertySetDynamic {
                        object: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        index: Box::new(LoweredExpr::Number(1, span)),
                        value: Box::new(LoweredExpr::Number(99, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGetDynamic {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: Box::new(LoweredExpr::Number(1, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm =
            emit_wasm_binary_native(&v).expect("native static array index assign should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-array-index-assign");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "99\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_object_property_assign_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![("x".to_owned(), LoweredExpr::Number(1, span))],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::PropertySet {
                        object: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        key: "x".to_owned(),
                        value: Box::new(LoweredExpr::Number(2, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "x".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm =
            emit_wasm_binary_native(&v).expect("native static object property assign should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-object-property-assign");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "2\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_object_logical_property_assign_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::LogicalPropertyAssign {
                        object: LocalId(0),
                        key: "x".to_owned(),
                        op: LoweredLogicalAssignOp::Nullish,
                        expr: Box::new(LoweredExpr::Number(42, span)),
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "x".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v)
            .expect("native static object logical property assign should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-object-logical-property-assign");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "42\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_object_console_values_run_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![
                            (
                                "s".to_owned(),
                                LoweredExpr::String("hello".to_owned(), span),
                            ),
                            ("b".to_owned(), LoweredExpr::Bool(true, span)),
                            ("u".to_owned(), LoweredExpr::Undefined(span)),
                        ],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "s".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "b".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGet {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: "u".to_owned(),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm =
            emit_wasm_binary_native(&v).expect("native static object console values should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-object-console-values");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "hello\ntrue\nundefined\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_object_dynamic_key_reads_run_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![
                            (
                                "name".to_owned(),
                                LoweredExpr::String("Alice".to_owned(), span),
                            ),
                            ("age".to_owned(), LoweredExpr::Number(30, span)),
                        ],
                        non_enumerable: 0,
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGetDynamic {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: Box::new(LoweredExpr::String("name".to_owned(), span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::PropertyGetDynamic {
                            obj: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            key: Box::new(LoweredExpr::String("age".to_owned(), span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm =
            emit_wasm_binary_native(&v).expect("native static object dynamic keys should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-object-dynamic-key");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "Alice\n30\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_block_expr_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::Block {
                        stmts: vec![
                            LoweredStmt::Let(LocalId(0), LoweredExpr::Undefined(span), span),
                            LoweredStmt::If {
                                condition: LoweredExpr::Bool(true, span),
                                then_body: vec![LoweredStmt::Assign(
                                    LocalId(0),
                                    LoweredExpr::Number(1, span),
                                    span,
                                )],
                                else_body: vec![LoweredStmt::Assign(
                                    LocalId(0),
                                    LoweredExpr::Number(2, span),
                                    span,
                                )],
                                span,
                            },
                        ],
                        result: Box::new(LoweredExpr::Local(LocalId(0), span)),
                        span,
                    }],
                    span,
                },
                span,
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native block expr binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-block-expr");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "1\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_switch_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Number(2, span), span),
                LoweredStmt::Switch {
                    expr: LoweredExpr::Local(LocalId(0), span),
                    cases: vec![
                        (
                            Some(LoweredExpr::Number(1, span)),
                            vec![LoweredStmt::Expr(
                                LoweredExpr::Call {
                                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                                    args: vec![LoweredExpr::Number(1, span)],
                                    span,
                                },
                                span,
                            )],
                        ),
                        (
                            Some(LoweredExpr::Number(2, span)),
                            vec![
                                LoweredStmt::Expr(
                                    LoweredExpr::Call {
                                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                                        args: vec![LoweredExpr::Number(2, span)],
                                        span,
                                    },
                                    span,
                                ),
                                LoweredStmt::Break { label: None, span },
                            ],
                        ),
                        (
                            None,
                            vec![LoweredStmt::Expr(
                                LoweredExpr::Call {
                                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                                    args: vec![LoweredExpr::Number(3, span)],
                                    span,
                                },
                                span,
                            )],
                        ),
                    ],
                    span,
                },
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native switch binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-switch");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "2\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_typeof_console_log_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Unary {
                            op: LoweredUnaryOp::TypeOf,
                            expr: Box::new(LoweredExpr::Number(42, span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Unary {
                            op: LoweredUnaryOp::TypeOf,
                            expr: Box::new(LoweredExpr::String("hi".to_owned(), span)),
                            span,
                        }],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native typeof binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-typeof");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "number\nstring\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_labeled_do_while_break_continue_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(LocalId(0), LoweredExpr::Number(0, span), span),
                LoweredStmt::Let(LocalId(1), LoweredExpr::Number(0, span), span),
                LoweredStmt::Labeled {
                    label: "outer".to_owned(),
                    body: Box::new(LoweredStmt::DoWhile {
                        body: vec![
                            LoweredStmt::Assign(
                                LocalId(0),
                                LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                    op: LoweredBinaryOp::Add,
                                    right: Box::new(LoweredExpr::Number(1, span)),
                                    span,
                                },
                                span,
                            ),
                            LoweredStmt::If {
                                condition: LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                    op: LoweredBinaryOp::StrictEqual,
                                    right: Box::new(LoweredExpr::Number(2, span)),
                                    span,
                                },
                                then_body: vec![LoweredStmt::Continue {
                                    label: Some("outer".to_owned()),
                                    span,
                                }],
                                else_body: vec![],
                                span,
                            },
                            LoweredStmt::If {
                                condition: LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                    op: LoweredBinaryOp::StrictEqual,
                                    right: Box::new(LoweredExpr::Number(4, span)),
                                    span,
                                },
                                then_body: vec![LoweredStmt::Break {
                                    label: Some("outer".to_owned()),
                                    span,
                                }],
                                else_body: vec![],
                                span,
                            },
                            LoweredStmt::Assign(
                                LocalId(1),
                                LoweredExpr::Binary {
                                    left: Box::new(LoweredExpr::Local(LocalId(1), span)),
                                    op: LoweredBinaryOp::Add,
                                    right: Box::new(LoweredExpr::Local(LocalId(0), span)),
                                    span,
                                },
                                span,
                            ),
                        ],
                        condition: LoweredExpr::Binary {
                            left: Box::new(LoweredExpr::Local(LocalId(0), span)),
                            op: LoweredBinaryOp::Less,
                            right: Box::new(LoweredExpr::Number(10, span)),
                            span,
                        },
                        span,
                    }),
                    span,
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Local(LocalId(1), span)],
                        span,
                    },
                    span,
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native labeled loop binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-labeled-loop");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "4\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn native_lowered_static_module_export_runs_without_wat_conversion() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::PropertyGet {
                        obj: Box::new(LoweredExpr::ModuleLoad {
                            module_id: 1,
                            kind: ModuleLoadKind::StaticRequire,
                            span,
                        }),
                        key: "value".to_owned(),
                        span,
                    }],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![ModuleInfo {
                id: 1,
                specifier: "./source".to_owned(),
                statements: vec![
                    LoweredStmt::Let(LocalId(0), LoweredExpr::Number(41, span), span),
                    LoweredStmt::Export {
                        name: "value".to_owned(),
                        expr: LoweredExpr::Local(LocalId(0), span),
                        span,
                    },
                    LoweredStmt::Assign(LocalId(0), LoweredExpr::Number(42, span), span),
                    LoweredStmt::ModuleExportsUpdate {
                        name: "value".to_owned(),
                        local: LocalId(0),
                        span,
                    },
                ],
                locals_count: 1,
            }],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wasm = emit_wasm_binary_native(&v).expect("native module binary should emit");
        let temp_dir = unique_temp_dir("native-lowered-static-module-export");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wasm_path = temp_dir.join("native.wasm");
        fs::write(&wasm_path, wasm).expect("native wasm should be written");

        assert_eq!(run_iwasm(&wasm_path), "42\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn emit_wasm_binary_uses_native_lowered_subset_when_supported() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::Number(12345, span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let native = emit_wasm_binary_native(&v).expect("native binary should emit");
        let main = emit_wasm_binary(&v).expect("main binary should emit");

        assert_eq!(main, native);
    }

    #[test]
    fn emit_wasm_binary_rejects_unsupported_native_shape_without_wat_fallback() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::String("side-effect-free".to_owned(), span),
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let native_err = emit_wasm_binary_native(&v).expect_err("native subset should reject");
        assert_eq!(native_err.code, DiagCode::UnsupportedSyntax);
        let public_err = emit_wasm_binary(&v).expect_err("public binary API should reject");
        assert_eq!(public_err.code, DiagCode::UnsupportedSyntax);
    }

    #[test]
    fn emit_wasm_binary_wat_debug_fallback_accepts_unsupported_native_shape() {
        let span = Span::generated("test");
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::String("side-effect-free".to_owned(), span),
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let native_err = emit_wasm_binary_native(&v).expect_err("native subset should reject");
        assert_eq!(native_err.code, DiagCode::UnsupportedSyntax);
        assert!(
            wasmparser::Validator::new()
                .validate_all(
                    &emit_wasm_binary_with_wat_debug_fallback(&v)
                        .expect("debug fallback should emit")
                )
                .is_ok()
        );
    }

    #[test]
    fn emit_wasm_binary_with_abi_embeds_section_for_native_path() {
        let span = Span::generated("test");
        let native_program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args: vec![LoweredExpr::String("hello".to_owned(), span)],
                    span,
                },
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (native, _) = Validated::new(native_program).expect("native program should validate");
        let abi_metadata = AbiMetadata::default();
        let expected_payload = abi_metadata.to_custom_section_payload();

        let native_wasm =
            emit_wasm_binary_native_with_abi(&native, &abi_metadata).expect("native should emit");
        let public_native_wasm =
            emit_wasm_binary_with_abi(&native, &abi_metadata).expect("public native should emit");

        assert_eq!(public_native_wasm, native_wasm);
        assert_eq!(
            wasm_custom_section_payload(&public_native_wasm, ABI_CUSTOM_SECTION_NAME),
            Some(expected_payload.as_slice())
        );
    }

    #[test]
    fn emit_wasm_binary_with_abi_debug_fallback_embeds_section() {
        let span = Span::generated("test");
        let fallback_program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::String("side-effect-free".to_owned(), span),
                span,
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let (fallback, _) =
            Validated::new(fallback_program).expect("fallback program should validate");
        let abi_metadata = AbiMetadata::default();
        let expected_payload = abi_metadata.to_custom_section_payload();

        let public_err =
            emit_wasm_binary_with_abi(&fallback, &abi_metadata).expect_err("native should reject");
        assert_eq!(public_err.code, DiagCode::UnsupportedSyntax);
        let debug_fallback_wasm =
            emit_wasm_binary_with_abi_wat_debug_fallback(&fallback, &abi_metadata)
                .expect("debug fallback should emit");

        assert_eq!(
            wasm_custom_section_payload(&debug_fallback_wasm, ABI_CUSTOM_SECTION_NAME),
            Some(expected_payload.as_slice())
        );
    }

    #[test]
    fn alloc_heap_emits_gc_header_and_trigger_contract() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ObjectNew {
                    props: vec![],
                    non_enumerable: 0,
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("object allocation should emit WAT");

        assert!(wat.contains(&format!(
            "(memory (export \"memory\") {} {})",
            Layout::MEMORY_MIN_PAGES,
            Layout::MEMORY_MAX_PAGES
        )));
        assert!(wat.contains("(global $alloc_bytes_since_last_gc (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global $gc_free_list (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global $gc_free_list_max_body_size (mut i32) (i32.const 0))"));
        assert!(
            wat.contains("(global $gc_free_list_second_max_body_size (mut i32) (i32.const 0))")
        );
        assert!(wat.contains("(func $gc_collect"));
        assert!(wat.contains("(local $header_base i32)"));
        assert!(wat.contains("(local $payload_base i32)"));
        assert!(wat.contains("(i32.const 16)"));
        assert!(wat.contains("(i32.const 65536)"));
        assert!(wat.contains(&format!(
            "(i32.const {})",
            Layout::GC_HEADROOM_PAGES * Layout::WASM_PAGE_SIZE
        )));
        assert!(wat.contains(&format!("(i32.const {})", Layout::MEMORY_MAX_PAGES)));
        assert!(wat.contains(&format!(
            "(i32.const {})",
            Layout::MEMORY_MAX_PAGES * Layout::WASM_PAGE_SIZE
        )));
        assert!(wat.contains("(i32.eq (local.get $memory_pages)"));
        assert!(wat.contains("(i32.gt_u (local.get $new_heap) (local.get $memory_bytes))"));
        assert!(wat.contains(&format!("(i32.const {})", Layout::HEAP_GROW_MIN_PAGES)));
        assert!(wat.contains("(local $needed_pages i32)"));
        assert!(wat.contains("(local $remaining_pages i32)"));
        assert!(wat.contains(&format!(
            "(i32.sub (i32.const {}) (local.get $memory_pages))",
            Layout::MEMORY_MAX_PAGES
        )));
        assert!(wat.contains("(i32.gt_u (local.get $needed_pages) (local.get $remaining_pages))"));
        assert!(wat.contains("(memory.grow (local.get $needed_pages))"));
        assert!(wat.contains("(i32.const -1)"));
        assert!(wat.contains("(global.get $alloc_bytes_since_last_gc)"));
        assert!(wat.contains("(call $gc_collect)"));
        let compact_wat = wat_words(&wat);
        assert!(compact_wat.contains("(call $gc_collect))) ;; A collection can tail-trim $heap."));
        assert!(wat.contains("(global.set $alloc_bytes_since_last_gc"));
        assert!(wat.contains("(local.get $payload_base))"));
    }

    #[test]
    fn gc_sweep_and_free_list_reuse_contract_is_emitted() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ObjectNew {
                    props: vec![],
                    non_enumerable: 0,
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("object allocation should emit WAT");

        assert!(wat.contains("(func $gc_sweep"));
        assert!(wat.contains("(global.get $gc_free_list)"));
        assert!(wat.contains("(global.set $gc_free_list (i32.const 0))"));
        assert!(wat.contains("(global.set $gc_free_list_max_body_size (i32.const 0))"));
        assert!(wat.contains("(global.set $gc_free_list_second_max_body_size (i32.const 0))"));
        assert!(wat.contains("(global.get $gc_free_list_max_body_size)"));
        assert!(wat.contains("(global.get $gc_free_list_second_max_body_size)"));
        assert!(wat.contains("(global.set $gc_free_list_max_body_size (local.get $body_size))"));
        let compact_wat = wat_words(&wat);
        assert!(compact_wat.contains(
            "(global.set $gc_free_list_second_max_body_size (global.get $gc_free_list_max_body_size))"
        ));
        assert!(
            wat.contains("(global.set $gc_free_list_second_max_body_size (local.get $body_size))")
        );
        assert!(compact_wat.contains(
            "(global.set $gc_free_list_max_body_size (global.get $gc_free_list_second_max_body_size))"
        ));
        assert!(wat.contains("(local $next_body_size i32)"));
        assert!(wat.contains("(loop $coalesce"));
        assert!(wat.contains("(i32.add (i32.const 16) (local.get $next_body_size))"));
        assert!(wat.contains("(global.set $heap (local.get $cursor))"));
        assert!(wat.contains("(global.set $gc_free_list (local.get $cursor))"));
        assert!(wat.contains("(local $free_header i32)"));
        assert!(wat.contains("(local $free_body_size i32)"));
        assert!(wat.contains("(return (i32.add (local.get $free_header) (i32.const 16)))"));
        assert!(wat.contains("(i32.and (local.get $flags) (i32.const -2))"));
    }

    #[test]
    fn concat_allocates_managed_heap_strings() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Binary {
                    op: LoweredBinaryOp::Add,
                    left: Box::new(LoweredExpr::String("a".to_owned(), Span::generated("test"))),
                    right: Box::new(LoweredExpr::String("b".to_owned(), Span::generated("test"))),
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("string concat should emit WAT");
        let concat_start = wat.find("(func $concat").expect("concat should be emitted");
        let concat_end = wat[concat_start + 1..]
            .find("\n  (func ")
            .map(|offset| concat_start + 1 + offset)
            .unwrap_or(wat.len());
        let concat_body = &wat[concat_start..concat_end];

        assert!(concat_body.contains("(call $alloc_heap"));
        assert!(concat_body.contains("(call $copy"));
        assert!(!concat_body.contains("(global.set $heap"));
    }

    #[test]
    fn generator_next_uses_generator_state_object() {
        let values = LoweredExpr::ArrayNew {
            elements: vec![
                LoweredExpr::Number(1, Span::generated("test")),
                LoweredExpr::Number(2, Span::generated("test")),
            ],
            span: Span::generated("test"),
        };
        let generator = LoweredExpr::RuntimeCall {
            intrinsic: RuntimeFn::GeneratorYield,
            args: vec![values],
            span: Span::generated("test"),
        };
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::GeneratorNext,
                    args: vec![generator],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("generator runtime calls should validate");
        let wat = emit_wat(&v).expect("generator runtime calls should emit WAT");
        let generator_yield = wat_function(&wat, "generator_yield");
        let generator_next = wat_function(&wat, "generator_next");

        assert!(generator_yield.contains("(i32.const 2)"));
        assert!(generator_yield.contains("(call $alloc_heap"));
        assert!(!generator_yield.contains("(call $array_values"));
        assert!(generator_next.contains("(call $array_get"));
        assert!(generator_next.contains("(i32.store"));
        assert!(!generator_next.contains("(call $array_iterator_next"));
    }

    #[test]
    fn top_level_locals_are_mirrored_into_gc_root_table() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::ObjectNew {
                    props: vec![],
                    non_enumerable: 0,
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program.clone()).expect("should validate");
        let wat = emit_wat(&v).expect("top-level local root should emit WAT");
        let root_count =
            LocalFrame::new(program.top_level_locals.len(), Some(0)).total_local_count();
        let root_bytes = root_count * std::mem::size_of::<u32>();

        assert!(wat.contains("(global $gc_root_base (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global $gc_root_count (mut i32) (i32.const 0))"));
        assert!(wat.contains(&format!(
            "(global.set $gc_root_count (i32.const {root_count}))"
        )));
        assert!(wat.contains(&format!(
            "(global.set $gc_root_base (call $alloc_heap (i32.const {root_bytes})))"
        )));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_root_base) (i32.const 0)) (local.get 0))"
        ));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_root_base) (i32.const 4)) (local.get 1))"
        ));
        assert!(wat.contains("(func $gc_mark_registered_roots"));
        assert!(wat.contains("(call $gc_mark_value (i32.load (local.get $slot)))"));
    }

    #[test]
    fn function_locals_are_mirrored_into_activation_gc_root_frames() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::User(FuncId(0)),
                    args: vec![],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![LocalId(0)],
                body: vec![
                    LoweredStmt::Let(
                        LocalId(0),
                        LoweredExpr::ObjectNew {
                            props: vec![],
                            non_enumerable: 0,
                            span: Span::generated("test"),
                        },
                        Span::generated("test"),
                    ),
                    LoweredStmt::Return(
                        LoweredExpr::Local(LocalId(0), Span::generated("test")),
                        Span::generated("test"),
                    ),
                ],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program.clone()).expect("should validate");
        let wat = emit_wat(&v).expect("function local root should emit WAT");
        let func_wat = wat_function(&wat, "func_0");
        let static_root_count = LocalFrame::new(0, Some(0)).total_local_count();
        let static_root_bytes = static_root_count * std::mem::size_of::<u32>();
        let root_bytes = static_root_bytes + Layout::GC_CALL_FRAME_ROOT_STACK_BYTES as usize;
        let activation_frame = LocalFrame::activation(program.functions[0].locals.len(), true);
        let activation_frame_bytes = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + activation_frame.total_local_count() * std::mem::size_of::<u32>();
        let backend_last_local = activation_frame.total_local_count() - 1;
        let backend_last_offset = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + backend_last_local * std::mem::size_of::<u32>();

        assert!(wat.contains("(global $gc_call_frame_current (mut i32) (i32.const 0))"));
        assert!(wat.contains(&format!(
            "(global.set $gc_root_count (i32.const {static_root_count}))"
        )));
        assert!(wat.contains(&format!(
            "(global.set $gc_root_base (call $alloc_heap (i32.const {root_bytes})))"
        )));
        assert!(wat.contains(&format!(
            "(global.set $gc_call_frame_base (i32.add (global.get $gc_root_base) (i32.const {static_root_bytes})))"
        )));
        assert!(
            wat.contains("(global.set $gc_call_frame_current (global.get $gc_call_frame_top))")
        );
        assert!(wat.contains(&format!(
            "(global.set $gc_call_frame_top (i32.add (global.get $gc_call_frame_top) (i32.const {activation_frame_bytes})))"
        )));
        assert!(func_wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 8)) (local.get 0))"
        ));
        assert!(func_wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 12)) (local.get 1))"
        ));
        assert!(func_wat.contains(&format!(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {backend_last_offset})) (local.get {backend_last_local}))"
        )));
        assert!(func_wat.contains(&format!(
            "(local.set 1 (i32.const {}))",
            ValueTag::UNDEFINED
        )));
        assert!(!func_wat.contains(&format!(
            "(local.set 0 (i32.const {}))",
            ValueTag::UNDEFINED
        )));
        assert!(func_wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 12)) (local.get 1))"
        ));
        assert!(wat.contains("(call $gc_mark_call_frame_roots"));
        assert!(
            func_wat
                .contains("(global.set $gc_call_frame_top (global.get $gc_call_frame_current))")
        );
    }

    #[test]
    fn promise_get_value_collects_nested_string_data() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::PromiseGetValue {
                    promise: Box::new(LoweredExpr::String(
                        "await".to_owned(),
                        Span::generated("test"),
                    )),
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("promise value expression should emit WAT");

        assert!(wat.contains("\\05\\00\\00\\00await"));
    }

    #[test]
    fn gc_mark_helpers_visit_heap_graph_payloads() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ObjectNew {
                    props: vec![(
                        "child".to_owned(),
                        LoweredExpr::ArrayNew {
                            elements: vec![],
                            span: Span::generated("test"),
                        },
                    )],
                    non_enumerable: 0,
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("object graph allocation should emit WAT");

        assert!(wat.contains("(func $gc_mark_payload_header"));
        assert!(wat.contains("(func $gc_mark_value"));
        assert!(wat.contains("(func $gc_mark_array_payload"));
        assert!(wat.contains("(func $gc_mark_object_payload"));
        assert!(wat.contains("(i32.or (local.get $flags) (i32.const 1))"));
        assert!(wat.contains("(call $gc_mark_value (i32.load (local.get $elem_ptr)))"));
        assert!(
            wat.contains("(call $gc_mark_value (i32.load (local.get $entry_ptr)))"),
            "object key raw values should be mark-scanned"
        );
        assert!(
            wat.contains("(i32.load (i32.add (local.get $entry_ptr) (i32.const 4))))"),
            "object property values should be mark-scanned"
        );
        assert!(
            wat.contains("(i32.or (local.get $proto) (i32.const 7))"),
            "object prototypes are raw payload pointers and must be tagged before marking"
        );
    }

    #[test]
    fn gc_collect_marks_module_cache_roots_when_module_runtime_is_enabled() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ModuleLoad {
                    module_id: 1,
                    kind: ModuleLoadKind::StaticRequire,
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![ModuleInfo {
                id: 1,
                specifier: "./dep".to_owned(),
                statements: vec![],
                locals_count: 0,
            }],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("module runtime should emit WAT");

        assert!(wat.contains("(call $gc_mark_module_cache_roots)"));
        assert!(wat.contains("(func $gc_mark_module_cache_roots"));
        assert!(wat.contains("(global.get $module_cache)"));
        assert!(wat.contains("(i32.const 64)"));
        assert!(wat.contains("(i32.const 8)"));
        assert!(wat.contains("(i32.const 4)"));
    }

    #[test]
    fn module_runtime_helpers_are_not_emitted_without_module_ir() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::Number(1, Span::generated("test")),
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("non-module program should emit WAT");

        assert!(!wat.contains("$module_require"));
        assert!(!wat.contains("$module_exports_set"));
        assert!(!wat.contains("$module_exports_assign"));
        assert!(!wat.contains("$module_cache"));
    }

    #[test]
    fn module_initializers_are_emitted_and_called_in_metadata_order() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::PropertyGet {
                    obj: Box::new(LoweredExpr::ModuleLoad {
                        module_id: 1,
                        kind: ModuleLoadKind::StaticRequire,
                        span: Span::generated("test"),
                    }),
                    key: "value".to_owned(),
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![
                ModuleInfo {
                    id: 2,
                    specifier: "./nested".to_owned(),
                    statements: vec![LoweredStmt::Export {
                        name: "nested".to_owned(),
                        expr: LoweredExpr::Number(2, Span::generated("test")),
                        span: Span::generated("test"),
                    }],
                    locals_count: 0,
                },
                ModuleInfo {
                    id: 1,
                    specifier: "./source".to_owned(),
                    statements: vec![LoweredStmt::Export {
                        name: "value".to_owned(),
                        expr: LoweredExpr::Number(1, Span::generated("test")),
                        span: Span::generated("test"),
                    }],
                    locals_count: 0,
                },
            ],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("module initializers should emit WAT");
        let compact = wat_words(&wat);

        assert!(wat.contains("(func $module_init_2"));
        assert!(wat.contains("(func $module_init_1"));
        assert!(compact.contains("(i32.const 2) (global.set $current_module_id)"));
        assert!(compact.contains("(i32.const 1) (global.set $current_module_id)"));
        let call_nested = wat
            .find("(call $module_init_2)")
            .expect("nested module init should be called");
        let call_source = wat
            .find("(call $module_init_1)")
            .expect("source module init should be called");
        let top_level_import = wat
            .find("(call $module_require (i32.const 1))")
            .expect("top-level static import read should remain module-backed");
        assert!(
            call_nested < call_source,
            "module initializer calls should preserve dependency-first metadata order"
        );
        assert!(
            call_source < top_level_import,
            "module initializer calls should run before top-level import reads"
        );

        let temp_dir = unique_temp_dir("module-initializers-typed-wasmir");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("out.wat");
        let wasm_path = temp_dir.join("out.wasm");
        fs::write(&wat_path, &wat).expect("wat should be written");
        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "module initializer WAT should validate\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr),
        );
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn gc_collect_marks_class_prototype_globals() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::ClassPrototype(
                    ClassPrototypeRef {
                        constructor: FuncId(0),
                        parent_constructors: vec![],
                    },
                    Span::generated("test"),
                ),
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("class prototype root should emit WAT");

        assert!(wat.contains("(global $class_proto_0 (mut i32) (i32.const 0))"));
        assert!(wat.contains("(call $gc_mark_value (i32.or (global.get $class_proto_0)"));
        assert!(wat.contains("(i32.const 7)"));
    }

    #[test]
    fn private_field_runtime_calls_do_not_create_slots_on_plain_objects() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PrivateFieldSet,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), Span::generated("test")),
                            LoweredExpr::Number(1, Span::generated("test")),
                            LoweredExpr::Number(0, Span::generated("test")),
                            LoweredExpr::Number(7, Span::generated("test")),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::PrivateFieldGet,
                            args: vec![
                                LoweredExpr::Local(LocalId(0), Span::generated("test")),
                                LoweredExpr::Number(1, Span::generated("test")),
                                LoweredExpr::Number(0, Span::generated("test")),
                            ],
                            span: Span::generated("test"),
                        }],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("private field guard fixture should emit WAT");
        assert!(wat.contains(&format!("(i32.const {})", Layout::GC_RESERVED_OFFSET)));

        let temp_dir = unique_temp_dir("private-field-plain-object-guard");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("guard.wat");
        let wasm_path = temp_dir.join("guard.wasm");
        fs::write(&wat_path, wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );

        let output = Command::new("iwasm")
            .arg(&wasm_path)
            .output()
            .expect("iwasm should run");
        assert!(
            !output.status.success(),
            "plain-object private field access should abort"
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(
                "TypeError: Cannot read private member from an object whose class did not declare it"
            ),
            "expected private brand TypeError diagnostic, got stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn private_field_runtime_calls_require_matching_brand() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::New {
                        constructor: FuncId(0),
                        prototype: ClassPrototypeRef {
                            constructor: FuncId(0),
                            parent_constructors: vec![],
                        },
                        args: vec![],
                        base_local: LocalId(1),
                        private_brand: Some(1),
                        private_slot_count: 1,
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::PrivateFieldGet,
                            args: vec![
                                LoweredExpr::Local(LocalId(0), Span::generated("test")),
                                LoweredExpr::Number(1, Span::generated("test")),
                                LoweredExpr::Number(0, Span::generated("test")),
                            ],
                            span: Span::generated("test"),
                        }],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: true,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PrivateFieldSet,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), Span::generated("test")),
                            LoweredExpr::Number(1, Span::generated("test")),
                            LoweredExpr::Number(0, Span::generated("test")),
                            LoweredExpr::Number(3, Span::generated("test")),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                )],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("private field brand fixture should emit WAT");
        assert!(wat.contains("(i32.const 65537)"));

        let temp_dir = unique_temp_dir("private-field-brand-guard");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("guard.wat");
        let wasm_path = temp_dir.join("guard.wasm");
        fs::write(&wat_path, wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );

        assert_eq!(run_iwasm(&wasm_path), "3\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn private_field_runtime_calls_reject_mismatched_brand() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::New {
                        constructor: FuncId(0),
                        prototype: ClassPrototypeRef {
                            constructor: FuncId(0),
                            parent_constructors: vec![],
                        },
                        args: vec![],
                        base_local: LocalId(1),
                        private_brand: Some(1),
                        private_slot_count: 1,
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PrivateFieldGet,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), Span::generated("test")),
                            LoweredExpr::Number(2, Span::generated("test")),
                            LoweredExpr::Number(0, Span::generated("test")),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: true,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("private field brand mismatch fixture should emit WAT");
        let temp_dir = unique_temp_dir("private-field-brand-mismatch");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("guard.wat");
        let wasm_path = temp_dir.join("guard.wasm");
        fs::write(&wat_path, wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );

        let output = Command::new("iwasm")
            .arg(&wasm_path)
            .output()
            .expect("iwasm should run");
        assert!(
            !output.status.success(),
            "mismatched private brand should abort"
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(
                "TypeError: Cannot read private member from an object whose class did not declare it"
            ),
            "expected private brand TypeError diagnostic, got stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn private_field_runtime_calls_raise_catchable_type_error() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::ObjectNew {
                        props: vec![],
                        non_enumerable: 0,
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::TryCatch {
                    try_body: vec![LoweredStmt::Expr(
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::PrivateFieldGet,
                            args: vec![
                                LoweredExpr::Local(LocalId(0), Span::generated("test")),
                                LoweredExpr::Number(1, Span::generated("test")),
                                LoweredExpr::Number(0, Span::generated("test")),
                            ],
                            span: Span::generated("test"),
                        },
                        Span::generated("test"),
                    )],
                    catch_var: Some(LocalId(1)),
                    catch_body: Some(vec![LoweredStmt::Expr(
                        LoweredExpr::Call {
                            kind: FunctionCallKind::Builtin(
                                ts2wasm_ir::builtin::BuiltinId::ConsoleLog,
                            ),
                            args: vec![LoweredExpr::String(
                                "caught".to_owned(),
                                Span::generated("test"),
                            )],
                            span: Span::generated("test"),
                        },
                        Span::generated("test"),
                    )]),
                    finally_body: None,
                    span: Span::generated("test"),
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::String(
                            "after".to_owned(),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1)],
            functions: vec![],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("private field catchable TypeError should emit WAT");
        let temp_dir = unique_temp_dir("private-field-catchable-type-error");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("guard.wat");
        let wasm_path = temp_dir.join("guard.wasm");
        fs::write(&wat_path, wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );

        assert_eq!(run_iwasm(&wasm_path), "caught\nafter\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn private_brand_check_runtime_call_checks_zero_slot_brand() {
        let program = LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::New {
                        constructor: FuncId(0),
                        prototype: ClassPrototypeRef {
                            constructor: FuncId(0),
                            parent_constructors: vec![],
                        },
                        args: vec![],
                        base_local: LocalId(2),
                        private_brand: Some(1),
                        private_slot_count: 0,
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::RuntimeCall {
                        intrinsic: RuntimeFn::PrivateBrandCheck,
                        args: vec![
                            LoweredExpr::Local(LocalId(0), Span::generated("test")),
                            LoweredExpr::Number(1, Span::generated("test")),
                        ],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
                LoweredStmt::TryCatch {
                    try_body: vec![LoweredStmt::Expr(
                        LoweredExpr::RuntimeCall {
                            intrinsic: RuntimeFn::PrivateBrandCheck,
                            args: vec![
                                LoweredExpr::Local(LocalId(0), Span::generated("test")),
                                LoweredExpr::Number(2, Span::generated("test")),
                            ],
                            span: Span::generated("test"),
                        },
                        Span::generated("test"),
                    )],
                    catch_var: Some(LocalId(1)),
                    catch_body: Some(vec![LoweredStmt::Expr(
                        LoweredExpr::Call {
                            kind: FunctionCallKind::Builtin(
                                ts2wasm_ir::builtin::BuiltinId::ConsoleLog,
                            ),
                            args: vec![LoweredExpr::String(
                                "caught".to_owned(),
                                Span::generated("test"),
                            )],
                            span: Span::generated("test"),
                        },
                        Span::generated("test"),
                    )]),
                    finally_body: None,
                    span: Span::generated("test"),
                },
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::String(
                            "after".to_owned(),
                            Span::generated("test"),
                        )],
                        span: Span::generated("test"),
                    },
                    Span::generated("test"),
                ),
            ],
            top_level_locals: vec![LocalId(0), LocalId(1), LocalId(2)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: true,
                min_required_params: 1,
                rest_param_index: None,
                metadata_length: None,
                metadata_name: None,
                locals: vec![],
                body: vec![],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
            }],
            modules: vec![],
        };

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("private brand check should emit WAT");
        assert!(wat.contains("(i32.const 65536)"));

        let temp_dir = unique_temp_dir("private-brand-check-zero-slot");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let wat_path = temp_dir.join("guard.wat");
        let wasm_path = temp_dir.join("guard.wasm");
        fs::write(&wat_path, wat).expect("WAT should be written");

        let wat2wasm = Command::new("wat2wasm")
            .arg(&wat_path)
            .arg("-o")
            .arg(&wasm_path)
            .output()
            .expect("wat2wasm should run");
        assert!(
            wat2wasm.status.success(),
            "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&wat2wasm.stdout),
            String::from_utf8_lossy(&wat2wasm.stderr)
        );

        assert_eq!(run_iwasm(&wasm_path), "caught\nafter\n");
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn math_random_imports_wasi_random_get() {
        let program = math_random_program();

        let (v, _) = Validated::new(program).expect("should validate");
        let wat = emit_wat(&v).expect("Math.random should emit with WASI random");

        assert!(wat.contains("(import \"wasi_snapshot_preview1\" \"random_get\""));
        assert!(wat.contains("(call $random_get"));
        assert!(!wat.contains("$random_counter"));
    }

    #[test]
    fn math_random_manifest_declares_wasi_random() {
        let program = math_random_program();
        let validated_plan = build_validated_runtime_link_plan(&program).expect("valid link plan");

        let manifest: serde_json::Value =
            serde_json::from_str(&emit_canonical_manifest_json(&validated_plan))
                .expect("manifest should be valid JSON");

        assert_eq!(manifest["standalone"], true);
        assert_eq!(manifest["node_host"]["required"], false);
        assert_eq!(manifest["wasi"]["random"], true);
        let reasons = manifest["capability_reasons"]["wasi.random"]
            .as_array()
            .expect("wasi.random should record audit reasons");
        assert!(
            reasons.iter().any(|reason| reason == "Math.random"),
            "wasi.random reasons should include Math.random: {reasons:?}"
        );
    }

    fn math_random_program() -> LoweredProgram {
        LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(
                LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::MathRandom,
                    args: vec![],
                    span: Span::generated("test"),
                },
                Span::generated("test"),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        }
    }

    fn wat_function<'a>(wat: &'a str, symbol: &str) -> &'a str {
        let marker = format!("  (func ${symbol}");
        let start = wat
            .find(&marker)
            .unwrap_or_else(|| panic!("WAT should contain function ${symbol}"));
        let rest = &wat[start..];
        let end = rest[1..]
            .find("\n  (func $")
            .map(|offset| offset + 1)
            .unwrap_or(rest.len());
        &rest[..end]
    }

    fn run_iwasm(wasm_path: &Path) -> String {
        let output = Command::new("iwasm")
            .arg(wasm_path)
            .output()
            .expect("iwasm should run");
        assert!(
            output.status.success(),
            "iwasm failed for {}\nstdout:\n{}\nstderr:\n{}",
            wasm_path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("iwasm stdout should be UTF-8")
    }
}
