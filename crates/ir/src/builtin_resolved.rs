use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, FunctionExprOrigin, LogicalAssignOp, UnaryOp};

use super::builtin::{BuiltinId, BuiltinPropertyId};
use crate::binding_pattern::BindingPattern;

pub type ResolvedConstructor = (Vec<ResolvedParam>, Vec<ResolvedStmt>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedParam {
    pub name: String,
    pub default: Option<ResolvedExpr>,
    pub is_rest: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedStmt {
    AmbientValue(String),
    Let(String, ResolvedExpr),
    DestructureLet {
        pattern: BindingPattern,
        expr: ResolvedExpr,
    },
    Assign(String, ResolvedExpr),
    Expr(ResolvedExpr),
    If {
        condition: ResolvedExpr,
        then_body: Vec<ResolvedStmt>,
        else_body: Vec<ResolvedStmt>,
    },
    While {
        condition: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    Return(ResolvedExpr),
    Function {
        name: String,
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
        is_generator: bool,
        is_async: bool,
        is_ambient: bool,
        source_text: String,
    },
    TryCatch {
        try_block: Vec<ResolvedStmt>,
        catch_param: Option<String>,
        catch_block: Option<Vec<ResolvedStmt>>,
        finally_block: Option<Vec<ResolvedStmt>>,
    },
    Throw(ResolvedExpr),
    Switch {
        expr: ResolvedExpr,
        cases: Vec<(Option<ResolvedExpr>, Vec<ResolvedStmt>)>,
    },
    DoWhile {
        body: Vec<ResolvedStmt>,
        condition: ResolvedExpr,
    },
    For {
        init: Option<Box<ResolvedStmt>>,
        condition: Option<ResolvedExpr>,
        update: Option<ResolvedExpr>,
        body: Vec<ResolvedStmt>,
    },
    ForIn {
        var: String,
        iter: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    ForOf {
        var: String,
        iter: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    ForAwaitOf {
        var: String,
        iter: ResolvedExpr,
        body: Vec<ResolvedStmt>,
    },
    Labeled {
        label: String,
        body: Box<ResolvedStmt>,
    },
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
    Export {
        name: String,
        expr: Box<ResolvedExpr>,
    },
    ModuleExportsAssign {
        expr: Box<ResolvedExpr>,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<ResolvedConstructor>,
        methods: Vec<ClassMethod>,
        statics: Vec<(String, ResolvedExpr)>,
        static_blocks: Vec<(Span, Vec<ResolvedStmt>)>,
        private_fields: Vec<String>,
        static_private_fields: Vec<(String, ResolvedExpr, Span)>,
    },
    Block {
        statements: Vec<ResolvedStmt>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMethod {
    pub name: String,
    pub kind: ClassMethodKind,
    pub params: Vec<ResolvedParam>,
    pub body: Vec<ResolvedStmt>,
    pub captures: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassMethodKind {
    Method,
    Getter,
    Setter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedExpr {
    Number(i32),
    DecimalNumber(String),
    BigIntLiteral {
        decimal: String,
        sign: i32,
        limb_low: u32,
        limb_high: u32,
    },
    String(String),
    Bool(bool),
    Null,
    Undefined,
    This {
        span: Span,
    },
    NewTarget {
        span: Span,
    },
    ImportMeta {
        span: Span,
    },
    Await {
        expr: Box<ResolvedExpr>,
    },
    Yield {
        expr: Option<Box<ResolvedExpr>>,
        delegate: bool,
    },
    Ident(String),
    Unary {
        op: UnaryOp,
        expr: Box<ResolvedExpr>,
    },
    Binary {
        left: Box<ResolvedExpr>,
        op: BinaryOp,
        right: Box<ResolvedExpr>,
    },
    Ternary {
        condition: Box<ResolvedExpr>,
        then_expr: Box<ResolvedExpr>,
        else_expr: Box<ResolvedExpr>,
        span: Span,
    },
    Call {
        callee: Box<ResolvedExpr>,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    Assign {
        name: String,
        expr: Box<ResolvedExpr>,
    },
    LogicalAssign {
        name: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalPropertyAssign {
        object: String,
        key: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalComputedPropertyAssign {
        object: String,
        key: Box<ResolvedExpr>,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalComputedMemberAssign {
        object: Box<ResolvedExpr>,
        key: Box<ResolvedExpr>,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    LogicalMemberAssign {
        object: Box<ResolvedExpr>,
        key: String,
        op: LogicalAssignOp,
        expr: Box<ResolvedExpr>,
    },
    Array(Vec<ResolvedArrayElement>),
    Object(Vec<ResolvedObjectProp>),
    ComputedIndex {
        object: Box<ResolvedExpr>,
        index: Box<ResolvedExpr>,
    },
    BuiltinCall {
        builtin: BuiltinId,
        args: Vec<ResolvedExpr>,
    },
    BuiltinProperty {
        builtin: BuiltinPropertyId,
        object: Box<ResolvedExpr>,
        span: Span,
    },
    PropertyAccess {
        object: Box<ResolvedExpr>,
        key: String,
        span: Span,
    },
    OptionalPropertyAccess {
        object: Box<ResolvedExpr>,
        key: String,
        span: Span,
    },
    OptionalComputedIndex {
        object: Box<ResolvedExpr>,
        index: Box<ResolvedExpr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<ResolvedExpr>,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<ResolvedExpr>,
        method: String,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    PropertyAssign {
        object: Box<ResolvedExpr>,
        key: String,
        value: Box<ResolvedExpr>,
        span: Span,
    },
    Spread(Box<ResolvedExpr>),
    PropertyAssignDynamic {
        object: Box<ResolvedExpr>,
        key: Box<ResolvedExpr>,
        value: Box<ResolvedExpr>,
    },
    FunctionConstructor {
        plan: FunctionConstructorPlan,
    },
    New {
        class_name: String,
        args: Vec<ResolvedExpr>,
        span: Span,
    },
    ModuleLoad {
        specifier: String,
        is_dynamic_import: bool,
    },
    ArrowFn {
        params: Vec<String>,
        body: Box<ResolvedExpr>,
        body_stmts: Vec<ResolvedStmt>,
        source_text: String,
    },
    FunctionExpr {
        name: String,
        params: Vec<ResolvedParam>,
        body: Vec<ResolvedStmt>,
        is_generator: bool,
        origin: FunctionExprOrigin,
        constructor_metadata: Option<FunctionConstructorGeneratedFunction>,
        source_text: String,
    },
    ClassExpr {
        name: String,
        body: Vec<ResolvedStmt>,
    },
    Sequence(Vec<ResolvedExpr>),
    EvalCompletion(EvalCompletionPlan),
    Eval {
        plan: EvalFragmentPlan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalFragmentPlan {
    pub kind: EvalKind,
    pub source: EvalSource,
    pub scope_mode: EvalScopeMode,
    pub caller_is_strict: bool,
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
            host_policy,
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionConstructorKind {
    Call,
    New,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionConstructorPlan {
    pub kind: FunctionConstructorKind,
    pub args: Vec<ResolvedExpr>,
    pub static_source: Option<StaticFunctionConstructorSource>,
    pub host_policy: FunctionConstructorHostPolicy,
    pub span: Span,
}

impl FunctionConstructorPlan {
    pub fn new(kind: FunctionConstructorKind, args: Vec<ResolvedExpr>, span: Span) -> Self {
        let static_source = StaticFunctionConstructorSource::from_args(&args);
        let host_policy = FunctionConstructorHostPolicy::for_static_source(&static_source);
        Self {
            kind,
            args,
            static_source,
            host_policy,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticFunctionConstructorSource {
    pub params: Vec<String>,
    pub body: String,
    pub parse_goals: FunctionConstructorParseGoals,
    pub generated_function: FunctionConstructorGeneratedFunction,
}

impl StaticFunctionConstructorSource {
    pub fn from_args(args: &[ResolvedExpr]) -> Option<Self> {
        if !args
            .iter()
            .all(|arg| matches!(arg, ResolvedExpr::String(_)))
        {
            return None;
        }
        let strings = args
            .iter()
            .map(|arg| match arg {
                ResolvedExpr::String(value) => value.clone(),
                _ => unreachable!("all args were validated as strings"),
            })
            .collect::<Vec<_>>();
        let (body, params) = strings
            .split_last()
            .map_or(("", &[][..]), |(body, params)| (body.as_str(), params));
        Some(Self {
            params: params.to_vec(),
            body: body.to_owned(),
            parse_goals: FunctionConstructorParseGoals::default(),
            generated_function: FunctionConstructorGeneratedFunction::anonymous(),
        })
    }

    pub fn synthetic_function_source(&self) -> String {
        let params_source = self
            .params
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "function {}({params_source}) {{\n{}\n}}",
            self.generated_function.name, self.body
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionConstructorParseGoals {
    pub params: FunctionConstructorParseGoal,
    pub body: FunctionConstructorParseGoal,
}

impl Default for FunctionConstructorParseGoals {
    fn default() -> Self {
        Self {
            params: FunctionConstructorParseGoal::FormalParameters,
            body: FunctionConstructorParseGoal::FunctionBody,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionConstructorParseGoal {
    FormalParameters,
    FunctionBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionConstructorGeneratedFunction {
    pub name: String,
    pub constructable: bool,
    pub suppress_captures: bool,
}

impl FunctionConstructorGeneratedFunction {
    pub fn anonymous() -> Self {
        Self {
            name: "anonymous".to_owned(),
            constructable: true,
            suppress_captures: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionConstructorHostPolicy {
    AotOnly,
    HostCompile,
}

impl FunctionConstructorHostPolicy {
    pub fn for_static_source(static_source: &Option<StaticFunctionConstructorSource>) -> Self {
        if static_source.is_some() {
            Self::AotOnly
        } else {
            Self::HostCompile
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
            (_, EvalSource::StaticLiteral(_)) => Self::AotOnly,
            (EvalKind::Direct, EvalSource::Runtime(_)) => Self::DirectHost,
            (EvalKind::Indirect, EvalSource::Runtime(_)) => Self::IndirectHost,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalSource {
    StaticLiteral(String),
    Runtime(Box<ResolvedExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalCompletionPlan {
    pub declarations: EvalDeclarationPlan,
    pub steps: Vec<EvalCompletionStep>,
}

impl EvalCompletionPlan {
    pub fn new(steps: Vec<EvalCompletionStep>) -> Self {
        Self {
            declarations: EvalDeclarationPlan::default(),
            steps,
        }
    }

    pub fn with_declarations(
        declarations: EvalDeclarationPlan,
        steps: Vec<EvalCompletionStep>,
    ) -> Self {
        Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalCompletionStep {
    Value(ResolvedExpr),
    Empty(Option<ResolvedExpr>),
    VarLet {
        name: String,
        init: ResolvedExpr,
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
        iter: ResolvedExpr,
        body_steps: Vec<EvalCompletionStep>,
    },
    ForIn {
        var: String,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalFunctionHoist {
    pub name: String,
    pub params: Vec<ResolvedParam>,
    pub body: Vec<ResolvedStmt>,
    pub is_async: bool,
}

impl EvalCompletionStep {
    pub fn expr(&self) -> Option<&ResolvedExpr> {
        match self {
            Self::Value(expr)
            | Self::Empty(Some(expr))
            | Self::VarLet { init: expr, .. }
            | Self::DestructureLet { init: expr, .. }
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
            | Self::Block(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedArrayElement {
    Present(ResolvedExpr),
    Hole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedObjectProp {
    KeyValue {
        key: String,
        value: ResolvedExpr,
    },
    Shorthand {
        key: String,
        value: ResolvedExpr,
    },
    ComputedKey {
        key: Box<ResolvedExpr>,
        value: ResolvedExpr,
    },
    MethodShorthand {
        key: String,
        value: ResolvedExpr,
    },
}

impl ResolvedObjectProp {
    pub fn static_key(&self) -> Option<&str> {
        match self {
            Self::KeyValue { key, .. }
            | Self::Shorthand { key, .. }
            | Self::MethodShorthand { key, .. } => Some(key),
            Self::ComputedKey { .. } => None,
        }
    }

    pub fn value(&self) -> &ResolvedExpr {
        match self {
            Self::KeyValue { value, .. }
            | Self::Shorthand { value, .. }
            | Self::ComputedKey { value, .. }
            | Self::MethodShorthand { value, .. } => value,
        }
    }

    pub fn computed_key(&self) -> Option<&ResolvedExpr> {
        match self {
            Self::ComputedKey { key, .. } => Some(key),
            _ => None,
        }
    }
}
