use crate::builtin::BuiltinId;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_diagnostic::Diagnostic;
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_builtin_call_expr(
        &mut self,
        builtin: BuiltinId,
        args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut lowered_args = args
            .iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Result<Vec<_>, _>>()?;
        if builtin == BuiltinId::ParseInt && lowered_args.len() == 1 {
            lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
        }
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(builtin),
            args: lowered_args,
            span: Span::generated("call"),
        })
    }
}
