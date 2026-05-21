use crate::binding_pattern::BindingPattern;
use crate::builtin_resolved::{
    ClassMethod, ResolvedConstructor, ResolvedExpr, ResolvedParam, ResolvedStmt,
};
use ts2wasm_source::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalCompletionStep {
    Value(ResolvedExpr),
    Empty(Option<ResolvedExpr>),
    VarLet {
        name: String,
        init: ResolvedExpr,
    },
    GlobalVarLet {
        name: String,
        init: ResolvedExpr,
    },
    GlobalFunctionDecl {
        name: String,
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
        is_generator: bool,
        is_async: bool,
        source_text: String,
    },
    FunctionDecl {
        name: String,
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
        is_async: bool,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<ResolvedConstructor>,
        methods: Vec<ClassMethod>,
        private_fields: Vec<String>,
        static_private_fields: Vec<(String, ResolvedExpr, Span)>,
        static_blocks: Vec<(Span, Vec<ResolvedStmt>)>,
    },
    Block(Vec<EvalCompletionStep>),
    If {
        condition: ResolvedExpr,
        then_steps: Vec<EvalCompletionStep>,
        else_steps: Vec<EvalCompletionStep>,
    },
    While {
        condition: ResolvedExpr,
        body_steps: Vec<EvalCompletionStep>,
    },
    DoWhile {
        body_steps: Vec<EvalCompletionStep>,
        condition: ResolvedExpr,
    },
    For {
        init: Option<Box<EvalCompletionStep>>,
        condition: Option<ResolvedExpr>,
        update: Option<ResolvedExpr>,
        body_steps: Vec<EvalCompletionStep>,
    },
    ForOf {
        var: String,
        var_landing: EvalForHeadVarLanding,
        var_pattern: Option<BindingPattern>,
        iter: ResolvedExpr,
        body_steps: Vec<EvalCompletionStep>,
    },
    ForIn {
        var: String,
        var_landing: EvalForHeadVarLanding,
        var_pattern: Option<BindingPattern>,
        iter: ResolvedExpr,
        body_steps: Vec<EvalCompletionStep>,
    },
    Switch {
        expr: ResolvedExpr,
        cases: Vec<(Option<ResolvedExpr>, Vec<EvalCompletionStep>)>,
    },
    TryCatch {
        try_steps: Vec<EvalCompletionStep>,
        catch_param: Option<String>,
        catch_steps: Option<Vec<EvalCompletionStep>>,
        finally_steps: Option<Vec<EvalCompletionStep>>,
    },
    Labeled {
        label: String,
        body: Box<EvalCompletionStep>,
    },
    Throw(ResolvedExpr),
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
    LexicalLet {
        name: String,
        init: ResolvedExpr,
    },
    DestructureLet {
        pattern: BindingPattern,
        init: ResolvedExpr,
    },
    DestructureVarLet {
        pattern: BindingPattern,
        init: ResolvedExpr,
        var_landing: EvalForHeadVarLanding,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalFunctionHoist {
    pub name: String,
    pub params: Vec<ResolvedParam>,
    pub body: Vec<ResolvedStmt>,
    pub is_generator: bool,
    pub is_async: bool,
    pub source_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalForHeadVarLanding {
    Local,
    Caller,
    Global,
}

impl EvalCompletionStep {
    pub fn has_caller_landing(&self) -> bool {
        match self {
            Self::VarLet { .. } | Self::FunctionDecl { .. } => true,
            Self::DestructureVarLet { var_landing, .. } => {
                *var_landing == EvalForHeadVarLanding::Caller
            }
            Self::Block(steps) => steps.iter().any(Self::has_caller_landing),
            Self::If {
                then_steps,
                else_steps,
                ..
            } => {
                then_steps.iter().any(Self::has_caller_landing)
                    || else_steps.iter().any(Self::has_caller_landing)
            }
            Self::While { body_steps, .. } | Self::DoWhile { body_steps, .. } => {
                body_steps.iter().any(Self::has_caller_landing)
            }
            Self::ForOf {
                var_landing,
                body_steps,
                ..
            }
            | Self::ForIn {
                var_landing,
                body_steps,
                ..
            } => {
                *var_landing == EvalForHeadVarLanding::Caller
                    || body_steps.iter().any(Self::has_caller_landing)
            }
            Self::For {
                init, body_steps, ..
            } => {
                init.as_deref().is_some_and(Self::has_caller_landing)
                    || body_steps.iter().any(Self::has_caller_landing)
            }
            Self::Switch { cases, .. } => cases
                .iter()
                .any(|(_, steps)| steps.iter().any(Self::has_caller_landing)),
            Self::TryCatch {
                try_steps,
                catch_steps,
                finally_steps,
                ..
            } => {
                try_steps.iter().any(Self::has_caller_landing)
                    || catch_steps
                        .as_deref()
                        .is_some_and(|steps| steps.iter().any(Self::has_caller_landing))
                    || finally_steps
                        .as_deref()
                        .is_some_and(|steps| steps.iter().any(Self::has_caller_landing))
            }
            Self::Labeled { body, .. } => body.has_caller_landing(),
            Self::Value(_)
            | Self::Empty(_)
            | Self::GlobalVarLet { .. }
            | Self::GlobalFunctionDecl { .. }
            | Self::ClassDecl { .. }
            | Self::Throw(_)
            | Self::Break { .. }
            | Self::Continue { .. }
            | Self::LexicalLet { .. }
            | Self::DestructureLet { .. } => false,
        }
    }

    pub fn has_global_landing(&self) -> bool {
        match self {
            Self::GlobalVarLet { .. } | Self::GlobalFunctionDecl { .. } => true,
            Self::DestructureVarLet { var_landing, .. } => {
                *var_landing == EvalForHeadVarLanding::Global
            }
            Self::Block(steps) => steps.iter().any(Self::has_global_landing),
            Self::If {
                then_steps,
                else_steps,
                ..
            } => {
                then_steps.iter().any(Self::has_global_landing)
                    || else_steps.iter().any(Self::has_global_landing)
            }
            Self::While { body_steps, .. } | Self::DoWhile { body_steps, .. } => {
                body_steps.iter().any(Self::has_global_landing)
            }
            Self::ForOf {
                var_landing,
                body_steps,
                ..
            }
            | Self::ForIn {
                var_landing,
                body_steps,
                ..
            } => {
                *var_landing == EvalForHeadVarLanding::Global
                    || body_steps.iter().any(Self::has_global_landing)
            }
            Self::For {
                init, body_steps, ..
            } => {
                init.as_deref().is_some_and(Self::has_global_landing)
                    || body_steps.iter().any(Self::has_global_landing)
            }
            Self::Switch { cases, .. } => cases
                .iter()
                .any(|(_, steps)| steps.iter().any(Self::has_global_landing)),
            Self::TryCatch {
                try_steps,
                catch_steps,
                finally_steps,
                ..
            } => {
                try_steps.iter().any(Self::has_global_landing)
                    || catch_steps
                        .as_deref()
                        .is_some_and(|steps| steps.iter().any(Self::has_global_landing))
                    || finally_steps
                        .as_deref()
                        .is_some_and(|steps| steps.iter().any(Self::has_global_landing))
            }
            Self::Labeled { body, .. } => body.has_global_landing(),
            Self::Value(_)
            | Self::Empty(_)
            | Self::VarLet { .. }
            | Self::FunctionDecl { .. }
            | Self::ClassDecl { .. }
            | Self::Throw(_)
            | Self::Break { .. }
            | Self::Continue { .. }
            | Self::LexicalLet { .. }
            | Self::DestructureLet { .. } => false,
        }
    }

    pub fn expr(&self) -> Option<&ResolvedExpr> {
        match self {
            Self::Value(expr)
            | Self::Empty(Some(expr))
            | Self::VarLet { init: expr, .. }
            | Self::GlobalVarLet { init: expr, .. }
            | Self::DestructureLet { init: expr, .. }
            | Self::DestructureVarLet { init: expr, .. }
            | Self::If {
                condition: expr, ..
            }
            | Self::While {
                condition: expr, ..
            }
            | Self::DoWhile {
                condition: expr, ..
            }
            | Self::LexicalLet { init: expr, .. } => Some(expr),
            Self::For {
                condition, update, ..
            } => condition.as_ref().or(update.as_ref()),
            Self::ForOf { iter: expr, .. } | Self::ForIn { iter: expr, .. } => Some(expr),
            Self::Switch { expr, .. } => Some(expr),
            Self::Throw(expr) => Some(expr),
            Self::ClassDecl { .. }
            | Self::Empty(None)
            | Self::TryCatch { .. }
            | Self::Labeled { .. }
            | Self::Break { .. }
            | Self::Continue { .. }
            | Self::FunctionDecl { .. }
            | Self::GlobalFunctionDecl { .. }
            | Self::Block(_) => None,
        }
    }
}
