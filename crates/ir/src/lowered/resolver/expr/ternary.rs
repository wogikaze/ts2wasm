use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_ternary_expr(
        &mut self,
        condition: &ResolvedExpr,
        then_expr: &ResolvedExpr,
        else_expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        let result = self.alloc_temp();
        Ok(LoweredExpr::Block {
            stmts: vec![
                LoweredStmt::Let(
                    result,
                    LoweredExpr::Undefined(Span::generated("undef")),
                    Span::generated("let_stmt"),
                ),
                LoweredStmt::If {
                    condition: self.lower_expr(condition)?,
                    then_body: vec![LoweredStmt::Assign(
                        result,
                        self.lower_expr(then_expr)?,
                        Span::generated("assign"),
                    )],
                    else_body: vec![LoweredStmt::Assign(
                        result,
                        self.lower_expr(else_expr)?,
                        Span::generated("assign"),
                    )],
                    span: Span::generated("if_stmt"),
                },
            ],
            result: Box::new(LoweredExpr::Local(result, Span::generated("local"))),
            span: Span::generated("block"),
        })
    }
}
