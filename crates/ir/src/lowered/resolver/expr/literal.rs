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
        if builtin == BuiltinId::ConsoleLog && lowered_args.len() > 1 {
            let mut joined = lowered_args.remove(0);
            for arg in lowered_args {
                joined = LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::Concat,
                    args: vec![
                        joined,
                        LoweredExpr::String(" ".to_owned(), Span::generated("str")),
                    ],
                    span: Span::generated("runtime_call"),
                };
                joined = LoweredExpr::RuntimeCall {
                    intrinsic: RuntimeFn::Concat,
                    args: vec![joined, arg],
                    span: Span::generated("runtime_call"),
                };
            }
            lowered_args = vec![joined];
        }
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
