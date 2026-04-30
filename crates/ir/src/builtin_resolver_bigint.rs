use super::*;

pub(super) fn parse_bigint_literal(raw: &str, span: Span) -> Result<ResolvedExpr, Diagnostic> {
    let Some(body) = raw.strip_suffix('n') else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("issue-259: invalid BigInt literal `{raw}` reached runtime lowering"),
            span: Some(span),
        });
    };
    let (radix, digits) =
        if let Some(digits) = body.strip_prefix("0b").or_else(|| body.strip_prefix("0B")) {
            (2_u32, digits)
        } else if let Some(digits) = body.strip_prefix("0o").or_else(|| body.strip_prefix("0O")) {
            (8_u32, digits)
        } else if let Some(digits) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
            (16_u32, digits)
        } else {
            (10_u32, body)
        };

    let mut decimal_digits = vec![0_u8];
    let mut magnitude: u64 = 0;
    let mut magnitude_overflowed = false;
    for ch in digits.chars() {
        let Some(digit) = ch.to_digit(radix) else {
            return Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!("issue-259: invalid BigInt literal digit in `{raw}`"),
                span: Some(span),
            });
        };
        decimal_mul_add(&mut decimal_digits, radix as u8, digit as u8);
        if !magnitude_overflowed {
            if let Some(next) = magnitude
                .checked_mul(radix as u64)
                .and_then(|value| value.checked_add(digit as u64))
            {
                magnitude = next;
            } else {
                magnitude_overflowed = true;
            }
        }
    }

    trim_decimal_zeroes(&mut decimal_digits);
    let decimal = decimal_digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect::<String>();
    let sign = if decimal == "0" { 0 } else { 1 };
    let (limb_low, limb_high) = if magnitude_overflowed {
        (0, 0)
    } else {
        (magnitude as u32, (magnitude >> 32) as u32)
    };

    Ok(ResolvedExpr::BigIntLiteral {
        decimal,
        sign,
        limb_low,
        limb_high,
    })
}

pub(super) fn decimal_mul_add(digits: &mut Vec<u8>, radix: u8, add: u8) {
    let mut carry = add as u16;
    for digit in digits.iter_mut().rev() {
        let value = (*digit as u16) * (radix as u16) + carry;
        *digit = (value % 10) as u8;
        carry = value / 10;
    }
    while carry > 0 {
        digits.insert(0, (carry % 10) as u8);
        carry /= 10;
    }
}

pub(super) fn trim_decimal_zeroes(digits: &mut Vec<u8>) {
    while digits.len() > 1 && digits.first() == Some(&0) {
        digits.remove(0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BigIntConst {
    pub(super) sign: i32,
    pub(super) digits: Vec<u8>,
}

impl BigIntConst {
    pub(super) fn zero() -> Self {
        Self {
            sign: 0,
            digits: vec![0],
        }
    }

    pub(super) fn from_decimal(sign: i32, decimal: &str) -> Self {
        let body = decimal.strip_prefix('-').unwrap_or(decimal);
        let mut digits = body
            .bytes()
            .filter(|byte| byte.is_ascii_digit())
            .map(|byte| byte - b'0')
            .collect::<Vec<_>>();
        if digits.is_empty() {
            digits.push(0);
        }
        trim_decimal_zeroes(&mut digits);
        let sign = if digits == [0] { 0 } else { sign.signum() };
        Self { sign, digits }
    }

    pub(super) fn negated(mut self) -> Self {
        self.sign = -self.sign;
        self
    }

    pub(super) fn decimal_string(&self) -> String {
        let mut out = String::new();
        if self.sign < 0 {
            out.push('-');
        }
        out.extend(self.digits.iter().map(|digit| char::from(b'0' + *digit)));
        out
    }

    pub(super) fn fits_runtime_signed_i64(&self) -> bool {
        decimal_digits_to_u64(&self.digits).is_some_and(|magnitude| magnitude <= i64::MAX as u64)
    }
}

pub(super) fn bigint_from_resolved(expr: &ResolvedExpr) -> Option<BigIntConst> {
    match expr {
        ResolvedExpr::BigIntLiteral { decimal, sign, .. } => {
            Some(BigIntConst::from_decimal(*sign, decimal))
        }
        _ => None,
    }
}

pub(super) fn static_number_bigint_const(expr: &Expr) -> Option<BigIntConst> {
    match expr {
        Expr::Number { value, .. } => Some(bigint_from_i32(*value)),
        Expr::Unary { op, expr, .. } if *op == UnaryOp::Negate => {
            let Expr::Number { value, .. } = expr.as_ref() else {
                return None;
            };
            Some(bigint_from_i64(-i64::from(*value)))
        }
        _ => None,
    }
}

pub(super) fn resolved_number_bigint_const(expr: &ResolvedExpr) -> Option<BigIntConst> {
    match expr {
        ResolvedExpr::Number(value) => Some(bigint_from_i32(*value)),
        ResolvedExpr::Unary { op, expr } if *op == UnaryOp::Negate => {
            let ResolvedExpr::Number(value) = expr.as_ref() else {
                return None;
            };
            Some(bigint_from_i64(-i64::from(*value)))
        }
        _ => None,
    }
}

pub(super) fn bigint_to_resolved(value: BigIntConst) -> ResolvedExpr {
    let magnitude = decimal_digits_to_u64(&value.digits);
    let (limb_low, limb_high) = magnitude
        .map(|magnitude| (magnitude as u32, (magnitude >> 32) as u32))
        .unwrap_or((0, 0));
    ResolvedExpr::BigIntLiteral {
        decimal: value.decimal_string(),
        sign: value.sign,
        limb_low,
        limb_high,
    }
}

pub(super) fn resolve_bigint_function_call(
    args: &[ResolvedExpr],
    span: Span,
) -> Result<ResolvedExpr, Diagnostic> {
    let [arg] = args else {
        return Err(bigint_builtin_unsupported_diagnostic(span));
    };
    let value = match arg {
        ResolvedExpr::BigIntLiteral { .. } => return Ok(arg.clone()),
        ResolvedExpr::String(value) => bigint_from_string_builtin(value, span)?,
        ResolvedExpr::Bool(true) => BigIntConst::from_decimal(1, "1"),
        ResolvedExpr::Bool(false) => BigIntConst::zero(),
        ResolvedExpr::Number(value) => bigint_from_i32(*value),
        ResolvedExpr::Unary { op, expr }
            if *op == UnaryOp::Negate && matches!(expr.as_ref(), ResolvedExpr::Number(_)) =>
        {
            let ResolvedExpr::Number(value) = expr.as_ref() else {
                unreachable!("guarded by matches")
            };
            bigint_from_i64(-i64::from(*value))
        }
        ResolvedExpr::Null | ResolvedExpr::Undefined => {
            return Err(bigint_builtin_unsupported_diagnostic(span));
        }
        _ => {
            return Ok(ResolvedExpr::MethodCall {
                object: Box::new(ResolvedExpr::Ident(BIGINT_RUNTIME_OBJECT.to_owned())),
                method: BIGINT_FROM_VALUE_RUNTIME_CALL.to_owned(),
                args: args.to_vec(),
                span,
            });
        }
    };
    Ok(bigint_to_resolved(value))
}

pub(super) fn bigint_from_i32(value: i32) -> BigIntConst {
    bigint_from_i64(i64::from(value))
}

pub(super) fn bigint_from_i64(value: i64) -> BigIntConst {
    if value == 0 {
        return BigIntConst::zero();
    }
    let sign = value.signum() as i32;
    BigIntConst::from_decimal(sign, value.unsigned_abs().to_string().as_str())
}

pub(super) fn bigint_from_string_builtin(
    value: &str,
    span: Span,
) -> Result<BigIntConst, Diagnostic> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(BigIntConst::zero());
    }
    let (sign, explicit_sign, digits) = if let Some(digits) = trimmed.strip_prefix('-') {
        (-1, true, digits)
    } else if let Some(digits) = trimmed.strip_prefix('+') {
        (1, true, digits)
    } else {
        (1, false, trimmed)
    };

    let (radix, digits) = if let Some(digits) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2_u32, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8_u32, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16_u32, digits)
    } else {
        (10_u32, digits)
    };

    if (explicit_sign && radix != 10) || digits.is_empty() {
        return Err(bigint_string_diagnostic(span));
    }
    let mut decimal_digits = vec![0_u8];
    for ch in digits.chars() {
        let Some(digit) = ch.to_digit(radix) else {
            return Err(bigint_string_diagnostic(span));
        };
        decimal_mul_add(&mut decimal_digits, radix as u8, digit as u8);
    }
    trim_decimal_zeroes(&mut decimal_digits);
    let decimal = decimal_digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect::<String>();
    Ok(BigIntConst::from_decimal(sign, &decimal))
}

pub(super) fn bigint_fits_runtime_from_string(value: &BigIntConst) -> bool {
    match value.sign.cmp(&0) {
        std::cmp::Ordering::Less => decimal_digits_to_u64(&value.digits)
            .is_some_and(|magnitude| magnitude <= i64::MAX as u64),
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => decimal_digits_to_u64(&value.digits).is_some(),
    }
}

pub(super) fn fold_bigint_static_abstract_equality(
    left: &ResolvedExpr,
    op: BinaryOp,
    right: &ResolvedExpr,
    span: Span,
) -> Result<Option<ResolvedExpr>, Diagnostic> {
    if !matches!(
        op,
        BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
    ) {
        return Ok(None);
    }

    if let Some(ordering) = fold_static_bigint_number_ordering(left, right) {
        let result = match op {
            BinaryOp::EqualEqual => ordering == std::cmp::Ordering::Equal,
            BinaryOp::BangEqual => ordering != std::cmp::Ordering::Equal,
            BinaryOp::Less => ordering == std::cmp::Ordering::Less,
            BinaryOp::LessEqual => {
                matches!(
                    ordering,
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal
                )
            }
            BinaryOp::Greater => ordering == std::cmp::Ordering::Greater,
            BinaryOp::GreaterEqual => {
                matches!(
                    ordering,
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
                )
            }
            _ => unreachable!("guarded BigInt/Number comparison op"),
        };
        return Ok(Some(ResolvedExpr::Bool(result)));
    }

    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return Ok(None);
    }

    let compare = if let (Some(bigint), ResolvedExpr::String(value)) =
        (bigint_from_resolved(left), right)
    {
        Some((bigint, bigint_from_string_builtin(value, span).ok()))
    } else if let (ResolvedExpr::String(value), Some(bigint)) = (left, bigint_from_resolved(right))
    {
        Some((bigint, bigint_from_string_builtin(value, span).ok()))
    } else if let (Some(bigint), ResolvedExpr::Bool(value)) = (bigint_from_resolved(left), right) {
        Some((bigint, Some(bigint_from_bool(*value))))
    } else if let (ResolvedExpr::Bool(value), Some(bigint)) = (left, bigint_from_resolved(right)) {
        Some((bigint, Some(bigint_from_bool(*value))))
    } else if bigint_from_resolved(left).is_some()
        && matches!(right, ResolvedExpr::Null | ResolvedExpr::Undefined)
    {
        Some((BigIntConst::zero(), None))
    } else if matches!(left, ResolvedExpr::Null | ResolvedExpr::Undefined)
        && bigint_from_resolved(right).is_some()
    {
        Some((BigIntConst::zero(), None))
    } else {
        None
    };

    let Some((bigint, parsed_string)) = compare else {
        return Ok(None);
    };
    let equal = parsed_string.is_some_and(|string_bigint| string_bigint == bigint);
    Ok(Some(ResolvedExpr::Bool(if op == BinaryOp::BangEqual {
        !equal
    } else {
        equal
    })))
}

pub(super) fn fold_static_bigint_number_ordering(
    left: &ResolvedExpr,
    right: &ResolvedExpr,
) -> Option<std::cmp::Ordering> {
    if let (Some(bigint), Some(number)) = (
        bigint_from_resolved(left),
        resolved_number_bigint_const(right),
    ) {
        Some(bigint_cmp(&bigint, &number))
    } else if let (Some(number), Some(bigint)) = (
        resolved_number_bigint_const(left),
        bigint_from_resolved(right),
    ) {
        Some(bigint_cmp(&number, &bigint))
    } else {
        None
    }
}

pub(super) fn bigint_cmp(left: &BigIntConst, right: &BigIntConst) -> std::cmp::Ordering {
    left.sign
        .cmp(&right.sign)
        .then_with(|| match left.sign.cmp(&0) {
            std::cmp::Ordering::Less => cmp_abs(&right.digits, &left.digits),
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => cmp_abs(&left.digits, &right.digits),
        })
}

pub(super) fn bigint_from_bool(value: bool) -> BigIntConst {
    if value {
        BigIntConst::from_decimal(1, "1")
    } else {
        BigIntConst::zero()
    }
}

pub(super) fn bigint_string_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-280: BigInt(string) currently supports decimal, binary, octal, or hexadecimal integer string literals"
            .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_dynamic_string_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-333: dynamic BigInt(string) inputs with provably invalid or out-of-range StringToBigInt values require compatible runtime exception support".to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_builtin_unsupported_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-280: BigInt(...) currently supports static string/boolean/integer number inputs and dynamic boolean/integer number/BigInt inputs in this builtin slice".to_owned(),
        span: Some(span),
    }
}

pub(super) fn resolve_bigint_static_function_call(
    callee: &Expr,
    args: &[ResolvedExpr],
    span: Span,
) -> Result<Option<ResolvedExpr>, Diagnostic> {
    let Expr::Member {
        object, property, ..
    } = callee
    else {
        return Ok(None);
    };
    let Expr::Ident { name, .. } = object.as_ref() else {
        return Ok(None);
    };
    if name != "BigInt" || !matches!(property.as_str(), "asIntN" | "asUintN") {
        return Ok(None);
    }

    let [bits_arg, value_arg] = args else {
        return Err(bigint_static_width_diagnostic(span));
    };
    let static_bits = match bits_arg {
        ResolvedExpr::Number(_) => Some(bigint_static_width(bits_arg, span)?),
        ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::BigIntLiteral { .. } => return Err(bigint_static_width_diagnostic(span)),
        _ => None,
    };
    let static_value = bigint_from_resolved(value_arg);
    if let (Some(bits), Some(value)) = (static_bits, static_value) {
        let value = if property == "asIntN" {
            bigint_as_int_n(bits, value)
        } else {
            bigint_as_uint_n(bits, value)
        };
        return Ok(Some(bigint_to_resolved(value)));
    }
    if matches!(
        value_arg,
        ResolvedExpr::Number(_)
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
    ) {
        return Err(bigint_as_value_diagnostic(span));
    }
    Ok(Some(ResolvedExpr::MethodCall {
        object: Box::new(ResolvedExpr::Ident(BIGINT_RUNTIME_OBJECT.to_owned())),
        method: if property == "asIntN" {
            BIGINT_AS_INT_N_RUNTIME_CALL
        } else {
            BIGINT_AS_UINT_N_RUNTIME_CALL
        }
        .to_owned(),
        args: args.to_vec(),
        span,
    }))
}

pub(super) fn bigint_runtime_call_name(name: &str) -> Option<&'static str> {
    match name {
        BIGINT_FROM_VALUE_RUNTIME_CALL => Some("BigIntFromValue"),
        BIGINT_AS_INT_N_RUNTIME_CALL => Some("BigIntAsIntN"),
        BIGINT_AS_UINT_N_RUNTIME_CALL => Some("BigIntAsUintN"),
        _ => None,
    }
}

pub(crate) fn bigint_runtime_fn_name(name: &str) -> Option<&'static str> {
    bigint_runtime_call_name(name)
}

pub(super) fn bigint_static_width(arg: &ResolvedExpr, span: Span) -> Result<u32, Diagnostic> {
    let ResolvedExpr::Number(bits) = arg else {
        return Err(bigint_static_width_diagnostic(span));
    };
    if !(0..=64).contains(bits) {
        return Err(bigint_static_width_diagnostic(span));
    }
    Ok(*bits as u32)
}

pub(super) fn bigint_static_width_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-280: BigInt.asIntN/asUintN currently support integer literal bit widths 0..64"
                .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_as_value_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-280: BigInt.asIntN/asUintN currently require a supported BigInt value input"
                .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_as_uint_n(bits: u32, value: BigIntConst) -> BigIntConst {
    if bits == 0 || value.sign == 0 {
        return BigIntConst::zero();
    }
    let modulo = decimal_power_of_two(bits);
    let (_, remainder) = div_rem_abs(&value.digits, &modulo);
    if value.sign > 0 || remainder == [0] {
        return BigIntConst {
            sign: if remainder == [0] { 0 } else { 1 },
            digits: remainder,
        };
    }
    BigIntConst {
        sign: 1,
        digits: sub_abs(&modulo, &remainder),
    }
}

pub(super) fn bigint_as_int_n(bits: u32, value: BigIntConst) -> BigIntConst {
    if bits == 0 {
        return BigIntConst::zero();
    }
    let unsigned = bigint_as_uint_n(bits, value);
    let threshold = decimal_power_of_two(bits - 1);
    if unsigned.sign == 0 || cmp_abs(&unsigned.digits, &threshold) == std::cmp::Ordering::Less {
        return unsigned;
    }
    BigIntConst {
        sign: -1,
        digits: sub_abs(&decimal_power_of_two(bits), &unsigned.digits),
    }
}

pub(super) fn decimal_power_of_two(bits: u32) -> Vec<u8> {
    let mut digits = vec![1_u8];
    for _ in 0..bits {
        digits = mul_abs(&digits, &[2]);
    }
    digits
}

pub(super) fn decimal_digits_to_u64(digits: &[u8]) -> Option<u64> {
    let mut magnitude = 0_u64;
    for digit in digits {
        magnitude = magnitude.checked_mul(10)?.checked_add(u64::from(*digit))?;
    }
    Some(magnitude)
}

pub(super) fn bigint_arithmetic_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Power
            | BinaryOp::Divide
            | BinaryOp::Modulo
    )
}

#[derive(Debug, Clone)]
pub(super) struct BigIntStaticInfo {
    pub(super) value: Option<BigIntConst>,
    pub(super) helper_safe: bool,
    pub(super) runtime_needed: bool,
}

impl BigIntStaticInfo {
    pub(super) fn from_const(value: BigIntConst) -> Self {
        let helper_safe = value.fits_runtime_signed_i64();
        Self {
            value: Some(value),
            helper_safe,
            runtime_needed: false,
        }
    }
}

#[derive(Default)]
pub(super) struct BigIntRuntimeGuard {
    locals: HashMap<String, BigIntStaticInfo>,
    string_locals: HashSet<String>,
    string_values: HashMap<String, String>,
    object_string_values: HashMap<String, HashMap<String, String>>,
    object_bigint_props: HashMap<String, HashSet<String>>,
    nullish_locals: HashSet<String>,
    object_toprimitive_locals: HashSet<String>,
}

impl BigIntRuntimeGuard {
    pub(super) fn visit_stmts(&mut self, stmts: &[Stmt]) -> Result<(), Diagnostic> {
        for stmt in stmts {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    pub(super) fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), Diagnostic> {
        match stmt {
            Stmt::Let { name, expr, .. } | Stmt::Assign { name, expr, .. } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = info {
                    self.locals.insert(name.clone(), info);
                } else {
                    self.locals.remove(name);
                }
                if self.expr_is_definitely_string(expr) {
                    self.string_locals.insert(name.clone());
                    if let Some(value) = self.expr_static_string_value(expr) {
                        self.string_values.insert(name.clone(), value);
                    } else {
                        self.string_values.remove(name);
                    }
                } else {
                    self.string_locals.remove(name);
                    self.string_values.remove(name);
                }
                if let Some(props) = self.expr_static_object_string_values(expr) {
                    self.object_string_values.insert(name.clone(), props);
                } else {
                    self.object_string_values.remove(name);
                }
                if let Some(props) = self.expr_static_object_bigint_props(expr) {
                    self.object_bigint_props.insert(name.clone(), props);
                } else {
                    self.object_bigint_props.remove(name);
                }
                if self.expr_is_definitely_nullish(expr) {
                    self.nullish_locals.insert(name.clone());
                } else {
                    self.nullish_locals.remove(name);
                }
                if self.expr_is_object_toprimitive_boundary(expr) {
                    self.object_toprimitive_locals.insert(name.clone());
                } else {
                    self.object_toprimitive_locals.remove(name);
                }
                Ok(())
            }
            Stmt::Expr { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
                self.expr_bigint_info(expr).map(|_| ())
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().visit_stmts(then_body)?;
                self.fork().visit_stmts(else_body)?;
                self.invalidate_assigned_in_stmts(then_body);
                self.invalidate_assigned_in_stmts(else_body);
                Ok(())
            }
            Stmt::While {
                condition, body, ..
            }
            | Stmt::DoWhile {
                condition, body, ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().visit_stmts(body)?;
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::Function { body, .. } => BigIntRuntimeGuard::default().visit_stmts(body),
            Stmt::ClassDecl { body, .. } => {
                for item in body {
                    if let Stmt::Function { body, .. } = item {
                        BigIntRuntimeGuard::default().visit_stmts(body)?;
                    }
                }
                Ok(())
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                self.fork().visit_stmts(try_block)?;
                if let Some(catch_block) = catch_block {
                    self.fork().visit_stmts(catch_block)?;
                }
                if let Some(finally_block) = finally_block {
                    self.fork().visit_stmts(finally_block)?;
                }
                self.invalidate_assigned_in_stmts(try_block);
                if let Some(catch_block) = catch_block {
                    self.invalidate_assigned_in_stmts(catch_block);
                }
                if let Some(finally_block) = finally_block {
                    self.invalidate_assigned_in_stmts(finally_block);
                }
                Ok(())
            }
            Stmt::Switch { expr, cases, .. } => {
                self.expr_bigint_info(expr)?;
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        self.expr_bigint_info(case_expr)?;
                    }
                    self.fork().visit_stmts(body)?;
                    self.invalidate_assigned_in_stmts(body);
                }
                Ok(())
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                let mut loop_guard = self.fork();
                if let Some(init) = init {
                    loop_guard.visit_stmt(init)?;
                }
                if let Some(condition) = condition {
                    loop_guard.expr_bigint_info(condition)?;
                }
                if let Some(update) = update {
                    loop_guard.expr_bigint_info(update)?;
                }
                loop_guard.visit_stmts(body)?;
                if let Some(update) = update {
                    self.invalidate_assigned_in_expr(update);
                }
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::ForIn {
                var, iter, body, ..
            }
            | Stmt::ForOf {
                var, iter, body, ..
            } => {
                self.expr_bigint_info(iter)?;
                let mut body_guard = self.fork();
                body_guard.locals.remove(var);
                body_guard.object_string_values.remove(var);
                body_guard.object_bigint_props.remove(var);
                body_guard.nullish_locals.remove(var);
                body_guard.object_toprimitive_locals.remove(var);
                body_guard.visit_stmts(body)?;
                self.locals.remove(var);
                self.object_string_values.remove(var);
                self.object_bigint_props.remove(var);
                self.nullish_locals.remove(var);
                self.object_toprimitive_locals.remove(var);
                self.invalidate_assigned_in_stmts(body);
                Ok(())
            }
            Stmt::Labeled { body, .. } => self.visit_stmt(body),
            Stmt::ImportSideEffect { .. }
            | Stmt::ImportNamed { .. }
            | Stmt::ImportDefault { .. }
            | Stmt::ImportDefaultNamed { .. }
            | Stmt::ImportNamespace { .. }
            | Stmt::ImportDefaultNamespace { .. }
            | Stmt::ExportNamed { .. }
            | Stmt::ExportNamedFrom { .. }
            | Stmt::ExportAllFrom { .. }
            | Stmt::ExportNamespaceFrom { .. }
            | Stmt::ExportDecl { .. }
            | Stmt::ExportDefault { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. } => Ok(()),
        }
    }

    pub(super) fn fork(&self) -> Self {
        Self {
            locals: self.locals.clone(),
            string_locals: self.string_locals.clone(),
            string_values: self.string_values.clone(),
            object_string_values: self.object_string_values.clone(),
            object_bigint_props: self.object_bigint_props.clone(),
            nullish_locals: self.nullish_locals.clone(),
            object_toprimitive_locals: self.object_toprimitive_locals.clone(),
        }
    }

    pub(super) fn invalidate_assigned_in_stmts(&mut self, stmts: &[Stmt]) {
        for name in assigned_names_in_stmts(stmts) {
            self.locals.remove(&name);
            self.string_locals.remove(&name);
            self.string_values.remove(&name);
            self.object_string_values.remove(&name);
            self.object_bigint_props.remove(&name);
            self.nullish_locals.remove(&name);
            self.object_toprimitive_locals.remove(&name);
        }
    }

    pub(super) fn invalidate_assigned_in_expr(&mut self, expr: &Expr) {
        for name in assigned_names_in_expr(expr) {
            self.locals.remove(&name);
            self.string_locals.remove(&name);
            self.string_values.remove(&name);
            self.object_string_values.remove(&name);
            self.object_bigint_props.remove(&name);
            self.nullish_locals.remove(&name);
            self.object_toprimitive_locals.remove(&name);
        }
    }

    pub(super) fn expr_is_definitely_string(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String { .. } => true,
            Expr::Ident { name, .. } => self.string_locals.contains(name),
            Expr::Binary {
                left,
                op: BinaryOp::Add,
                right,
                ..
            } => self.expr_is_definitely_string(left) || self.expr_is_definitely_string(right),
            _ => false,
        }
    }

    pub(super) fn expr_static_string_value(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::String { value, .. } => Some(value.clone()),
            Expr::Ident { name, .. } => self.string_values.get(name).cloned(),
            Expr::Binary {
                left,
                op: BinaryOp::Add,
                right,
                ..
            } => {
                let mut value = self.expr_static_string_value(left)?;
                value.push_str(&self.expr_static_string_value(right)?);
                Some(value)
            }
            _ => None,
        }
    }

    pub(super) fn expr_static_object_string_values(
        &self,
        expr: &Expr,
    ) -> Option<HashMap<String, String>> {
        let Expr::Object { props, .. } = expr else {
            return None;
        };
        let values = props
            .iter()
            .filter_map(|(key, value)| {
                self.expr_static_string_value(value)
                    .map(|value| (key.clone(), value))
            })
            .collect::<HashMap<_, _>>();
        (!values.is_empty()).then_some(values)
    }

    pub(super) fn expr_static_object_bigint_props(&self, expr: &Expr) -> Option<HashSet<String>> {
        let Expr::Object { props, .. } = expr else {
            return None;
        };
        let values = props
            .iter()
            .filter_map(|(key, value)| {
                self.expr_is_tracked_bigint_value(value)
                    .then(|| key.clone())
            })
            .collect::<HashSet<_>>();
        (!values.is_empty()).then_some(values)
    }

    pub(super) fn expr_is_tracked_bigint_value(&self, expr: &Expr) -> bool {
        match expr {
            Expr::BigInt { .. } => true,
            Expr::Ident { name, .. } => self.locals.contains_key(name),
            Expr::Unary { op, expr, .. } => {
                *op == UnaryOp::Negate && self.expr_is_tracked_bigint_value(expr)
            }
            _ => false,
        }
    }

    pub(super) fn expr_static_object_member_string_value(
        &self,
        object: &Expr,
        property: &str,
    ) -> Option<String> {
        match object {
            Expr::Ident { name, .. } => self
                .object_string_values
                .get(name)
                .and_then(|props| props.get(property))
                .cloned(),
            Expr::Object { props, .. } => props
                .iter()
                .find(|(key, _)| key == property)
                .and_then(|(_, value)| self.expr_static_string_value(value)),
            _ => None,
        }
    }

    pub(super) fn expr_literal_derived_string_value(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Member {
                object, property, ..
            }
            | Expr::OptionalMember {
                object, property, ..
            } => self.expr_static_object_member_string_value(object, property),
            _ => self.expr_static_string_value(expr),
        }
    }

    pub(super) fn expr_is_object_carried_bigint(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Member {
                object, property, ..
            }
            | Expr::OptionalMember {
                object, property, ..
            } => match object.as_ref() {
                Expr::Ident { name, .. } => self
                    .object_bigint_props
                    .get(name)
                    .is_some_and(|props| props.contains(property)),
                Expr::Object { props, .. } => props.iter().any(|(key, value)| {
                    key == property && self.expr_is_tracked_bigint_value(value)
                }),
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn expr_is_definitely_nullish(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Null { .. } | Expr::Undefined { .. } => true,
            Expr::Ident { name, .. } => self.nullish_locals.contains(name),
            _ => false,
        }
    }

    pub(super) fn expr_is_object_toprimitive_boundary(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Ident { name, .. } => self.object_toprimitive_locals.contains(name),
            Expr::Object { props, .. } => props
                .iter()
                .any(|(key, _)| matches!(key.as_str(), "valueOf" | "toString")),
            _ => false,
        }
    }

    pub(super) fn expr_bigint_info(
        &mut self,
        expr: &Expr,
    ) -> Result<Option<BigIntStaticInfo>, Diagnostic> {
        match expr {
            Expr::BigInt { raw, span } => {
                let resolved = parse_bigint_literal(raw, *span)?;
                Ok(bigint_from_resolved(&resolved).map(BigIntStaticInfo::from_const))
            }
            Expr::Ident { name, .. } => Ok(self.locals.get(name).cloned().map(|mut info| {
                info.runtime_needed = true;
                info
            })),
            Expr::Unary { op, expr, span } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = info
                    && *op == UnaryOp::Negate
                {
                    let value = info.value.map(BigIntConst::negated);
                    let helper_safe = value
                        .as_ref()
                        .is_some_and(BigIntConst::fits_runtime_signed_i64);
                    if info.runtime_needed && !helper_safe {
                        return Err(bigint_dynamic_runtime_diagnostic(*span));
                    }
                    return Ok(Some(BigIntStaticInfo {
                        value,
                        helper_safe,
                        runtime_needed: info.runtime_needed,
                    }));
                }
                Ok(None)
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                let left_info = self.expr_bigint_info(left)?;
                let right_info = self.expr_bigint_info(right)?;
                if left_info.is_none() && right_info.is_none() {
                    if bigint_equality_or_comparison_op(*op) {
                        self.guard_object_carried_bigint_mixed_string(left, right, *span)?;
                    }
                    return Ok(None);
                }
                if !bigint_arithmetic_or_bitwise_op(*op) {
                    if bigint_equality_or_comparison_op(*op) {
                        let both_bigint = left_info.is_some() && right_info.is_some();
                        let strict_equality =
                            matches!(op, BinaryOp::StrictEqual | BinaryOp::StrictNotEqual);
                        let static_bigint_string_equality =
                            is_static_bigint_string_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        let static_bigint_boolean_equality =
                            is_static_bigint_boolean_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        let static_bigint_number_comparison = is_static_bigint_number_comparison(
                            left,
                            left_info.as_ref(),
                            *op,
                            right,
                            right_info.as_ref(),
                        );
                        let static_bigint_nullish_equality =
                            is_static_bigint_nullish_abstract_equality(
                                left,
                                left_info.as_ref(),
                                *op,
                                right,
                                right_info.as_ref(),
                            );
                        if both_bigint || strict_equality {
                            return Ok(None);
                        }
                        if static_bigint_string_equality
                            || static_bigint_boolean_equality
                            || static_bigint_number_comparison
                            || static_bigint_nullish_equality
                        {
                            return Ok(None);
                        }
                        if (left_info.is_some() && self.expr_is_object_toprimitive_boundary(right))
                            || (right_info.is_some()
                                && self.expr_is_object_toprimitive_boundary(left))
                        {
                            return Err(bigint_object_toprimitive_diagnostic(*span));
                        }
                        self.guard_bigint_mixed_runtime_string(left, left_info.as_ref(), *span)?;
                        self.guard_bigint_mixed_runtime_string(right, right_info.as_ref(), *span)?;
                        return Err(bigint_comparison_runtime_diagnostic(*span));
                    }
                    return Ok(None);
                }
                let (Some(left_info), Some(right_info)) = (left_info, right_info) else {
                    return Err(bigint_mixed_runtime_diagnostic(*span));
                };
                if !matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Subtract
                        | BinaryOp::Multiply
                        | BinaryOp::Divide
                        | BinaryOp::Modulo
                        | BinaryOp::Power
                ) {
                    return Ok(None);
                }
                let runtime_needed = left_info.runtime_needed || right_info.runtime_needed;
                let value = match (left_info.value, right_info.value) {
                    (Some(left), Some(right)) => {
                        if runtime_needed
                            && matches!(op, BinaryOp::Divide | BinaryOp::Modulo)
                            && right.sign == 0
                        {
                            return Ok(Some(BigIntStaticInfo {
                                value: None,
                                helper_safe: left_info.helper_safe && right_info.helper_safe,
                                runtime_needed,
                            }));
                        }
                        let result = fold_bigint_binary(left, *op, right, *span)?;
                        if runtime_needed && !result.fits_runtime_signed_i64() {
                            if *op == BinaryOp::Power {
                                return Err(bigint_exponentiation_diagnostic(*span));
                            }
                            return Err(bigint_dynamic_runtime_diagnostic(*span));
                        }
                        Some(result)
                    }
                    _ if runtime_needed && *op == BinaryOp::Power => {
                        return Err(bigint_exponentiation_diagnostic(*span));
                    }
                    _ if runtime_needed => return Err(bigint_dynamic_runtime_diagnostic(*span)),
                    _ => None,
                };
                Ok(Some(BigIntStaticInfo {
                    value,
                    helper_safe: left_info.helper_safe && right_info.helper_safe,
                    runtime_needed,
                }))
            }
            Expr::Call { callee, args, span } if matches!(callee.as_ref(), Expr::Ident { name, .. } if name == "BigInt") =>
            {
                let [arg] = args.as_slice() else {
                    return Err(bigint_builtin_unsupported_diagnostic(*span));
                };
                self.expr_bigint_info(arg)?;
                let static_supported_arg = match arg {
                    Expr::String { .. }
                    | Expr::Bool { .. }
                    | Expr::Number { .. }
                    | Expr::BigInt { .. } => true,
                    Expr::Unary {
                        op: UnaryOp::Negate,
                        expr,
                        ..
                    } => matches!(expr.as_ref(), Expr::Number { .. }),
                    _ => false,
                };
                if static_supported_arg {
                    return Ok(None);
                }
                if let Some(value) = self.expr_literal_derived_string_value(arg) {
                    let parsed = bigint_from_string_builtin(&value, *span)
                        .map_err(|_| bigint_dynamic_string_diagnostic(*span))?;
                    if !bigint_fits_runtime_from_string(&parsed) {
                        return Err(bigint_dynamic_string_diagnostic(*span));
                    }
                }
                if self.expr_is_definitely_nullish(arg) {
                    return Err(bigint_builtin_unsupported_diagnostic(*span));
                }
                Ok(Some(BigIntStaticInfo {
                    value: None,
                    helper_safe: true,
                    runtime_needed: true,
                }))
            }
            Expr::Call { callee, args, span }
                if is_bigint_static_builtin_callee(callee.as_ref()) =>
            {
                let [bits, value] = args.as_slice() else {
                    return Err(bigint_static_width_diagnostic(*span));
                };
                let static_bits = match bits {
                    Expr::Number { value, .. } if (0..=64).contains(value) => true,
                    Expr::Number { .. }
                    | Expr::String { .. }
                    | Expr::Bool { .. }
                    | Expr::Null { .. }
                    | Expr::Undefined { .. }
                    | Expr::BigInt { .. } => return Err(bigint_static_width_diagnostic(*span)),
                    _ => {
                        self.expr_bigint_info(bits)?;
                        false
                    }
                };
                let Some(value_info) = self.expr_bigint_info(value)? else {
                    return Err(bigint_as_value_diagnostic(*span));
                };
                let runtime_needed = !static_bits || value_info.runtime_needed;
                if runtime_needed && !value_info.helper_safe {
                    return Err(bigint_as_value_diagnostic(*span));
                }
                Ok(Some(BigIntStaticInfo {
                    value: None,
                    helper_safe: true,
                    runtime_needed: true,
                }))
            }
            Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
                self.expr_bigint_info(callee)?;
                for arg in args {
                    self.expr_bigint_info(arg)?;
                }
                Ok(None)
            }
            Expr::Member { object, .. }
            | Expr::OptionalMember { object, .. }
            | Expr::TypeOf { expr: object, .. }
            | Expr::Await { expr: object, .. }
            | Expr::Spread { expr: object, .. } => {
                self.expr_bigint_info(object)?;
                Ok(None)
            }
            Expr::Assign { name, expr, .. } | Expr::LogicalAssign { name, expr, .. } => {
                let info = self.expr_bigint_info(expr)?;
                if let Some(info) = &info {
                    self.locals.insert(name.clone(), info.clone());
                } else {
                    self.locals.remove(name);
                }
                if self.expr_is_definitely_string(expr) {
                    self.string_locals.insert(name.clone());
                    if let Some(value) = self.expr_static_string_value(expr) {
                        self.string_values.insert(name.clone(), value);
                    } else {
                        self.string_values.remove(name);
                    }
                } else {
                    self.string_locals.remove(name);
                    self.string_values.remove(name);
                }
                if let Some(props) = self.expr_static_object_string_values(expr) {
                    self.object_string_values.insert(name.clone(), props);
                } else {
                    self.object_string_values.remove(name);
                }
                if let Some(props) = self.expr_static_object_bigint_props(expr) {
                    self.object_bigint_props.insert(name.clone(), props);
                } else {
                    self.object_bigint_props.remove(name);
                }
                if self.expr_is_object_toprimitive_boundary(expr) {
                    self.object_toprimitive_locals.insert(name.clone());
                } else {
                    self.object_toprimitive_locals.remove(name);
                }
                Ok(info)
            }
            Expr::LogicalPropertyAssign {
                object,
                object_expr,
                computed_key,
                expr,
                ..
            } => {
                if let Some(object_expr) = object_expr {
                    self.expr_bigint_info(object_expr)?;
                }
                if let Some(computed_key) = computed_key {
                    self.expr_bigint_info(computed_key)?;
                }
                self.expr_bigint_info(expr)?;
                self.object_string_values.remove(object);
                self.object_bigint_props.remove(object);
                Ok(None)
            }
            Expr::Array { elements, .. } => {
                for element in elements {
                    self.expr_bigint_info(element)?;
                }
                Ok(None)
            }
            Expr::Object { props, .. } => {
                for (_, value) in props {
                    self.expr_bigint_info(value)?;
                }
                Ok(None)
            }
            Expr::Index { object, index, .. } | Expr::OptionalIndex { object, index, .. } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(index)?;
                Ok(None)
            }
            Expr::New { expr, args, .. } => {
                self.expr_bigint_info(expr)?;
                for arg in args {
                    self.expr_bigint_info(arg)?;
                }
                Ok(None)
            }
            Expr::InstanceOf {
                expr, type_expr, ..
            } => {
                self.expr_bigint_info(expr)?;
                self.expr_bigint_info(type_expr)?;
                Ok(None)
            }
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.expr_bigint_info(condition)?;
                self.fork().expr_bigint_info(then_expr)?;
                self.fork().expr_bigint_info(else_expr)?;
                Ok(None)
            }
            Expr::PropertyAssign { object, value, .. } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(value)?;
                self.invalidate_static_object_props_for_object(object);
                Ok(None)
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
                self.expr_bigint_info(object)?;
                self.expr_bigint_info(index)?;
                self.expr_bigint_info(value)?;
                self.invalidate_static_object_props_for_object(object);
                Ok(None)
            }
            Expr::ArrowFn { body, .. } => BigIntRuntimeGuard::default().expr_bigint_info(body),
            Expr::FunctionExpr { body, .. } => {
                BigIntRuntimeGuard::default().visit_stmts(body)?;
                Ok(None)
            }
            Expr::Number { .. }
            | Expr::String { .. }
            | Expr::Bool { .. }
            | Expr::Null { .. }
            | Expr::Undefined { .. }
            | Expr::This { .. } => Ok(None),
        }
    }

    pub(super) fn guard_bigint_mixed_runtime_string(
        &self,
        expr: &Expr,
        bigint_info: Option<&BigIntStaticInfo>,
        span: Span,
    ) -> Result<(), Diagnostic> {
        if bigint_info.is_some() || matches!(expr, Expr::String { .. }) {
            return Ok(());
        }
        let Some(value) = self.expr_literal_derived_string_value(expr) else {
            return Ok(());
        };
        let parsed = match bigint_from_string_builtin(&value, span) {
            Ok(parsed) => parsed,
            Err(_) => return Ok(()),
        };
        if !bigint_fits_runtime_mixed_string(&parsed) {
            return Err(bigint_comparison_string_boundary_diagnostic(span));
        }
        Ok(())
    }

    pub(super) fn guard_object_carried_bigint_mixed_string(
        &self,
        left: &Expr,
        right: &Expr,
        span: Span,
    ) -> Result<(), Diagnostic> {
        if self.expr_is_object_carried_bigint(left) {
            self.guard_bigint_mixed_runtime_string(right, None, span)?;
        }
        if self.expr_is_object_carried_bigint(right) {
            self.guard_bigint_mixed_runtime_string(left, None, span)?;
        }
        Ok(())
    }

    pub(super) fn invalidate_static_object_props_for_object(&mut self, object: &Expr) {
        if let Expr::Ident { name, .. } = object {
            self.object_string_values.remove(name);
            self.object_bigint_props.remove(name);
        }
    }
}

pub(super) fn bigint_fits_runtime_mixed_string(value: &BigIntConst) -> bool {
    match value.sign.cmp(&0) {
        std::cmp::Ordering::Less => decimal_digits_to_u64(&value.digits)
            .is_some_and(|magnitude| magnitude <= i32::MAX as u64 + 1),
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => decimal_digits_to_u64(&value.digits)
            .is_some_and(|magnitude| magnitude <= i32::MAX as u64),
    }
}

pub(super) fn bigint_comparison_string_boundary_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-282: dynamic BigInt/String comparison is limited to signed-i32 StringToBigInt values in this runtime coercion slice".to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_object_toprimitive_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-282: object ToPrimitive for mixed BigInt comparison is not implemented in this runtime coercion slice".to_owned(),
        span: Some(span),
    }
}
