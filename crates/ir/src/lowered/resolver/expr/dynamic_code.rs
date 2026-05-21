use crate::builtin_resolved::{
    EvalHostPolicy, EvalSource, FunctionConstructorHostPolicy, ResolvedExpr,
};
use crate::lowered::*;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

impl super::super::Resolver {
    pub(super) fn lower_dynamic_code_expr(
        &mut self,
        expr: &ResolvedExpr,
    ) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::FunctionConstructor { plan } => {
                if !plan.host_policy_is_consistent() {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message: format!(
                            "Function constructor host policy {:?} does not match source classification",
                            plan.host_policy
                        ),
                        span: Some(plan.span),
                        phase: None,
                    });
                }
                if !plan.static_source_is_consistent() {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message: "Function constructor static source metadata does not match plan"
                            .to_owned(),
                        span: Some(plan.span),
                        phase: None,
                    });
                }
                if plan.host_policy == FunctionConstructorHostPolicy::AotOnly {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message:
                            "static Function constructor reached lowering without AOT expansion"
                                .to_owned(),
                        span: Some(plan.span),
                        phase: None,
                    });
                }
                self.lower_dynamic_function_constructor_host_compile(&plan.args, plan.span)
            }
            ResolvedExpr::Eval { plan } => {
                if !plan.scope_mode_is_consistent() {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message: format!(
                            "eval scope mode {:?} does not match {:?} eval",
                            plan.scope_mode, plan.kind
                        ),
                        span: Some(plan.span),
                        phase: None,
                    });
                }
                if !plan.host_policy_is_consistent() {
                    return Err(Diagnostic {
                        code: DiagCode::UnsupportedEval,
                        message: format!(
                            "eval host policy {:?} does not match {:?} eval source",
                            plan.host_policy, plan.kind
                        ),
                        span: Some(plan.span),
                        phase: None,
                    });
                }
                self.lower_runtime_eval_expr(plan)
            }
            _ => unreachable!("dynamic code lowering expects eval or Function constructor"),
        }
    }

    fn lower_runtime_eval_expr(
        &mut self,
        plan: &crate::builtin_resolved::EvalFragmentPlan,
    ) -> Result<LoweredExpr, Diagnostic> {
        if !plan.completion_state_is_consistent() {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedEval,
                message: "eval completion/declaration plan does not match fragment plan".to_owned(),
                span: Some(plan.span),
                phase: None,
            });
        }
        if let EvalSource::NonStringStatic(source_expr) = &plan.source {
            return self.lower_expr(source_expr);
        }
        let EvalSource::Runtime(source_value_expr) = &plan.source else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedEval,
                message: "static eval fragment reached lowering without AOT expansion".to_owned(),
                span: Some(plan.span),
                phase: None,
            });
        };
        let source_expr = self.lower_expr(source_value_expr)?;
        let intrinsic = match plan.host_policy {
            EvalHostPolicy::DirectHost => RuntimeFn::EvalDirectHost,
            EvalHostPolicy::IndirectHost => RuntimeFn::EvalIndirectHost,
            EvalHostPolicy::AotOnly => {
                return Err(Diagnostic {
                    code: DiagCode::UnsupportedEval,
                    message: "AOT-only eval fragment cannot use a runtime host eval lane"
                        .to_owned(),
                    span: Some(plan.span),
                    phase: None,
                });
            }
        };
        let args = match plan.host_policy {
            EvalHostPolicy::DirectHost => vec![
                source_expr,
                self.lower_direct_eval_env_descriptor(plan.caller_is_strict, source_value_expr),
            ],
            EvalHostPolicy::IndirectHost => vec![source_expr],
            EvalHostPolicy::AotOnly => unreachable!(),
        };
        Ok(LoweredExpr::RuntimeCall {
            intrinsic,
            args,
            span: Span::generated("eval"),
        })
    }
}
