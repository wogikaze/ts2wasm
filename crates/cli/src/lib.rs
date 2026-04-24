use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildInput {
    pub message: String,
}

pub fn build_file(input: &Path, output: &Path) -> Result<(), String> {
    let source = fs::read_to_string(input)
        .map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let parsed = parse_build_input(&source)?;
    let wasm = emit_console_log_wasm(&parsed.message);
    fs::write(output, wasm)
        .map_err(|error| format!("failed to write {}: {error}", output.display()))
}

pub fn parse_build_input(source: &str) -> Result<BuildInput, String> {
    let source = source.trim();
    let rest = source
        .strip_prefix("console.log")
        .ok_or_else(|| "M1 only supports console.log(\"literal\")".to_owned())?
        .trim_start();
    let rest = rest
        .strip_prefix('(')
        .ok_or_else(|| "expected '(' after console.log".to_owned())?
        .trim_start();
    let (message, rest) = parse_string_literal(rest)?;
    let rest = rest.trim_start();
    let rest = rest
        .strip_prefix(')')
        .ok_or_else(|| "expected ')' after console.log argument".to_owned())?
        .trim();
    let rest = rest.strip_suffix(';').unwrap_or(rest).trim();

    if !rest.is_empty() {
        return Err("M1 only supports a single console.log(\"literal\") statement".to_owned());
    }

    Ok(BuildInput { message })
}

pub fn emit_console_log_wasm(message: &str) -> Vec<u8> {
    let stdout = format!("{message}\n");
    let bytes = stdout.as_bytes();
    let iovec_offset = 8u32;
    let data_offset = 16u32;

    let mut module = Vec::new();
    module.extend_from_slice(b"\0asm");
    module.extend_from_slice(&[1, 0, 0, 0]);

    section(&mut module, 1, &type_section());
    section(&mut module, 2, &import_section());
    section(&mut module, 3, &function_section());
    section(&mut module, 5, &memory_section());
    section(&mut module, 7, &export_section());
    section(&mut module, 10, &code_section(iovec_offset));
    section(
        &mut module,
        11,
        &data_section(iovec_offset, data_offset, bytes),
    );

    module
}

fn parse_string_literal(input: &str) -> Result<(String, &str), String> {
    let mut chars = input.char_indices();
    let Some((_, quote @ ('"' | '\''))) = chars.next() else {
        return Err("expected string literal argument".to_owned());
    };

    let mut value = String::new();
    let mut escaped = false;

    for (index, ch) in chars {
        if escaped {
            let decoded = match ch {
                '"' => '"',
                '\'' => '\'',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => {
                    return Err(format!("unsupported escape sequence: \\{other}"));
                }
            };
            value.push(decoded);
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == quote {
            let rest = &input[index + ch.len_utf8()..];
            return Ok((value, rest));
        }

        value.push(ch);
    }

    Err("unterminated string literal".to_owned())
}

fn type_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    bytes.push(0x60);
    vec_len(&mut bytes, 4);
    bytes.extend_from_slice(&[0x7f, 0x7f, 0x7f, 0x7f]);
    vec_len(&mut bytes, 1);
    bytes.push(0x7f);
    bytes.push(0x60);
    vec_len(&mut bytes, 0);
    vec_len(&mut bytes, 0);
    bytes
}

fn import_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    name(&mut bytes, "wasi_snapshot_preview1");
    name(&mut bytes, "fd_write");
    bytes.push(0x00);
    u32_leb(&mut bytes, 0);
    bytes
}

fn function_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    u32_leb(&mut bytes, 1);
    bytes
}

fn memory_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    bytes.push(0x00);
    u32_leb(&mut bytes, 1);
    bytes
}

fn export_section() -> Vec<u8> {
    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    name(&mut bytes, "memory");
    bytes.push(0x02);
    u32_leb(&mut bytes, 0);
    name(&mut bytes, "_start");
    bytes.push(0x00);
    u32_leb(&mut bytes, 1);
    bytes
}

fn code_section(iovec_offset: u32) -> Vec<u8> {
    let mut body = Vec::new();
    vec_len(&mut body, 0);
    i32_const(&mut body, 1);
    i32_const(&mut body, iovec_offset);
    i32_const(&mut body, 1);
    i32_const(&mut body, 0);
    body.push(0x10);
    u32_leb(&mut body, 0);
    body.push(0x1a);
    body.push(0x0b);

    let mut bytes = Vec::new();
    vec_len(&mut bytes, 1);
    u32_leb(&mut bytes, body.len() as u32);
    bytes.extend(body);
    bytes
}

fn data_section(iovec_offset: u32, data_offset: u32, data: &[u8]) -> Vec<u8> {
    let mut iovec = Vec::new();
    iovec.extend_from_slice(&data_offset.to_le_bytes());
    iovec.extend_from_slice(&(data.len() as u32).to_le_bytes());

    let mut bytes = Vec::new();
    vec_len(&mut bytes, 2);
    data_segment(&mut bytes, iovec_offset, &iovec);
    data_segment(&mut bytes, data_offset, data);
    bytes
}

fn data_segment(bytes: &mut Vec<u8>, offset: u32, data: &[u8]) {
    bytes.push(0x00);
    i32_const(bytes, offset);
    bytes.push(0x0b);
    u32_leb(bytes, data.len() as u32);
    bytes.extend_from_slice(data);
}

fn section(module: &mut Vec<u8>, id: u8, payload: &[u8]) {
    module.push(id);
    u32_leb(module, payload.len() as u32);
    module.extend_from_slice(payload);
}

fn name(bytes: &mut Vec<u8>, value: &str) {
    u32_leb(bytes, value.len() as u32);
    bytes.extend_from_slice(value.as_bytes());
}

fn vec_len(bytes: &mut Vec<u8>, len: u32) {
    u32_leb(bytes, len);
}

fn i32_const(bytes: &mut Vec<u8>, value: u32) {
    bytes.push(0x41);
    u32_leb(bytes, value);
}

fn u32_leb(bytes: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_console_log_string() {
        let input = parse_build_input("console.log(\"hi\");").unwrap();
        assert_eq!(input.message, "hi");
    }

    #[test]
    fn rejects_non_m1_source() {
        let error = parse_build_input("let x = 1;").unwrap_err();
        assert!(error.contains("M1 only supports"));
    }

    #[test]
    fn emits_wasm_module_header() {
        let wasm = emit_console_log_wasm("hi");
        assert_eq!(&wasm[0..8], b"\0asm\x01\0\0\0");
    }
}
