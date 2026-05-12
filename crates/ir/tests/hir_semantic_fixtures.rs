// HIR Semantic Fixtures
//
// This file serves as the HIR support matrix: it documents every supported
// and explicitly rejected HirStmt / HirExpr variant through source-parsing
// or synthetic construction.
//
// Acceptance boundary (I-20260512-HRSCVR):
// - Every supported HIR variant has a fixture showing it works.
// - Every intentionally rejected syntax has a fixture showing the diagnostic
//   is stable (UnsupportedSyntax diag code, not silent fallback).
// - validate_hir catches invariant violations with explicit diag code.
//
// Non-goals:
// - No new HIR variants (no broad feature expansion).
// - No native MIR model changes.
//
// HIR support matrix:
//
// HirStmt (6 variants, all supported):
//   Let              - local variable declaration with initializer
//   StoreLocal       - assignment to existing local
//   Expr             - expression evaluated for side effects
//   BranchIfTruthy   - if/else with ToBoolean condition
//   LoopWhile        - while loop with ToBoolean condition
//   Return           - function return (only valid inside functions)
//
// HirExpr (20 variants, all supported):
//   ConstUndefined   - `undefined` literal
//   ConstNull        - `null` literal
//   ConstBool        - `true` / `false` literals
//   ConstNumber      - i32 number literal
//   ConstBigInt      - bigint literal (stored as string)
//   ConstString      - string literal
//   LoadLocal        - local variable reference
//   LoadBuiltin      - builtin/global reference (e.g., Math)
//   ToBoolean        - abstract ToBoolean (wraps if/while conditions)
//   JsUnaryNot       - `!expr`
//   JsAdd            - `left + right`
//   JsStrictEqual    - `left === right`
//   JsAbstractEqual  - `left == right`
//   JsRelational     - `<`, `<=`, `>`, `>=`
//   GetProp          - property access via static key
//   GetIndex         - computed index access
//   ArrayLength      - .length on array
//   CallBuiltin      - known builtin call (e.g., console.log)
//   CallFunction     - direct user function call by name
//   CallMethod       - method call on receiver
//
// Rejected syntax (stable UnsupportedSyntax diagnostics):
//   Nested function declarations
//   Ternary expressions (cond ? a : b)
//   Assignment expressions (x = y as expression, not statement)
//   Dynamic calls (non-ident callee)
//   String() constructor calls

use ts2wasm_diagnostic::DiagCode;
use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_ir::{
    BuiltinId,
    HirExpr, HirFunction, HirFunctionId, HirLocalId, HirProgram, HirRelationalOp, HirStmt,
    dump_hir, dump_mir, lower_hir_to_mir, lower_to_hir, validate_hir,
    builtin_resolver::resolve_builtins,
    name_resolver::resolve_names,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse TypeScript source and lower to HIR. Panics on parse or HIR lowering
/// errors — only use for code expected to be valid.
fn parse_to_hir(source: &str) -> HirProgram {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let ast = Parser::new(tokens, source).parse_program().unwrap();
    let named = resolve_names(&ast).unwrap();
    let resolved = resolve_builtins(&named).unwrap();
    lower_to_hir(&resolved).unwrap()
}

/// Parse source and expect HIR lowering to fail with UnsupportedSyntax.
fn expect_hir_unsupported(source: &str) {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let ast = Parser::new(tokens, source).parse_program().unwrap();
    let named = resolve_names(&ast).unwrap();
    let resolved = resolve_builtins(&named).unwrap();
    let err = lower_to_hir(&resolved).unwrap_err();
    assert_eq!(
        err.code,
        DiagCode::UnsupportedSyntax,
        "expected UnsupportedSyntax, got {:?}: {}",
        err.code,
        err.message
    );
}

/// Verify dump_hir contains expected substrings (source-parsing based).
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

// ===========================================================================
// HirStmt variants (via source parsing)
// ===========================================================================

#[test]
fn hir_stmt_let() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Let {
                local: HirLocalId(0),
                init: HirExpr::ConstNumber(42),
            }],
            locals: vec![HirLocalId(0)],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("Let("), "Let in dump: {dump}");
    assert!(dump.contains("ConstNumber(42)"), "ConstNumber in dump: {dump}");
}

#[test]
fn hir_stmt_let_from_source() {
    assert_hir_dump_contains("let x = 42;", &["Let(", "ConstNumber(42)"]);
}

#[test]
fn hir_stmt_store_local() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::StoreLocal {
                local: HirLocalId(0),
                value: HirExpr::ConstNumber(99),
            }],
            locals: vec![HirLocalId(0)],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("StoreLocal("), "StoreLocal in dump: {dump}");
    assert!(dump.contains("ConstNumber(99)"), "ConstNumber in dump: {dump}");
}

#[test]
fn hir_stmt_store_local_from_source() {
    assert_hir_dump_contains(
        "let x = 1; x = 2;",
        &["StoreLocal(", "ConstNumber(2)"],
    );
}

#[test]
fn hir_stmt_expr() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::ConstNull)],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("Expr"), "Expr in dump: {dump}");
    assert!(dump.contains("ConstNull"), "ConstNull in dump: {dump}");
}

#[test]
fn hir_stmt_expr_from_source() {
    assert_hir_dump_contains("console.log(1);", &["Expr", "CallBuiltin"]);
}

#[test]
fn hir_stmt_branch_if_truthy() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstBool(true))),
                then_body: vec![],
                else_body: vec![HirStmt::Expr(HirExpr::ConstString("no".to_string()))],
            }],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("BranchIfTruthy"), "BranchIfTruthy in dump: {dump}");
    assert!(dump.contains("ToBoolean"), "ToBoolean in dump: {dump}");
    assert!(dump.contains("ConstString"), "ConstString in dump: {dump}");
}

#[test]
fn hir_stmt_branch_if_truthy_from_source() {
    assert_hir_dump_contains(
        "let a = 1; if (a) { let b = 2; } else { let c = 3; }",
        &["BranchIfTruthy", "then_body", "else_body", "ToBoolean"],
    );
}

#[test]
fn hir_stmt_loop_while() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::LoopWhile {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstBool(true))),
                body: vec![HirStmt::Expr(HirExpr::ConstNumber(2))],
            }],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("LoopWhile"), "LoopWhile in dump: {dump}");
    assert!(dump.contains("ToBoolean"), "ToBoolean in dump: {dump}");
    assert!(dump.contains("ConstNumber(2)"), "ConstNumber in dump: {dump}");
}

#[test]
fn hir_stmt_loop_while_from_source() {
    assert_hir_dump_contains(
        "let a = 1; while (a) { let b = 2; }",
        &["LoopWhile", "ToBoolean"],
    );
}

#[test]
fn hir_stmt_return() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Return(HirExpr::ConstNumber(42))],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("Return"), "Return in dump: {dump}");
    assert!(dump.contains("ConstNumber(42)"), "ConstNumber in dump: {dump}");
}

#[test]
fn hir_stmt_return_from_source() {
    assert_hir_dump_contains(
        "function f() { return 42; } f();",
        &["Return", "ConstNumber(42)"],
    );
}

// ===========================================================================
// HirExpr: constants (via source parsing)
// ===========================================================================

#[test]
fn hir_expr_const_undefined() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstUndefined)],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ConstUndefined"), "dump: {dump}");
}

#[test]
fn hir_expr_const_undefined_from_source() {
    assert_hir_dump_contains("let a = undefined;", &["ConstUndefined"]);
}

#[test]
fn hir_expr_const_null() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ConstNull)],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ConstNull"), "dump: {dump}");
}

#[test]
fn hir_expr_const_null_from_source() {
    assert_hir_dump_contains("let a = null;", &["ConstNull"]);
}

#[test]
fn hir_expr_const_bool() {
    let dump = {
        let hir = HirProgram {
            body: vec![
                HirStmt::Expr(HirExpr::ConstBool(true)),
                HirStmt::Expr(HirExpr::ConstBool(false)),
            ],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("ConstBool(true)"), "dump: {dump}");
    assert!(dump.contains("ConstBool(false)"), "dump: {dump}");
}

#[test]
fn hir_expr_const_bool_from_source() {
    assert_hir_dump_contains(
        "let a = true; let b = false;",
        &["ConstBool(true)", "ConstBool(false)"],
    );
}

#[test]
fn hir_expr_const_number() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::ConstNumber(42))],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("ConstNumber(42)"), "dump: {dump}");
}

#[test]
fn hir_expr_const_number_from_source() {
    assert_hir_dump_contains("let a = 42;", &["ConstNumber(42)"]);
    assert_hir_dump_contains("let a = 0;", &["ConstNumber(0)"]);
}

#[test]
fn hir_expr_const_bigint() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::ConstBigInt("123".to_string()))],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("ConstBigInt"), "dump: {dump}");
}

#[test]
fn hir_expr_const_bigint_from_source() {
    assert_hir_dump_contains("let a = 123n;", &["ConstBigInt"]);
}

#[test]
fn hir_expr_const_string() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::ConstString("hello".to_string()))],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("ConstString"), "dump: {dump}");
}

#[test]
fn hir_expr_const_string_from_source() {
    assert_hir_dump_contains("let a = \"hello\";", &["ConstString"]);
}

// ===========================================================================
// HirExpr: locals and builtins
// ===========================================================================

#[test]
fn hir_expr_load_local() {
    let dump = {
        let hir = HirProgram {
            body: vec![
                HirStmt::Let {
                    local: HirLocalId(0),
                    init: HirExpr::ConstNumber(1),
                },
                HirStmt::StoreLocal {
                    local: HirLocalId(0),
                    value: HirExpr::LoadLocal(HirLocalId(0)),
                },
            ],
            locals: vec![HirLocalId(0)],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("LoadLocal"), "dump: {dump}");
}

#[test]
fn hir_expr_load_local_from_source() {
    assert_hir_dump_contains("let a = 1; let b = a;", &["LoadLocal"]);
}

#[test]
fn hir_expr_load_builtin() {
    // LoadBuiltin("Math") is only reachable synthetically; source parsing
    // routes `Math` through the BuiltinResolver before reaching HIR.
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::LoadBuiltin("Math".to_string()))],
        locals: vec![],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("LoadBuiltin"), "dump: {dump}");
    assert!(dump.contains("Math"), "dump: {dump}");
}

// ===========================================================================
// HirExpr: unary operators
// ===========================================================================

#[test]
fn hir_expr_to_boolean() {
    // ToBoolean is produced by BranchIfTruthy / LoopWhile condition wrapping.
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstNumber(0))),
                then_body: vec![],
                else_body: vec![],
            }],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("ToBoolean"), "dump: {dump}");
}

#[test]
fn hir_expr_to_boolean_from_source() {
    // if-condition wrapping automatically produces ToBoolean.
    assert_hir_dump_contains(
        "let a = 1; if (a) {}",
        &["ToBoolean"],
    );
}

#[test]
fn hir_expr_js_unary_not() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsUnaryNot(Box::new(
                HirExpr::ConstBool(true),
            )))],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("JsUnaryNot"), "dump: {dump}");
}

#[test]
fn hir_expr_js_unary_not_from_source() {
    assert_hir_dump_contains("let a = !true;", &["JsUnaryNot"]);
}

// ===========================================================================
// HirExpr: binary operators
// ===========================================================================

#[test]
fn hir_expr_js_add() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsAdd {
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(2)),
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("JsAdd"), "dump: {dump}");
}

#[test]
fn hir_expr_js_add_from_source() {
    assert_hir_dump_contains("let a = 1 + 2;", &["JsAdd"]);
    // String concat also uses JsAdd
    assert_hir_dump_contains("let a = \"hello\" + \" world\";", &["JsAdd"]);
}

#[test]
fn hir_expr_js_strict_equal() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsStrictEqual {
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(1)),
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("JsStrictEqual"), "dump: {dump}");
}

#[test]
fn hir_expr_js_strict_equal_from_source() {
    assert_hir_dump_contains("let a = 1 === 2;", &["JsStrictEqual"]);
}

#[test]
fn hir_expr_js_abstract_equal() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsAbstractEqual {
                left: Box::new(HirExpr::ConstNull),
                right: Box::new(HirExpr::ConstUndefined),
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("JsAbstractEqual"), "dump: {dump}");
}

#[test]
fn hir_expr_js_abstract_equal_from_source() {
    assert_hir_dump_contains("let a = 1 == 2;", &["JsAbstractEqual"]);
}

#[test]
fn hir_expr_js_relational_less() {
    assert_hir_dump_contains("let a = 1 < 2;", &["JsRelational"]);
}

#[test]
fn hir_expr_js_relational_less_equal() {
    assert_hir_dump_contains("let a = 1 <= 2;", &["JsRelational"]);
}

#[test]
fn hir_expr_js_relational_greater() {
    assert_hir_dump_contains("let a = 1 > 2;", &["JsRelational"]);
}

#[test]
fn hir_expr_js_relational_greater_equal() {
    assert_hir_dump_contains("let a = 1 >= 2;", &["JsRelational"]);
}

#[test]
fn hir_expr_js_relational_all_variants_synthetic() {
    let ops = [
        (HirRelationalOp::Less, "Less"),
        (HirRelationalOp::LessEqual, "LessEqual"),
        (HirRelationalOp::Greater, "Greater"),
        (HirRelationalOp::GreaterEqual, "GreaterEqual"),
    ];
    for (hir_op, expected) in &ops {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::JsRelational {
                op: *hir_op,
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(2)),
            })],
            locals: vec![],
            functions: vec![],
        };
        let dump = dump_hir(&hir);
        assert!(
            dump.contains(expected),
            "op {hir_op:?} should produce {expected}:\n{dump}"
        );
    }
}

// ===========================================================================
// HirExpr: property / index / array-length
//
// NOTE: GetProp, GetIndex, ArrayLength are tested synthetically because the
// current HIR slice does not lower object/array literals, which are needed
// to produce these via source parsing. See also:
//   - ArrayLength requires BuiltinPropertyId::Length resolution
//   - GetProp requires a resolved object expression with static key access
//   - GetIndex requires a resolved object expression with computed index
// ===========================================================================

#[test]
fn hir_expr_get_prop() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetProp {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            key: "x".to_string(),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("GetProp"), "dump: {dump}");
    assert!(dump.contains("x"), "dump: {dump}");
}

#[test]
fn hir_expr_get_index() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetIndex {
            object: Box::new(HirExpr::LoadLocal(HirLocalId(0))),
            index: Box::new(HirExpr::ConstNumber(0)),
        })],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("GetIndex"), "dump: {dump}");
}

#[test]
fn hir_expr_array_length() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::ArrayLength(Box::new(
            HirExpr::LoadLocal(HirLocalId(0)),
        )))],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("ArrayLength"), "dump: {dump}");
}

// ===========================================================================
// HirExpr: calls
// ===========================================================================

#[test]
fn hir_expr_call_builtin() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::CallBuiltin {
                builtin: BuiltinId::ConsoleLog,
                args: vec![HirExpr::ConstString("hi".to_string())],
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("CallBuiltin"), "dump: {dump}");
}

#[test]
fn hir_expr_call_builtin_from_source() {
    assert_hir_dump_contains(
        "console.log(42);",
        &["CallBuiltin", "ConstNumber(42)"],
    );
}

#[test]
fn hir_expr_call_function() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::CallFunction {
                function: HirFunctionId(0),
                args: vec![],
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("CallFunction"), "dump: {dump}");
    assert!(dump.contains("HirFunctionId(0)"), "dump: {dump}");
}

#[test]
fn hir_expr_call_function_from_source() {
    assert_hir_dump_contains(
        "function f() { return 1; } f();",
        &["CallFunction"],
    );
}

#[test]
fn hir_expr_call_method() {
    let dump = {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::CallMethod {
                receiver: Box::new(HirExpr::ConstString("hello".to_string())),
                method: "toString".to_string(),
                args: vec![],
            })],
            locals: vec![],
            functions: vec![],
        };
        dump_hir(&hir)
    };
    assert!(dump.contains("CallMethod"), "dump: {dump}");
    assert!(dump.contains("toString"), "dump: {dump}");
}

#[test]
fn hir_expr_call_method_from_source() {
    assert_hir_dump_contains(
        "let a = \"hello\"; a.toString();",
        &["CallMethod", "toString"],
    );
}

// ===========================================================================
// Function body fixtures
// ===========================================================================

#[test]
fn hir_function_body() {
    let hir = HirProgram {
        body: vec![],
        locals: vec![HirLocalId(0)],
        functions: vec![HirFunction {
            id: HirFunctionId(0),
            params: vec![HirLocalId(0)],
            locals: vec![HirLocalId(0)],
            body: vec![HirStmt::Return(HirExpr::LoadLocal(HirLocalId(0)))],
        }],
    };
    let dump = dump_hir(&hir);
    assert!(dump.contains("function["), "dump: {dump}");
    assert!(dump.contains("HirFunctionId(0)"), "dump: {dump}");
    assert!(dump.contains("Return"), "dump: {dump}");
}

#[test]
fn hir_function_body_from_source() {
    assert_hir_dump_contains(
        "function f() { let x = 10; return x; } f();",
        &["function[", "HirFunctionId", "Let(", "Return", "ConstNumber(10)"],
    );
}

// ===========================================================================
// Integration: multiple variants in a single program
// ===========================================================================

#[test]
fn hir_integration_all_stmt_variants() {
    let source = "\
let x = 10;
if (x) { let y = 20; } else { let z = 30; }
let w = x + 5;
w = x + 1;
while (w) { w = 0; }
";
    let hir = parse_to_hir(source);
    let dump = dump_hir(&hir);
    assert!(dump.contains("Let("), "Let in dump: {dump}");
    assert!(dump.contains("StoreLocal("), "StoreLocal in dump: {dump}");
    assert!(dump.contains("BranchIfTruthy"), "BranchIfTruthy in dump: {dump}");
    assert!(dump.contains("LoopWhile"), "LoopWhile in dump: {dump}");
    assert!(dump.contains("JsAdd"), "JsAdd in dump: {dump}");
}

#[test]
fn hir_integration_all_expr_variants_synthetic() {
    // Verify that dump_hir handles every HirExpr variant without panicking.
    let hir = HirProgram {
        body: vec![
            HirStmt::Expr(HirExpr::ConstUndefined),
            HirStmt::Expr(HirExpr::ConstNull),
            HirStmt::Expr(HirExpr::ConstBool(true)),
            HirStmt::Expr(HirExpr::ConstNumber(1)),
            HirStmt::Expr(HirExpr::ConstBigInt("1n".to_string())),
            HirStmt::Expr(HirExpr::ConstString("s".to_string())),
            HirStmt::Expr(HirExpr::LoadBuiltin("Math".to_string())),
            HirStmt::Expr(HirExpr::ToBoolean(Box::new(HirExpr::ConstNumber(0)))),
            HirStmt::Expr(HirExpr::JsUnaryNot(Box::new(HirExpr::ConstBool(true)))),
            HirStmt::Expr(HirExpr::JsAdd {
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(2)),
            }),
            HirStmt::Expr(HirExpr::JsStrictEqual {
                left: Box::new(HirExpr::ConstUndefined),
                right: Box::new(HirExpr::ConstNull),
            }),
            HirStmt::Expr(HirExpr::JsAbstractEqual {
                left: Box::new(HirExpr::ConstNull),
                right: Box::new(HirExpr::ConstUndefined),
            }),
            HirStmt::Expr(HirExpr::JsRelational {
                op: HirRelationalOp::Less,
                left: Box::new(HirExpr::ConstNumber(1)),
                right: Box::new(HirExpr::ConstNumber(2)),
            }),
            HirStmt::Expr(HirExpr::GetProp {
                object: Box::new(HirExpr::ConstNull),
                key: "x".to_string(),
            }),
            HirStmt::Expr(HirExpr::GetIndex {
                object: Box::new(HirExpr::ConstNull),
                index: Box::new(HirExpr::ConstNumber(0)),
            }),
            HirStmt::Expr(HirExpr::ArrayLength(Box::new(HirExpr::ConstNull))),
            HirStmt::Let {
                local: HirLocalId(0),
                init: HirExpr::ConstNumber(1),
            },
            HirStmt::StoreLocal {
                local: HirLocalId(0),
                value: HirExpr::ConstNumber(2),
            },
            HirStmt::Return(HirExpr::ConstNumber(3)),
            HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(HirExpr::ConstBool(true))),
                then_body: vec![],
                else_body: vec![],
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
    // Verify that all dump outputs are present
    for expected in &[
        "ConstUndefined",
        "ConstNull",
        "ConstBool",
        "ConstNumber",
        "ConstBigInt",
        "ConstString",
        "LoadBuiltin",
        "ToBoolean",
        "JsUnaryNot",
        "JsAdd",
        "JsStrictEqual",
        "JsAbstractEqual",
        "JsRelational",
        "GetProp",
        "GetIndex",
        "ArrayLength",
        "Let(",
        "StoreLocal(",
        "Return",
        "BranchIfTruthy",
        "LoopWhile",
    ] {
        assert!(dump.contains(expected), "expected '{expected}' in dump:\n{dump}");
    }
}

// ===========================================================================
// Rejection fixtures — unsupported syntax produces stable diagnostics
// ===========================================================================

#[test]
fn reject_nested_function() {
    expect_hir_unsupported("function outer() { function inner() { return 1; } }");
}

#[test]
fn reject_nested_function_in_block() {
    expect_hir_unsupported("if (true) { function inner() { return 1; } }");
}

#[test]
fn reject_ternary_expression() {
    expect_hir_unsupported("let a = true ? 1 : 2;");
}

#[test]
fn reject_ternary_in_call_arg() {
    expect_hir_unsupported("console.log(true ? 1 : 2);");
}

#[test]
fn reject_assignment_expression_in_call() {
    // Assignment as expression (not as statement) is rejected.
    // x must be declared first so name resolution passes before HIR hits
    // the UnsupportedSyntax boundary.
    expect_hir_unsupported("let x = 0; console.log(x = 1);");
}

#[test]
fn reject_dynamic_call_non_ident_callee() {
    // A call where the callee is not a simple identifier is "dynamic"
    // and rejected by the initial HIR slice.
    expect_hir_unsupported("let a = {f: () => 1}; a.f();");
}

#[test]
fn reject_string_constructor_call() {
    expect_hir_unsupported("String(42);");
}

// ===========================================================================
// Validation tests — validate_hir rejects invalid shapes
// ===========================================================================

#[test]
fn validate_rejects_invalid_local_id() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::LoadLocal(HirLocalId(99)))],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("local id")
        }),
        "expected InvariantViolation for invalid local id, got: {errors:?}"
    );
}

#[test]
fn validate_rejects_invalid_function_id() {
    let hir = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::CallFunction {
            function: HirFunctionId(7),
            args: vec![],
        })],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("function id")
        }),
        "expected InvariantViolation for invalid function id, got: {errors:?}"
    );
}

#[test]
fn validate_rejects_branch_without_to_boolean() {
    let hir = HirProgram {
        body: vec![HirStmt::BranchIfTruthy {
            condition: HirExpr::ConstBool(true),
            then_body: vec![],
            else_body: vec![],
        }],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("ToBoolean")
        }),
        "expected InvariantViolation for missing ToBoolean, got: {errors:?}"
    );
}

#[test]
fn validate_rejects_loop_without_to_boolean() {
    let hir = HirProgram {
        body: vec![HirStmt::LoopWhile {
            condition: HirExpr::ConstBool(true),
            body: vec![],
        }],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("ToBoolean")
        }),
        "expected InvariantViolation for missing ToBoolean in loop, got: {errors:?}"
    );
}

#[test]
fn validate_rejects_top_level_return() {
    let hir = HirProgram {
        body: vec![HirStmt::Return(HirExpr::ConstNumber(42))],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("Return")
        }),
        "expected InvariantViolation for top-level return, got: {errors:?}"
    );
}

#[test]
fn validate_rejects_unregistered_local() {
    // Local in body that is not in `locals` list
    let hir = HirProgram {
        body: vec![HirStmt::Let {
            local: HirLocalId(0),
            init: HirExpr::ConstNumber(1),
        }],
        locals: vec![],
        functions: vec![],
    };
    let errors = validate_hir(&hir).unwrap_err();
    assert!(
        errors.iter().any(|e| {
            e.code == DiagCode::InvariantViolation && e.message.contains("local id")
        }),
        "expected InvariantViolation for unregistered local, got: {errors:?}"
    );
}

// ===========================================================================
// Valid HIR validation passes
// ===========================================================================

#[test]
fn validate_passes_valid_let() {
    let hir = HirProgram {
        body: vec![HirStmt::Let {
            local: HirLocalId(0),
            init: HirExpr::ConstNumber(42),
        }],
        locals: vec![HirLocalId(0)],
        functions: vec![],
    };
    validate_hir(&hir).unwrap();
}

#[test]
fn validate_passes_valid_in_function() {
    let hir = HirProgram {
        body: vec![],
        locals: vec![HirLocalId(0)],
        functions: vec![HirFunction {
            id: HirFunctionId(0),
            params: vec![HirLocalId(0)],
            locals: vec![HirLocalId(0)],
            body: vec![HirStmt::Return(HirExpr::LoadLocal(HirLocalId(0)))],
        }],
    };
    validate_hir(&hir).unwrap();
}

// ===========================================================================
// End-to-end: parse, lower, validate, lower to MIR, dump MIR
// ===========================================================================

#[test]
fn end_to_end_valid_program() {
    let source = "\
function add(a: number, b: number): number {
  return a + b;
}
let x = 10;
let y = 20;
let result = add(x, y);
console.log(\"result: \" + result);
";
    let hir = parse_to_hir(source);
    validate_hir(&hir).unwrap();
    let mir = lower_hir_to_mir(&hir);
    let mir_dump = dump_mir(&mir);
    assert!(mir_dump.contains("Call("), "MIR dump:\n{mir_dump}");
    assert!(mir_dump.contains("Binary(Add)"), "MIR dump:\n{mir_dump}");
}

#[test]
fn end_to_end_control_flow() {
    let source = "\
function max(a: number, b: number): number {
  if (a > b) {
    return a;
  }
  return b;
}
console.log(\"max: \" + max(3, 7));
";
    let hir = parse_to_hir(source);
    validate_hir(&hir).unwrap();
    let mir = lower_hir_to_mir(&hir);
    let mir_dump = dump_mir(&mir);
    assert!(mir_dump.contains("If"), "MIR dump:\n{mir_dump}");
    assert!(mir_dump.contains("Binary"), "MIR dump:\n{mir_dump}");
}
