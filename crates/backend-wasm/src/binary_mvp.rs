use std::collections::HashMap;

use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::{
    builtin::BuiltinId,
    lowered::{
        FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredProgram, LoweredStmt,
    },
};
use ts2wasm_runtime_abi::{Layout, RuntimeConst};

use super::{
    runtime_fn::{HostImport, RuntimeFn},
    runtime_link_plan::RuntimeLinkPlan,
};

const WASM_MAGIC_AND_VERSION: &[u8] = b"\0asm\x01\0\0\0";
const SECTION_TYPE: u8 = 1;
const SECTION_IMPORT: u8 = 2;
const SECTION_FUNCTION: u8 = 3;
const SECTION_MEMORY: u8 = 5;
const SECTION_EXPORT: u8 = 7;
const SECTION_CODE: u8 = 10;
const SECTION_DATA: u8 = 11;

const FUNC_REF: u8 = 0x60;
const I32: u8 = 0x7f;
const LIMITS_MIN_MAX: u8 = 0x01;
const IMPORT_FUNC: u8 = 0x00;
const EXPORT_FUNC: u8 = 0x00;
const EXPORT_MEMORY: u8 = 0x02;

const OP_END: u8 = 0x0b;
const OP_DROP: u8 = 0x1a;
const OP_CALL: u8 = 0x10;
const OP_I32_CONST: u8 = 0x41;
const OP_I32_STORE: u8 = 0x36;

pub(crate) fn emit_wasm_binary_mvp(program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    let stdout = hello_stdout(program)?;
    let link_plan = RuntimeLinkPlan::from_program(program);
    if !link_plan
        .required_runtime_functions()
        .contains(&RuntimeFn::Log)
        || !link_plan.required_imports().contains(&HostImport::FdWrite)
    {
        return Err(invariant(
            "wasm binary MVP requires the runtime link plan to select console.log and WASI fd_write",
        ));
    }

    Ok(encode_stdout_module(&stdout))
}

/// Resolve all console.log statements to a single stdout byte buffer.
///
/// Supports:
/// - String literals ("hi")
/// - Number literals (42, -1)
/// - Bool literals (true, false)
/// - Null / Undefined
/// - Local variable reads (via compile-time initializer tracing)
/// - Basic binary expressions (arithmetic and comparison)
/// - Multiple sequential console.log statements
/// - Const locals (`let x = 5; console.log(x)`)
fn hello_stdout(program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    if !program.functions.is_empty() || !program.modules.is_empty() {
        return Err(unsupported(
            "functions and modules are not supported in MVP",
        ));
    }

    let mut initializers: HashMap<LocalId, i32> = HashMap::new();
    let mut stdout: Vec<u8> = Vec::new();

    for stmt in &program.top_level_statements {
        match stmt {
            LoweredStmt::Let(id, expr) | LoweredStmt::Assign(id, expr) => {
                if let Ok(val) = resolve_const_i32(expr, &initializers) {
                    initializers.insert(*id, val);
                }
            }
            LoweredStmt::Expr(LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                args,
            }) => {
                let line = resolve_console_log_line(args, &initializers)?;
                stdout.extend_from_slice(&line);
                stdout.push(b'\n');
            }
            _ => {
                return Err(unsupported(
                    "only let/assign and console.log are supported in MVP",
                ));
            }
        }
    }

    if stdout.is_empty() {
        return Err(unsupported("no console.log calls found"));
    }
    Ok(stdout)
}

/// Resolve console.log arguments into a string byte buffer.
///
/// JavaScript `console.log(a, b)` prints "a b\n" with a space between arguments.
fn resolve_console_log_line(
    args: &[LoweredExpr],
    initializers: &HashMap<LocalId, i32>,
) -> Result<Vec<u8>, Diagnostic> {
    let mut line = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            line.push(b' ');
        }
        let part = resolve_expr_to_string(arg, initializers)?;
        line.extend_from_slice(&part);
    }
    Ok(line)
}

/// Resolve an expression to its string representation for console.log output.
fn resolve_expr_to_string(
    expr: &LoweredExpr,
    initializers: &HashMap<LocalId, i32>,
) -> Result<Vec<u8>, Diagnostic> {
    match expr {
        LoweredExpr::String(s) => Ok(s.as_bytes().to_vec()),
        LoweredExpr::Number(n) => Ok(n.to_string().as_bytes().to_vec()),
        LoweredExpr::Bool(b) => Ok(if *b {
            b"true".to_vec()
        } else {
            b"false".to_vec()
        }),
        LoweredExpr::Null => Ok(b"null".to_vec()),
        LoweredExpr::Undefined => Ok(b"undefined".to_vec()),
        LoweredExpr::Local(id) => {
            let n = initializers
                .get(id)
                .copied()
                .ok_or_else(|| unsupported("local variable without compile-time known value"))?;
            Ok(n.to_string().as_bytes().to_vec())
        }
        LoweredExpr::Binary { left, op, right } => {
            let l = resolve_const_i32(left, initializers)?;
            let r = resolve_const_i32(right, initializers)?;
            let result = eval_binary_i32(*op, l, r)?;
            Ok(result.to_string().as_bytes().to_vec())
        }
        _ => Err(unsupported("unsupported expression type in console.log")),
    }
}

/// Resolve an expression to a compile-time known i32 value.
fn resolve_const_i32(
    expr: &LoweredExpr,
    initializers: &HashMap<LocalId, i32>,
) -> Result<i32, Diagnostic> {
    match expr {
        LoweredExpr::Number(n) => Ok(*n),
        LoweredExpr::Bool(b) => Ok(if *b { 1 } else { 0 }),
        LoweredExpr::Null => Ok(0),
        LoweredExpr::Undefined => Ok(0),
        LoweredExpr::Local(id) => initializers
            .get(id)
            .copied()
            .ok_or_else(|| unsupported("local variable without compile-time known value")),
        LoweredExpr::Binary { left, op, right } => {
            let l = resolve_const_i32(left, initializers)?;
            let r = resolve_const_i32(right, initializers)?;
            eval_binary_i32(*op, l, r)
        }
        _ => Err(unsupported("unsupported expression for i32 evaluation")),
    }
}

/// Evaluate a binary operation on i32 values at compile time.
fn eval_binary_i32(op: LoweredBinaryOp, l: i32, r: i32) -> Result<i32, Diagnostic> {
    Ok(match op {
        LoweredBinaryOp::Add => l.wrapping_add(r),
        LoweredBinaryOp::Subtract => l.wrapping_sub(r),
        LoweredBinaryOp::Multiply => l.wrapping_mul(r),
        LoweredBinaryOp::Divide => l.checked_div(r).unwrap_or(0),
        LoweredBinaryOp::Modulo => l.checked_rem(r).unwrap_or(0),
        LoweredBinaryOp::Power => l.wrapping_pow(r as u32),
        LoweredBinaryOp::BitwiseAnd => l & r,
        LoweredBinaryOp::BitwiseXor => l ^ r,
        LoweredBinaryOp::And => {
            if l != 0 {
                r
            } else {
                0
            }
        }
        LoweredBinaryOp::Or => {
            if l != 0 {
                l
            } else {
                r
            }
        }
        LoweredBinaryOp::Less => (l < r) as i32,
        LoweredBinaryOp::LessEqual => (l <= r) as i32,
        LoweredBinaryOp::Greater => (l > r) as i32,
        LoweredBinaryOp::GreaterEqual => (l >= r) as i32,
        LoweredBinaryOp::EqualEqual | LoweredBinaryOp::StrictEqual => (l == r) as i32,
        LoweredBinaryOp::BangEqual | LoweredBinaryOp::StrictNotEqual => (l != r) as i32,
        LoweredBinaryOp::NullishCoalesce => {
            if l != 0 {
                l
            } else {
                r
            }
        }
    })
}

fn unsupported(detail: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: format!("direct wasm binary MVP: {detail}"),
        span: None,
    }
}

fn invariant(message: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::InvariantViolation,
        message: message.to_owned(),
        span: None,
    }
}

fn encode_stdout_module(stdout: &[u8]) -> Vec<u8> {
    let mut module = WASM_MAGIC_AND_VERSION.to_vec();
    append_type_section(&mut module);
    append_import_section(&mut module);
    append_function_section(&mut module);
    append_memory_section(&mut module);
    append_export_section(&mut module);
    append_code_section(&mut module, stdout.len() as u32);
    append_data_section(&mut module, stdout);
    module
}

fn append_type_section(module: &mut Vec<u8>) {
    let mut section = Vec::new();
    encode_u32(2, &mut section);

    section.push(FUNC_REF);
    encode_u32(4, &mut section);
    section.extend_from_slice(&[I32, I32, I32, I32]);
    encode_u32(1, &mut section);
    section.push(I32);

    section.push(FUNC_REF);
    encode_u32(0, &mut section);
    encode_u32(0, &mut section);

    append_section(module, SECTION_TYPE, &section);
}

fn append_import_section(module: &mut Vec<u8>) {
    let spec = HostImport::FdWrite.spec();
    let mut section = Vec::new();
    encode_u32(1, &mut section);
    append_name(&mut section, spec.module);
    append_name(&mut section, spec.name);
    section.push(IMPORT_FUNC);
    encode_u32(0, &mut section);
    append_section(module, SECTION_IMPORT, &section);
}

fn append_function_section(module: &mut Vec<u8>) {
    let mut section = Vec::new();
    encode_u32(1, &mut section);
    encode_u32(1, &mut section);
    append_section(module, SECTION_FUNCTION, &section);
}

fn append_memory_section(module: &mut Vec<u8>) {
    let mut section = Vec::new();
    encode_u32(1, &mut section);
    section.push(LIMITS_MIN_MAX);
    encode_u32(Layout::MEMORY_MIN_PAGES, &mut section);
    encode_u32(Layout::MEMORY_MAX_PAGES, &mut section);
    append_section(module, SECTION_MEMORY, &section);
}

fn append_export_section(module: &mut Vec<u8>) {
    let mut section = Vec::new();
    encode_u32(2, &mut section);

    append_name(&mut section, "memory");
    section.push(EXPORT_MEMORY);
    encode_u32(0, &mut section);

    append_name(&mut section, "_start");
    section.push(EXPORT_FUNC);
    encode_u32(1, &mut section);

    append_section(module, SECTION_EXPORT, &section);
}

fn append_code_section(module: &mut Vec<u8>, stdout_len: u32) {
    let mut body = Vec::new();
    encode_u32(0, &mut body);

    append_i32_const(&mut body, Layout::IOVEC_PTR as i32);
    append_i32_const(&mut body, Layout::DATA_START as i32);
    body.push(OP_I32_STORE);
    encode_u32(2, &mut body);
    encode_u32(0, &mut body);

    append_i32_const(&mut body, Layout::IOVEC_LEN as i32);
    append_i32_const(&mut body, stdout_len as i32);
    body.push(OP_I32_STORE);
    encode_u32(2, &mut body);
    encode_u32(0, &mut body);

    append_i32_const(&mut body, RuntimeConst::STDOUT_FD);
    append_i32_const(&mut body, Layout::IOVEC_PTR as i32);
    append_i32_const(&mut body, RuntimeConst::ONE);
    append_i32_const(&mut body, RuntimeConst::ZERO);
    body.push(OP_CALL);
    encode_u32(0, &mut body);
    body.push(OP_DROP);
    body.push(OP_END);

    let mut section = Vec::new();
    encode_u32(1, &mut section);
    encode_u32(body.len() as u32, &mut section);
    section.extend_from_slice(&body);
    append_section(module, SECTION_CODE, &section);
}

fn append_data_section(module: &mut Vec<u8>, stdout: &[u8]) {
    let mut section = Vec::new();
    encode_u32(1, &mut section);
    section.push(0);
    append_i32_const(&mut section, Layout::DATA_START as i32);
    section.push(OP_END);
    encode_u32(stdout.len() as u32, &mut section);
    section.extend_from_slice(stdout);
    append_section(module, SECTION_DATA, &section);
}

fn append_section(module: &mut Vec<u8>, id: u8, section: &[u8]) {
    module.push(id);
    encode_u32(section.len() as u32, module);
    module.extend_from_slice(section);
}

fn append_name(bytes: &mut Vec<u8>, value: &str) {
    encode_u32(value.len() as u32, bytes);
    bytes.extend_from_slice(value.as_bytes());
}

fn append_i32_const(bytes: &mut Vec<u8>, value: i32) {
    bytes.push(OP_I32_CONST);
    encode_i32(value, bytes);
}

fn encode_u32(mut value: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn encode_i32(mut value: i32, out: &mut Vec<u8>) {
    loop {
        let byte = (value as u8) & 0x7f;
        value >>= 7;
        let done = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
        out.push(if done { byte } else { byte | 0x80 });
        if done {
            break;
        }
    }
}
