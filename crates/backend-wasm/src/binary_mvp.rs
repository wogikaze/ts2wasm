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
    wasm_binary::{self, I32, WasmBinaryWriter},
};

/// Emit a direct WASM binary for the lowered program.
///
/// This uses the `WasmBinaryWriter` infrastructure to produce valid `.wasm`
/// bytes. Currently supports a limited subset of the full lowered program
/// (console.log-based programs without functions or modules).
pub(crate) fn emit_wasm_binary_mvp(program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    let stdout = hello_stdout(program)?;
    let link_plan = RuntimeLinkPlan::from_program(program);

    // Check that the runtime link plan includes console.log support
    if link_plan
        .required_runtime_functions()
        .contains(&RuntimeFn::Log)
        && link_plan.required_imports().contains(&HostImport::FdWrite)
    {
        Ok(encode_runtime_stdout_module(&stdout))
    } else {
        Err(invariant(
            "wasm binary MVP requires the runtime link plan to select console.log and WASI fd_write",
        ))
    }
}

/// Build a minimal .wasm module that writes precomputed stdout bytes
/// through the WASI `fd_write` import.
fn encode_runtime_stdout_module(stdout: &[u8]) -> Vec<u8> {
    let mut writer = WasmBinaryWriter::new();
    writer.begin_module();

    // Type 0: (i32, i32, i32, i32) -> i32 (fd_write signature)
    let fd_write_type = writer.get_or_create_type_index(&[I32, I32, I32, I32], &[I32]);
    // Type 1: () -> () (_start signature)
    let _start_type = writer.get_or_create_type_index(&[], &[]);

    // Import fd_write
    let spec = HostImport::FdWrite.spec();
    writer.register_import(spec.module, spec.name, fd_write_type);

    // Register _start with type _start_type (index 1)
    writer.register_function(_start_type);

    // Export memory (index 0) and _start (index 1 = first non-import function)
    writer.add_export("memory", wasm_binary::EXPORT_MEMORY, 0);
    writer.add_export("_start", wasm_binary::EXPORT_FUNC, 1);

    // Build _start function body
    let mut body = Vec::new();
    body.push(0); // zero locals

    // Set up iovec: store DATA_START addr at IOVEC_PTR
    WasmBinaryWriter::emit_i32_const(&mut body, Layout::IOVEC_PTR as i32);
    WasmBinaryWriter::emit_i32_const(&mut body, Layout::DATA_START as i32);
    WasmBinaryWriter::emit_i32_store(&mut body, 2, 0);

    // Set up iovec len: store stdout_len at IOVEC_LEN
    WasmBinaryWriter::emit_i32_const(&mut body, Layout::IOVEC_LEN as i32);
    WasmBinaryWriter::emit_i32_const(&mut body, stdout.len() as i32);
    WasmBinaryWriter::emit_i32_store(&mut body, 2, 0);

    // Call fd_write(1, iovec_ptr, 1, &nwritten)
    WasmBinaryWriter::emit_i32_const(&mut body, RuntimeConst::STDOUT_FD);
    WasmBinaryWriter::emit_i32_const(&mut body, Layout::IOVEC_PTR as i32);
    WasmBinaryWriter::emit_i32_const(&mut body, RuntimeConst::ONE);
    WasmBinaryWriter::emit_i32_const(&mut body, RuntimeConst::ZERO);
    WasmBinaryWriter::emit_call(&mut body, 0); // call fd_write (import index 0)
    WasmBinaryWriter::emit_drop(&mut body);

    writer.finish_function_body(&mut body);

    // Data segment at DATA_START
    writer.add_data_segment(Layout::DATA_START, stdout);

    writer.into_bytes()
}

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
            LoweredStmt::Let(id, expr, _) | LoweredStmt::Assign(id, expr, _) => {
                if let Ok(val) = resolve_const_i32(expr, &initializers) {
                    initializers.insert(*id, val);
                }
            }
            LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args,
                    ..
                },
                _,
            ) => {
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

fn resolve_expr_to_string(
    expr: &LoweredExpr,
    initializers: &HashMap<LocalId, i32>,
) -> Result<Vec<u8>, Diagnostic> {
    match expr {
        LoweredExpr::String(s, _) => Ok(s.as_bytes().to_vec()),
        LoweredExpr::Number(n, _) => Ok(n.to_string().as_bytes().to_vec()),
        LoweredExpr::Bool(b, _) => Ok(if *b {
            b"true".to_vec()
        } else {
            b"false".to_vec()
        }),
        LoweredExpr::Null(..) => Ok(b"null".to_vec()),
        LoweredExpr::Undefined(..) => Ok(b"undefined".to_vec()),
        LoweredExpr::Local(id, _) => {
            let n = initializers
                .get(id)
                .copied()
                .ok_or_else(|| unsupported("local variable without compile-time known value"))?;
            Ok(n.to_string().as_bytes().to_vec())
        }
        LoweredExpr::Binary {
            left, op, right, ..
        } => {
            let l = resolve_const_i32(left, initializers)?;
            let r = resolve_const_i32(right, initializers)?;
            let result = eval_binary_i32(*op, l, r)?;
            Ok(result.to_string().as_bytes().to_vec())
        }
        _ => Err(unsupported("unsupported expression type in console.log")),
    }
}

fn resolve_const_i32(
    expr: &LoweredExpr,
    initializers: &HashMap<LocalId, i32>,
) -> Result<i32, Diagnostic> {
    match expr {
        LoweredExpr::Number(n, _) => Ok(*n),
        LoweredExpr::Bool(b, _) => Ok(if *b { 1 } else { 0 }),
        LoweredExpr::Null(..) => Ok(0),
        LoweredExpr::Undefined(..) => Ok(0),
        LoweredExpr::Local(id, _) => initializers
            .get(id)
            .copied()
            .ok_or_else(|| unsupported("local variable without compile-time known value")),
        LoweredExpr::Binary {
            left, op, right, ..
        } => {
            let l = resolve_const_i32(left, initializers)?;
            let r = resolve_const_i32(right, initializers)?;
            eval_binary_i32(*op, l, r)
        }
        _ => Err(unsupported("unsupported expression for i32 evaluation")),
    }
}

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
        LoweredBinaryOp::BitwiseOr => l | r,
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

        phase: None,
    }
}

fn invariant(message: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::InvariantViolation,
        message: message.to_owned(),
        span: None,

        phase: None,
    }
}
