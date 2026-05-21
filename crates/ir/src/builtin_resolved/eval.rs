use crate::builtin_resolved::{EvalCompletionStep, EvalFunctionHoist, ResolvedExpr};
use ts2wasm_source::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalFragmentPlan {
    pub kind: EvalKind,
    pub source: EvalSource,
    pub scope_mode: EvalScopeMode,
    pub caller_is_strict: bool,
    pub eval_source_is_strict: Option<bool>,
    pub declaration_plan: Option<EvalDeclarationPlan>,
    pub completion_plan: Option<EvalCompletionPlan>,
    pub host_policy: EvalHostPolicy,
    pub span: Span,
}

impl EvalFragmentPlan {
    pub fn new(kind: EvalKind, source: EvalSource, caller_is_strict: bool, span: Span) -> Self {
        let scope_mode = EvalScopeMode::for_kind(kind);
        let host_policy = EvalHostPolicy::for_kind_and_source(kind, &source);
        Self {
            kind,
            source,
            scope_mode,
            caller_is_strict,
            eval_source_is_strict: None,
            declaration_plan: None,
            completion_plan: None,
            host_policy,
            span,
        }
    }

    pub fn with_completion_plan(
        &self,
        caller_is_strict: bool,
        eval_is_strict: bool,
        declarations: EvalDeclarationPlan,
        steps: Vec<EvalCompletionStep>,
    ) -> Self {
        let completion_plan = EvalCompletionPlan::with_eval_context(
            self.scope_mode,
            caller_is_strict,
            eval_is_strict,
            declarations.clone(),
            steps,
        );
        Self {
            eval_source_is_strict: Some(eval_is_strict),
            declaration_plan: Some(declarations),
            completion_plan: Some(completion_plan),
            ..self.clone()
        }
    }

    pub fn completion_expr(&self) -> Option<ResolvedExpr> {
        self.completion_plan
            .clone()
            .map(ResolvedExpr::EvalCompletion)
    }

    pub fn completion_expr_with_context(
        &self,
        caller_is_strict: bool,
        eval_is_strict: bool,
        declarations: EvalDeclarationPlan,
        steps: Vec<EvalCompletionStep>,
    ) -> ResolvedExpr {
        self.with_completion_plan(caller_is_strict, eval_is_strict, declarations, steps)
            .completion_expr()
            .expect("EvalFragmentPlan::with_completion_plan must set completion_plan")
    }

    pub fn expected_host_policy(&self) -> EvalHostPolicy {
        EvalHostPolicy::for_kind_and_source(self.kind, &self.source)
    }

    pub fn host_policy_is_consistent(&self) -> bool {
        self.host_policy == self.expected_host_policy()
    }

    pub fn expected_scope_mode(&self) -> EvalScopeMode {
        EvalScopeMode::for_kind(self.kind)
    }

    pub fn scope_mode_is_consistent(&self) -> bool {
        self.scope_mode == self.expected_scope_mode()
    }

    pub fn completion_state_is_consistent(&self) -> bool {
        match (&self.declaration_plan, &self.completion_plan) {
            (None, None) => self.eval_source_is_strict.is_none(),
            (Some(declarations), Some(completion_plan)) => {
                self.host_policy == EvalHostPolicy::AotOnly
                    && matches!(self.source, EvalSource::StaticLiteral(_))
                    && self.scope_mode == completion_plan.scope_mode
                    && self.caller_is_strict == completion_plan.caller_is_strict
                    && self.eval_source_is_strict == Some(completion_plan.eval_is_strict)
                    && declarations == &completion_plan.declarations
                    && completion_plan.landing_state_is_consistent()
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum EvalKind {
    Direct,
    Indirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalScopeMode {
    Caller,
    Global { realm: EvalRealm },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalRealm {
    Current,
}

impl EvalScopeMode {
    pub fn for_kind(kind: EvalKind) -> Self {
        match kind {
            EvalKind::Direct => Self::Caller,
            EvalKind::Indirect => Self::Global {
                realm: EvalRealm::Current,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalHostPolicy {
    AotOnly,
    DirectHost,
    IndirectHost,
}

impl EvalHostPolicy {
    pub fn for_kind_and_source(kind: EvalKind, source: &EvalSource) -> Self {
        match (kind, source) {
            (_, EvalSource::StaticLiteral(_) | EvalSource::NonStringStatic(_)) => Self::AotOnly,
            (EvalKind::Direct, EvalSource::Runtime(_)) => Self::DirectHost,
            (EvalKind::Indirect, EvalSource::Runtime(_)) => Self::IndirectHost,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalSource {
    StaticLiteral(String),
    NonStringStatic(Box<ResolvedExpr>),
    Runtime(Box<ResolvedExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalCompletionPlan {
    pub scope_mode: EvalScopeMode,
    pub caller_is_strict: bool,
    pub eval_is_strict: bool,
    pub declarations: EvalDeclarationPlan,
    pub steps: Vec<EvalCompletionStep>,
}

impl EvalCompletionPlan {
    pub fn new(steps: Vec<EvalCompletionStep>) -> Self {
        Self {
            scope_mode: EvalScopeMode::Caller,
            caller_is_strict: false,
            eval_is_strict: false,
            declarations: EvalDeclarationPlan::default(),
            steps,
        }
    }

    pub fn with_declarations(
        declarations: EvalDeclarationPlan,
        steps: Vec<EvalCompletionStep>,
    ) -> Self {
        Self {
            scope_mode: EvalScopeMode::Caller,
            caller_is_strict: false,
            eval_is_strict: false,
            declarations,
            steps,
        }
    }

    pub fn with_eval_context(
        scope_mode: EvalScopeMode,
        caller_is_strict: bool,
        eval_is_strict: bool,
        declarations: EvalDeclarationPlan,
        steps: Vec<EvalCompletionStep>,
    ) -> Self {
        Self {
            scope_mode,
            caller_is_strict,
            eval_is_strict,
            declarations,
            steps,
        }
    }

    pub fn steps(&self) -> &[EvalCompletionStep] {
        &self.steps
    }

    pub fn as_slice(&self) -> &[EvalCompletionStep] {
        self.steps()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, EvalCompletionStep> {
        self.steps.iter()
    }

    pub fn last(&self) -> Option<&EvalCompletionStep> {
        self.steps.last()
    }

    pub fn landing_state_is_consistent(&self) -> bool {
        if self.eval_is_strict
            && (!self.declarations.is_empty()
                || self
                    .steps
                    .iter()
                    .any(EvalCompletionStep::has_caller_landing))
        {
            return false;
        }

        match self.scope_mode {
            EvalScopeMode::Caller => self.steps.iter().all(|step| !step.has_global_landing()),
            EvalScopeMode::Global { .. } => {
                self.declarations.is_empty()
                    && self.steps.iter().all(|step| !step.has_caller_landing())
            }
        }
    }
}

impl<'a> IntoIterator for &'a EvalCompletionPlan {
    type Item = &'a EvalCompletionStep;
    type IntoIter = std::slice::Iter<'a, EvalCompletionStep>;

    fn into_iter(self) -> Self::IntoIter {
        self.steps.iter()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvalDeclarationPlan {
    pub var_names: Vec<String>,
    pub function_hoists: Vec<EvalFunctionHoist>,
}

impl EvalDeclarationPlan {
    pub fn is_empty(&self) -> bool {
        self.var_names.is_empty() && self.function_hoists.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResolvedStmt;
    use crate::builtin_resolved::{
        FunctionConstructorHostPolicy, FunctionConstructorKind, FunctionConstructorPlan,
    };

    #[test]
    fn eval_fragment_plan_records_eval_source_strictness() {
        let plan = EvalFragmentPlan::new(
            EvalKind::Direct,
            EvalSource::StaticLiteral("\"use strict\"; 1".to_owned()),
            false,
            Span::generated("eval_fragment_plan_test"),
        );
        assert_eq!(plan.eval_source_is_strict, None);

        let plan = plan.with_completion_plan(
            false,
            true,
            EvalDeclarationPlan::default(),
            vec![EvalCompletionStep::Value(ResolvedExpr::Number(1))],
        );

        assert_eq!(plan.eval_source_is_strict, Some(true));
        assert!(
            plan.completion_plan
                .as_ref()
                .is_some_and(|completion| completion.eval_is_strict)
        );
    }

    #[test]
    fn eval_fragment_plan_derives_expected_host_policy() {
        let static_direct = EvalFragmentPlan::new(
            EvalKind::Direct,
            EvalSource::StaticLiteral("1".to_owned()),
            false,
            Span::generated("static_direct_eval_policy_test"),
        );
        assert_eq!(
            static_direct.expected_host_policy(),
            EvalHostPolicy::AotOnly
        );
        assert!(static_direct.host_policy_is_consistent());

        let runtime_direct = EvalFragmentPlan::new(
            EvalKind::Direct,
            EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
            false,
            Span::generated("runtime_direct_eval_policy_test"),
        );
        assert_eq!(
            runtime_direct.expected_host_policy(),
            EvalHostPolicy::DirectHost
        );
        assert!(runtime_direct.host_policy_is_consistent());

        let runtime_indirect = EvalFragmentPlan::new(
            EvalKind::Indirect,
            EvalSource::Runtime(Box::new(ResolvedExpr::Ident("source".to_owned()))),
            false,
            Span::generated("runtime_indirect_eval_policy_test"),
        );
        assert_eq!(
            runtime_indirect.expected_host_policy(),
            EvalHostPolicy::IndirectHost
        );
        assert!(runtime_indirect.host_policy_is_consistent());

        let inconsistent = EvalFragmentPlan {
            host_policy: EvalHostPolicy::AotOnly,
            ..runtime_direct
        };
        assert_eq!(
            inconsistent.expected_host_policy(),
            EvalHostPolicy::DirectHost
        );
        assert!(!inconsistent.host_policy_is_consistent());
    }

    #[test]
    fn eval_fragment_plan_derives_expected_scope_mode() {
        let direct = EvalFragmentPlan::new(
            EvalKind::Direct,
            EvalSource::StaticLiteral("1".to_owned()),
            false,
            Span::generated("direct_eval_scope_policy_test"),
        );
        assert_eq!(direct.expected_scope_mode(), EvalScopeMode::Caller);
        assert!(direct.scope_mode_is_consistent());

        let indirect = EvalFragmentPlan::new(
            EvalKind::Indirect,
            EvalSource::StaticLiteral("1".to_owned()),
            false,
            Span::generated("indirect_eval_scope_policy_test"),
        );
        assert_eq!(
            indirect.expected_scope_mode(),
            EvalScopeMode::Global {
                realm: EvalRealm::Current
            }
        );
        assert!(indirect.scope_mode_is_consistent());

        let inconsistent = EvalFragmentPlan {
            scope_mode: EvalScopeMode::Caller,
            ..indirect
        };
        assert_eq!(
            inconsistent.expected_scope_mode(),
            EvalScopeMode::Global {
                realm: EvalRealm::Current
            }
        );
        assert!(!inconsistent.scope_mode_is_consistent());
    }

    #[test]
    fn eval_fragment_plan_validates_embedded_completion_state() {
        let plan = EvalFragmentPlan::new(
            EvalKind::Direct,
            EvalSource::StaticLiteral("1".to_owned()),
            false,
            Span::generated("eval_completion_state_policy_test"),
        )
        .with_completion_plan(
            false,
            false,
            EvalDeclarationPlan {
                var_names: vec!["value".to_owned()],
                function_hoists: vec![],
            },
            vec![EvalCompletionStep::Value(ResolvedExpr::Ident(
                "value".to_owned(),
            ))],
        );
        assert!(plan.completion_state_is_consistent());

        let missing_declaration_plan = EvalFragmentPlan {
            declaration_plan: None,
            ..plan.clone()
        };
        assert!(!missing_declaration_plan.completion_state_is_consistent());

        let mismatched_strictness = EvalFragmentPlan {
            eval_source_is_strict: Some(true),
            ..plan.clone()
        };
        assert!(!mismatched_strictness.completion_state_is_consistent());

        let mismatched_scope = EvalFragmentPlan {
            completion_plan: plan
                .completion_plan
                .clone()
                .map(|completion| EvalCompletionPlan {
                    scope_mode: EvalScopeMode::Global {
                        realm: EvalRealm::Current,
                    },
                    ..completion
                }),
            ..plan.clone()
        };
        assert!(!mismatched_scope.completion_state_is_consistent());

        let mismatched_landing = EvalFragmentPlan {
            completion_plan: plan
                .completion_plan
                .clone()
                .map(|completion| EvalCompletionPlan {
                    steps: vec![EvalCompletionStep::GlobalVarLet {
                        name: "value".to_owned(),
                        init: ResolvedExpr::Number(1),
                    }],
                    ..completion
                }),
            ..plan.clone()
        };
        assert!(!mismatched_landing.completion_state_is_consistent());

        let runtime_with_completion = EvalFragmentPlan {
            source: EvalSource::Runtime(Box::new(ResolvedExpr::Ident("src".to_owned()))),
            host_policy: EvalHostPolicy::DirectHost,
            ..plan.clone()
        };
        assert!(!runtime_with_completion.completion_state_is_consistent());

        let non_string_static_with_completion = EvalFragmentPlan {
            source: EvalSource::NonStringStatic(Box::new(ResolvedExpr::Object(vec![]))),
            ..plan
        };
        assert!(!non_string_static_with_completion.completion_state_is_consistent());
    }

    #[test]
    fn eval_completion_plan_validates_scope_landing_state() {
        let caller_plan = EvalCompletionPlan::with_eval_context(
            EvalScopeMode::Caller,
            false,
            false,
            EvalDeclarationPlan {
                var_names: vec!["value".to_owned()],
                function_hoists: vec![],
            },
            vec![EvalCompletionStep::VarLet {
                name: "value".to_owned(),
                init: ResolvedExpr::Number(1),
            }],
        );
        assert!(caller_plan.landing_state_is_consistent());

        let caller_with_global_landing = EvalCompletionPlan::with_eval_context(
            EvalScopeMode::Caller,
            false,
            false,
            EvalDeclarationPlan::default(),
            vec![EvalCompletionStep::GlobalVarLet {
                name: "value".to_owned(),
                init: ResolvedExpr::Number(1),
            }],
        );
        assert!(!caller_with_global_landing.landing_state_is_consistent());

        let global_plan = EvalCompletionPlan::with_eval_context(
            EvalScopeMode::Global {
                realm: EvalRealm::Current,
            },
            false,
            false,
            EvalDeclarationPlan::default(),
            vec![EvalCompletionStep::GlobalVarLet {
                name: "value".to_owned(),
                init: ResolvedExpr::Number(1),
            }],
        );
        assert!(global_plan.landing_state_is_consistent());

        let global_with_caller_landing = EvalCompletionPlan::with_eval_context(
            EvalScopeMode::Global {
                realm: EvalRealm::Current,
            },
            false,
            false,
            EvalDeclarationPlan::default(),
            vec![EvalCompletionStep::Block(vec![
                EvalCompletionStep::VarLet {
                    name: "value".to_owned(),
                    init: ResolvedExpr::Number(1),
                },
            ])],
        );
        assert!(!global_with_caller_landing.landing_state_is_consistent());

        let strict_eval_with_caller_declaration = EvalCompletionPlan::with_eval_context(
            EvalScopeMode::Caller,
            true,
            true,
            EvalDeclarationPlan {
                var_names: vec!["value".to_owned()],
                function_hoists: vec![],
            },
            vec![EvalCompletionStep::VarLet {
                name: "value".to_owned(),
                init: ResolvedExpr::Number(1),
            }],
        );
        assert!(!strict_eval_with_caller_declaration.landing_state_is_consistent());
    }

    #[test]
    fn function_constructor_plan_derives_expected_host_policy() {
        let static_plan = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::String("return 1".to_owned())],
            Span::generated("static_function_constructor_policy_test"),
        );
        assert_eq!(
            static_plan.expected_host_policy(),
            FunctionConstructorHostPolicy::AotOnly
        );
        assert!(static_plan.host_policy_is_consistent());

        let runtime_plan = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::Ident("body".to_owned())],
            Span::generated("runtime_function_constructor_policy_test"),
        );
        assert_eq!(
            runtime_plan.expected_host_policy(),
            FunctionConstructorHostPolicy::HostCompile
        );
        assert!(runtime_plan.host_policy_is_consistent());

        let inconsistent = FunctionConstructorPlan {
            host_policy: FunctionConstructorHostPolicy::HostCompile,
            ..static_plan
        };
        assert_eq!(
            inconsistent.expected_host_policy(),
            FunctionConstructorHostPolicy::AotOnly
        );
        assert!(!inconsistent.host_policy_is_consistent());
    }

    #[test]
    fn function_constructor_plan_validates_static_source_metadata() {
        let static_plan = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::String("return 1".to_owned())],
            Span::generated("static_function_constructor_metadata_policy_test"),
        );
        assert!(static_plan.static_source_is_consistent());

        let runtime_plan = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::Ident("body".to_owned())],
            Span::generated("runtime_function_constructor_metadata_policy_test"),
        );
        assert!(runtime_plan.static_source_is_consistent());

        let mut mismatched_name = static_plan.clone();
        mismatched_name
            .static_source
            .as_mut()
            .expect("static source should exist")
            .generated_function
            .name = "notAnonymous".to_owned();
        assert!(!mismatched_name.static_source_is_consistent());

        let mut mismatched_length = static_plan.clone();
        mismatched_length
            .static_source
            .as_mut()
            .expect("static source should exist")
            .generated_function
            .length = Some(1);
        assert!(!mismatched_length.static_source_is_consistent());

        let mut mismatched_body = static_plan.clone();
        mismatched_body
            .static_source
            .as_mut()
            .expect("static source should exist")
            .body = "return 2".to_owned();
        assert!(!mismatched_body.static_source_is_consistent());

        let missing_static_source = FunctionConstructorPlan {
            static_source: None,
            host_policy: FunctionConstructorHostPolicy::HostCompile,
            ..static_plan.clone()
        };
        assert!(!missing_static_source.host_policy_is_consistent());
        assert!(!missing_static_source.static_source_is_consistent());

        let mismatched_policy = FunctionConstructorPlan {
            host_policy: FunctionConstructorHostPolicy::HostCompile,
            ..static_plan
        };
        assert!(!mismatched_policy.static_source_is_consistent());
    }

    #[test]
    fn lowering_rejects_function_constructor_static_metadata_drift() {
        let mut plan = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::String("return 1".to_owned())],
            Span::generated("lowering_function_constructor_metadata_policy_test"),
        );
        plan.static_source
            .as_mut()
            .expect("static source should exist")
            .generated_function
            .name = "notAnonymous".to_owned();

        let err = crate::lowered::lower_program(&[ResolvedStmt::Expr(
            ResolvedExpr::FunctionConstructor { plan },
        )])
        .expect_err("lowering should reject malformed Function constructor metadata");

        assert_eq!(err.code, ts2wasm_diagnostic::DiagCode::UnsupportedEval);
        assert!(err.message.contains("static source metadata"));
    }

    #[test]
    fn function_constructor_sequence_sources_require_static_prefixes() {
        let static_sequence = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::Sequence(vec![
                ResolvedExpr::Number(0),
                ResolvedExpr::String("return 1".to_owned()),
            ])],
            Span::generated("static_function_constructor_sequence_policy_test"),
        );
        assert!(static_sequence.static_source.is_some());
        assert_eq!(
            static_sequence.expected_host_policy(),
            FunctionConstructorHostPolicy::AotOnly
        );

        let effectful_sequence = FunctionConstructorPlan::new(
            FunctionConstructorKind::Call,
            vec![ResolvedExpr::Sequence(vec![
                ResolvedExpr::Assign {
                    name: "side".to_owned(),
                    expr: Box::new(ResolvedExpr::Number(1)),
                },
                ResolvedExpr::String("return 1".to_owned()),
            ])],
            Span::generated("effectful_function_constructor_sequence_policy_test"),
        );
        assert!(effectful_sequence.static_source.is_none());
        assert_eq!(
            effectful_sequence.expected_host_policy(),
            FunctionConstructorHostPolicy::HostCompile
        );
    }
}
