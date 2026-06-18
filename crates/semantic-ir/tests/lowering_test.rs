use ts2wasm_ir::builtin_resolved::{ResolvedExpr, ResolvedStmt};
use ts2wasm_semantic_ir::lowering::lower_to_sem_ir;

#[test]
fn lower_empty_program() {
    let stmts = vec![];
    let program = lower_to_sem_ir(&stmts);
    assert_eq!(program.functions.len(), 0);
    assert!(program.top_level_blocks.len() >= 1);
}

#[test]
fn lower_number_constant() {
    let stmts = vec![ResolvedStmt::Expr(ResolvedExpr::Number(42))];
    let program = lower_to_sem_ir(&stmts);
    assert!(program.functions.is_empty());
    assert!(program.top_level_blocks.len() >= 1);
}

#[test]
fn lower_let_stmt() {
    use ts2wasm_semantic_ir::stmt::SemStmt;

    let stmts = vec![
        ResolvedStmt::Let("x".to_string(), ResolvedExpr::Number(1)),
        ResolvedStmt::Expr(ResolvedExpr::Ident("x".to_string())),
    ];
    let program = lower_to_sem_ir(&stmts);
    let entry_block = &program.top_level_blocks[0];
    assert!(
        entry_block.stmts.len() >= 1,
        "expected at least one stmt in entry block"
    );
    assert!(
        entry_block
            .stmts
            .iter()
            .any(|s| matches!(s, SemStmt::Let { .. }))
    );
}

#[test]
fn lower_if_stmt() {
    let stmts = vec![ResolvedStmt::If {
        condition: ResolvedExpr::Bool(true),
        then_body: vec![ResolvedStmt::Expr(ResolvedExpr::Number(1))],
        else_body: vec![ResolvedStmt::Expr(ResolvedExpr::Number(2))],
    }];
    let program = lower_to_sem_ir(&stmts);
    let block_count = program.top_level_blocks.len();
    assert!(
        block_count >= 4,
        "expected >=4 blocks for if/else, got {block_count}"
    );
}

#[test]
fn lower_function() {
    let stmts = vec![ResolvedStmt::Function {
        name: "f".to_string(),
        params: vec![],
        body: vec![ResolvedStmt::Return(ResolvedExpr::Number(1))],
        is_generator: false,
        is_async: false,
        is_ambient: false,
        source_text: String::new(),
    }];
    let program = lower_to_sem_ir(&stmts);
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "f");
}

#[test]
fn lower_while_stmt() {
    let stmts = vec![ResolvedStmt::While {
        condition: ResolvedExpr::Bool(true),
        body: vec![ResolvedStmt::Expr(ResolvedExpr::Number(1))],
    }];
    let program = lower_to_sem_ir(&stmts);
    let block_count = program.top_level_blocks.len();
    assert!(
        block_count >= 4,
        "expected >=4 blocks for while, got {block_count}"
    );
}

#[test]
fn lower_try_catch() {
    let stmts = vec![ResolvedStmt::TryCatch {
        try_block: vec![ResolvedStmt::Expr(ResolvedExpr::Number(1))],
        catch_param: Some("e".to_string()),
        catch_block: Some(vec![ResolvedStmt::Expr(ResolvedExpr::Ident(
            "e".to_string(),
        ))]),
        finally_block: None,
    }];
    let program = lower_to_sem_ir(&stmts);
    let block_count = program.top_level_blocks.len();
    assert!(
        block_count >= 3,
        "expected >=3 blocks for try/catch, got {block_count}"
    );
}

#[test]
fn lower_binary_expr() {
    let stmts = vec![ResolvedStmt::Expr(ResolvedExpr::Binary {
        left: Box::new(ResolvedExpr::Number(1)),
        op: ts2wasm_syntax::BinaryOp::Add,
        right: Box::new(ResolvedExpr::Number(2)),
    })];
    let program = lower_to_sem_ir(&stmts);
    assert!(program.top_level_blocks.len() >= 1);
}

#[test]
fn lower_call_expr() {
    use ts2wasm_source::Span;
    let stmts = vec![ResolvedStmt::Expr(ResolvedExpr::Call {
        callee: Box::new(ResolvedExpr::Ident("console.log".to_string())),
        args: vec![ResolvedExpr::String("hello".to_string())],
        span: Span::default(),
    })];
    let program = lower_to_sem_ir(&stmts);
    assert!(program.top_level_blocks.len() >= 1);
}
