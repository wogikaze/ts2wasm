use ts2wasm_ir::lowered::hir::{
    HirBinaryOp, HirExpr, HirFunction, HirProgram, HirStmt,
};
use ts2wasm_ir::lowered::lower_hir_to_mir;
use ts2wasm_ir::lowered::mir::{MirExpr, MirStmt};
use ts2wasm_ir::lowered::{FuncId, LocalId};
use ts2wasm_runtime_catalog::RuntimeFn;

#[test]
fn lower_empty_program() {
    let program = HirProgram {
        body: vec![],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    assert!(mir.top_level_statements.is_empty());
    assert!(mir.functions.is_empty());
    assert!(mir.top_level_locals.is_empty());
}

#[test]
fn lower_number_literal() {
    let program = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::Number(42))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    assert_eq!(mir.top_level_statements.len(), 1);
    match &mir.top_level_statements[0] {
        MirStmt::Expr(MirExpr::I32Const(n)) => assert_eq!(*n, 42),
        other => panic!("expected I32Const, got: {other:?}"),
    }
}

#[test]
fn lower_string_literal() {
    let program = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::String("hello".to_owned()))],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    match &mir.top_level_statements[0] {
        MirStmt::Expr(MirExpr::StringConst(s)) => assert_eq!(s, "hello"),
        other => panic!("expected StringConst, got: {other:?}"),
    }
}

#[test]
fn lower_binary_add() {
    let program = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::Binary {
            left: Box::new(HirExpr::Number(1)),
            op: HirBinaryOp::Add,
            right: Box::new(HirExpr::Number(2)),
        })],
        locals: vec![],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    match &mir.top_level_statements[0] {
        MirStmt::Expr(MirExpr::CallRuntime { intrinsic: RuntimeFn::Add, args: _ }) => {
            // Match arm confirms CallRuntime with RuntimeFn variant
        }
        MirStmt::Expr(MirExpr::CallRuntime { intrinsic, args }) => {
            panic!("expected CallRuntime(Add), got: {:?}", intrinsic);
        }
        other => panic!("expected CallRuntime, got: {other:?}"),
    }
    match &mir.top_level_statements[0] {
        MirStmt::Expr(MirExpr::CallRuntime { intrinsic, args }) => {
            assert_eq!(*intrinsic, RuntimeFn::Add);
            assert_eq!(args.len(), 2);
        }
        other => panic!("expected CallRuntime(Add), got: {other:?}"),
    }
}

#[test]
fn lower_let_statement() {
    let program = HirProgram {
        body: vec![HirStmt::Let {
            local: LocalId(0),
            init: HirExpr::Number(42),
        }],
        locals: vec![LocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    match &mir.top_level_statements[0] {
        MirStmt::Let { local, init } => {
            assert_eq!(local.0, 0);
            assert!(matches!(init, MirExpr::I32Const(42)));
        }
        other => panic!("expected Let, got: {other:?}"),
    }
}

#[test]
fn lower_call_runtime_intrinsic() {
    let program = HirProgram {
        body: vec![HirStmt::Expr(HirExpr::GetProp {
            object: Box::new(HirExpr::Local(LocalId(0))),
            key: "length".to_owned(),
        })],
        locals: vec![LocalId(0)],
        functions: vec![],
    };
    let mir = lower_hir_to_mir(&program);
    match &mir.top_level_statements[0] {
        MirStmt::Expr(MirExpr::CallRuntime { intrinsic, args }) => {
            assert_eq!(*intrinsic, RuntimeFn::PropertyGet);
            assert_eq!(args.len(), 2);
            match &args[1] {
                MirExpr::StringConst(s) => assert_eq!(s, "length"),
                other => panic!("expected StringConst, got: {other:?}"),
            }
        }
        other => panic!("expected CallRuntime(PropertyGet), got: {other:?}"),
    }
}
