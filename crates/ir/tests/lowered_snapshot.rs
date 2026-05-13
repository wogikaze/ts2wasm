//! Lowered snapshot tests — verify full pipeline output structure.
//!
//! These tests parse source code, run the builtin resolver, lower to
//! LoweredProgram, and verify the resulting LoweredStmt / LoweredExpr
//! trees have the expected shape. This is a higher-fidelity test than
//! resolver_snapshot because it exercises the full resolver + lowering chain.

use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::builtin_resolver::resolve_builtins;
use ts2wasm_ir::lowered::validate::validate_lowered;
use ts2wasm_ir::lowered::{
    FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
    ModuleLoadKind,
};
use ts2wasm_ir::lowered::{lower_program, lower_program_with_module_url};

fn parse_resolve_lower(source: &str) -> LoweredProgram {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let stmts = Parser::new(tokens, source).parse_program().unwrap();
    let resolved = resolve_builtins(&stmts).unwrap();
    lower_program(&resolved).unwrap()
}

fn parse_resolve_lower_result(
    source: &str,
) -> Result<LoweredProgram, ts2wasm_diagnostic::Diagnostic> {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let stmts = Parser::new(tokens, source).parse_program().unwrap();
    let resolved = resolve_builtins(&stmts).unwrap();
    lower_program(&resolved)
}

fn parse_resolve_lower_with_module_url(source: &str, module_url: &str) -> LoweredProgram {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let stmts = Parser::new(tokens, source).parse_program().unwrap();
    let resolved = resolve_builtins(&stmts).unwrap();
    lower_program_with_module_url(&resolved, module_url).unwrap()
}

fn lowered_stmt_contains_class_prototype(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Expr(expr, _)
        | LoweredStmt::Return(expr, _)
        | LoweredStmt::Throw(expr, _) => lowered_expr_contains_class_prototype(expr),
        LoweredStmt::Block(stmts, _) => stmts.iter().any(lowered_stmt_contains_class_prototype),
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            lowered_expr_contains_class_prototype(condition)
                || then_body.iter().any(lowered_stmt_contains_class_prototype)
                || else_body.iter().any(lowered_stmt_contains_class_prototype)
        }
        _ => false,
    }
}

fn lowered_expr_contains_class_prototype(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::ClassPrototype(_, _) => true,
        LoweredExpr::Block { stmts, result, .. } => {
            stmts.iter().any(lowered_stmt_contains_class_prototype)
                || lowered_expr_contains_class_prototype(result)
        }
        LoweredExpr::Call { args, .. }
        | LoweredExpr::RuntimeCall { args, .. }
        | LoweredExpr::ArrayNew { elements: args, .. } => {
            args.iter().any(lowered_expr_contains_class_prototype)
        }
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .any(|(_, value)| lowered_expr_contains_class_prototype(value)),
        _ => false,
    }
}

#[test]
fn lowered_snapshot_empty() {
    let program = parse_resolve_lower("");
    assert!(
        program.top_level_statements.is_empty(),
        "empty input should have no top-level statements"
    );
    assert!(
        program.functions.is_empty(),
        "empty input should have no functions"
    );
}

#[test]
fn lowered_snapshot_let_number() {
    let program = parse_resolve_lower("let x = 42;");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::Number(42, _), _) => {}
        other => panic!("expected LoweredStmt::Let(0, Number(42)), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_let_string() {
    let program = parse_resolve_lower(r#"let s = "hello";"#);
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::String(value, _), _) => {
            assert_eq!(value, "hello");
        }
        other => panic!("expected LoweredStmt::Let(_, String), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_import_meta_url() {
    let program = parse_resolve_lower_with_module_url(
        r#"let url = import.meta.url; let meta = import.meta;"#,
        "./dep.ts",
    );
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::String(value, _), _) => {
            assert_eq!(value, "./dep.ts");
        }
        other => panic!("expected import.meta.url to lower to module URL, got: {other:?}"),
    }
    match &program.top_level_statements[1] {
        LoweredStmt::Let(
            _,
            LoweredExpr::ObjectNew {
                props,
                non_enumerable,
                ..
            },
            _,
        ) => {
            assert_eq!(*non_enumerable, 0);
            assert_eq!(props.len(), 1);
            assert_eq!(props[0].0, "url");
            assert!(matches!(
                &props[0].1,
                LoweredExpr::String(value, _) if value == "./dep.ts"
            ));
        }
        other => panic!("expected import.meta to lower to metadata object, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_dynamic_import_module_load() {
    let program = parse_resolve_lower(r#"let ns = import("./dep.ts");"#);
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(
            _,
            LoweredExpr::ModuleLoad {
                module_id,
                kind: ModuleLoadKind::DynamicImport,
                ..
            },
            _,
        ) => {
            assert_eq!(*module_id, 1);
        }
        other => {
            panic!("expected dynamic import to lower to DynamicImport ModuleLoad, got: {other:?}")
        }
    }
    assert_eq!(program.modules.len(), 1);
    assert_eq!(program.modules[0].id, 1);
    assert_eq!(program.modules[0].specifier, "./dep.ts");
}

#[test]
fn lowered_snapshot_new_target_arrow_inherits_constructor_context() {
    let program = parse_resolve_lower("class C { constructor() { let f = () => new.target; } }");
    assert!(
        program.functions.iter().any(|function| function
            .body
            .iter()
            .any(lowered_stmt_contains_class_prototype)),
        "expected arrow function lowered body to preserve constructor new.target"
    );
}

#[test]
fn lowered_snapshot_new_target_outside_constructor_lowers_to_undefined() {
    let program = parse_resolve_lower("function f() { return new.target; }");
    assert!(
        program.functions.iter().any(|function| {
            matches!(
                function.body.as_slice(),
                [LoweredStmt::Return(LoweredExpr::Undefined(_), _)]
            )
        }),
        "expected non-constructor new.target to lower to undefined"
    );
}

#[test]
fn lowered_snapshot_let_bool() {
    let program = parse_resolve_lower("let a = true; let b = false;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::Bool(true, _), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Bool(true)), got: {other:?}"),
    }
    match &program.top_level_statements[1] {
        LoweredStmt::Let(_, LoweredExpr::Bool(false, _), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Bool(false)), got: {other:?}"),
    }
}

#[test]
fn strict_function_direct_this_call_passes_undefined_receiver() {
    let program = parse_resolve_lower(
        r#"
        function read() {
          "use strict";
          return this;
        }
        let value = read();
        "#,
    );

    let read = &program.functions[0];
    assert!(
        program.top_level_statements.iter().any(|stmt| matches!(
            stmt,
            LoweredStmt::Let(
                _,
                LoweredExpr::Call {
                    kind: FunctionCallKind::User(_),
                    args,
                    ..
                },
                _
            ) if matches!(args.as_slice(), [LoweredExpr::Undefined(_)])
        )),
        "strict direct function call should pass undefined as receiver: {program:?}"
    );
    assert_eq!(read.params.len(), 1);
    assert!(read.uses_receiver);
}

#[test]
fn strict_function_expression_iife_return_this_lowers_to_undefined() {
    let program = parse_resolve_lower(
        r#"
        let value = (function() {
          "use strict";
          return this;
        })();
        "#,
    );

    assert!(matches!(
        program.top_level_statements.as_slice(),
        [LoweredStmt::Let(_, LoweredExpr::Undefined(_), _)]
    ));
}

#[test]
fn strict_delete_identifier_reports_strict_delete_check() {
    let err = parse_resolve_lower_result(
        r#"
        "use strict";
        let value = 1;
        delete value;
        "#,
    )
    .unwrap_err();

    assert!(
        err.message.contains("StrictDelete"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn lowered_snapshot_null_undefined() {
    let program = parse_resolve_lower("let n = null; let u = undefined;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(_, LoweredExpr::Null(_), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Null), got: {other:?}"),
    }
    match &program.top_level_statements[1] {
        LoweredStmt::Let(_, LoweredExpr::Undefined(_), _) => {}
        other => panic!("expected LoweredStmt::Let(_, Undefined), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_binary_addition() {
    let program = parse_resolve_lower("1 + 2;");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Binary {
                left,
                right,
                op: LoweredBinaryOp::Add,
                ..
            },
            _,
        ) => {
            assert!(matches!(left.as_ref(), LoweredExpr::Number(1, _)));
            assert!(matches!(right.as_ref(), LoweredExpr::Number(2, _)));
        }
        other => panic!("expected LoweredExpr::Binary(Add), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_var_declaration() {
    let program = parse_resolve_lower("var y = \"str\";");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::String(value, _), _) => {
            assert_eq!(value, "str");
        }
        other => panic!("expected LoweredStmt::Let(_, String), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_function_decl() {
    let program = parse_resolve_lower("function f() { return 42; }");
    assert!(
        !program.functions.is_empty(),
        "should have at least one function"
    );
    assert_eq!(program.top_level_statements.len(), 1);
}

#[test]
fn lowered_snapshot_generator_function_metadata() {
    let program = parse_resolve_lower("function* gen() {}");
    assert_eq!(program.functions.len(), 1);
    let function = &program.functions[0];
    assert!(function.is_generator);
    assert!(!function.is_async);
    let generator_state = function
        .generator_state
        .as_ref()
        .expect("generator functions should carry generator state metadata");
    assert!(generator_state.suspend_points.is_empty());
    assert_eq!(generator_state.completed_state, 0);
}

#[test]
fn lowered_snapshot_generator_yields_suspend_points() {
    let program = parse_resolve_lower("function* gen() { yield 1; yield 2; }");
    validate_lowered(&program).expect("generator yield lowered IR should validate");
    assert_eq!(program.functions.len(), 1);
    let function = &program.functions[0];
    assert!(function.is_generator);
    let generator_state = function
        .generator_state
        .as_ref()
        .expect("generator function should carry generator state");
    assert_eq!(generator_state.suspend_points.len(), 2);
    assert_eq!(generator_state.suspend_points[0].index, 0);
    assert_eq!(generator_state.suspend_points[0].resume_state, 1);
    assert_eq!(generator_state.suspend_points[1].index, 1);
    assert_eq!(generator_state.suspend_points[1].resume_state, 2);
    assert_eq!(generator_state.completed_state, 3);
}

#[test]
fn lowered_snapshot_for_await_of_keeps_async_iterator_ir() {
    let program = parse_resolve_lower(
        "async function f(values) { for await (let value of values) { console.log(value); } }",
    );
    validate_lowered(&program).expect("for-await-of lowered IR should validate");
    assert_eq!(program.functions.len(), 1);
    let function = &program.functions[0];
    assert!(function.is_async);
    match &function.body[0] {
        LoweredStmt::ForAwaitOfLower {
            var,
            iter,
            async_iter_local,
            next_result_local,
            done_local,
            value_local,
            body,
            ..
        } => {
            assert_ne!(var, async_iter_local);
            assert_ne!(async_iter_local, next_result_local);
            assert_ne!(next_result_local, done_local);
            assert_ne!(done_local, value_local);
            assert!(matches!(iter, LoweredExpr::Local(LocalId(0), _)));
            assert!(!body.is_empty());
        }
        other => panic!("expected ForAwaitOfLower, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_if_statement() {
    let program = parse_resolve_lower("if (true) { let x = 1; } else { let x = 0; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            assert!(matches!(condition, LoweredExpr::Bool(true, _)));
            assert!(!then_body.is_empty(), "then body should not be empty");
            assert!(!else_body.is_empty(), "else body should not be empty");
        }
        other => panic!("expected LoweredStmt::If, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_while_loop() {
    let program = parse_resolve_lower("while (true) { break; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::While { .. } => {}
        other => panic!("expected LoweredStmt::While, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_produces_validated() {
    // Verify that the lowered program passes validation
    let program = parse_resolve_lower("let x = 42; console.log(x);");
    let result = validate_lowered(&program);
    assert!(result.is_ok(), "validation should pass: {:?}", result.err());
}

#[test]
fn lowered_snapshot_assignment() {
    let program = parse_resolve_lower("let x = 1; x = 42;");
    assert_eq!(program.top_level_statements.len(), 2);
    match &program.top_level_statements[1] {
        LoweredStmt::Assign(LocalId(0), LoweredExpr::Number(42, _), _) => {}
        other => panic!("expected LoweredStmt::Assign, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_console_log() {
    let program = parse_resolve_lower("console.log(42);");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(builtin_id),
                args,
                ..
            },
            _,
        ) => {
            assert_eq!(format!("{builtin_id:?}"), "ConsoleLog");
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected LoweredStmt::Expr(Call(Builtin(ConsoleLog))), got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_runtime_call() {
    // Verify ConsoleLog produces a Builtin call, not a RuntimeCall
    let program = parse_resolve_lower("console.log(42);");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(_),
                ..
            },
            _,
        ) => {}
        other => panic!("expected FunctionCallKind::Builtin, got: {other:?}"),
    }
}

#[test]
fn lowered_snapshot_try_catch() {
    let program = parse_resolve_lower("try { 1; } catch(e) { 2; }");
    assert_eq!(program.top_level_statements.len(), 1);
    match &program.top_level_statements[0] {
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            ..
        } => {
            assert!(!try_body.is_empty());
            assert!(catch_var.is_some());
            assert!(catch_body.is_some());
        }
        other => panic!("expected LoweredStmt::TryCatch, got: {other:?}"),
    }
}
