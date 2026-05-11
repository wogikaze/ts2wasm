use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_shared::{DiagCode, Diagnostic, Span};

impl<'a> super::super::Resolver {
    pub(crate) fn lower_optional_call(
        &mut self,
        callee: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<LoweredExpr, Diagnostic> {
        let func_name = match callee {
            ResolvedExpr::Ident(name) => name,
            _ => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message:
                        "issue-253: optional calls are currently supported only for identifier callees"
                            .to_owned(),
                    span: Some(span),

                    phase: None,
                });
            }
        };

        if let Ok(local_id) = self.resolve_local(func_name) {
            if self.ctx.facts.nullish_locals.contains(&local_id) {
                return Ok(LoweredExpr::Undefined(Span::generated("undef")));
            }

            if let Some(closure) = self.ctx.facts.arrow_locals.get(&local_id).cloned() {
                let mut lowered_args = self.lower_call_args(args)?;
                lowered_args.extend(
                    closure
                        .captures
                        .iter()
                        .copied()
                        .map(|id| LoweredExpr::Local(id, Span::generated("local"))),
                );
                return Ok(LoweredExpr::OptionalCall {
                    callee: Box::new(LoweredExpr::Local(local_id, Span::generated("local"))),
                    call: Box::new(LoweredExpr::Call {
                        kind: FunctionCallKind::User(closure.func_id),
                        args: lowered_args,

                        span: Span::generated("call"),
                    }),
                    span: Span::generated("opt_call"),
                });
            }
        }

        let func_id = self.resolve_func(func_name)?;
        if self
            .ctx
            .symbols
            .function_signatures
            .get(&func_id)
            .is_some_and(|signature| signature.needs_receiver)
        {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-062d: optional direct call `{func_name}?.(...)` cannot bind a supported receiver for `this`; call through a supported receiver object"
                ),
                span: Some(span),

                phase: None,
            });
        }
        let lowered_args = self.lower_function_call_args(
            func_id,
            LoweredExpr::Undefined(Span::generated("undef")),
            args,
        )?;
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::User(func_id),
            args: lowered_args,

            span: Span::generated("call"),
        })
    }
}
