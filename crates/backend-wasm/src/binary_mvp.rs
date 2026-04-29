use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::{
    builtin::BuiltinId,
    lowered::{FunctionCallKind, LoweredExpr, LoweredProgram, LoweredStmt},
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

fn hello_stdout(program: &LoweredProgram) -> Result<Vec<u8>, Diagnostic> {
    if !program.top_level_locals.is_empty()
        || !program.functions.is_empty()
        || !program.modules.is_empty()
    {
        return Err(unsupported());
    }

    let [
        LoweredStmt::Expr(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
            args,
        }),
    ] = program.top_level_statements.as_slice()
    else {
        return Err(unsupported());
    };
    let [LoweredExpr::String(value)] = args.as_slice() else {
        return Err(unsupported());
    };

    let mut stdout = value.as_bytes().to_vec();
    stdout.push(b'\n');
    Ok(stdout)
}

fn unsupported() -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "direct wasm binary MVP only supports `console.log(<string literal>)`".to_owned(),
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
