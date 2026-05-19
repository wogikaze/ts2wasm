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

    pub fn expected_host_policy(&self) -> FunctionConstructorHostPolicy {
        FunctionConstructorHostPolicy::for_static_source(&self.static_source)
    }

    pub fn host_policy_is_consistent(&self) -> bool {
        self.host_policy == self.expected_host_policy()
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
        let strings = function_constructor_source_strings(args)?;
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
        self.generated_source_text()
    }

    pub fn generated_source_text(&self) -> String {
        let params_source = self
            .params
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "function {}({params_source}\n) {{\n{}\n}}",
            self.generated_function.name, self.body
        )
    }
}

fn function_constructor_source_strings(args: &[ResolvedExpr]) -> Option<Vec<String>> {
    let mut strings = Vec::new();
    for arg in args {
        match arg {
            ResolvedExpr::Spread(expr) => {
                let ResolvedExpr::Array(elements) = expr.as_ref() else {
                    return None;
                };
                for element in elements {
                    match element {
                        ResolvedArrayElement::Hole => strings.push("undefined".to_owned()),
                        ResolvedArrayElement::Present(expr) => {
                            strings.push(function_constructor_source_string(expr)?);
                        }
                    }
                }
            }
            _ => strings.push(function_constructor_source_string(arg)?),
        }
    }
    Some(strings)
}

fn function_constructor_source_string(arg: &ResolvedExpr) -> Option<String> {
    function_constructor_static_source_value(arg).map(|value| value.to_js_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FunctionConstructorStaticSourceValue {
    String(String),
    Number(i32),
    DecimalNumber(String),
    BigInt(String),
    Bool(bool),
    Null,
    Undefined,
    Array(Vec<Option<FunctionConstructorStaticSourceValue>>),
}

impl FunctionConstructorStaticSourceValue {
    fn to_js_string(&self) -> String {
        match self {
            Self::String(value) => value.clone(),
            Self::Number(value) => value.to_string(),
            Self::DecimalNumber(value) | Self::BigInt(value) => value.clone(),
            Self::Bool(true) => "true".to_owned(),
            Self::Bool(false) => "false".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Undefined => "undefined".to_owned(),
            Self::Array(elements) => function_constructor_static_array_to_string(elements),
        }
    }
}

fn function_constructor_static_source_value(
    arg: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    match arg {
        ResolvedExpr::String(value) => {
            Some(FunctionConstructorStaticSourceValue::String(value.clone()))
        }
        ResolvedExpr::Number(value) => Some(FunctionConstructorStaticSourceValue::Number(*value)),
        ResolvedExpr::DecimalNumber(value) => Some(
            FunctionConstructorStaticSourceValue::DecimalNumber(value.clone()),
        ),
        ResolvedExpr::BigIntLiteral { decimal, sign, .. } => {
            Some(FunctionConstructorStaticSourceValue::BigInt(if *sign < 0 {
                format!("-{decimal}")
            } else {
                decimal.clone()
            }))
        }
        ResolvedExpr::Bool(value) => Some(FunctionConstructorStaticSourceValue::Bool(*value)),
        ResolvedExpr::Null => Some(FunctionConstructorStaticSourceValue::Null),
        ResolvedExpr::Undefined => Some(FunctionConstructorStaticSourceValue::Undefined),
        ResolvedExpr::Array(elements) => function_constructor_static_array_source_value(elements),
        ResolvedExpr::Binary { left, op, right } => match op {
            BinaryOp::Add => function_constructor_static_add_source_value(left, right),
            BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo
            | BinaryOp::Power => {
                function_constructor_static_numeric_binary_source_value(left, *op, right)
            }
            BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
            | BinaryOp::StrictEqual
            | BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::StrictNotEqual => {
                function_constructor_static_comparison_source_value(left, *op, right)
            }
            BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::LeftShift
            | BinaryOp::RightShift
            | BinaryOp::UnsignedRightShift => {
                function_constructor_static_bitwise_source_value(left, *op, right)
            }
            BinaryOp::And | BinaryOp::Or | BinaryOp::NullishCoalesce => {
                function_constructor_static_logical_source_value(left, *op, right)
            }
            _ => None,
        },
        ResolvedExpr::Unary { op, expr } => {
            function_constructor_static_unary_source_value(*op, expr)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            let condition = function_constructor_static_source_value(condition)?;
            let selected = if function_constructor_static_to_boolean(&condition) {
                then_expr
            } else {
                else_expr
            };
            function_constructor_static_source_value(selected)
        }
        ResolvedExpr::Sequence(exprs) => {
            let (last, preceding) = exprs.split_last()?;
            for expr in preceding {
                function_constructor_static_source_value(expr)?;
            }
            function_constructor_static_source_value(last)
        }
        _ => None,
    }
}

fn function_constructor_static_array_source_value(
    elements: &[ResolvedArrayElement],
) -> Option<FunctionConstructorStaticSourceValue> {
    let mut values = Vec::with_capacity(elements.len());
    for element in elements {
        match element {
            ResolvedArrayElement::Hole => values.push(None),
            ResolvedArrayElement::Present(expr) => {
                values.push(Some(function_constructor_static_source_value(expr)?));
            }
        }
    }
    Some(FunctionConstructorStaticSourceValue::Array(values))
}

fn function_constructor_static_array_to_string(
    elements: &[Option<FunctionConstructorStaticSourceValue>],
) -> String {
    elements
        .iter()
        .map(|element| match element {
            Some(FunctionConstructorStaticSourceValue::Null)
            | Some(FunctionConstructorStaticSourceValue::Undefined)
            | None => String::new(),
            Some(value) => value.to_js_string(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn function_constructor_static_unary_source_value(
    op: UnaryOp,
    expr: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    if op == UnaryOp::TypeOf {
        return function_constructor_static_typeof_source_value(expr);
    }
    let value = function_constructor_static_source_value(expr)?;
    match op {
        UnaryOp::Not => Some(FunctionConstructorStaticSourceValue::Bool(
            !function_constructor_static_to_boolean(&value),
        )),
        UnaryOp::Void => Some(FunctionConstructorStaticSourceValue::Undefined),
        UnaryOp::Plus => match value {
            FunctionConstructorStaticSourceValue::Number(value) => {
                Some(FunctionConstructorStaticSourceValue::Number(value))
            }
            FunctionConstructorStaticSourceValue::DecimalNumber(value) => {
                Some(FunctionConstructorStaticSourceValue::DecimalNumber(value))
            }
            FunctionConstructorStaticSourceValue::Bool(true) => {
                Some(FunctionConstructorStaticSourceValue::Number(1))
            }
            FunctionConstructorStaticSourceValue::Bool(false)
            | FunctionConstructorStaticSourceValue::Null => {
                Some(FunctionConstructorStaticSourceValue::Number(0))
            }
            FunctionConstructorStaticSourceValue::String(value) => {
                Some(FunctionConstructorStaticSourceValue::DecimalNumber(
                    function_constructor_static_js_number_string(
                        function_constructor_static_string_to_number(&value)?,
                    ),
                ))
            }
            _ => None,
        },
        UnaryOp::Negate => match value {
            FunctionConstructorStaticSourceValue::Number(value) => {
                Some(FunctionConstructorStaticSourceValue::Number(-value))
            }
            FunctionConstructorStaticSourceValue::DecimalNumber(value) => {
                Some(FunctionConstructorStaticSourceValue::DecimalNumber(
                    negate_static_numeric_string(value),
                ))
            }
            FunctionConstructorStaticSourceValue::String(value) => {
                Some(FunctionConstructorStaticSourceValue::DecimalNumber(
                    function_constructor_static_js_number_string(
                        -function_constructor_static_string_to_number(&value)?,
                    ),
                ))
            }
            _ => None,
        },
        UnaryOp::BitwiseNot => {
            let value = function_constructor_static_to_int32(&value)?;
            Some(FunctionConstructorStaticSourceValue::Number(!value))
        }
        _ => None,
    }
}

fn function_constructor_static_typeof_source_value(
    expr: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let value = function_constructor_static_source_value(expr)?;
    let typeof_result = match value {
        FunctionConstructorStaticSourceValue::String(_) => "string",
        FunctionConstructorStaticSourceValue::Number(_)
        | FunctionConstructorStaticSourceValue::DecimalNumber(_) => "number",
        FunctionConstructorStaticSourceValue::BigInt(_) => "bigint",
        FunctionConstructorStaticSourceValue::Bool(_) => "boolean",
        FunctionConstructorStaticSourceValue::Undefined => "undefined",
        FunctionConstructorStaticSourceValue::Null
        | FunctionConstructorStaticSourceValue::Array(_) => "object",
    };
    Some(FunctionConstructorStaticSourceValue::String(
        typeof_result.to_owned(),
    ))
}

fn negate_static_numeric_string(value: String) -> String {
    value
        .strip_prefix('-')
        .map(str::to_owned)
        .unwrap_or_else(|| format!("-{value}"))
}

fn function_constructor_static_to_boolean(value: &FunctionConstructorStaticSourceValue) -> bool {
    match value {
        FunctionConstructorStaticSourceValue::String(value) => !value.is_empty(),
        FunctionConstructorStaticSourceValue::Number(value) => *value != 0,
        FunctionConstructorStaticSourceValue::DecimalNumber(value)
        | FunctionConstructorStaticSourceValue::BigInt(value) => value != "0",
        FunctionConstructorStaticSourceValue::Bool(value) => *value,
        FunctionConstructorStaticSourceValue::Array(_) => true,
        FunctionConstructorStaticSourceValue::Null
        | FunctionConstructorStaticSourceValue::Undefined => false,
    }
}

fn function_constructor_static_is_nullish(value: &FunctionConstructorStaticSourceValue) -> bool {
    matches!(
        value,
        FunctionConstructorStaticSourceValue::Null
            | FunctionConstructorStaticSourceValue::Undefined
    )
}

fn function_constructor_static_logical_source_value(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let left_value = function_constructor_static_source_value(left)?;
    let use_left = match op {
        BinaryOp::And => !function_constructor_static_to_boolean(&left_value),
        BinaryOp::Or => function_constructor_static_to_boolean(&left_value),
        BinaryOp::NullishCoalesce => !function_constructor_static_is_nullish(&left_value),
        _ => unreachable!("logical Function constructor source op"),
    };
    if use_left {
        Some(left_value)
    } else {
        function_constructor_static_source_value(right)
    }
}

fn function_constructor_static_add_source_value(
    left: &ResolvedExpr,
    right: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let left = function_constructor_static_source_value(left)?;
    let right = function_constructor_static_source_value(right)?;
    if matches!(left, FunctionConstructorStaticSourceValue::String(_))
        || matches!(right, FunctionConstructorStaticSourceValue::String(_))
    {
        return Some(FunctionConstructorStaticSourceValue::String(format!(
            "{}{}",
            left.to_js_string(),
            right.to_js_string()
        )));
    }
    match (left, right) {
        (
            FunctionConstructorStaticSourceValue::Number(left),
            FunctionConstructorStaticSourceValue::Number(right),
        ) => Some(FunctionConstructorStaticSourceValue::Number(left + right)),
        (left, right) => {
            let left = function_constructor_static_number_to_f64(&left)?;
            let right = function_constructor_static_number_to_f64(&right)?;
            let sum = left + right;
            sum.is_finite().then(|| {
                FunctionConstructorStaticSourceValue::DecimalNumber(
                    function_constructor_static_js_number_string(sum),
                )
            })
        }
    }
}

fn function_constructor_static_numeric_binary_source_value(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let left = function_constructor_static_source_value(left)?;
    let right = function_constructor_static_source_value(right)?;
    let left = function_constructor_static_number_to_f64(&left)?;
    let right = function_constructor_static_number_to_f64(&right)?;
    let result = match op {
        BinaryOp::Subtract => left - right,
        BinaryOp::Multiply => left * right,
        BinaryOp::Divide => left / right,
        BinaryOp::Modulo => left % right,
        BinaryOp::Power => left.powf(right),
        _ => unreachable!("numeric Function constructor source op"),
    };
    result.is_finite().then(|| {
        FunctionConstructorStaticSourceValue::DecimalNumber(
            function_constructor_static_js_number_string(result),
        )
    })
}

fn function_constructor_static_bitwise_source_value(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let left = function_constructor_static_source_value(left)?;
    let right = function_constructor_static_source_value(right)?;
    let result = match op {
        BinaryOp::BitwiseAnd => FunctionConstructorStaticSourceValue::Number(
            function_constructor_static_to_int32(&left)?
                & function_constructor_static_to_int32(&right)?,
        ),
        BinaryOp::BitwiseOr => FunctionConstructorStaticSourceValue::Number(
            function_constructor_static_to_int32(&left)?
                | function_constructor_static_to_int32(&right)?,
        ),
        BinaryOp::BitwiseXor => FunctionConstructorStaticSourceValue::Number(
            function_constructor_static_to_int32(&left)?
                ^ function_constructor_static_to_int32(&right)?,
        ),
        BinaryOp::LeftShift => FunctionConstructorStaticSourceValue::Number(
            function_constructor_static_to_int32(&left)?
                .wrapping_shl(function_constructor_static_shift_count(&right)?),
        ),
        BinaryOp::RightShift => FunctionConstructorStaticSourceValue::Number(
            function_constructor_static_to_int32(&left)?
                >> function_constructor_static_shift_count(&right)?,
        ),
        BinaryOp::UnsignedRightShift => {
            let shifted = function_constructor_static_to_uint32(&left)?
                >> function_constructor_static_shift_count(&right)?;
            FunctionConstructorStaticSourceValue::DecimalNumber(shifted.to_string())
        }
        _ => unreachable!("bitwise Function constructor source op"),
    };
    Some(result)
}

fn function_constructor_static_comparison_source_value(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
) -> Option<FunctionConstructorStaticSourceValue> {
    let left = function_constructor_static_source_value(left)?;
    let right = function_constructor_static_source_value(right)?;
    let result = match op {
        BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
            function_constructor_static_relational_compare(&left, op, &right)?
        }
        BinaryOp::StrictEqual => function_constructor_static_strict_equal(&left, &right)?,
        BinaryOp::StrictNotEqual => !function_constructor_static_strict_equal(&left, &right)?,
        BinaryOp::EqualEqual => function_constructor_static_loose_equal(&left, &right)?,
        BinaryOp::BangEqual => !function_constructor_static_loose_equal(&left, &right)?,
        _ => unreachable!("comparison Function constructor source op"),
    };
    Some(FunctionConstructorStaticSourceValue::Bool(result))
}

fn function_constructor_static_relational_compare(
    left: &FunctionConstructorStaticSourceValue,
    op: BinaryOp,
    right: &FunctionConstructorStaticSourceValue,
) -> Option<bool> {
    if let (
        FunctionConstructorStaticSourceValue::String(left),
        FunctionConstructorStaticSourceValue::String(right),
    ) = (left, right)
    {
        return Some(match op {
            BinaryOp::Less => left < right,
            BinaryOp::LessEqual => left <= right,
            BinaryOp::Greater => left > right,
            BinaryOp::GreaterEqual => left >= right,
            _ => unreachable!("relational Function constructor source op"),
        });
    }

    let left = function_constructor_static_number_to_f64(left)?;
    let right = function_constructor_static_number_to_f64(right)?;
    Some(match op {
        BinaryOp::Less => left < right,
        BinaryOp::LessEqual => left <= right,
        BinaryOp::Greater => left > right,
        BinaryOp::GreaterEqual => left >= right,
        _ => unreachable!("relational Function constructor source op"),
    })
}

fn function_constructor_static_strict_equal(
    left: &FunctionConstructorStaticSourceValue,
    right: &FunctionConstructorStaticSourceValue,
) -> Option<bool> {
    match (left, right) {
        (
            FunctionConstructorStaticSourceValue::String(left),
            FunctionConstructorStaticSourceValue::String(right),
        )
        | (
            FunctionConstructorStaticSourceValue::BigInt(left),
            FunctionConstructorStaticSourceValue::BigInt(right),
        ) => Some(left == right),
        (
            FunctionConstructorStaticSourceValue::Bool(left),
            FunctionConstructorStaticSourceValue::Bool(right),
        ) => Some(left == right),
        (
            FunctionConstructorStaticSourceValue::Null,
            FunctionConstructorStaticSourceValue::Null,
        )
        | (
            FunctionConstructorStaticSourceValue::Undefined,
            FunctionConstructorStaticSourceValue::Undefined,
        ) => Some(true),
        (
            FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
            FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
        ) => Some(
            function_constructor_static_number_to_f64(left)?
                == function_constructor_static_number_to_f64(right)?,
        ),
        (
            FunctionConstructorStaticSourceValue::Array(_),
            FunctionConstructorStaticSourceValue::Array(_),
        ) => None,
        _ => Some(false),
    }
}

fn function_constructor_static_loose_equal(
    left: &FunctionConstructorStaticSourceValue,
    right: &FunctionConstructorStaticSourceValue,
) -> Option<bool> {
    if function_constructor_static_strict_equal(left, right)? {
        return Some(true);
    }
    match (left, right) {
        (
            FunctionConstructorStaticSourceValue::Null,
            FunctionConstructorStaticSourceValue::Undefined,
        )
        | (
            FunctionConstructorStaticSourceValue::Undefined,
            FunctionConstructorStaticSourceValue::Null,
        ) => Some(true),
        (
            FunctionConstructorStaticSourceValue::Null
            | FunctionConstructorStaticSourceValue::Undefined,
            _,
        )
        | (
            _,
            FunctionConstructorStaticSourceValue::Null
            | FunctionConstructorStaticSourceValue::Undefined,
        ) => Some(false),
        (
            FunctionConstructorStaticSourceValue::Bool(_),
            FunctionConstructorStaticSourceValue::String(_)
            | FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
        )
        | (
            FunctionConstructorStaticSourceValue::String(_)
            | FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
            FunctionConstructorStaticSourceValue::Bool(_),
        )
        | (
            FunctionConstructorStaticSourceValue::String(_),
            FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
        )
        | (
            FunctionConstructorStaticSourceValue::Number(_)
            | FunctionConstructorStaticSourceValue::DecimalNumber(_),
            FunctionConstructorStaticSourceValue::String(_),
        ) => Some(
            function_constructor_static_number_to_f64(left)?
                == function_constructor_static_number_to_f64(right)?,
        ),
        _ => Some(false),
    }
}

fn function_constructor_static_shift_count(
    value: &FunctionConstructorStaticSourceValue,
) -> Option<u32> {
    Some(function_constructor_static_to_uint32(value)? & 0x1f)
}

fn function_constructor_static_to_int32(
    value: &FunctionConstructorStaticSourceValue,
) -> Option<i32> {
    let value = function_constructor_static_to_uint32(value)?;
    if value >= 0x8000_0000 {
        Some((i64::from(value) - 0x1_0000_0000) as i32)
    } else {
        Some(value as i32)
    }
}

fn function_constructor_static_to_uint32(
    value: &FunctionConstructorStaticSourceValue,
) -> Option<u32> {
    let value = function_constructor_static_number_to_f64(value)?;
    if !value.is_finite() || value == 0.0 {
        return Some(0);
    }
    let integer = value.trunc();
    Some(integer.rem_euclid(4_294_967_296.0) as u32)
}

fn function_constructor_static_number_to_f64(
    value: &FunctionConstructorStaticSourceValue,
) -> Option<f64> {
    match value {
        FunctionConstructorStaticSourceValue::Number(value) => Some(f64::from(*value)),
        FunctionConstructorStaticSourceValue::DecimalNumber(value) => value.parse().ok(),
        FunctionConstructorStaticSourceValue::Bool(true) => Some(1.0),
        FunctionConstructorStaticSourceValue::Bool(false)
        | FunctionConstructorStaticSourceValue::Null => Some(0.0),
        FunctionConstructorStaticSourceValue::String(value) => {
            function_constructor_static_string_to_number(value)
        }
        _ => None,
    }
}

fn function_constructor_static_string_to_number(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Some(0.0);
    }
    let parsed = trimmed.parse::<f64>().ok()?;
    parsed.is_finite().then_some(parsed)
}

fn function_constructor_static_js_number_string(value: f64) -> String {
    let text = value.to_string();
    if text == "-0" { "0".to_owned() } else { text }
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
    pub length: Option<usize>,
    pub constructable: bool,
    pub suppress_captures: bool,
}

impl FunctionConstructorGeneratedFunction {
    pub fn anonymous() -> Self {
        Self {
            name: "anonymous".to_owned(),
            length: None,
            constructable: true,
            suppress_captures: true,
        }
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
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

#[cfg(test)]
mod tests {
    use super::*;

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
