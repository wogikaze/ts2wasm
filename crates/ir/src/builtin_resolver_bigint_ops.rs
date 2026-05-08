use super::builtin_resolver_bigint::*;
use super::*;

pub(super) fn bigint_arithmetic_or_bitwise_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo
            | BinaryOp::Power
            | BinaryOp::BitwiseAnd
            | BinaryOp::BitwiseOr
            | BinaryOp::BitwiseXor
            | BinaryOp::LeftShift
            | BinaryOp::RightShift
            | BinaryOp::UnsignedRightShift
    )
}

pub(super) fn bigint_dynamic_runtime_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-369: dynamic BigInt runtime arithmetic outside the signed-i64-backed first-limb slice is not implemented".to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_comparison_runtime_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-282: mixed BigInt abstract equality and relational comparison coercion is not implemented in this runtime coercion slice".to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_exponentiation_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message:
            "issue-376: BigInt exponentiation beyond literal non-negative exponent folding is not implemented"
                .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_bitwise_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-387: BigInt bitwise outside the signed-i64 helper slice is not implemented"
            .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_shift_diagnostic(span: Span) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-378: BigInt shift operators and unsigned right shift TypeError policy are not implemented"
            .to_owned(),
        span: Some(span),
    }
}

pub(super) fn bigint_equality_or_comparison_op(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
            | BinaryOp::StrictEqual
            | BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::StrictNotEqual
    )
}

pub(super) fn is_static_bigint_string_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(right, Expr::String { .. })
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(left, Expr::String { .. })
    });
    left_static_bigint || right_static_bigint
}

pub(super) fn is_static_bigint_boolean_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(
        op,
        BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
    ) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(right, Expr::Bool { .. })
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && matches!(left, Expr::Bool { .. })
    });
    left_static_bigint || right_static_bigint
}

pub(super) fn is_static_bigint_number_comparison(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(
        op,
        BinaryOp::EqualEqual
            | BinaryOp::BangEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
    ) {
        return false;
    }
    let left_static_bigint = left_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && static_number_bigint_const(right).is_some()
    });
    let right_static_bigint = right_info.is_some_and(|info| {
        !info.runtime_needed && info.value.is_some() && static_number_bigint_const(left).is_some()
    });
    left_static_bigint || right_static_bigint
}

pub(super) fn is_static_bigint_nullish_abstract_equality(
    left: &Expr,
    left_info: Option<&BigIntStaticInfo>,
    op: BinaryOp,
    right: &Expr,
    right_info: Option<&BigIntStaticInfo>,
) -> bool {
    if !matches!(op, BinaryOp::EqualEqual | BinaryOp::BangEqual) {
        return false;
    }
    let right_nullish = matches!(right, Expr::Null { .. } | Expr::Undefined { .. });
    let left_nullish = matches!(left, Expr::Null { .. } | Expr::Undefined { .. });
    let left_static_bigint =
        left_info.is_some_and(|info| !info.runtime_needed && info.value.is_some() && right_nullish);
    let right_static_bigint =
        right_info.is_some_and(|info| !info.runtime_needed && info.value.is_some() && left_nullish);
    left_static_bigint || right_static_bigint
}

pub(super) fn assigned_names_in_stmts(stmts: &[Stmt]) -> HashSet<String> {
    let mut names = HashSet::new();
    for stmt in stmts {
        collect_assigned_names_in_stmt(stmt, &mut names);
    }
    names
}

pub(super) fn assigned_names_in_expr(expr: &Expr) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_assigned_names_in_expr(expr, &mut names);
    names
}

pub(super) fn collect_assigned_names_in_stmt(stmt: &Stmt, names: &mut HashSet<String>) {
    match stmt {
        Stmt::Let { name, .. } | Stmt::Assign { name, .. } => {
            names.insert(name.clone());
        }
        Stmt::Expr { expr, .. } | Stmt::Return { expr, .. } | Stmt::Throw { expr, .. } => {
            collect_assigned_names_in_expr(expr, names);
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_stmts(then_body, names);
            collect_assigned_names_in_stmts(else_body, names);
        }
        Stmt::While {
            condition, body, ..
        }
        | Stmt::DoWhile {
            condition, body, ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            collect_assigned_names_in_stmts(try_block, names);
            if let Some(catch_block) = catch_block {
                collect_assigned_names_in_stmts(catch_block, names);
            }
            if let Some(finally_block) = finally_block {
                collect_assigned_names_in_stmts(finally_block, names);
            }
        }
        Stmt::Switch { expr, cases, .. } => {
            collect_assigned_names_in_expr(expr, names);
            for (case_expr, body) in cases {
                if let Some(case_expr) = case_expr {
                    collect_assigned_names_in_expr(case_expr, names);
                }
                collect_assigned_names_in_stmts(body, names);
            }
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            if let Some(init) = init {
                collect_assigned_names_in_stmt(init, names);
            }
            if let Some(condition) = condition {
                collect_assigned_names_in_expr(condition, names);
            }
            if let Some(update) = update {
                collect_assigned_names_in_expr(update, names);
            }
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::ForIn {
            var, iter, body, ..
        }
        | Stmt::ForOf {
            var, iter, body, ..
        } => {
            names.insert(var.clone());
            collect_assigned_names_in_expr(iter, names);
            collect_assigned_names_in_stmts(body, names);
        }
        Stmt::Labeled { body, .. } => collect_assigned_names_in_stmt(body, names),
        Stmt::Function { .. }
        | Stmt::ClassDecl { .. }
        | Stmt::AmbientValueDecl { .. }
        | Stmt::ImportSideEffect { .. }
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
        | Stmt::ExportAssignment { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. } => {}
        Stmt::Block { .. } => {}
        Stmt::EnumDecl { .. } => {}
    }
}

pub(super) fn collect_assigned_names_in_stmts(stmts: &[Stmt], names: &mut HashSet<String>) {
    for stmt in stmts {
        collect_assigned_names_in_stmt(stmt, names);
    }
}

pub(super) fn collect_assigned_names_in_expr(expr: &Expr, names: &mut HashSet<String>) {
    match expr {
        Expr::Assign { name, expr, .. } | Expr::LogicalAssign { name, expr, .. } => {
            names.insert(name.clone());
            collect_assigned_names_in_expr(expr, names);
        }
        Expr::Binary { left, right, .. }
        | Expr::Index {
            object: left,
            index: right,
            ..
        }
        | Expr::OptionalIndex {
            object: left,
            index: right,
            ..
        }
        | Expr::InstanceOf {
            expr: left,
            type_expr: right,
            ..
        } => {
            collect_assigned_names_in_expr(left, names);
            collect_assigned_names_in_expr(right, names);
        }
        Expr::Unary { expr, .. }
        | Expr::Member { object: expr, .. }
        | Expr::OptionalMember { object: expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => collect_assigned_names_in_expr(expr, names),
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            collect_assigned_names_in_expr(callee, names);
            for arg in args {
                collect_assigned_names_in_expr(arg, names);
            }
        }
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            if let Some(object_expr) = object_expr {
                collect_assigned_names_in_expr(object_expr, names);
            }
            if let Some(computed_key) = computed_key {
                collect_assigned_names_in_expr(computed_key, names);
            }
            collect_assigned_names_in_expr(expr, names);
        }
        Expr::Array { elements, .. } => {
            for element in elements {
                if let ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) =
                    element
                {
                    collect_assigned_names_in_expr(expr, names);
                }
            }
        }
        Expr::Object { props, .. } => {
            for (_, value) in props {
                collect_assigned_names_in_expr(value, names);
            }
        }
        Expr::ClassExpr { body, .. } => {
            for stmt in body {
                collect_assigned_names_in_stmt(stmt, names);
            }
        }
        Expr::New { expr, args, .. } => {
            collect_assigned_names_in_expr(expr, names);
            for arg in args {
                collect_assigned_names_in_expr(arg, names);
            }
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_assigned_names_in_expr(condition, names);
            collect_assigned_names_in_expr(then_expr, names);
            collect_assigned_names_in_expr(else_expr, names);
        }
        Expr::PropertyAssign { object, value, .. } => {
            collect_assigned_names_in_expr(object, names);
            collect_assigned_names_in_expr(value, names);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            collect_assigned_names_in_expr(object, names);
            collect_assigned_names_in_expr(index, names);
            collect_assigned_names_in_expr(value, names);
        }
        Expr::ArrowFn { .. }
        | Expr::FunctionExpr { .. }
        | Expr::Number { .. }
        | Expr::BigInt { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::Undefined { .. }
        | Expr::This { .. }
        | Expr::Ident { .. } => {}
    }
}

pub(super) fn fold_bigint_binary(
    left: BigIntConst,
    op: BinaryOp,
    right: BigIntConst,
    span: Span,
) -> Result<BigIntConst, Diagnostic> {
    match op {
        BinaryOp::Add => Ok(bigint_add(left, right)),
        BinaryOp::Subtract => Ok(bigint_add(left, right.negated())),
        BinaryOp::Multiply => Ok(bigint_mul(left, right)),
        BinaryOp::Power if right.sign < 0 => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-370: BigInt negative exponent RangeError parity is not implemented in this literal-folding slice"
                    .to_owned(),
            span: Some(span),
        }),
        BinaryOp::Power => {
            let Some(exponent) = decimal_digits_to_u64(&right.digits) else {
                return Err(bigint_exponentiation_diagnostic(span));
            };
            if exponent > 64 {
                return Err(bigint_exponentiation_diagnostic(span));
            }
            Ok(bigint_pow(left, exponent))
        }
        BinaryOp::Divide | BinaryOp::Modulo if right.sign == 0 => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message:
                "issue-370: BigInt division by zero RangeError parity is not implemented in this literal-folding slice"
                    .to_owned(),
            span: Some(span),
        }),
        BinaryOp::Divide => {
            let (quotient, _) = div_rem_abs(&left.digits, &right.digits);
            let sign = if quotient == [0] {
                0
            } else {
                left.sign * right.sign
            };
            Ok(BigIntConst {
                sign,
                digits: quotient,
            })
        }
        BinaryOp::Modulo => {
            let (_, remainder) = div_rem_abs(&left.digits, &right.digits);
            let sign = if remainder == [0] { 0 } else { left.sign };
            Ok(BigIntConst {
                sign,
                digits: remainder,
            })
        }
        BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr | BinaryOp::BitwiseXor => {
            Ok(fold_bigint_binary_bitwise(left, op, right))
        }
        BinaryOp::LeftShift | BinaryOp::RightShift => fold_bigint_binary_shift(left, op, right, span),
        BinaryOp::UnsignedRightShift => Err(bigint_shift_diagnostic(span)),
        _ => unreachable!("non-arithmetic BigInt operator reached literal fold"),
    }
}

pub(super) fn fold_bigint_unary_bitwise_not(
    value: BigIntConst,
    _span: Span,
) -> Result<BigIntConst, Diagnostic> {
    Ok(bigint_add(
        value.negated(),
        BigIntConst::from_decimal(-1, "1"),
    ))
}

fn fold_bigint_binary_bitwise(left: BigIntConst, op: BinaryOp, right: BigIntConst) -> BigIntConst {
    let width = bigint_bit_width(&left).max(bigint_bit_width(&right)) + 2;
    let left_bits = bigint_to_twos_complement_bits(&left, width);
    let right_bits = bigint_to_twos_complement_bits(&right, width);
    let result_bits = left_bits
        .iter()
        .zip(right_bits.iter())
        .map(|(left, right)| match op {
            BinaryOp::BitwiseAnd => *left & *right,
            BinaryOp::BitwiseOr => *left | *right,
            BinaryOp::BitwiseXor => *left ^ *right,
            _ => unreachable!("checked by caller"),
        })
        .collect::<Vec<_>>();
    bigint_from_twos_complement_bits(&result_bits)
}

const MAX_LITERAL_BIGINT_SHIFT_BITS: u32 = 4096;

fn fold_bigint_binary_shift(
    left: BigIntConst,
    op: BinaryOp,
    right: BigIntConst,
    span: Span,
) -> Result<BigIntConst, Diagnostic> {
    let bits = bigint_shift_amount_bits(&right, span)?;
    let effective_op = if right.sign < 0 {
        match op {
            BinaryOp::LeftShift => BinaryOp::RightShift,
            BinaryOp::RightShift => BinaryOp::LeftShift,
            _ => unreachable!("checked by caller"),
        }
    } else {
        op
    };
    Ok(match effective_op {
        BinaryOp::LeftShift => bigint_shift_left(left, bits),
        BinaryOp::RightShift => bigint_shift_right(left, bits),
        _ => unreachable!("checked by caller"),
    })
}

fn bigint_shift_amount_bits(value: &BigIntConst, span: Span) -> Result<u32, Diagnostic> {
    let Some(bits) = decimal_digits_to_u64(&value.digits) else {
        return Err(bigint_shift_diagnostic(span));
    };
    if bits > u64::from(MAX_LITERAL_BIGINT_SHIFT_BITS) {
        return Err(bigint_shift_diagnostic(span));
    }
    Ok(bits as u32)
}

fn bigint_shift_left(value: BigIntConst, bits: u32) -> BigIntConst {
    if value.sign == 0 || bits == 0 {
        return value;
    }
    BigIntConst {
        sign: value.sign,
        digits: mul_abs(&value.digits, &decimal_power_of_two(bits)),
    }
}

fn bigint_shift_right(value: BigIntConst, bits: u32) -> BigIntConst {
    if value.sign == 0 || bits == 0 {
        return value;
    }
    let divisor = decimal_power_of_two(bits);
    let (mut quotient, remainder) = div_rem_abs(&value.digits, &divisor);
    if value.sign > 0 {
        return BigIntConst {
            sign: if quotient == [0] { 0 } else { 1 },
            digits: quotient,
        };
    }
    if remainder != [0] {
        quotient = add_abs(&quotient, &[1]);
    }
    BigIntConst {
        sign: if quotient == [0] { 0 } else { -1 },
        digits: quotient,
    }
}

fn bigint_bit_width(value: &BigIntConst) -> usize {
    let bits = decimal_digits_to_binary_bits(&value.digits);
    bits.iter()
        .rposition(|bit| *bit)
        .map_or(0, |index| index + 1)
}

fn bigint_to_twos_complement_bits(value: &BigIntConst, width: usize) -> Vec<bool> {
    let mut bits = vec![false; width];
    let magnitude_bits = decimal_digits_to_binary_bits(&value.digits);
    for (index, bit) in magnitude_bits.into_iter().enumerate().take(width) {
        bits[index] = bit;
    }
    if value.sign < 0 {
        for bit in &mut bits {
            *bit = !*bit;
        }
        add_one_to_bits(&mut bits);
    }
    bits
}

fn bigint_from_twos_complement_bits(bits: &[bool]) -> BigIntConst {
    if bits.last().copied().unwrap_or(false) {
        let mut magnitude_bits = bits.iter().map(|bit| !*bit).collect::<Vec<_>>();
        add_one_to_bits(&mut magnitude_bits);
        let digits = binary_bits_to_decimal_digits(&magnitude_bits);
        let sign = if digits == [0] { 0 } else { -1 };
        BigIntConst { sign, digits }
    } else {
        let digits = binary_bits_to_decimal_digits(bits);
        let sign = if digits == [0] { 0 } else { 1 };
        BigIntConst { sign, digits }
    }
}

fn add_one_to_bits(bits: &mut [bool]) {
    for bit in bits {
        if *bit {
            *bit = false;
        } else {
            *bit = true;
            return;
        }
    }
}

fn decimal_digits_to_binary_bits(digits: &[u8]) -> Vec<bool> {
    let mut value = digits.to_vec();
    trim_decimal_zeroes(&mut value);
    if value == [0] {
        return vec![false];
    }
    let mut bits = Vec::new();
    while value != [0] {
        let (quotient, remainder) = div_rem_abs(&value, &[2]);
        bits.push(remainder != [0]);
        value = quotient;
    }
    bits
}

fn binary_bits_to_decimal_digits(bits: &[bool]) -> Vec<u8> {
    let mut digits = vec![0_u8];
    for bit in bits.iter().rev() {
        digits = mul_abs(&digits, &[2]);
        if *bit {
            digits = add_abs(&digits, &[1]);
        }
    }
    trim_decimal_zeroes(&mut digits);
    digits
}

pub(super) fn bigint_add(left: BigIntConst, right: BigIntConst) -> BigIntConst {
    if left.sign == 0 {
        return right;
    }
    if right.sign == 0 {
        return left;
    }
    if left.sign == right.sign {
        return BigIntConst {
            sign: left.sign,
            digits: add_abs(&left.digits, &right.digits),
        };
    }
    match cmp_abs(&left.digits, &right.digits) {
        std::cmp::Ordering::Greater => BigIntConst {
            sign: left.sign,
            digits: sub_abs(&left.digits, &right.digits),
        },
        std::cmp::Ordering::Less => BigIntConst {
            sign: right.sign,
            digits: sub_abs(&right.digits, &left.digits),
        },
        std::cmp::Ordering::Equal => BigIntConst::zero(),
    }
}

pub(super) fn bigint_mul(left: BigIntConst, right: BigIntConst) -> BigIntConst {
    if left.sign == 0 || right.sign == 0 {
        return BigIntConst::zero();
    }
    BigIntConst {
        sign: left.sign * right.sign,
        digits: mul_abs(&left.digits, &right.digits),
    }
}

pub(super) fn bigint_pow(base: BigIntConst, exponent: u64) -> BigIntConst {
    let mut result = BigIntConst::from_decimal(1, "1");
    for _ in 0..exponent {
        result = bigint_mul(result, base.clone());
    }
    result
}

pub(super) fn cmp_abs(left: &[u8], right: &[u8]) -> std::cmp::Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

pub(super) fn add_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut carry = 0_u8;
    let mut li = left.len();
    let mut ri = right.len();
    while li > 0 || ri > 0 || carry > 0 {
        let ld = if li > 0 {
            li -= 1;
            left[li]
        } else {
            0
        };
        let rd = if ri > 0 {
            ri -= 1;
            right[ri]
        } else {
            0
        };
        let sum = ld + rd + carry;
        out.push(sum % 10);
        carry = sum / 10;
    }
    out.reverse();
    out
}

pub(super) fn sub_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut borrow = 0_i8;
    let mut li = left.len();
    let mut ri = right.len();
    while li > 0 {
        li -= 1;
        let mut ld = left[li] as i8 - borrow;
        let rd = if ri > 0 {
            ri -= 1;
            right[ri] as i8
        } else {
            0
        };
        if ld < rd {
            ld += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push((ld - rd) as u8);
    }
    out.reverse();
    trim_decimal_zeroes(&mut out);
    out
}

pub(super) fn mul_abs(left: &[u8], right: &[u8]) -> Vec<u8> {
    if left == [0] || right == [0] {
        return vec![0];
    }
    let mut out = vec![0_u16; left.len() + right.len()];
    for (li, ld) in left.iter().rev().enumerate() {
        for (ri, rd) in right.iter().rev().enumerate() {
            let idx = out.len() - 1 - li - ri;
            out[idx] += u16::from(*ld) * u16::from(*rd);
        }
    }
    for idx in (1..out.len()).rev() {
        let carry = out[idx] / 10;
        out[idx] %= 10;
        out[idx - 1] += carry;
    }
    let mut digits = out.into_iter().map(|digit| digit as u8).collect::<Vec<_>>();
    trim_decimal_zeroes(&mut digits);
    digits
}

pub(super) fn div_rem_abs(left: &[u8], right: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut quotient = Vec::with_capacity(left.len());
    let mut remainder = vec![0_u8];
    for digit in left {
        if remainder == [0] {
            remainder[0] = *digit;
        } else {
            remainder.push(*digit);
        }
        trim_decimal_zeroes(&mut remainder);
        let mut q = 0_u8;
        while cmp_abs(&remainder, right) != std::cmp::Ordering::Less {
            remainder = sub_abs(&remainder, right);
            q += 1;
        }
        quotient.push(q);
    }
    trim_decimal_zeroes(&mut quotient);
    trim_decimal_zeroes(&mut remainder);
    (quotient, remainder)
}

pub(super) fn bigint_unary_op_issue(op: UnaryOp) -> Option<&'static str> {
    match op {
        UnaryOp::BitwiseNot => {
            Some("issue-387: BigInt bitwise outside the signed-i64 helper slice is not implemented")
        }
        UnaryOp::Negate
        | UnaryOp::Plus
        | UnaryOp::Increment
        | UnaryOp::Decrement
        | UnaryOp::PreIncrement
        | UnaryOp::PreDecrement => Some(
            "issue-369: dynamic BigInt runtime arithmetic outside the signed-i64-backed first-limb slice is not implemented",
        ),
        UnaryOp::Not | UnaryOp::TypeOf | UnaryOp::Delete | UnaryOp::Void => None,
    }
}

pub(super) fn expr_contains_bigint(expr: &Expr) -> bool {
    match expr {
        Expr::BigInt { .. } => true,
        Expr::Unary { expr, .. }
        | Expr::TypeOf { expr, .. }
        | Expr::Await { expr, .. }
        | Expr::Spread { expr, .. } => expr_contains_bigint(expr),
        Expr::Binary { left, right, .. }
        | Expr::InstanceOf {
            expr: left,
            type_expr: right,
            ..
        }
        | Expr::Index {
            object: left,
            index: right,
            ..
        } => expr_contains_bigint(left) || expr_contains_bigint(right),
        Expr::Call { callee, args, .. } | Expr::OptionalCall { callee, args, .. } => {
            expr_contains_bigint(callee) || args.iter().any(expr_contains_bigint)
        }
        Expr::Member { object, .. } | Expr::OptionalMember { object, .. } => {
            expr_contains_bigint(object)
        }
        Expr::OptionalIndex { object, index, .. } => {
            expr_contains_bigint(object) || expr_contains_bigint(index)
        }
        Expr::Assign { expr, .. } | Expr::LogicalAssign { expr, .. } => expr_contains_bigint(expr),
        Expr::LogicalPropertyAssign {
            object_expr,
            computed_key,
            expr,
            ..
        } => {
            object_expr.as_deref().is_some_and(expr_contains_bigint)
                || computed_key.as_deref().is_some_and(expr_contains_bigint)
                || expr_contains_bigint(expr)
        }
        Expr::Array { elements, .. } => elements.iter().any(|element| match element {
            ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                expr_contains_bigint(expr)
            }
            ArrayLiteralElement::Hole(_) => false,
        }),
        Expr::Object { props, .. } => props.iter().any(|(_, value)| expr_contains_bigint(value)),
        Expr::New { args, .. } => args.iter().any(expr_contains_bigint),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_bigint(condition)
                || expr_contains_bigint(then_expr)
                || expr_contains_bigint(else_expr)
        }
        Expr::ArrowFn { body, .. } => expr_contains_bigint(body),
        Expr::FunctionExpr { .. } | Expr::ClassExpr { .. } => false,
        Expr::PropertyAssign { object, value, .. } => {
            expr_contains_bigint(object) || expr_contains_bigint(value)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            expr_contains_bigint(object)
                || expr_contains_bigint(index)
                || expr_contains_bigint(value)
        }
        Expr::Number { .. }
        | Expr::String { .. }
        | Expr::Bool { .. }
        | Expr::Null { .. }
        | Expr::This { .. }
        | Expr::Undefined { .. }
        | Expr::Ident { .. } => false,
    }
}
