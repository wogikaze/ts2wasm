use crate::{ResolvedArrayElement, ResolvedExpr};
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp, UnaryOp};

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
        FunctionConstructorHostPolicy::for_static_source(
            &StaticFunctionConstructorSource::from_args(&self.args),
        )
    }

    pub fn host_policy_is_consistent(&self) -> bool {
        self.host_policy == self.expected_host_policy()
    }

    pub fn static_source_is_consistent(&self) -> bool {
        let expected_static_source = StaticFunctionConstructorSource::from_args(&self.args);
        if self.static_source != expected_static_source {
            return false;
        }

        match (&self.static_source, self.host_policy) {
            (None, FunctionConstructorHostPolicy::HostCompile) => true,
            (Some(source), FunctionConstructorHostPolicy::AotOnly) => source.is_consistent(),
            _ => false,
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

    pub fn is_consistent(&self) -> bool {
        self.parse_goals == FunctionConstructorParseGoals::default()
            && self.generated_function.is_anonymous_constructor_base()
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

    pub fn is_anonymous_constructor_base(&self) -> bool {
        self.name == "anonymous"
            && self.length.is_none()
            && self.constructable
            && self.suppress_captures
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
