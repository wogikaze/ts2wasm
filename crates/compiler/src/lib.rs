mod dump;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use ts2wasm_backend_wasm as backend;
#[cfg(test)]
use ts2wasm_frontend::{BinaryOp, Expr};
use ts2wasm_frontend::{
    DiagCode, Diagnostic, Lexer, Parser, Stmt, validate_type_reference_directives,
};
use ts2wasm_ir::builtin_resolver;
use ts2wasm_ir::lowered;
use ts2wasm_ir::name_resolver;

const ENABLE_READ_STDIN_BYTES_RUNTIME: bool = true;

pub use dump::{DumpOptions, DumpPhase, dump_file_with_options};
pub use ts2wasm_frontend::{
    TypeScriptCheckReport, TypeScriptDiagnostic, check_typescript_file,
    collect_typescript_diagnostics,
};
pub use ts2wasm_ir::OptimizationLevel;

pub fn build_file(input: &Path, output: &Path) -> Result<(), Diagnostic> {
    build_file_with_options(input, output, None)
}

pub fn build_file_with_options(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
) -> Result<(), Diagnostic> {
    build_file_with_host_deny(input, output, capability_manifest_output, false)
}

pub fn build_file_with_host_deny(
    input: &Path,
    output: &Path,
    capability_manifest_output: Option<&Path>,
    host_deny: bool,
) -> Result<(), Diagnostic> {
    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,
    })?;
    validate_type_reference_directives(&source)?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_ast(&program)?;
    let name_resolved = name_resolver::resolve_names(&program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered = lowered::lower_program(&resolved)?;
    lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        })
    })?;
    ensure_runtime_feature_gates(&lowered)?;

    if host_deny {
        validate_host_deny(&lowered)?;
    }

    if let Some(path) = capability_manifest_output {
        let manifest = backend::emit_canonical_manifest_json(&lowered);
        fs::write(path, manifest).map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to write {}: {error}", path.display()),
            span: None,
        })?;
    }
    let wat = backend::emit_wat(&lowered)?;
    write_wasm_from_wat(&wat, output)
}

fn validate_optimized_hir_slice(
    resolved: &[ts2wasm_ir::ResolvedStmt],
    level: OptimizationLevel,
) -> Result<(), Diagnostic> {
    match ts2wasm_ir::semantic::lower_to_hir(resolved) {
        Ok(hir) => dump::optimize_typed_ir(&hir, level).map(|_| ()),
        Err(error) if error.code == DiagCode::UnsupportedSyntax => Ok(()),
        Err(error) => Err(error),
    }
}

fn ensure_runtime_feature_gates(lowered: &lowered::LoweredProgram) -> Result<(), Diagnostic> {
    if ENABLE_READ_STDIN_BYTES_RUNTIME {
        return Ok(());
    }
    if backend::program_requires_read_stdin_bytes_runtime(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "require(\"fs\").readFileSync(0, \"utf8\") is lowered to byte-backed runtime path, but runtime execution is disabled"
                .to_owned(),
            span: None,
        });
    }
    Ok(())
}

fn validate_host_deny(lowered: &lowered::LoweredProgram) -> Result<(), Diagnostic> {
    // Check if any Node host imports are required
    if backend::has_node_host_imports(lowered) {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "host-deny mode rejects Node host imports".to_owned(),
            span: None,
        });
    }

    Ok(())
}

pub fn parse_program(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens).parse_program()
}

fn validate_ast(program: &[Stmt]) -> Result<(), Diagnostic> {
    let mut top_functions = HashMap::new();
    let mut top_scope = HashMap::new();

    for stmt in program {
        match stmt {
            Stmt::Return { span, .. } => {
                return Err(Diagnostic {
                    code: DiagCode::InvalidTopLevelReturn,
                    message: "top-level return is not supported".to_owned(),
                    span: Some(*span),
                });
            }
            Stmt::Function {
                name, body, span, ..
            } => {
                if top_scope.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateLocal,
                        message: format!(
                            "top-level function `{name}` conflicts with existing lexical binding"
                        ),
                        span: Some(*span),
                    });
                }
                if top_functions.contains_key(name) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: Some(*span),
                    });
                }
                top_functions.insert(name.clone(), ());
                validate_block(body)?;
            }
            _ => validate_stmt(stmt, true, &mut top_scope, &top_functions)?,
        }
    }

    Ok(())
}

fn validate_block(statements: &[Stmt]) -> Result<(), Diagnostic> {
    let mut scope = HashMap::new();
    let functions = HashMap::new();
    for stmt in statements {
        validate_stmt(stmt, false, &mut scope, &functions)?;
    }
    Ok(())
}

fn validate_class_body(statements: &[Stmt]) -> Result<(), Diagnostic> {
    for stmt in statements {
        match stmt {
            Stmt::Function { body, .. } => validate_block(body)?,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: "class body currently supports methods only".to_owned(),
                    span: Some(stmt.span()),
                });
            }
        }
    }
    Ok(())
}

fn validate_stmt(
    stmt: &Stmt,
    in_top_level: bool,
    scope: &mut HashMap<String, ()>,
    top_functions: &HashMap<String, ()>,
) -> Result<(), Diagnostic> {
    match stmt {
        Stmt::Let { name, span, .. } => {
            if in_top_level && top_functions.contains_key(name) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateLocal,
                    message: format!(
                        "top-level lexical binding `{name}` conflicts with function declaration"
                    ),
                    span: Some(*span),
                });
            }
            if scope.contains_key(name) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateLocal,
                    message: format!("duplicate local binding: `{name}`"),
                    span: Some(*span),
                });
            }
            scope.insert(name.clone(), ());
            Ok(())
        }
        Stmt::Return { span, .. } if in_top_level => Err(Diagnostic {
            code: DiagCode::InvalidTopLevelReturn,
            message: "top-level return is not supported".to_owned(),
            span: Some(*span),
        }),
        Stmt::Return { .. } => Ok(()),
        Stmt::If {
            then_body,
            else_body,
            ..
        } => {
            validate_block(then_body)?;
            validate_block(else_body)?;
            Ok(())
        }
        Stmt::While { body, .. } => validate_block(body),
        Stmt::DoWhile { body, .. } => validate_block(body),
        Stmt::For { body, .. } => validate_block(body),
        Stmt::ForIn { body, .. } => validate_block(body),
        Stmt::ForOf { body, .. } => validate_block(body),
        Stmt::Switch { cases, .. } => {
            for (_, case_body) in cases {
                validate_block(case_body)?;
            }
            Ok(())
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            validate_block(try_block)?;
            if let Some(catch) = catch_block {
                validate_block(catch)?;
            }
            if let Some(finally) = finally_block {
                validate_block(finally)?;
            }
            Ok(())
        }
        Stmt::ClassDecl { body, .. } => validate_class_body(body),
        Stmt::Expr { .. } => Ok(()),
        Stmt::Function { span, .. } => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "nested function declarations are not supported in this milestone".to_owned(),
            span: Some(*span),
        }),
        Stmt::Throw { .. } => Ok(()),
        Stmt::Labeled { body, .. } => validate_stmt(body, in_top_level, scope, top_functions),
        Stmt::Break { .. } => Ok(()),
        Stmt::Continue { .. } => Ok(()),
        Stmt::Assign { .. } => Ok(()),
        Stmt::ImportSideEffect { .. }
        | Stmt::ImportNamed { .. }
        | Stmt::ImportDefault { .. }
        | Stmt::ImportNamespace { .. }
        | Stmt::ExportNamed { .. }
        | Stmt::ExportNamedFrom { .. } => Ok(()),
    }
}

fn write_wasm_from_wat(wat: &str, output: &Path) -> Result<(), Diagnostic> {
    use std::sync::atomic::{AtomicU32, Ordering};
    static WAT_COUNTER: AtomicU32 = AtomicU32::new(0);
    let unique = WAT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let wat_path =
        std::env::temp_dir().join(format!("ts2wasm-{}-{}.wat", std::process::id(), unique));
    fs::write(&wat_path, wat).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!(
            "failed to write temporary wat {}: {error}",
            wat_path.display()
        ),
        span: None,
    })?;
    let command_output = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|error| Diagnostic {
            code: DiagCode::BackendIo,
            message: format!("failed to execute wat2wasm: {error}"),
            span: None,
        })?;

    let _ = fs::remove_file(&wat_path);

    if command_output.status.success() {
        Ok(())
    } else {
        Err(Diagnostic {
            code: DiagCode::BackendIo,
            message: format!(
                "wat2wasm failed\nstdout:\n{}\nstderr:\n{}\nwat:\n{}",
                String::from_utf8_lossy(&command_output.stdout),
                String::from_utf8_lossy(&command_output.stderr),
                wat
            ),
            span: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_console_log_string() {
        let program = parse_program("console.log(\"hi\");").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Expr {
                expr: Expr::Call { callee, args, .. },
                ..
            } => {
                assert!(matches!(
                    args.as_slice(),
                    [Expr::String { value, .. }] if value == "hi"
                ));
                assert!(matches!(
                    callee.as_ref(),
                    Expr::Member { property, .. } if property == "log"
                ));
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_m2_subset() {
        let source = r#"
            let i = 0;
            let sum = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            function add(a, b) { return a + b; }
            if (true) { console.log("sum=" + sum); } else { console.log("bad"); }
            console.log(add(2, 3));
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 6);
    }

    #[test]
    fn parses_m3_semantics() {
        let source = r#"
            console.log(undefined);
            console.log(null);
            console.log(null === undefined);
            console.log("x" + true);
            if (!0) { console.log("zero false"); }
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 5);
    }

    #[test]
    fn parses_program_with_utf8_bom() {
        let program = parse_program("\u{feff}console.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_line_comment_prefix() {
        let program = parse_program("// lead comment\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_block_comment_prefix() {
        let program = parse_program("/*--- metadata ---*/\nconsole.log(1);").unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn parses_program_with_dollar_identifier() {
        let program = parse_program("let $done = 1; console.log($done);").unwrap();
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let err = parse_program("/* unterminated").unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
        assert!(err.message.contains("unterminated block comment"));
        assert!(err.span.is_some());
    }

    #[test]
    fn parses_const_statement() {
        let program = parse_program("const x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_var_statement() {
        let program = parse_program("var x = 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Number { value, .. } => assert_eq!(*value, 1),
                    _ => panic!("expected number expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn rejects_top_level_return_in_ast_validation() {
        let program = parse_program("return 1;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::InvalidTopLevelReturn);
    }

    #[test]
    fn rejects_nested_function_in_ast_validation() {
        let program = parse_program("if (true) { function f() { return 1; } }").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    }

    #[test]
    fn rejects_duplicate_let_in_same_scope() {
        let program = parse_program("let x = 1; let x = 2;").unwrap();
        let err = validate_ast(&program).unwrap_err();
        assert_eq!(err.code, DiagCode::DuplicateLocal);
        assert!(err.span.is_some());
    }

    #[test]
    fn m6_3b_1_runtime_gate_permits_read_stdin_bytes_execution_path() {
        let ast = parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
        let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
        ensure_runtime_feature_gates(&lowered)
            .expect("gate must pass after M6-3b-1 enables runtime");
    }

    #[test]
    fn parses_logical_and_operator() {
        let program = parse_program("let x = 1 && 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::And));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_logical_or_operator() {
        let program = parse_program("let x = 1 || 2;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Or));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_greater_than_operator() {
        let program = parse_program("let x = 5 > 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Binary { op, .. } => {
                        assert!(matches!(op, BinaryOp::Greater));
                    }
                    _ => panic!("expected binary expression"),
                }
            }
            other => panic!("unexpected stmt: {other:?}"),
        }
    }

    #[test]
    fn parses_typeof_operator() {
        let program = parse_program("let t = typeof x;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "t");
                assert!(matches!(expr, Expr::TypeOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_typescript_type_annotations_as_syntax_only() {
        let source = r#"
            function add(a: number, b: number): number { return a + b; }
            const limit: number = 4;
            let done: boolean = limit >= 4;
            console.log(add(limit, 2), done);
        "#;
        let program = parse_program(source).unwrap();
        assert_eq!(program.len(), 4);
        match &program[0] {
            Stmt::Function { params, .. } => {
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].0, "a");
                assert_eq!(params[1].0, "b");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_instanceof_expression() {
        let program = parse_program("let b = x instanceof Array;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "b");
                assert!(matches!(expr, Expr::InstanceOf { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_ternary_expression() {
        let program = parse_program("let x = a ? b : c;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "x");
                assert!(matches!(expr, Expr::Ternary { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_arrow_function_single_param() {
        let program = parse_program("let f = x => x + 1;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "f");
                assert!(matches!(expr, Expr::ArrowFn { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_new_expression() {
        let program = parse_program("let obj = new Array(10);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { name, expr, .. } => {
                assert_eq!(name, "obj");
                assert!(matches!(expr, Expr::New { .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_do_while_loop() {
        let program = parse_program("do { x = 1; } while (x);").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::DoWhile { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_for_loop_with_init_cond_update() {
        // For loop variant (full traditional for loop)
        // Note: Parser supports for statement dispatch, full expression parsing in for update may be deferred
        let program = parse_program("for (;;) { break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::For { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_power_operator() {
        let program = parse_program("let p = 2 ** 3;").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Let { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::Binary {
                        op: BinaryOp::Power,
                        ..
                    }
                ));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_bitwise_operators() {
        let program = parse_program("let b = (a & b) | (c ^ d) | ~e;").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start < usize::MAX);
    }

    #[test]
    fn parses_shift_operators() {
        let program = parse_program("let s = (a << 2) | (b >> 1) | (c >>> 3);").unwrap();
        assert_eq!(program.len(), 1);
        let span = program[0].span();
        assert!(span.start < usize::MAX);
    }

    #[test]
    fn parses_throw_statement() {
        let program = parse_program("throw new Error();").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Throw { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_try_catch_finally() {
        let program = parse_program("try { x = 1; } catch (e) { } finally { cleanup(); }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::TryCatch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn parses_switch_statement() {
        let program = parse_program("switch (x) { case 1: break; default: break; }").unwrap();
        assert_eq!(program.len(), 1);
        match &program[0] {
            Stmt::Switch { .. } => {}
            other => panic!("unexpected: {other:?}"),
        }
    }
}
