use std::collections::HashMap;

// ── WASM binary constants ──────────────────────────────────────────────

const WASM_MAGIC: &[u8] = b"\0asm\x01\0\0\0";

// Section IDs
pub(crate) const SECTION_TYPE: u8 = 1;
pub(crate) const SECTION_IMPORT: u8 = 2;
pub(crate) const SECTION_FUNCTION: u8 = 3;
pub(crate) const SECTION_MEMORY: u8 = 5;
pub(crate) const SECTION_EXPORT: u8 = 7;
pub(crate) const SECTION_CODE: u8 = 10;
pub(crate) const SECTION_DATA: u8 = 11;

// Type opcode
const FUNC_REF: u8 = 0x60;

// Value types
pub(crate) const I32: u8 = 0x7f;
#[allow(dead_code)]
pub(crate) const I64: u8 = 0x7e;
pub(crate) const EMPTY_BLOCK_TYPE: u8 = 0x40;

// Memory limits
const LIMITS_MIN_MAX: u8 = 0x01;

// Import/Export kinds
pub(crate) const IMPORT_FUNC: u8 = 0x00;
pub(crate) const EXPORT_FUNC: u8 = 0x00;
pub(crate) const EXPORT_MEMORY: u8 = 0x02;
#[allow(dead_code)]
const EXPORT_TABLE: u8 = 0x01;
#[allow(dead_code)]
const EXPORT_GLOBAL: u8 = 0x03;

// ── WASM opcodes ──────────────────────────────────────────────────────

pub(crate) const OP_UNREACHABLE: u8 = 0x00;
pub(crate) const OP_NOP: u8 = 0x01;
pub(crate) const OP_BLOCK: u8 = 0x02;
pub(crate) const OP_LOOP: u8 = 0x03;
pub(crate) const OP_IF: u8 = 0x04;
pub(crate) const OP_ELSE: u8 = 0x05;
pub(crate) const OP_END: u8 = 0x0b;
pub(crate) const OP_BR: u8 = 0x0c;
pub(crate) const OP_BR_IF: u8 = 0x0d;
pub(crate) const OP_RETURN: u8 = 0x0f;
pub(crate) const OP_CALL: u8 = 0x10;
pub(crate) const OP_DROP: u8 = 0x1a;
pub(crate) const OP_LOCAL_GET: u8 = 0x20;
pub(crate) const OP_LOCAL_SET: u8 = 0x21;
pub(crate) const OP_LOCAL_TEE: u8 = 0x22;
pub(crate) const OP_GLOBAL_GET: u8 = 0x23;
pub(crate) const OP_GLOBAL_SET: u8 = 0x24;
pub(crate) const OP_I32_LOAD: u8 = 0x28;
pub(crate) const OP_I32_STORE: u8 = 0x36;
pub(crate) const OP_MEMORY_SIZE: u8 = 0x3f;
pub(crate) const OP_MEMORY_GROW: u8 = 0x40;
pub(crate) const OP_I32_CONST: u8 = 0x41;
pub(crate) const OP_I64_CONST: u8 = 0x42;
pub(crate) const OP_I32_EQZ: u8 = 0x45;
pub(crate) const OP_I32_EQ: u8 = 0x46;
pub(crate) const OP_I32_NE: u8 = 0x47;
pub(crate) const OP_I32_LT_S: u8 = 0x48;
pub(crate) const OP_I32_LT_U: u8 = 0x49;
pub(crate) const OP_I32_GT_S: u8 = 0x4a;
pub(crate) const OP_I32_GT_U: u8 = 0x4b;
pub(crate) const OP_I32_LE_S: u8 = 0x4c;
pub(crate) const OP_I32_GE_S: u8 = 0x4e;
pub(crate) const OP_I32_ADD: u8 = 0x6a;
pub(crate) const OP_I32_SUB: u8 = 0x6b;
pub(crate) const OP_I32_MUL: u8 = 0x6c;
pub(crate) const OP_I32_DIV_S: u8 = 0x6d;
pub(crate) const OP_I32_REM_S: u8 = 0x6f;
pub(crate) const OP_I32_AND: u8 = 0x71;
pub(crate) const OP_I32_OR: u8 = 0x72;
pub(crate) const OP_I32_XOR: u8 = 0x73;
pub(crate) const OP_I32_SHL: u8 = 0x74;
pub(crate) const OP_I32_SHR_S: u8 = 0x75;
pub(crate) const OP_I32_SHR_U: u8 = 0x76;
pub(crate) const OP_I32_CLZ: u8 = 0x67;
pub(crate) const OP_I32_CTZ: u8 = 0x68;
pub(crate) const OP_I32_POPCNT: u8 = 0x69;
pub(crate) const OP_I32_WRAP_I64: u8 = 0xa7;
pub(crate) const OP_SELECT: u8 = 0x1b;

// ── Import entry ──────────────────────────────────────────────────────

/// Describes a single import entry for batch emission.
#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub module: String,
    pub name: String,
    pub type_index: u32,
}

impl ImportEntry {
    pub fn new(module: &str, name: &str, type_index: u32) -> Self {
        Self {
            module: module.to_owned(),
            name: name.to_owned(),
            type_index,
        }
    }
}

/// Accumulate section content that is emitted at finalize time.
#[derive(Default)]
pub struct SectionAccum {
    pub bytes: Vec<u8>,
}

impl SectionAccum {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }
}

// ── Writer ──────────────────────────────────────────────────────────────

/// A WASM binary module writer.
///
/// Collects types, imports, function type assignments, and code/data
/// segments, then emits a complete `.wasm` module on `into_bytes()`.
pub struct WasmBinaryWriter {
    bytes: Vec<u8>,
    // Type section data
    types: Vec<TypeSig>,
    type_indices: HashMap<TypeSig, u32>,
    // Import entries (collected, emitted at finalize)
    imports: Vec<ImportEntry>,
    // Per-function type-index assignments (same length as function count)
    func_type_indices: Vec<u32>,
    // Accumulated code bodies (raw instructions, local decls already included)
    code_bodies: Vec<Vec<u8>>,
    // Data segments (offset, bytes)
    data_segments: Vec<(u32, Vec<u8>)>,
    // Export entries (name, kind, index)
    exports: Vec<(String, u8, u32)>,
}

impl Default for WasmBinaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// A complete function-signature key for type-index deduplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TypeSig {
    params: Vec<u8>,
    results: Vec<u8>,
}

impl WasmBinaryWriter {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            types: Vec::new(),
            type_indices: HashMap::new(),
            imports: Vec::new(),
            func_type_indices: Vec::new(),
            code_bodies: Vec::new(),
            data_segments: Vec::new(),
            exports: Vec::new(),
        }
    }

    /// Write the WASM magic number and version header.
    pub fn begin_module(&mut self) {
        self.bytes.extend_from_slice(WASM_MAGIC);
    }

    /// Finalize and return the complete WASM binary.
    /// Emits all sections in the correct WASM order:
    /// 1=Type, 2=Import, 3=Function, 5=Memory, 7=Export, 10=Code, 11=Data
    pub fn into_bytes(mut self) -> Vec<u8> {
        self.emit_type_section();
        self.emit_import_section();
        self.emit_function_section();
        self.emit_memory_section();
        self.emit_export_section();
        self.emit_code_section();
        self.emit_data_section();
        self.bytes
    }

    // ── Type helpers ────────────────────────────────────────────────

    /// Get or create a type index for the given parameter/result types.
    pub fn get_or_create_type_index(&mut self, params: &[u8], results: &[u8]) -> u32 {
        let sig = TypeSig {
            params: params.to_vec(),
            results: results.to_vec(),
        };
        if let Some(&idx) = self.type_indices.get(&sig) {
            return idx;
        }
        let idx = self.types.len() as u32;
        self.type_indices.insert(sig.clone(), idx);
        self.types.push(sig);
        idx
    }

    // ── Type section ────────────────────────────────────────────────

    fn emit_type_section(&mut self) {
        let mut content = Vec::new();
        encode_u32(self.types.len() as u32, &mut content);
        for sig in &self.types {
            content.push(FUNC_REF);
            encode_u32(sig.params.len() as u32, &mut content);
            content.extend_from_slice(&sig.params);
            encode_u32(sig.results.len() as u32, &mut content);
            content.extend_from_slice(&sig.results);
        }
        self.emit_section(SECTION_TYPE, &content);
    }

    // ── Import section ──────────────────────────────────────────────

    /// Register an imported function. Imports are emitted in `into_bytes()`.
    pub fn register_import(&mut self, module: &str, name: &str, type_idx: u32) {
        self.imports.push(ImportEntry::new(module, name, type_idx));
    }

    fn emit_import_section(&mut self) {
        if self.imports.is_empty() {
            return;
        }
        let mut content = Vec::new();
        encode_u32(self.imports.len() as u32, &mut content);
        for import in &self.imports {
            encode_name(&mut content, &import.module);
            encode_name(&mut content, &import.name);
            content.push(IMPORT_FUNC);
            encode_u32(import.type_index, &mut content);
        }
        self.emit_section(SECTION_IMPORT, &content);
    }

    // ── Function section ────────────────────────────────────────────

    /// Register a function with its type index.
    pub fn register_function(&mut self, type_index: u32) {
        self.func_type_indices.push(type_index);
    }

    /// Register a function body. The body should already include local
    /// declarations. Returns the function index.
    pub fn register_function_body(&mut self, body: Vec<u8>) -> u32 {
        let idx = self.code_bodies.len() as u32;
        self.code_bodies.push(body);
        idx
    }

    fn emit_function_section(&mut self) {
        if self.func_type_indices.is_empty() {
            return;
        }
        let mut content = Vec::new();
        encode_u32(self.func_type_indices.len() as u32, &mut content);
        for &type_idx in &self.func_type_indices {
            encode_u32(type_idx, &mut content);
        }
        self.emit_section(SECTION_FUNCTION, &content);
    }

    // ── Memory section ──────────────────────────────────────────────

    fn emit_memory_section(&mut self) {
        let mut content = Vec::new();
        encode_u32(1, &mut content); // 1 memory
        content.push(LIMITS_MIN_MAX);
        encode_u32(ts2wasm_runtime_abi::Layout::MEMORY_MIN_PAGES, &mut content);
        encode_u32(ts2wasm_runtime_abi::Layout::MEMORY_MAX_PAGES, &mut content);
        self.emit_section(SECTION_MEMORY, &content);
    }

    // ── Export section ──────────────────────────────────────────────

    /// Add an export entry.
    pub fn add_export(&mut self, name: &str, kind: u8, index: u32) {
        self.exports.push((name.to_owned(), kind, index));
    }

    fn emit_export_section(&mut self) {
        if self.exports.is_empty() {
            return;
        }
        let mut content = Vec::new();
        encode_u32(self.exports.len() as u32, &mut content);
        for (name, kind, index) in &self.exports {
            encode_name(&mut content, name);
            content.push(*kind);
            encode_u32(*index, &mut content);
        }
        self.emit_section(SECTION_EXPORT, &content);
    }

    // ── Code section ────────────────────────────────────────────────

    fn emit_code_section(&mut self) {
        if self.code_bodies.is_empty() {
            return;
        }
        let mut content = Vec::new();
        encode_u32(self.code_bodies.len() as u32, &mut content);
        for body in &self.code_bodies {
            encode_u32(body.len() as u32, &mut content);
            content.extend_from_slice(body);
        }
        self.emit_section(SECTION_CODE, &content);
    }

    /// Create a new function body buffer.
    pub fn new_function_body(&mut self) -> Vec<u8> {
        Vec::new()
    }

    /// Add local declarations to a function body buffer.
    pub fn emit_local_declaration(body: &mut Vec<u8>, count: u32, value_type: u8) {
        encode_u32(count, body);
        body.push(value_type);
    }

    /// Finish a function body (push OP_END) and register it. Returns the
    /// function index.
    pub fn finish_function_body(&mut self, body: &mut Vec<u8>) -> u32 {
        body.push(OP_END);
        let idx = self.code_bodies.len() as u32;
        self.code_bodies.push(std::mem::take(body));
        idx
    }

    // ── Data section ────────────────────────────────────────────────

    /// Add a data segment.
    pub fn add_data_segment(&mut self, offset: u32, data: &[u8]) {
        self.data_segments.push((offset, data.to_vec()));
    }

    fn emit_data_section(&mut self) {
        if self.data_segments.is_empty() {
            return;
        }
        let mut content = Vec::new();
        encode_u32(self.data_segments.len() as u32, &mut content);
        for (offset, data) in &self.data_segments {
            // Active segment: memory index 0, offset expression, data
            content.push(0); // memory index 0 (active)
            // offset expression: i32.const + OP_END
            content.push(OP_I32_CONST);
            encode_i32(*offset as i32, &mut content);
            content.push(OP_END);
            // data bytes
            encode_u32(data.len() as u32, &mut content);
            content.extend_from_slice(data);
        }
        self.emit_section(SECTION_DATA, &content);
    }

    // ── Section framing ─────────────────────────────────────────────

    fn emit_section(&mut self, id: u8, content: &[u8]) {
        self.bytes.push(id);
        encode_u32(content.len() as u32, &mut self.bytes);
        self.bytes.extend_from_slice(content);
    }

    // ── Instruction encoding (static helpers) ──────────────────────

    /// Encode an unsigned LEB128 u32.
    pub fn encode_u32(value: u32, out: &mut Vec<u8>) {
        encode_u32(value, out)
    }

    /// Encode a signed LEB128 i32.
    pub fn encode_i32(value: i32, out: &mut Vec<u8>) {
        encode_i32(value, out)
    }

    /// Encode a name (length-prefixed UTF-8 string).
    pub fn encode_name(value: &str, out: &mut Vec<u8>) {
        encode_name(out, value);
    }

    /// Emit a single opcode byte.
    pub fn emit_op(body: &mut Vec<u8>, op: u8) {
        body.push(op);
    }

    /// Emit `i32.const` with the given value.
    pub fn emit_i32_const(body: &mut Vec<u8>, value: i32) {
        body.push(OP_I32_CONST);
        encode_i32(value, body);
    }

    /// Emit `call` with the given function index.
    pub fn emit_call(body: &mut Vec<u8>, func_idx: u32) {
        body.push(OP_CALL);
        encode_u32(func_idx, body);
    }

    /// Emit `block` with the given block type.
    pub fn emit_block(body: &mut Vec<u8>, block_type: u8) {
        body.push(OP_BLOCK);
        body.push(block_type);
    }

    /// Emit `loop` with the given block type.
    pub fn emit_loop(body: &mut Vec<u8>, block_type: u8) {
        body.push(OP_LOOP);
        body.push(block_type);
    }

    /// Emit `if` with the given block type.
    pub fn emit_if(body: &mut Vec<u8>, block_type: u8) {
        body.push(OP_IF);
        body.push(block_type);
    }

    /// Emit `else`.
    pub fn emit_else(body: &mut Vec<u8>) {
        body.push(OP_ELSE);
    }

    /// Emit `end`.
    pub fn emit_end(body: &mut Vec<u8>) {
        body.push(OP_END);
    }

    /// Emit `br` with the given label depth.
    pub fn emit_br(body: &mut Vec<u8>, depth: u32) {
        body.push(OP_BR);
        encode_u32(depth, body);
    }

    /// Emit `br_if` with the given label depth.
    pub fn emit_br_if(body: &mut Vec<u8>, depth: u32) {
        body.push(OP_BR_IF);
        encode_u32(depth, body);
    }

    /// Emit `local.get` with the given local index.
    pub fn emit_local_get(body: &mut Vec<u8>, idx: u32) {
        body.push(OP_LOCAL_GET);
        encode_u32(idx, body);
    }

    /// Emit `local.set` with the given local index.
    pub fn emit_local_set(body: &mut Vec<u8>, idx: u32) {
        body.push(OP_LOCAL_SET);
        encode_u32(idx, body);
    }

    /// Emit `local.tee` with the given local index.
    pub fn emit_local_tee(body: &mut Vec<u8>, idx: u32) {
        body.push(OP_LOCAL_TEE);
        encode_u32(idx, body);
    }

    /// Emit `drop`.
    pub fn emit_drop(body: &mut Vec<u8>) {
        body.push(OP_DROP);
    }

    /// Emit `return`.
    pub fn emit_return(body: &mut Vec<u8>) {
        body.push(OP_RETURN);
    }

    /// Emit `unreachable`.
    pub fn emit_unreachable(body: &mut Vec<u8>) {
        body.push(OP_UNREACHABLE);
    }

    /// Emit `nop`.
    pub fn emit_nop(body: &mut Vec<u8>) {
        body.push(OP_NOP);
    }

    /// Emit `i32.load` with the given align/offset.
    pub fn emit_i32_load(body: &mut Vec<u8>, align: u32, offset: u32) {
        body.push(OP_I32_LOAD);
        encode_u32(align, body);
        encode_u32(offset, body);
    }

    /// Emit `i32.store` with the given align/offset.
    pub fn emit_i32_store(body: &mut Vec<u8>, align: u32, offset: u32) {
        body.push(OP_I32_STORE);
        encode_u32(align, body);
        encode_u32(offset, body);
    }

    /// Emit `global.get` with the given global index.
    pub fn emit_global_get(body: &mut Vec<u8>, idx: u32) {
        body.push(OP_GLOBAL_GET);
        encode_u32(idx, body);
    }

    /// Emit `global.set` with the given global index.
    pub fn emit_global_set(body: &mut Vec<u8>, idx: u32) {
        body.push(OP_GLOBAL_SET);
        encode_u32(idx, body);
    }

    /// Emit `memory.size`.
    pub fn emit_memory_size(body: &mut Vec<u8>) {
        body.push(OP_MEMORY_SIZE);
        encode_u32(0, body); // reserved
    }

    /// Emit `memory.grow`.
    pub fn emit_memory_grow(body: &mut Vec<u8>) {
        body.push(OP_MEMORY_GROW);
        encode_u32(0, body); // reserved
    }

    /// Emit `i32.eqz`.
    pub fn emit_i32_eqz(body: &mut Vec<u8>) {
        body.push(OP_I32_EQZ);
    }

    /// Emit `i32.eq`.
    pub fn emit_i32_eq(body: &mut Vec<u8>) {
        body.push(OP_I32_EQ);
    }

    /// Emit `i32.ne`.
    pub fn emit_i32_ne(body: &mut Vec<u8>) {
        body.push(OP_I32_NE);
    }

    /// Emit `i32.lt_s`.
    pub fn emit_i32_lt_s(body: &mut Vec<u8>) {
        body.push(OP_I32_LT_S);
    }

    /// Emit `i32.le_s`.
    pub fn emit_i32_le_s(body: &mut Vec<u8>) {
        body.push(OP_I32_LE_S);
    }

    /// Emit `i32.gt_s`.
    pub fn emit_i32_gt_s(body: &mut Vec<u8>) {
        body.push(OP_I32_GT_S);
    }

    /// Emit `i32.ge_s`.
    pub fn emit_i32_ge_s(body: &mut Vec<u8>) {
        body.push(OP_I32_GE_S);
    }

    /// Emit `i32.lt_u`.
    pub fn emit_i32_lt_u(body: &mut Vec<u8>) {
        body.push(OP_I32_LT_U);
    }

    /// Emit `i32.gt_u`.
    pub fn emit_i32_gt_u(body: &mut Vec<u8>) {
        body.push(OP_I32_GT_U);
    }

    /// Emit `i32.add`.
    pub fn emit_i32_add(body: &mut Vec<u8>) {
        body.push(OP_I32_ADD);
    }

    /// Emit `i32.sub`.
    pub fn emit_i32_sub(body: &mut Vec<u8>) {
        body.push(OP_I32_SUB);
    }

    /// Emit `i32.mul`.
    pub fn emit_i32_mul(body: &mut Vec<u8>) {
        body.push(OP_I32_MUL);
    }

    /// Emit `i32.div_s`.
    pub fn emit_i32_div_s(body: &mut Vec<u8>) {
        body.push(OP_I32_DIV_S);
    }

    /// Emit `i32.rem_s`.
    pub fn emit_i32_rem_s(body: &mut Vec<u8>) {
        body.push(OP_I32_REM_S);
    }

    /// Emit `i32.and`.
    pub fn emit_i32_and(body: &mut Vec<u8>) {
        body.push(OP_I32_AND);
    }

    /// Emit `i32.or`.
    pub fn emit_i32_or(body: &mut Vec<u8>) {
        body.push(OP_I32_OR);
    }

    /// Emit `i32.xor`.
    pub fn emit_i32_xor(body: &mut Vec<u8>) {
        body.push(OP_I32_XOR);
    }

    /// Emit `i32.shl`.
    pub fn emit_i32_shl(body: &mut Vec<u8>) {
        body.push(OP_I32_SHL);
    }

    /// Emit `i32.shr_s`.
    pub fn emit_i32_shr_s(body: &mut Vec<u8>) {
        body.push(OP_I32_SHR_S);
    }

    /// Emit `i32.shr_u`.
    pub fn emit_i32_shr_u(body: &mut Vec<u8>) {
        body.push(OP_I32_SHR_U);
    }

    /// Emit `i32.clz`.
    pub fn emit_i32_clz(body: &mut Vec<u8>) {
        body.push(OP_I32_CLZ);
    }

    /// Emit `i32.ctz`.
    pub fn emit_i32_ctz(body: &mut Vec<u8>) {
        body.push(OP_I32_CTZ);
    }

    /// Emit `i32.popcnt`.
    pub fn emit_i32_popcnt(body: &mut Vec<u8>) {
        body.push(OP_I32_POPCNT);
    }

    /// Emit `i32.wrap_i64`.
    pub fn emit_i32_wrap_i64(body: &mut Vec<u8>) {
        body.push(OP_I32_WRAP_I64);
    }

    /// Emit `select`.
    pub fn emit_select(body: &mut Vec<u8>) {
        body.push(OP_SELECT);
    }
}

// ── LEB128 encoding ──────────────────────────────────────────────────

/// Encode an unsigned LEB128 u32.
pub fn encode_u32(mut value: u32, out: &mut Vec<u8>) {
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

/// Encode a signed LEB128 i32.
pub fn encode_i32(mut value: i32, out: &mut Vec<u8>) {
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

/// Encode a name (length-prefixed UTF-8 string).
pub fn encode_name(out: &mut Vec<u8>, value: &str) {
    encode_u32(value.len() as u32, out);
    out.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_u32_small() {
        let mut buf = Vec::new();
        encode_u32(0, &mut buf);
        assert_eq!(buf, vec![0x00]);
    }

    #[test]
    fn encode_u32_medium() {
        let mut buf = Vec::new();
        encode_u32(128, &mut buf);
        assert_eq!(buf, vec![0x80, 0x01]);
    }

    #[test]
    fn encode_i32_negative() {
        let mut buf = Vec::new();
        encode_i32(-1, &mut buf);
        assert_eq!(buf, vec![0x7f]);
    }

    #[test]
    fn writer_creates_valid_module() {
        let mut writer = WasmBinaryWriter::new();
        writer.begin_module();
        let wasm = writer.into_bytes();
        assert_eq!(&wasm[..8], b"\0asm\x01\0\0\0");
    }

    #[test]
    fn type_index_dedup() {
        let mut writer = WasmBinaryWriter::new();
        let t1 = writer.get_or_create_type_index(&[I32, I32], &[I32]);
        let t2 = writer.get_or_create_type_index(&[I32, I32], &[I32]);
        assert_eq!(t1, t2);
        let t3 = writer.get_or_create_type_index(&[I32], &[]);
        assert_ne!(t1, t3);
    }

    #[test]
    fn register_function_updates_count() {
        let mut writer = WasmBinaryWriter::new();
        let t0 = writer.get_or_create_type_index(&[], &[]);
        writer.register_function(t0);
        writer.register_function(t0);
        assert_eq!(writer.func_type_indices.len(), 2);
    }
}
