use crate::runtime_fn::HostImportSpec;
use crate::wasm_ir::{WasmBlockType, WasmExportKind, WasmFunction, WasmInstr, WasmModule};

// ---------------------------------------------------------------------------
// Existing types — preserved exactly.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WatValueType {
    I32,
    I64,
}

impl WatValueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct WatFunctionType {
    params: Vec<WatValueType>,
    results: Vec<WatValueType>,
}

impl WatFunctionType {
    pub(super) fn from_spec(params: &str, results: &str) -> Self {
        Self {
            params: parse_type_list(params, "param"),
            results: parse_type_list(results, "result"),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub(super) struct WatModuleBuilder {
    output: String,
}

impl WatModuleBuilder {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn push_import_func(&mut self, spec: &HostImportSpec) {
        let signature = WatFunctionType::from_spec(spec.params, spec.result);
        self.output.push_str("  (import \"");
        self.output.push_str(spec.module);
        self.output.push_str("\" \"");
        self.output.push_str(spec.name);
        self.output.push_str("\" (func ");
        self.output.push_str(spec.wat_symbol);
        self.append_sig(&signature);
        self.output.push_str("))\n");
    }

    pub(super) fn push_global_i32(&mut self, symbol: &str, initial: i32) {
        self.output.push_str("  (global ");
        self.output.push_str(symbol);
        self.output.push_str(" (mut i32) (i32.const ");
        self.output.push_str(&initial.to_string());
        self.output.push_str("))\n");
    }

    pub(super) fn push_data_segment_escaped(&mut self, offset: u32, escaped: &str) {
        self.output.push_str("  (data (i32.const ");
        self.output.push_str(&offset.to_string());
        self.output.push_str(") \"");
        self.output.push_str(escaped);
        self.output.push_str("\")\n");
    }

    pub(super) fn into_inner(self) -> String {
        self.output
    }

    fn append_sig(&mut self, sig: &WatFunctionType) {
        self.append_type_group("param", &sig.params);
        self.append_type_group("result", &sig.results);
    }

    fn append_type_group(&mut self, kind: &str, types: &[WatValueType]) {
        if types.is_empty() {
            return;
        }
        self.output.push(' ');
        self.output.push('(');
        self.output.push_str(kind);
        for ty in types {
            self.output.push(' ');
            self.output.push_str(ty.as_str());
        }
        self.output.push(')');
    }
}

fn parse_type_list(raw: &str, kind: &str) -> Vec<WatValueType> {
    let parts = raw.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return Vec::new();
    }
    if parts[0] != kind {
        return Vec::new();
    }
    parts
        .iter()
        .skip(1)
        .filter_map(|value| match *value {
            "i32" => Some(WatValueType::I32),
            "i64" => Some(WatValueType::I64),
            _ => None,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// WatWriter — typed instruction-level WAT writer.
//
// Each typed method produces:
//   " ".repeat(indent) + "(keyword arg1 arg2 ...)" + "\n"
//
// Use `line` / `line_fmt` as a fallback for patterns that do not fit the
// typed methods (e.g. multi-line multi-instruction inline expressions, or
// constructs whose WAT syntax is complex/dynamic).
// ---------------------------------------------------------------------------

pub struct WatWriter {
    output: String,
}

impl Default for WatWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl WatWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    /// Consume the writer and return the accumulated WAT text.
    pub fn into_string(self) -> String {
        self.output
    }

    /// Clear the buffer (reset to empty).
    pub fn clear(&mut self) {
        self.output.clear();
    }

    // ---- raw line helpers --------------------------------------------------

    /// Emit `" ".repeat(indent) + content + "\n"`.
    pub fn line(&mut self, indent: usize, content: &str) {
        let pad = " ".repeat(indent);
        self.output.push_str(&pad);
        self.output.push_str(content);
        self.output.push('\n');
    }

    /// Emit `" ".repeat(indent) + content + "\n"` where content is produced
    /// by format_args!().  Example:
    ///   w.line_fmt(4, format_args!("(local.get {})", id));
    pub fn line_fmt(&mut self, indent: usize, args: std::fmt::Arguments<'_>) {
        let pad = " ".repeat(indent);
        self.output.push_str(&pad);
        self.output.push_str(&std::fmt::format(args));
        self.output.push('\n');
    }

    // ---- local access -------------------------------------------------------

    pub fn local_get(&mut self, indent: usize, id: usize) {
        self.line(indent, &format!("(local.get {id})"));
    }

    pub fn local_set(&mut self, indent: usize, id: usize) {
        self.line(indent, &format!("(local.set {id})"));
    }

    pub fn local_tee(&mut self, indent: usize, id: usize) {
        self.line(indent, &format!("(local.tee {id})"));
    }

    // ---- constants ----------------------------------------------------------

    pub fn i32_const(&mut self, indent: usize, value: i32) {
        self.line(indent, &format!("(i32.const {value})"));
    }

    pub fn i64_const(&mut self, indent: usize, value: i64) {
        self.line(indent, &format!("(i64.const {value})"));
    }

    // ---- calls --------------------------------------------------------------

    /// Emit `(call name)` — name should include `$` if it is a symbolic name.
    /// (RuntimeFn symbols already include `$`; for plain names use `$prefix`.)
    pub fn call(&mut self, indent: usize, name: &str) {
        self.line(indent, &format!("(call {name})"));
    }

    /// Emit `(call N)` (direct call by function index).
    pub fn call_direct(&mut self, indent: usize, idx: u32) {
        self.line(indent, &format!("(call {idx})"));
    }

    // ---- control flow -------------------------------------------------------

    pub fn drop(&mut self, indent: usize) {
        self.line(indent, "(drop)");
    }

    pub fn unreachable(&mut self, indent: usize) {
        self.line(indent, "(unreachable)");
    }

    pub fn nop(&mut self, indent: usize) {
        self.line(indent, "(nop)");
    }

    pub fn return_(&mut self, indent: usize) {
        self.line(indent, "(return)");
    }

    /// Emit `(br $target)`.
    pub fn r#br(&mut self, indent: usize, target: &str) {
        self.line(indent, &format!("(br ${target})"));
    }

    /// Emit `(br_if $target)`.
    pub fn br_if(&mut self, indent: usize, target: &str) {
        self.line(indent, &format!("(br_if ${target})"));
    }

    pub fn select(&mut self, indent: usize) {
        self.line(indent, "(select)");
    }

    // ---- block structure ----------------------------------------------------

    /// Open `(if` (no result type).
    pub fn r#if(&mut self, indent: usize) {
        self.line(indent, "(if");
    }

    /// Open `(if (result i32)` — single-value result.
    pub fn if_result(&mut self, indent: usize, result: &str) {
        self.line(indent, &format!("(if (result {result})"));
    }

    /// Emit `  (then` (note: two-space indent *inside* the construct).
    pub fn then(&mut self, indent: usize) {
        self.line(indent, "  (then");
    }

    /// Emit `  (else`.
    pub fn r#else(&mut self, indent: usize) {
        self.line(indent, "  (else");
    }

    /// Emit a single close-paren `)` — used to close brackets opened by
    /// `r#if`, `block`, `r#loop`, `then`, `r#else` etc.
    pub fn end(&mut self, indent: usize) {
        self.line(indent, ")");
    }

    /// Emit `(block $label`.
    pub fn block(&mut self, indent: usize, label: &str) {
        self.line(indent, &format!("(block ${label}"));
    }

    /// Emit `  (loop $label`.
    pub fn r#loop(&mut self, indent: usize, label: &str) {
        self.line(indent, &format!("  (loop ${label}"));
    }

    // ---- i32 comparison / arithmetic ---------------------------------------

    pub fn i32_eqz(&mut self, indent: usize) {
        self.line(indent, "(i32.eqz)");
    }
    pub fn i32_eq(&mut self, indent: usize) {
        self.line(indent, "(i32.eq)");
    }
    pub fn i32_ne(&mut self, indent: usize) {
        self.line(indent, "(i32.ne)");
    }
    pub fn i32_lt_s(&mut self, indent: usize) {
        self.line(indent, "(i32.lt_s)");
    }
    pub fn i32_le_s(&mut self, indent: usize) {
        self.line(indent, "(i32.le_s)");
    }
    pub fn i32_gt_s(&mut self, indent: usize) {
        self.line(indent, "(i32.gt_s)");
    }
    pub fn i32_ge_s(&mut self, indent: usize) {
        self.line(indent, "(i32.ge_s)");
    }
    pub fn i32_lt_u(&mut self, indent: usize) {
        self.line(indent, "(i32.lt_u)");
    }
    pub fn i32_le_u(&mut self, indent: usize) {
        self.line(indent, "(i32.le_u)");
    }
    pub fn i32_gt_u(&mut self, indent: usize) {
        self.line(indent, "(i32.gt_u)");
    }
    pub fn i32_ge_u(&mut self, indent: usize) {
        self.line(indent, "(i32.ge_u)");
    }
    pub fn i32_add(&mut self, indent: usize) {
        self.line(indent, "(i32.add)");
    }
    pub fn i32_sub(&mut self, indent: usize) {
        self.line(indent, "(i32.sub)");
    }
    pub fn i32_mul(&mut self, indent: usize) {
        self.line(indent, "(i32.mul)");
    }
    pub fn i32_div_s(&mut self, indent: usize) {
        self.line(indent, "(i32.div_s)");
    }
    pub fn i32_div_u(&mut self, indent: usize) {
        self.line(indent, "(i32.div_u)");
    }
    pub fn i32_rem_s(&mut self, indent: usize) {
        self.line(indent, "(i32.rem_s)");
    }
    pub fn i32_rem_u(&mut self, indent: usize) {
        self.line(indent, "(i32.rem_u)");
    }
    pub fn i32_and(&mut self, indent: usize) {
        self.line(indent, "(i32.and)");
    }
    pub fn i32_or(&mut self, indent: usize) {
        self.line(indent, "(i32.or)");
    }
    pub fn i32_xor(&mut self, indent: usize) {
        self.line(indent, "(i32.xor)");
    }
    pub fn i32_shl(&mut self, indent: usize) {
        self.line(indent, "(i32.shl)");
    }
    pub fn i32_shr_s(&mut self, indent: usize) {
        self.line(indent, "(i32.shr_s)");
    }
    pub fn i32_shr_u(&mut self, indent: usize) {
        self.line(indent, "(i32.shr_u)");
    }
    pub fn i32_clz(&mut self, indent: usize) {
        self.line(indent, "(i32.clz)");
    }
    pub fn i32_ctz(&mut self, indent: usize) {
        self.line(indent, "(i32.ctz)");
    }
    pub fn i32_popcnt(&mut self, indent: usize) {
        self.line(indent, "(i32.popcnt)");
    }
    pub fn i32_wrap_i64(&mut self, indent: usize) {
        self.line(indent, "(i32.wrap_i64)");
    }

    // ---- memory -------------------------------------------------------------

    pub fn memory_size(&mut self, indent: usize) {
        self.line(indent, "(memory.size)");
    }
    pub fn memory_grow(&mut self, indent: usize) {
        self.line(indent, "(memory.grow)");
    }

    // ---- globals ------------------------------------------------------------

    /// Emit `(global.get $name)`.
    pub fn global_get(&mut self, indent: usize, name: &str) {
        self.line(indent, &format!("(global.get {name})"));
    }

    /// Emit `(global.set $name)`.
    pub fn global_set(&mut self, indent: usize, name: &str) {
        self.line(indent, &format!("(global.set {name})"));
    }

    // ---- raw escape hatch ----------------------------------------------------

    /// Emit a raw WAT line at the given indentation.
    ///
    /// This is an explicit alternative to using `push_str` on the internal
    /// buffer.  Use this when a pattern cannot be expressed via the existing
    /// typed methods (e.g. multi-line inline expressions, complex inline
    /// sequences).  New code should prefer typed methods.
    pub fn raw_line(&mut self, indent: usize, content: &str) {
        self.line(indent, content);
    }

    // ---- load / store -------------------------------------------------------

    /// Emit `(i32.store)` / `(i32.store offset=N)` / `(i32.store align=N)`.
    ///
    /// wat2wasm 1.0.27 does not accept both `align=N` and `offset=N` together
    /// in the flat instruction form, so we emit whichever memarg is non-default.
    /// When both are at the i32 default (align=2, offset=0) no keyword is emitted.
    pub fn i32_store(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = if offset != 0 {
            format!(" offset={offset}")
        } else if align != 2 {
            format!(" align={align}")
        } else {
            String::new()
        };
        self.line(indent, &format!("(i32.store{keyword})"));
    }

    /// Emit `(i32.load)` / `(i32.load offset=N)` / `(i32.load align=N)`.
    ///
    /// wat2wasm 1.0.27 does not accept both `align=N` and `offset=N` together
    /// in the flat instruction form, so we emit whichever memarg is non-default.
    /// When both are at the i32 default (align=2, offset=0) no keyword is emitted.
    pub fn i32_load(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = if offset != 0 {
            format!(" offset={offset}")
        } else if align != 2 {
            format!(" align={align}")
        } else {
            String::new()
        };
        self.line(indent, &format!("(i32.load{keyword})"));
    }

    pub fn i32_load8_u(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = if offset != 0 {
            format!(" offset={offset}")
        } else if align != 0 {
            format!(" align={}", 1_u32 << align)
        } else {
            String::new()
        };
        self.line(indent, &format!("(i32.load8_u{keyword})"));
    }

    pub fn i32_load8_s(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 0);
        self.line(indent, &format!("(i32.load8_s{keyword})"));
    }

    pub fn i32_load16_u(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 1);
        self.line(indent, &format!("(i32.load16_u{keyword})"));
    }

    pub fn i32_load16_s(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 1);
        self.line(indent, &format!("(i32.load16_s{keyword})"));
    }

    pub fn i64_load(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 3);
        self.line(indent, &format!("(i64.load{keyword})"));
    }

    pub fn i32_store8(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = if offset != 0 {
            format!(" offset={offset}")
        } else if align != 0 {
            format!(" align={}", 1_u32 << align)
        } else {
            String::new()
        };
        self.line(indent, &format!("(i32.store8{keyword})"));
    }

    pub fn i32_store16(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 1);
        self.line(indent, &format!("(i32.store16{keyword})"));
    }

    pub fn i64_store(&mut self, indent: usize, align: u32, offset: u32) {
        let keyword = memarg_keyword_log2(align, offset, 3);
        self.line(indent, &format!("(i64.store{keyword})"));
    }

    // ---- module-level helpers ------------------------------------------------

    /// Emit `(module\n`.
    pub fn open_module(&mut self) {
        self.output.push_str("(module\n");
    }

    /// Emit `)\n`.
    pub fn close_module(&mut self) {
        self.output.push_str(")\n");
    }

    /// Emit `  (func $name ` (with trailing space — callers should follow
    /// with params and `func_end()`).
    pub fn func_start(&mut self, name: &str) {
        self.output.push_str(&format!("  (func ${name} "));
    }

    /// Emit `(param i32) ` (string accumulation, no newline).
    pub fn func_param_i32(&mut self) {
        self.output.push_str("(param i32) ");
    }

    /// Emit `(result i32)\n`.
    pub fn func_result_i32(&mut self) {
        self.output.push_str("(result i32)\n");
    }

    /// Emit `    (local i32)\n`.
    pub fn func_local_i32(&mut self) {
        self.output.push_str("    (local i32)\n");
    }

    /// Emit `  )\n`.
    pub fn func_end(&mut self) {
        self.output.push_str("  )\n");
    }

    /// Emit `  (export "name" (func $name))\n`.
    pub fn export_func(&mut self, name: &str) {
        self.output
            .push_str(&format!("  (export \"{name}\" (func ${name}))\n"));
    }

    // ---- helpers common in both module-level and instruction-level -----------

    /// Append raw content to the internal buffer (no newline, no indent).
    /// Use this only for inline composition (e.g. concatenating a complex
    /// instruction that spans part of a line).
    pub fn push_str(&mut self, s: &str) {
        self.output.push_str(s);
    }

    /// Borrow the current buffer for inspection (tests, debugging).
    pub fn as_str(&self) -> &str {
        &self.output
    }

    /// Temporary escape hatch: get a mutable reference to the underlying
    /// string buffer.  Use this only when calling legacy APIs that still
    /// take `&mut String`.  Will be removed once all callers migrate to
    /// typed methods.
    pub fn output_mut(&mut self) -> &mut String {
        &mut self.output
    }

    // ---- typed IR emission (WasmInstr, WasmModule) --------------------------

    /// Emit a single `WasmInstr` at the given indentation level.
    ///
    /// Dispatches to the existing typed convenience methods so there is no
    /// behaviour change — this is just a typed dispatch layer on top of the
    /// same WAT output.
    pub fn emit_instr(&mut self, indent: usize, instr: &WasmInstr) {
        match instr {
            WasmInstr::LocalGet(id) => self.local_get(indent, *id),
            WasmInstr::LocalSet(id) => self.local_set(indent, *id),
            WasmInstr::LocalTee(id) => self.local_tee(indent, *id),
            WasmInstr::I32Const(v) => self.i32_const(indent, *v),
            WasmInstr::I64Const(v) => self.i64_const(indent, *v),
            WasmInstr::Call(name) => self.call(indent, name),
            WasmInstr::CallDirect(idx) => self.call_direct(indent, *idx),
            WasmInstr::GlobalGet(name) => {
                self.line(indent, &format!("(global.get {name})"));
            }
            WasmInstr::GlobalSet(name) => {
                self.line(indent, &format!("(global.set {name})"));
            }
            WasmInstr::Drop => self.drop(indent),
            WasmInstr::Unreachable => self.unreachable(indent),
            WasmInstr::Nop => self.nop(indent),
            WasmInstr::Return => self.return_(indent),
            WasmInstr::Br(target) => self.r#br(indent, target),
            WasmInstr::BrIf(target) => self.br_if(indent, target),
            WasmInstr::BrDepth(depth) => self.line(indent, &format!("(br {depth})")),
            WasmInstr::BrIfDepth(depth) => self.line(indent, &format!("(br_if {depth})")),
            WasmInstr::Select => self.select(indent),
            WasmInstr::If { result_ty } => match result_ty {
                WasmBlockType::Result(ty) => self.if_result(indent, ty.as_str()),
                WasmBlockType::Empty => self.r#if(indent),
            },
            WasmInstr::Then => self.then(indent),
            WasmInstr::Else => self.r#else(indent),
            WasmInstr::End => self.end(indent),
            WasmInstr::Block(label) => self.block(indent, label),
            WasmInstr::Loop(label) => self.r#loop(indent, label),
            WasmInstr::I32Eqz => self.i32_eqz(indent),
            WasmInstr::I32Eq => self.i32_eq(indent),
            WasmInstr::I32Ne => self.i32_ne(indent),
            WasmInstr::I32LtS => self.i32_lt_s(indent),
            WasmInstr::I32LeS => self.i32_le_s(indent),
            WasmInstr::I32GtS => self.i32_gt_s(indent),
            WasmInstr::I32GeS => self.i32_ge_s(indent),
            WasmInstr::I32LtU => self.i32_lt_u(indent),
            WasmInstr::I32LeU => self.i32_le_u(indent),
            WasmInstr::I32GtU => self.i32_gt_u(indent),
            WasmInstr::I32GeU => self.i32_ge_u(indent),
            WasmInstr::I32Add => self.i32_add(indent),
            WasmInstr::I32Sub => self.i32_sub(indent),
            WasmInstr::I32Mul => self.i32_mul(indent),
            WasmInstr::I32DivS => self.i32_div_s(indent),
            WasmInstr::I32DivU => self.i32_div_u(indent),
            WasmInstr::I32RemS => self.i32_rem_s(indent),
            WasmInstr::I32RemU => self.i32_rem_u(indent),
            WasmInstr::I32And => self.i32_and(indent),
            WasmInstr::I32Or => self.i32_or(indent),
            WasmInstr::I32Xor => self.i32_xor(indent),
            WasmInstr::I32Shl => self.i32_shl(indent),
            WasmInstr::I32ShrS => self.i32_shr_s(indent),
            WasmInstr::I32ShrU => self.i32_shr_u(indent),
            WasmInstr::I32Clz => self.i32_clz(indent),
            WasmInstr::I32Ctz => self.i32_ctz(indent),
            WasmInstr::I32Popcnt => self.i32_popcnt(indent),
            WasmInstr::I32WrapI64 => self.i32_wrap_i64(indent),
            WasmInstr::I64ExtendI32S => self.line(indent, "(i64.extend_i32_s)"),
            WasmInstr::I64ExtendI32U => self.line(indent, "(i64.extend_i32_u)"),
            WasmInstr::I64Eqz => self.line(indent, "(i64.eqz)"),
            WasmInstr::I64Eq => self.line(indent, "(i64.eq)"),
            WasmInstr::I64LtS => self.line(indent, "(i64.lt_s)"),
            WasmInstr::I64GeU => self.line(indent, "(i64.ge_u)"),
            WasmInstr::I64Add => self.line(indent, "(i64.add)"),
            WasmInstr::I64Sub => self.line(indent, "(i64.sub)"),
            WasmInstr::I64Mul => self.line(indent, "(i64.mul)"),
            WasmInstr::I64DivU => self.line(indent, "(i64.div_u)"),
            WasmInstr::I64RemU => self.line(indent, "(i64.rem_u)"),
            WasmInstr::I64GtU => self.line(indent, "(i64.gt_u)"),
            WasmInstr::I64And => self.line(indent, "(i64.and)"),
            WasmInstr::I64Or => self.line(indent, "(i64.or)"),
            WasmInstr::I64Xor => self.line(indent, "(i64.xor)"),
            WasmInstr::I64Shl => self.line(indent, "(i64.shl)"),
            WasmInstr::I64ShrS => self.line(indent, "(i64.shr_s)"),
            WasmInstr::I64ShrU => self.line(indent, "(i64.shr_u)"),
            WasmInstr::MemorySize => self.memory_size(indent),
            WasmInstr::MemoryGrow => self.memory_grow(indent),
            WasmInstr::MemoryCopy => self.line(indent, "(memory.copy)"),
            WasmInstr::MemoryFill => self.line(indent, "(memory.fill)"),
            WasmInstr::I32Load { align, offset } => self.i32_load(indent, *align, *offset),
            WasmInstr::I32Load8S { align, offset } => self.i32_load8_s(indent, *align, *offset),
            WasmInstr::I32Load8U { align, offset } => self.i32_load8_u(indent, *align, *offset),
            WasmInstr::I32Load16S { align, offset } => self.i32_load16_s(indent, *align, *offset),
            WasmInstr::I32Load16U { align, offset } => self.i32_load16_u(indent, *align, *offset),
            WasmInstr::I64Load { align, offset } => self.i64_load(indent, *align, *offset),
            WasmInstr::I32Store { align, offset } => self.i32_store(indent, *align, *offset),
            WasmInstr::I32Store8 { align, offset } => self.i32_store8(indent, *align, *offset),
            WasmInstr::I32Store16 { align, offset } => self.i32_store16(indent, *align, *offset),
            WasmInstr::I64Store { align, offset } => self.i64_store(indent, *align, *offset),
            WasmInstr::Raw(content) => self.line(indent, content),
        }
    }

    /// Emit a sequence of `WasmInstr` values at the given indentation level.
    pub fn emit_instrs(&mut self, indent: usize, instrs: &[WasmInstr]) {
        for instr in instrs {
            self.emit_instr(indent, instr);
        }
    }

    /// Emit a full `WasmFunction` at module level.
    ///
    /// This produces:
    /// ```wat
    ///   (func $name (param ...) (result ...)
    ///     (local ...)
    ///     <body>
    ///   )
    /// ```
    pub fn emit_function(&mut self, f: &WasmFunction) {
        self.func_start(&f.symbol);
        for p in &f.params {
            self.push_str(&format!("(param {}) ", p.as_str()));
        }
        for r in &f.results {
            self.push_str(&format!("(result {})\n", r.as_str()));
        }
        for l in &f.locals {
            self.line(4, &format!("(local {})", l.as_str()));
        }
        self.emit_instrs(4, &f.body);
        self.func_end();
    }

    /// Emit a full `WasmModule` (all top-level constructs).
    ///
    /// This is a convenience that produces a complete valid WAT module.
    /// For incremental emission (e.g. mixing typed and untyped output),
    /// call the individual methods instead.
    pub fn emit_module(&mut self, module: &WasmModule) {
        self.open_module();

        for imp in &module.imports {
            let symbol = wat_symbol(&imp.func_symbol);
            let params = wat_type_group("param", &imp.params);
            let results = wat_type_group("result", &imp.results);
            self.line(
                2,
                &format!(
                    "(import \"{}\" \"{}\" (func {}{}{}))",
                    imp.module, imp.name, symbol, params, results,
                ),
            );
        }

        if let Some(mem) = &module.memory {
            match &mem.export_name {
                Some(export) => self.line(
                    2,
                    &format!(
                        "(memory (export \"{export}\") {} {})",
                        mem.min_pages, mem.max_pages
                    ),
                ),
                None => self.line(2, &format!("(memory {} {})", mem.min_pages, mem.max_pages)),
            }
        }

        for g in &module.globals {
            let mut_str = if g.is_mut { " (mut " } else { " " };
            let mut_end = if g.is_mut { ")" } else { "" };
            self.push_str(&format!(
                "  (global {}{}{}{} ",
                g.symbol,
                mut_str,
                g.val_type.as_str(),
                mut_end
            ));
            match &g.init {
                WasmInstr::I32Const(v) => self.push_str(&format!("(i32.const {v})")),
                WasmInstr::I64Const(v) => self.push_str(&format!("(i64.const {v})")),
                _ => self.push_str("(unreachable)"),
            }
            self.push_str(")\n");
        }

        for seg in &module.data_segments {
            let escaped: String = seg
                .data
                .iter()
                .flat_map(|b| std::ascii::escape_default(*b))
                .map(|c| c as char)
                .collect();
            self.line(
                2,
                &format!("(data (i32.const {}) \"{}\")", seg.offset, escaped),
            );
        }

        for f in &module.functions {
            self.emit_function(f);
        }

        for e in &module.exports {
            match &e.kind {
                WasmExportKind::Func(_sym) => {
                    self.export_func(&e.name);
                }
                WasmExportKind::Memory => {
                    self.line(2, &format!("(export \"{}\" (memory 0))", e.name));
                }
            }
        }

        // Custom sections: emit as WAT comments
        for section in &module.custom_sections {
            let payload_str = String::from_utf8_lossy(&section.payload);
            self.line(2, &format!(";; custom-section: {}", section.name));
            for line in payload_str.lines() {
                self.line(2, &format!(";;   {}", line));
            }
        }

        self.close_module();
    }
}

fn wat_symbol(symbol: &str) -> std::borrow::Cow<'_, str> {
    if symbol.starts_with('$') {
        std::borrow::Cow::Borrowed(symbol)
    } else {
        std::borrow::Cow::Owned(format!("${symbol}"))
    }
}

fn wat_type_group(kind: &str, types: &[crate::wasm_ir::WasmValType]) -> String {
    if types.is_empty() {
        return String::new();
    }

    let values = types
        .iter()
        .map(|t| t.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    format!(" ({kind} {values})")
}

fn memarg_keyword_log2(align: u32, offset: u32, default_align_log2: u32) -> String {
    if offset != 0 {
        format!(" offset={offset}")
    } else if align != default_align_log2 {
        format!(" align={}", 1_u32 << align)
    } else {
        String::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
