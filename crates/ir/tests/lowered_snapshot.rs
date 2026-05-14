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
    ModuleLoadKind, RuntimeFn,
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

fn count_user_calls_in_stmt(stmt: &LoweredStmt) -> usize {
    match stmt {
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Expr(expr, _)
        | LoweredStmt::Return(expr, _)
        | LoweredStmt::Throw(expr, _) => count_user_calls_in_expr(expr),
        LoweredStmt::Block(stmts, _) => stmts.iter().map(count_user_calls_in_stmt).sum(),
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            count_user_calls_in_expr(condition)
                + then_body
                    .iter()
                    .map(count_user_calls_in_stmt)
                    .sum::<usize>()
                + else_body
                    .iter()
                    .map(count_user_calls_in_stmt)
                    .sum::<usize>()
        }
        _ => 0,
    }
}

fn count_user_calls_in_expr(expr: &LoweredExpr) -> usize {
    match expr {
        LoweredExpr::Call { kind, args, .. } => {
            usize::from(matches!(kind, FunctionCallKind::User(_)))
                + args.iter().map(count_user_calls_in_expr).sum::<usize>()
        }
        LoweredExpr::Block { stmts, result, .. } => {
            stmts.iter().map(count_user_calls_in_stmt).sum::<usize>()
                + count_user_calls_in_expr(result)
        }
        LoweredExpr::RuntimeCall { args, .. } | LoweredExpr::ArrayNew { elements: args, .. } => {
            args.iter().map(count_user_calls_in_expr).sum()
        }
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .map(|(_, value)| count_user_calls_in_expr(value))
            .sum(),
        LoweredExpr::Unary { expr, .. } => count_user_calls_in_expr(expr),
        LoweredExpr::Binary { left, right, .. } => {
            count_user_calls_in_expr(left) + count_user_calls_in_expr(right)
        }
        _ => 0,
    }
}

fn lowered_stmt_contains_runtime_call(stmt: &LoweredStmt, runtime: RuntimeFn) -> bool {
    match stmt {
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Expr(expr, _)
        | LoweredStmt::Return(expr, _)
        | LoweredStmt::Throw(expr, _) => lowered_expr_contains_runtime_call(expr, runtime),
        LoweredStmt::Block(stmts, _) => stmts
            .iter()
            .any(|stmt| lowered_stmt_contains_runtime_call(stmt, runtime)),
        LoweredStmt::While {
            condition, body, ..
        } => {
            lowered_expr_contains_runtime_call(condition, runtime)
                || body
                    .iter()
                    .any(|stmt| lowered_stmt_contains_runtime_call(stmt, runtime))
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            lowered_expr_contains_runtime_call(condition, runtime)
                || then_body
                    .iter()
                    .any(|stmt| lowered_stmt_contains_runtime_call(stmt, runtime))
                || else_body
                    .iter()
                    .any(|stmt| lowered_stmt_contains_runtime_call(stmt, runtime))
        }
        _ => false,
    }
}

fn lowered_expr_contains_runtime_call(expr: &LoweredExpr, runtime: RuntimeFn) -> bool {
    match expr {
        LoweredExpr::RuntimeCall {
            intrinsic, args, ..
        } => {
            *intrinsic == runtime
                || args
                    .iter()
                    .any(|arg| lowered_expr_contains_runtime_call(arg, runtime))
        }
        LoweredExpr::Call { args, .. } | LoweredExpr::ArrayNew { elements: args, .. } => args
            .iter()
            .any(|arg| lowered_expr_contains_runtime_call(arg, runtime)),
        LoweredExpr::Block { stmts, result, .. } => {
            stmts
                .iter()
                .any(|stmt| lowered_stmt_contains_runtime_call(stmt, runtime))
                || lowered_expr_contains_runtime_call(result, runtime)
        }
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .any(|(_, value)| lowered_expr_contains_runtime_call(value, runtime)),
        LoweredExpr::PropertyGet { obj, .. }
        | LoweredExpr::OptionalPropertyGet { obj, .. }
        | LoweredExpr::MethodCall { object: obj, .. }
        | LoweredExpr::Unary { expr: obj, .. } => lowered_expr_contains_runtime_call(obj, runtime),
        LoweredExpr::PropertyGetDynamic { obj, key, .. }
        | LoweredExpr::Index {
            object: obj,
            index: key,
            ..
        } => {
            lowered_expr_contains_runtime_call(obj, runtime)
                || lowered_expr_contains_runtime_call(key, runtime)
        }
        LoweredExpr::Binary { left, right, .. } => {
            lowered_expr_contains_runtime_call(left, runtime)
                || lowered_expr_contains_runtime_call(right, runtime)
        }
        _ => false,
    }
}

fn lowered_stmt_contains_user_call_with_local_receiver(stmt: &LoweredStmt) -> bool {
    match stmt {
        LoweredStmt::Let(_, expr, _)
        | LoweredStmt::Expr(expr, _)
        | LoweredStmt::Return(expr, _)
        | LoweredStmt::Throw(expr, _) => lowered_expr_contains_user_call_with_local_receiver(expr),
        LoweredStmt::Block(stmts, _) => stmts
            .iter()
            .any(lowered_stmt_contains_user_call_with_local_receiver),
        LoweredStmt::While {
            condition, body, ..
        } => {
            lowered_expr_contains_user_call_with_local_receiver(condition)
                || body
                    .iter()
                    .any(lowered_stmt_contains_user_call_with_local_receiver)
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            lowered_expr_contains_user_call_with_local_receiver(condition)
                || then_body
                    .iter()
                    .any(lowered_stmt_contains_user_call_with_local_receiver)
                || else_body
                    .iter()
                    .any(lowered_stmt_contains_user_call_with_local_receiver)
        }
        _ => false,
    }
}

fn lowered_expr_contains_user_call_with_local_receiver(expr: &LoweredExpr) -> bool {
    match expr {
        LoweredExpr::Call { kind, args, .. } => {
            (matches!(kind, FunctionCallKind::User(_))
                && matches!(args.first(), Some(LoweredExpr::Local(_, _))))
                || args
                    .iter()
                    .any(lowered_expr_contains_user_call_with_local_receiver)
        }
        LoweredExpr::RuntimeCall { args, .. } | LoweredExpr::ArrayNew { elements: args, .. } => {
            args.iter()
                .any(lowered_expr_contains_user_call_with_local_receiver)
        }
        LoweredExpr::ObjectNew { props, .. } => props
            .iter()
            .any(|(_, value)| lowered_expr_contains_user_call_with_local_receiver(value)),
        LoweredExpr::Block { stmts, result, .. } => {
            stmts
                .iter()
                .any(lowered_stmt_contains_user_call_with_local_receiver)
                || lowered_expr_contains_user_call_with_local_receiver(result)
        }
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
fn lowered_snapshot_proxy_property_ops_dispatch_to_handler_traps() {
    let program = parse_resolve_lower(
        r#"
        const target = { x: 10 };
        function proxyGet(obj: any, prop: string) { return obj[prop]; }
        function proxySet(obj: any, prop: string, value: number) { obj[prop] = value; return true; }
        function proxyHas(obj: any, prop: string) { return true; }
        function proxyDeleteProperty(obj: any, prop: string) { delete obj[prop]; return true; }
        const handler = { get: proxyGet, set: proxySet, has: proxyHas, deleteProperty: proxyDeleteProperty };
        const proxy = new Proxy(target, handler);
        let getValue = proxy.x;
        proxy.y = 7;
        let hasValue = "x" in proxy;
        let deleteValue = delete proxy.x;
        "#,
    );
    let top_level_user_calls = program
        .top_level_statements
        .iter()
        .map(count_user_calls_in_stmt)
        .sum::<usize>();
    assert_eq!(
        top_level_user_calls, 4,
        "expected get/set/has/delete proxy operations to dispatch through handler functions"
    );
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
fn lowered_snapshot_super_call_passes_this_to_parent_constructor() {
    let program = parse_resolve_lower(
        r#"
        class Base { constructor(value) { this.value = value; } }
        class Derived extends Base { constructor(value) { super(value); } }
        "#,
    );

    assert!(
        program.functions.iter().any(|function| function
            .body
            .iter()
            .any(lowered_stmt_contains_user_call_with_local_receiver)),
        "expected derived constructor super(...) to pass this as the first parent constructor arg"
    );
}

#[test]
fn lowered_snapshot_object_method_super_property_uses_object_prototype() {
    let program = parse_resolve_lower(
        r#"
        let parent = { value: 42 };
        let child = { read() { return super.value; } };
        Object.setPrototypeOf(child, parent);
        let result = child.read();
        "#,
    );

    assert!(
        program.functions.iter().any(|function| function
            .body
            .iter()
            .any(|stmt| lowered_stmt_contains_runtime_call(stmt, RuntimeFn::ObjectGetPrototypeOf))),
        "expected object method super.property to read from Object.getPrototypeOf(this)"
    );
    assert!(
        program
            .top_level_statements
            .iter()
            .any(lowered_stmt_contains_user_call_with_local_receiver),
        "expected object literal method call to dispatch with the object receiver"
    );
}

#[test]
fn lowered_snapshot_object_group_by_arrow_callback_builds_buckets() {
    let program = parse_resolve_lower(
        r#"
        let grouped = Object.groupBy([1, 2, 3], (value) => value % 2);
        "#,
    );

    assert!(
        program
            .top_level_statements
            .iter()
            .any(|stmt| { lowered_stmt_contains_runtime_call(stmt, RuntimeFn::ArrayPushGrow) }),
        "expected Object.groupBy to append repeated-key values into bucket arrays"
    );
    assert!(
        program
            .top_level_statements
            .iter()
            .any(lowered_stmt_contains_user_call_with_local_receiver),
        "expected Object.groupBy to call the static arrow callback"
    );
}

#[test]
fn lowered_snapshot_map_group_by_arrow_callback_builds_buckets() {
    let program = parse_resolve_lower(
        r#"
        let grouped = Map.groupBy([1, 2, 3], (value) => value % 2);
        "#,
    );

    assert!(
        program
            .top_level_statements
            .iter()
            .any(|stmt| { lowered_stmt_contains_runtime_call(stmt, RuntimeFn::MapSet) }),
        "expected Map.groupBy to create bucket arrays in the result Map"
    );
    assert!(
        program
            .top_level_statements
            .iter()
            .any(|stmt| { lowered_stmt_contains_runtime_call(stmt, RuntimeFn::ArrayPushGrow) }),
        "expected Map.groupBy to append repeated-key values into bucket arrays"
    );
    assert!(
        program
            .top_level_statements
            .iter()
            .any(lowered_stmt_contains_user_call_with_local_receiver),
        "expected Map.groupBy to call the static arrow callback"
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
fn lowered_generator_iterator_without_static_steps_still_lowers_next() {
    let program = parse_resolve_lower(
        "function* gen() { let obj = { [yield 9]: 9 }; }\n\
         let iter = gen();\n\
         while (iter.next().done === false) ;",
    );

    validate_lowered(&program).expect("generator iterator fallback should validate");
    assert!(
        program
            .top_level_statements
            .iter()
            .any(|stmt| { lowered_stmt_contains_runtime_call(stmt, RuntimeFn::GeneratorNext) })
    );
}

#[test]
fn lowered_generator_function_captures_top_level_assignment() {
    let program = parse_resolve_lower(
        "var obj;\n\
         function* gen() { obj = { get [yield]() { return 1; } }; }\n\
         let iter = gen();\n\
         iter.next();\n\
         iter.next(\"key\");\n\
         obj.key;",
    );

    validate_lowered(&program).expect("generator top-level assignment capture should validate");
}

#[test]
fn lowered_object_computed_method_key_uses_to_string_for_static_dispatch() {
    let program = parse_resolve_lower(
        "let assert = { sameValue(actual, expected, message) { return true; } };\n\
         let counter = 0;\n\
         let key1 = { toString: function() { assert.sameValue(counter++, 0, \"key1\"); return \"b\"; } };\n\
         let key2 = { toString: function() { assert.sameValue(counter++, 1, \"key2\"); return \"d\"; } };\n\
         let object = { a() { return \"A\"; }, [key1]() { return \"B\"; }, c() { return \"C\"; }, [key2]() { return \"D\"; } };\n\
         object.a();\n\
         object.b();\n\
         object.c();\n\
         object.d();",
    );

    validate_lowered(&program)
        .expect("computed method key with toString should preserve method metadata");
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
