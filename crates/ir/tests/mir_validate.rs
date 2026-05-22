use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_frontend::Span;
use ts2wasm_ir::lowered::{
    FuncId, FunctionCallKind, LocalId, MirExpr, MirFunction, MirProgram, MirStmt, ModuleInfo,
    ModuleLoadKind, Validated, validate_mir,
};

fn span() -> Span {
    Span::default()
}

fn empty_mir() -> MirProgram {
    MirProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
        escape_status: vec![],
    }
}

fn function(id: usize) -> MirFunction {
    MirFunction {
        id: FuncId(id),
        params: vec![],
        uses_receiver: false,
        min_required_params: 0,
        rest_param_index: None,
        locals: vec![],
        body: vec![MirStmt::Return(MirExpr::Number(1, span()), span())],
        recursion_depth: 0,
        is_async: false,
        is_generator: false,
        generator_state: None,
        induction_vars: vec![],
        escape_status: vec![],
        value_reps: vec![],
    }
}

fn assert_invariant(result: Result<(), Vec<Diagnostic>>, message: &str) {
    let diagnostics = result.expect_err("expected MIR validation to fail");
    assert!(
        diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvariantViolation && d.message.contains(message)),
        "expected InvariantViolation containing {message:?}, got {diagnostics:?}"
    );
}

#[test]
fn native_mir_validate_accepts_valid_program() {
    let program = MirProgram {
        top_level_statements: vec![
            MirStmt::Let(LocalId(0), MirExpr::Number(42, span()), span()),
            MirStmt::Expr(
                MirExpr::Call {
                    kind: FunctionCallKind::User(FuncId(0)),
                    args: vec![],
                    span: span(),
                },
                span(),
            ),
        ],
        top_level_locals: vec![LocalId(0)],
        functions: vec![function(0)],
        modules: vec![ModuleInfo {
            id: 0,
            specifier: "<entry>".to_owned(),
            statements: vec![],
            locals_count: 0,
        }],
        escape_status: vec![],
    };

    validate_mir(&program).expect("valid native MIR should pass");
}

#[test]
fn validated_native_mir_rejects_invalid_local_id_as_fatal() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Expr(MirExpr::Local(LocalId(1), span()), span())],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };

    let err = Validated::<MirProgram>::new_mir(program).expect_err("invalid MIR should be fatal");
    assert_eq!(err.code, DiagCode::InvariantViolation);
    assert!(err.message.contains("LocalId 1 is out of range"));
}

#[test]
fn native_mir_validate_rejects_non_contiguous_function_id() {
    let program = MirProgram {
        functions: vec![function(1)],
        ..empty_mir()
    };

    assert_invariant(
        validate_mir(&program),
        "function id 1 does not match its index 0",
    );
}

#[test]
fn native_mir_validate_rejects_invalid_function_reference() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Expr(
            MirExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args: vec![],
                span: span(),
            },
            span(),
        )],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "FuncId 2 is out of range");
}

#[test]
fn native_mir_validate_rejects_invalid_module_reference() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Expr(
            MirExpr::ModuleLoad {
                module_id: 7,
                kind: ModuleLoadKind::StaticRequire,
                span: span(),
            },
            span(),
        )],
        modules: vec![ModuleInfo {
            id: 0,
            specifier: "<entry>".to_owned(),
            statements: vec![],
            locals_count: 0,
        }],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "ModuleLoad references module_id 7");
}

#[test]
fn native_mir_validate_rejects_invalid_class_method_reference() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::ClassDecl {
            name: "C".to_owned(),
            extends: None,
            constructor: None,
            methods: vec![("m".to_owned(), FuncId(3))],
            static_methods: vec![],
            private_fields: vec![],
            span: span(),
        }],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "FuncId 3 is out of range");
}

#[test]
fn native_mir_validate_rejects_top_level_return() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Return(MirExpr::Number(1, span()), span())],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "top-level return is invalid in MIR");
}

#[test]
fn native_mir_validate_rejects_try_without_catch_or_finally() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::TryCatch {
            try_body: vec![],
            catch_var: None,
            catch_body: None,
            finally_body: None,
            span: span(),
        }],
        ..empty_mir()
    };

    assert_invariant(
        validate_mir(&program),
        "try-catch must have at least a catch or finally block",
    );
}

#[test]
fn mir_rejects_host_import_strings() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Expr(
            MirExpr::String("wasi_snapshot_preview1.fd_write".to_owned(), span()),
            span(),
        )],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "wasi_snapshot_preview1");
}

#[test]
fn mir_rejects_host_import_string_in_local() {
    let program = MirProgram {
        top_level_statements: vec![MirStmt::Let(
            LocalId(0),
            MirExpr::String("host.fs.readFileSync".to_owned(), span()),
            span(),
        )],
        top_level_locals: vec![LocalId(0)],
        ..empty_mir()
    };

    assert_invariant(validate_mir(&program), "host.");
}
