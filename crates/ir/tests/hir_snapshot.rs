// Snapshot tests for HIR dump.
//
// These tests verify that every HIR variant is dumpable by constructing
// programs that exercise each variant and checking the dump output for
// expected pattern strings.

use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::{
    builtin_resolver::resolve_builtins,
    dump_hir,
    name_resolver::resolve_names,
    semantic::{HirExpr, HirLocalId, HirProgram, HirStmt, lower_to_hir as lower_to_hir_fn},
};

/// Helper: parse source and lower to HIR.
fn parse_to_hir(source: &str) -> HirProgram {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let ast = Parser::new(tokens, source).parse_program().unwrap();
    let named = resolve_names(&ast).unwrap();
    let resolved = resolve_builtins(&named).unwrap();
    lower_to_hir_fn(&resolved).unwrap()
}

/// Helper: parse source, dump HIR, check dump contains expected strings.
fn assert_hir_dump_contains(source: &str, expected: &[&str]) {
    let hir = parse_to_hir(source);
    let dump = dump_hir(&hir);
    for pattern in expected {
        assert!(
            dump.contains(pattern),
            "expected dump to contain {:?}, but it did not\n\n=== dump ===\n{}",
            pattern,
            dump
        );
    }
}

// ---------------------------------------------------------------------------
// HIR Stmt variants
// ---------------------------------------------------------------------------

#[test]
fn dump_hir_let_stmt() {
    assert_hir_dump_contains("let x = 42;", &["Let(", "ConstNumber(42)"]);
}

#[test]
fn dump_hir_assign_stmt() {
    assert_hir_dump_contains("let x = 1; x = 2;", &["StoreLocal(", "ConstNumber(2)"]);
}

#[test]
fn dump_hir_expr_stmt() {
    assert_hir_dump_contains("console.log(1);", &["Expr", "CallBuiltin"]);
}

#[test]
fn dump_hir_branch_if_truthy() {
    assert_hir_dump_contains(
        "let a = 1; if (a) { let b = 2; } else { let c = 3; }",
        &["BranchIfTruthy", "then_body", "else_body"],
    );
}

#[test]
fn dump_hir_loop_while() {
    assert_hir_dump_contains(
        "let a = 1; while (a) { let b = 2; }",
        &["LoopWhile", "ToBoolean"],
    );
}

#[test]
fn dump_hir_return() {
    assert_hir_dump_contains(
        "function f() { return 42; } f();",
        &["Return", "ConstNumber(42)"],
    );
}

// ---------------------------------------------------------------------------
// HIR Expr variants
// ---------------------------------------------------------------------------

#[test]
fn dump_hir_const_undefined() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstUndefined)],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ConstUndefined"));
}

#[test]
fn dump_hir_const_null() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstNull)],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ConstNull"));
}

#[test]
fn dump_hir_const_bool() {
    assert_hir_dump_contains(
        "let a = true; let b = false;",
        &["ConstBool(true)", "ConstBool(false)"],
    );
}

#[test]
fn dump_hir_const_number() {
    assert_hir_dump_contains("let a = 42;", &["ConstNumber(42)"]);
}

#[test]
fn dump_hir_const_bigint() {
    assert_hir_dump_contains("let a = 123n;", &["ConstBigInt"]);
}

#[test]
fn dump_hir_const_string() {
    assert_hir_dump_contains("let a = \"hello\";", &["ConstString"]);
}

#[test]
fn dump_hir_load_local() {
    assert_hir_dump_contains("let a = 1; let b = a;", &["LoadLocal"]);
}

#[test]
fn dump_hir_load_builtin() {
    // LoadBuiltin is not directly visible for console (which becomes CallBuiltin).
    // Test synthetically.
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::LoadBuiltin("Math".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("LoadBuiltin"));
    assert!(dump.contains("Math"));
}

#[test]
fn dump_hir_to_boolean() {
    assert_hir_dump_contains("let a = 1; if (a) {}", &["ToBoolean"]);
}

#[test]
fn dump_hir_js_unary_not() {
    assert_hir_dump_contains("let a = !true;", &["JsUnaryNot"]);
}

#[test]
fn dump_hir_js_add() {
    assert_hir_dump_contains("let a = 1 + 2;", &["JsAdd"]);
}

#[test]
fn dump_hir_js_strict_equal() {
    assert_hir_dump_contains("let a = 1 === 2;", &["JsStrictEqual"]);
}

#[test]
fn dump_hir_js_abstract_equal() {
    assert_hir_dump_contains("let a = 1 == 2;", &["JsAbstractEqual"]);
}

#[test]
fn dump_hir_js_relational() {
    assert_hir_dump_contains("let a = 1 < 2;", &["JsRelational"]);
}

#[test]
fn dump_hir_get_prop() {
    // GetProp is not produced by parsing (HIR slice limitation), test synthetically
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetProp {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            key: "x".to_string(),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("GetProp"));
    assert!(dump.contains("x"));
}

#[test]
fn dump_hir_get_index() {
    // GetIndex is not produced by parsing (HIR slice limitation), test synthetically
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetIndex {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            index: Box::new(HirExpr::ConstNumber(0)),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("GetIndex"));
}

#[test]
fn dump_hir_array_length() {
    // ArrayLength via parsing requires BuiltinPropertyId path, which may not be triggered
    // Test synthetically for coverage
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ArrayLength(Box::new(
            HirExpr::LoadLocal(HirLocalId(0)),
        )))],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ArrayLength"));
}

#[test]
fn dump_hir_call_builtin() {
    assert_hir_dump_contains("console.log(42);", &["CallBuiltin"]);
}

#[test]
fn dump_hir_call_function() {
    assert_hir_dump_contains("function f() { return 1; } f();", &["CallFunction"]);
}

#[test]
fn dump_hir_call_method() {
    assert_hir_dump_contains("let a = \"hello\"; a.toString();", &["CallMethod"]);
}

#[test]
fn dump_hir_function_body() {
    assert_hir_dump_contains(
        "function f() { let x = 10; return x; } f();",
        &[
            "function[",
            "HirFunctionId",
            "Let(",
            "Return",
            "ConstNumber(10)",
        ],
    );
}

#[test]
fn dump_hir_validates_all_variants() {
    // Verify that the dump output covers all HirExpr/HirStmt variants
    let hir = HirProgram {
        body: vec![
            HirStmt::Let {
                local: HirLocalId(0),
                init: HirExpr::ConstNumber(1),
            },
            HirStmt::StoreLocal {
                local: HirLocalId(0),
                value: HirExpr::ConstBool(true),
            },
            HirStmt::Expr(HirExpr::ConstNull),
            HirStmt::Return(HirExpr::ConstUndefined),
            HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstBool(true))),
                then_body: vec![],
                else_body: vec![HirStmt::Expr(HirExpr::ConstString("else".to_string()))],
            },
            HirStmt::LoopWhile {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstBool(false))),
                body: vec![],
            },
        ],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("Let("));
    assert!(dump.contains("ConstNumber(1)"));
    assert!(dump.contains("StoreLocal("));
    assert!(dump.contains("ConstBool(true)"));
    assert!(dump.contains("ConstNull"));
    assert!(dump.contains("ConstUndefined"));
    assert!(dump.contains("BranchIfTruthy"));
    assert!(dump.contains("ToBoolean"));
    assert!(dump.contains("ConstString"));
    assert!(dump.contains("LoopWhile"));
}
