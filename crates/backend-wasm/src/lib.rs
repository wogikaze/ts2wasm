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
    use ts2wasm_ir::lowered::{
        ClassPrototypeRef, FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr,
        LoweredFunction, LoweredProgram, LoweredStmt, ModuleInfo,
    };

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
    fn alloc_heap_emits_gc_header_and_trigger_contract() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::ObjectNew { props: vec![] })],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                min_required_params: 0,
                locals: vec![],
                body: vec![],
            }],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("object allocation should emit WAT");

        assert!(wat.contains("(memory (export \"memory\") 2 16)"));
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
        assert!(wat.contains("(then (call $gc_collect))"));
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

        assert!(wat.contains("(global $gc_root_base (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global $gc_root_count (mut i32) (i32.const 0))"));
        assert!(wat.contains("(global.set $gc_root_count (i32.const 4))"));
        assert!(wat.contains("(global.set $gc_root_base (call $alloc_heap (i32.const 16)))"));
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
    fn function_locals_are_mirrored_into_gc_root_table() {
        let program = LoweredProgram {
            top_level_statements: vec![LoweredStmt::Expr(LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(0)),
                args: vec![],
            })],
            top_level_locals: vec![],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![],
                min_required_params: 0,
                locals: vec![LocalId(0)],
                body: vec![
                    LoweredStmt::Let(LocalId(0), LoweredExpr::ObjectNew { props: vec![] }),
                    LoweredStmt::Return(LoweredExpr::Local(LocalId(0))),
                ],
            }],
            modules: vec![],
        };

        let wat = emit_wat(&program).expect("function local root should emit WAT");

        assert!(wat.contains("(global.set $gc_root_count (i32.const 7))"));
        assert!(wat.contains("(global.set $gc_root_base (call $alloc_heap (i32.const 28)))"));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_root_base) (i32.const 12)) (local.get 0))"
        ));
        assert!(wat.contains(
            "(i32.store (i32.add (global.get $gc_root_base) (i32.const 16)) (local.get 1))"
        ));
        assert!(wat.contains("(call $gc_mark_registered_roots"));
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
                min_required_params: 0,
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
}
