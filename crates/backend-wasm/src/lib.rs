mod binary_mvp;
mod capability_manifest;
mod emitter;
mod expr_emit;
mod runtime_arrays_objects;
mod runtime_builder;
mod runtime_builtins_host;
mod runtime_collections;
mod runtime_core;
mod runtime_dates;
mod runtime_fn;
mod runtime_link_plan;
mod runtime_regexp;
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

pub fn emit_wasm_binary_mvp(program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    if let Err(errors) = ts2wasm_ir::lowered::validate_lowered(program) {
        let first = errors.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        });
        return Err(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "refusing to emit wasm binary from invalid lowered IR: [{:?}] {}",
                first.code, first.message
            ),
            span: first.span,
        });
    }
    binary_mvp::emit_wasm_binary_mvp(program)
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
    use super::{
        emit_canonical_manifest_json, emit_wasm_binary_mvp, emit_wat, emitter::LocalFrame,
    };
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};
    use ts2wasm_frontend::DiagCode;
    use ts2wasm_frontend::{Lexer, Parser};
    use ts2wasm_ir::lowered::{
        ClassPrototypeRef, FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr,
        LoweredFunction, LoweredProgram, LoweredStmt, ModuleInfo,
    };
    use ts2wasm_ir::{builtin_resolver, lowered, name_resolver};
    use ts2wasm_runtime_abi::{Layout, ValueTag};

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
    fn emit_wat_rejects_residual_this_before_emission() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::This)],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let err = emit_wat(&program).expect_err("emit_wat must reject residual this");
        assert_eq!(err.code, DiagCode::InvariantViolation);
        assert!(err.message.contains("issue-211: residual `this`"));
    }

    #[test]
    fn direct_wasm_binary_mvp_runs_basics_hello_like_wat_path() {
        let program = lower_fixture("../../fixtures/basics-hello/hello.ts");
        let direct_wasm =
            emit_wasm_binary_mvp(&program).expect("hello fixture should emit direct wasm binary");
        assert_binary_imports_fd_write(&direct_wasm);

        let manifest: serde_json::Value =
            serde_json::from_str(&emit_canonical_manifest_json(&program))
                .expect("manifest should be valid JSON");
        assert_eq!(manifest["wasi"]["stdout"], true);
        assert!(
            manifest["capability_reasons"]["wasi.stdout"]
                .as_array()
                .expect("wasi.stdout should record audit reasons")
                .iter()
                .any(|reason| reason == "console.log")
        );

        let wat = emit_wat(&program).expect("hello fixture should still emit WAT");
        let temp_dir = unique_temp_dir("direct-wasm-binary-mvp");
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let direct_path = temp_dir.join("hello-direct.wasm");
        let wat_path = temp_dir.join("hello-wat.wat");
        let wat_wasm_path = temp_dir.join("hello-wat.wasm");
        fs::write(&direct_path, direct_wasm).expect("direct wasm should be written");
        fs::write(&wat_path, wat).expect("wat should be written");

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
        assert_eq!(direct_out, "hi\n");
        assert_eq!(direct_out, wat_out);

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn direct_wasm_binary_mvp_rejects_non_hello_shape() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::String("hi".to_owned()))],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let err = emit_wasm_binary_mvp(&program).expect_err("non-console.log shape is out of MVP");
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("console.log(<string literal>)"));
    }

    #[test]
    fn alloc_heap_emits_gc_header_and_trigger_contract() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ObjectNew { props: vec![] })],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![],
                body: vec![],
            }],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("object allocation should emit WAT");

        assert!(wat.contains(&format!(
            "(memory (export \"memory\") {} {})",
            Layout::MEMORY_MIN_PAGES,
            Layout::MEMORY_MAX_PAGES
        )));
        assert!(wat.contains("(global $alloc_bytes_since_last_gc (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global $gc_free_list (mut i32) (i32.const 0))"));
        assert!(wat.contains("(func $gc_collect"));
        assert!(wat.contains("(local $header_base i32)"));
        assert!(wat.contains("(local $payload_base i32)"));
        assert!(wat.contains("(i32.const 16)"));
        assert!(wat.contains("(i32.const 65536)"));
        assert!(wat.contains("(local $needed_pages i32)"));
        assert!(wat.contains("(memory.grow (local.get $needed_pages))"));
        assert!(wat.contains("(i32.const -1)"));
        assert!(wat.contains("(global.get $alloc_bytes_since_last_gc)"));
        assert!(wat.contains("(call $gc_collect)"));
        assert!(wat.contains("(global.set $alloc_bytes_since_last_gc"));
        assert!(wat.contains("(local.get $payload_base))"));
    }

    #[test]
    fn gc_sweep_and_free_list_reuse_contract_is_emitted() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ObjectNew { props: vec![] })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("object allocation should emit WAT");

        assert!(wat.contains("(func $gc_sweep"));
        assert!(wat.contains("(global.get $gc_free_list)"));
        assert!(wat.contains("(global.set $gc_free_list (i32.const 0))"));
        assert!(wat.contains("(local $next_body_size i32)"));
        assert!(wat.contains("(loop $coalesce"));
        assert!(wat.contains("(i32.add (i32.const 16) (local.get $next_body_size))"));
        assert!(wat.contains("(global.set $gc_free_list (local.get $cursor))"));
        assert!(wat.contains("(local $free_header i32)"));
        assert!(wat.contains("(local $free_body_size i32)"));
        assert!(wat.contains("(return (i32.add (local.get $free_header) (i32.const 16)))"));
        assert!(wat.contains("(i32.and (local.get $flags) (i32.const -2))"));
    }

    #[test]
    fn concat_allocates_managed_heap_strings() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::Binary {
                op: LoweredBinaryOp::Add,
                left: Box::new(LoweredExpr::String("a".to_owned())),
                right: Box::new(LoweredExpr::String("b".to_owned())),
            })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("string concat should emit WAT");
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
    fn top_level_locals_are_mirrored_into_gc_root_table() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::ObjectNew { props: vec![] },
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("top-level local root should emit WAT");
        let backend_root_count = LocalFrame::new(0, None).backend_local_count();
        let root_count = program.top_level_locals.len() + backend_root_count;
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
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(0)),
                args: vec![],
            })],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![LocalId(0)],
                body: vec![
                    LoweredStmt::Let(LocalId(0), LoweredExpr::ObjectNew { props: vec![] }),
                    LoweredStmt::Return(LoweredExpr::Local(LocalId(0))),
                ],
            }],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("function local root should emit WAT");
        let backend_root_count = LocalFrame::new(0, None).backend_local_count();
        let static_root_bytes = backend_root_count * std::mem::size_of::<u32>();
        let root_bytes = static_root_bytes + Layout::GC_CALL_FRAME_ROOT_STACK_BYTES as usize;
        let activation_root_count = program.functions[0].locals.len() + backend_root_count;
        let activation_frame_bytes = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + activation_root_count * std::mem::size_of::<u32>();
        let backend_last_local = activation_root_count - 1;
        let backend_last_offset = Layout::GC_CALL_FRAME_HEADER_SIZE as usize
            + backend_last_local * std::mem::size_of::<u32>();

        assert!(wat.contains("(global $gc_call_frame_current (mut i32) (i32.const 0))"));
        assert!(wat.contains(&format!(
            "(global.set $gc_root_count (i32.const {backend_root_count}))"
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
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 8)) (local.get 0))"
        ));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 12)) (local.get 1))"
        ));
        assert!(wat.contains(&format!(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const {backend_last_offset})) (local.get {backend_last_local}))"
        )));
        assert!(wat.contains("(call $gc_mark_call_frame_roots"));
        assert!(
            wat.contains("(global.set $gc_call_frame_top (global.get $gc_call_frame_current))")
        );
    }

    #[test]
    fn heap_closure_allocation_and_dispatch_emit_abi_payload_and_roots() {
        let program =
            lower_fixture("../../fixtures/core-semantics/ordinary-function-closure-make-adder.ts");

        let wat = emit_wat(&program).expect("returned closure fixture should emit WAT");

        assert!(wat.contains("(i32.const -2)"));
        assert!(wat.contains("(i32.const 20)"));
        assert!(wat.contains("(i32.const 16)"));
        assert!(wat.contains("(block $heap_closure_dispatch_done (result i32)"));
        assert!(wat.contains("(call $func_1)"));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 8)) (local.get 0))"
        ));
    }

    #[test]
    fn gc_mark_object_payload_marks_heap_closure_capture_slots() {
        let program =
            lower_fixture("../../fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts");

        let wat = emit_wat(&program).expect("returned closure GC fixture should emit WAT");

        assert!(wat.contains("(func $gc_mark_object_payload"));
        assert!(wat.contains("(i32.const -2)"));
        assert!(wat.contains("(i32.const 8)"));
        assert!(wat.contains("(block $closure_done"));
        assert!(wat.contains("(loop $closure_scan"));
        assert!(wat.contains("(i32.const 16)"));
        assert!(wat.contains("(i32.const 4)"));
        assert!(wat.contains("(call $gc_mark_value (i32.load (local.get $entry_ptr)))"));
        assert!(
            wat.contains("(return)))\n    (if (i32.eq (local.get $count) (i32.const -1))"),
            "closure marking must return before ordinary object payload scanning"
        );
    }

    #[test]
    fn env_cells_are_tagged_array_payloads_for_gc_tracing() {
        let program =
            lower_fixture("../../fixtures/core-semantics/class-method-mutable-outer-capture.ts");

        let wat =
            emit_wat(&program).expect("mutable class method env cell fixture should emit WAT");

        assert!(
            wat.contains("(call $alloc_heap (i32.const 8))"),
            "env cells need an array header plus one captured value slot"
        );
        assert!(
            wat.contains("(i32.const 1))"),
            "env cell payload should use array length 1 so GC scans its value slot"
        );
        assert!(
            wat.contains(&format!("(i32.const {}))", ValueTag::ARRAY_TAG)),
            "env cell roots/captures must hold a tagged heap value"
        );
        assert!(
            wat.contains(&format!(
                "(i32.load (i32.add (i32.and (local.get 0) (i32.const {})) (i32.const 4)))",
                ValueTag::HEAP_MASK
            )),
            "env cell reads should mask the tagged cell before loading the value slot"
        );
        assert!(
            wat.contains(&format!(
                "(i32.store (i32.add (i32.and (local.get 1) (i32.const {})) (i32.const 4))",
                ValueTag::HEAP_MASK
            )),
            "env cell writes should mask the tagged captured cell before storing the value slot"
        );
        assert!(
            wat.contains("(call $gc_mark_value (i32.load (local.get $elem_ptr)))"),
            "tagged env cells should be traced through the existing array GC scanner"
        );
    }

    #[test]
    fn gc_mark_helpers_visit_heap_graph_payloads() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ObjectNew {
                props: vec![(
                    "child".to_owned(),
                    LoweredExpr::ArrayNew { elements: vec![] },
                )],
            })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("object graph allocation should emit WAT");

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
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ModuleLoad { module_id: 1 })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![ModuleInfo {
                id: 1,
                specifier: "./dep".to_owned(),
                statements: vec![],
                locals_count: 0,
            }],
        };

        let wat = emit_wat(&program).expect("module runtime should emit WAT");

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
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::Number(1))],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("non-module program should emit WAT");

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
                    obj: Box::new(LoweredExpr::ModuleLoad { module_id: 1 }),
                    key: "value".to_owned(),
                },
            )],
            top_level_locals: vec![LocalId(0)],
            functions: vec![],
            modules: vec![
                ModuleInfo {
                    id: 2,
                    specifier: "./nested".to_owned(),
                    statements: vec![LoweredStmt::Export {
                        name: "nested".to_owned(),
                        expr: LoweredExpr::Number(2),
                    }],
                    locals_count: 0,
                },
                ModuleInfo {
                    id: 1,
                    specifier: "./source".to_owned(),
                    statements: vec![LoweredStmt::Export {
                        name: "value".to_owned(),
                        expr: LoweredExpr::Number(1),
                    }],
                    locals_count: 0,
                },
            ],
        };

        let wat = emit_wat(&program).expect("module initializers should emit WAT");

        assert!(wat.contains("(func $module_init_2"));
        assert!(wat.contains("(func $module_init_1"));
        assert!(wat.contains("(global.set $current_module_id (i32.const 2))"));
        assert!(wat.contains("(global.set $current_module_id (i32.const 1))"));
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
    }

    #[test]
    fn gc_collect_marks_class_prototype_globals() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ClassPrototype(
                ClassPrototypeRef {
                    constructor: FuncId(0),
                    parent_constructors: vec![],
                },
            ))],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                uses_receiver: false,
                min_required_params: 0,
                rest_param_index: None,
                locals: vec![],
                body: vec![],
            }],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("class prototype root should emit WAT");

        assert!(wat.contains("(global $class_proto_0 (mut i32) (i32.const 0))"));
        assert!(wat.contains("(call $gc_mark_value (i32.or (global.get $class_proto_0)"));
        assert!(wat.contains("(i32.const 7)"));
    }

    #[test]
    fn math_random_imports_wasi_random_get() {
        let program = math_random_program();

        let wat = emit_wat(&program).expect("Math.random should emit with WASI random");

        assert!(wat.contains("(import \"wasi_snapshot_preview1\" \"random_get\""));
        assert!(wat.contains("(call $random_get"));
        assert!(!wat.contains("$random_counter"));
    }

    #[test]
    fn math_random_manifest_declares_wasi_random() {
        let program = math_random_program();

        let manifest: serde_json::Value =
            serde_json::from_str(&emit_canonical_manifest_json(&program))
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
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::RuntimeCall {
                runtime_fn: "MathRandom".to_owned(),
                args: vec![],
            })],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        }
    }

    fn lower_fixture(relative_path: &str) -> LoweredProgram {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
        let source = fs::read_to_string(&path).expect("fixture should be readable");
        let tokens = Lexer::new(&source)
            .tokenize()
            .expect("fixture should tokenize");
        let parsed = Parser::new(tokens)
            .parse_program()
            .expect("fixture should parse");
        let named = name_resolver::resolve_names(&parsed).expect("fixture should resolve names");
        let resolved =
            builtin_resolver::resolve_builtins(&named).expect("fixture should resolve builtins");
        let lowered = lowered::lower_program(&resolved).expect("fixture should lower");
        lowered::validate_lowered(&lowered).expect("fixture lowered IR should validate");
        lowered
    }

    fn assert_binary_imports_fd_write(wasm: &[u8]) {
        assert!(
            wasm.windows(b"wasi_snapshot_preview1".len())
                .any(|window| window == b"wasi_snapshot_preview1")
        );
        assert!(
            wasm.windows(b"fd_write".len())
                .any(|window| window == b"fd_write")
        );
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

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX_EPOCH")
            .as_nanos();
        std::env::temp_dir().join(format!("ts2wasm-{label}-{unique}-{}", std::process::id()))
    }
}
