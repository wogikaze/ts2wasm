// ---------------------------------------------------------------------------
// Typed Wasm IR — structured wasm/WAT representation for backend emission.
//
// This module defines:
//   - `WasmInstr` — a typed enum of WAT instructions (no raw strings).
//   - `WasmModule` — a structured representation of an entire wasm module.
//
// Emitters should prefer building `WasmInstr` sequences and `WasmModule`
// structs over hand-concatenating raw WAT strings.  `WatWriter` can emit
// both the typed IR (via `emit_instr`, `emit_function`, `emit_module`) and
// the existing untyped convenience methods.
//
// Non-goals:
//   - No `wasm-encoder` dependency (that is a separate feature-gated path).
//   - No validation or formal type-checking — correctness of the generated
//     WAT is the emitter's responsibility.
// ---------------------------------------------------------------------------

/// A single wasm instruction, without indentation.
///
/// Structured control-flow instructions (`If`, `Block`, `Loop`, `Then`,
/// `Else`, `End`) are represented as separate flat entries.  The emitter
/// (e.g. `WatWriter`) manages indentation for nesting.
#[derive(Debug, Clone)]
pub enum WasmInstr {
    // ---- local access -------------------------------------------------------
    LocalGet(usize),
    LocalSet(usize),
    LocalTee(usize),

    // ---- constants ----------------------------------------------------------
    I32Const(i32),
    I64Const(i64),

    // ---- calls --------------------------------------------------------------
    /// `(call $name)` — name should include `$` prefix.
    Call(String),
    /// `(call N)` — direct call by function index.
    CallDirect(u32),

    // ---- control flow -------------------------------------------------------
    Drop,
    Unreachable,
    Nop,
    Return,
    /// `(br $label)`
    Br(String),
    /// `(br_if $label)`
    BrIf(String),
    Select,

    /// `(if` or `(if (result $ty))` — result_ty is Some when an if-result.
    If {
        result_ty: Option<String>,
    },
    /// `  (then`
    Then,
    /// `  (else`
    Else,
    /// `)` — closes any open bracket (if, block, loop, then, else).
    End,

    /// `(block $label`
    Block(String),
    /// `  (loop $label`
    Loop(String),

    // ---- i32 comparison / arithmetic ----------------------------------------
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LeS,
    I32GtS,
    I32GeS,
    I32LtU,
    I32LeU,
    I32GtU,
    I32GeU,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32RemS,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32WrapI64,

    // ---- memory ------------------------------------------------------------
    MemorySize,
    MemoryGrow,

    // ---- load / store ------------------------------------------------------
    /// `(i32.load align=$0 offset=$1)`
    I32Load {
        align: u32,
        offset: u32,
    },
    /// `(i32.store align=$0 offset=$1)`
    I32Store {
        align: u32,
        offset: u32,
    },

    // ---- globals -----------------------------------------------------------
    /// `(global.get $name)`
    GlobalGet(String),
    /// `(global.set $name)`
    GlobalSet(String),

    // ---- raw escape hatch --------------------------------------------------
    /// Emit an arbitrary line (no indentation managed — caller pre-formats).
    /// Use only for patterns that cannot be expressed via typed variants.
    Raw(String),
}

/// Wasm value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmValType {
    I32,
    I64,
}

impl WasmValType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
        }
    }
}

/// A structured representation of a wasm function body.
#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub symbol: String,
    pub params: Vec<WasmValType>,
    pub results: Vec<WasmValType>,
    pub locals: Vec<WasmValType>,
    pub body: Vec<WasmInstr>,
}

impl WasmFunction {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn param(mut self, ty: WasmValType) -> Self {
        self.params.push(ty);
        self
    }

    pub fn result(mut self, ty: WasmValType) -> Self {
        self.results.push(ty);
        self
    }

    pub fn local(mut self, ty: WasmValType) -> Self {
        self.locals.push(ty);
        self
    }

    pub fn body(mut self, instrs: Vec<WasmInstr>) -> Self {
        self.body = instrs;
        self
    }
}

/// A global variable declaration.
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    pub symbol: String,
    pub is_mut: bool,
    pub val_type: WasmValType,
    pub init: WasmInstr,
}

impl WasmGlobal {
    pub fn i32_mut(symbol: impl Into<String>, initial: i32) -> Self {
        Self {
            symbol: symbol.into(),
            is_mut: true,
            val_type: WasmValType::I32,
            init: WasmInstr::I32Const(initial),
        }
    }
}

/// A data segment declaration.
#[derive(Debug, Clone)]
pub struct WasmDataSegment {
    pub offset: u32,
    pub data: Vec<u8>,
}

impl WasmDataSegment {
    pub fn new(offset: u32, data: impl Into<Vec<u8>>) -> Self {
        Self {
            offset,
            data: data.into(),
        }
    }
}

/// An import declaration.
#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub func_symbol: String,
    pub params: Vec<WasmValType>,
    pub results: Vec<WasmValType>,
}

impl WasmImport {
    pub fn func(
        module: impl Into<String>,
        name: impl Into<String>,
        func_symbol: impl Into<String>,
        params: impl IntoIterator<Item = WasmValType>,
        results: impl IntoIterator<Item = WasmValType>,
    ) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
            func_symbol: func_symbol.into(),
            params: params.into_iter().collect(),
            results: results.into_iter().collect(),
        }
    }

    pub fn from_catalog_type_specs(
        module: impl Into<String>,
        name: impl Into<String>,
        func_symbol: impl Into<String>,
        params: &str,
        results: &str,
    ) -> Self {
        Self::func(
            module,
            name,
            func_symbol,
            parse_catalog_type_list(params),
            parse_catalog_type_list(results),
        )
    }
}

/// Parse a runtime-catalog-style type spec ("param i32 i32", "result i32").
fn parse_catalog_type_list(raw: &str) -> Vec<WasmValType> {
    raw.split_whitespace()
        .skip(1) // skip the "param"/"result" keyword
        .filter_map(|s| match s {
            "i32" => Some(WasmValType::I32),
            "i64" => Some(WasmValType::I64),
            _ => None,
        })
        .collect()
}

/// A memory declaration.
#[derive(Debug, Clone)]
pub struct WasmMemory {
    pub min_pages: u32,
    pub max_pages: u32,
    pub export_name: Option<String>,
}

impl WasmMemory {
    pub fn new(min_pages: u32, max_pages: u32) -> Self {
        Self {
            min_pages,
            max_pages,
            export_name: None,
        }
    }

    pub fn exported(min_pages: u32, max_pages: u32, export_name: impl Into<String>) -> Self {
        Self {
            min_pages,
            max_pages,
            export_name: Some(export_name.into()),
        }
    }
}

/// An export declaration.
#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub kind: WasmExportKind,
}

impl WasmExport {
    pub fn func(name: impl Into<String>, symbol: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: WasmExportKind::Func(symbol.into()),
        }
    }

    pub fn memory(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: WasmExportKind::Memory,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WasmExportKind {
    Func(String),
    Memory,
}

/// A structured representation of an entire wasm module.
///
/// This is the highest-level typed IR for wasm modules.  Emitters can
/// construct a `WasmModule` and hand it to `WatWriter::emit_module` for
/// WAT output.  The individual section elements (functions, globals, etc.)
/// can also be emitted incrementally via the existing builder-style methods.
#[derive(Debug, Clone, Default)]
pub struct WasmModule {
    pub imports: Vec<WasmImport>,
    pub globals: Vec<WasmGlobal>,
    pub memory: Option<WasmMemory>,
    pub data_segments: Vec<WasmDataSegment>,
    pub functions: Vec<WasmFunction>,
    pub exports: Vec<WasmExport>,
}

impl WasmModule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn import(mut self, imp: WasmImport) -> Self {
        self.imports.push(imp);
        self
    }

    pub fn global(mut self, g: WasmGlobal) -> Self {
        self.globals.push(g);
        self
    }

    pub fn memory(mut self, mem: WasmMemory) -> Self {
        self.memory = Some(mem);
        self
    }

    pub fn data_segment(mut self, seg: WasmDataSegment) -> Self {
        self.data_segments.push(seg);
        self
    }

    pub fn function(mut self, f: WasmFunction) -> Self {
        self.functions.push(f);
        self
    }

    pub fn export(mut self, e: WasmExport) -> Self {
        self.exports.push(e);
        self
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_val_type_as_str() {
        assert_eq!(WasmValType::I32.as_str(), "i32");
        assert_eq!(WasmValType::I64.as_str(), "i64");
    }

    #[test]
    fn wasm_function_builder() {
        let f = WasmFunction::new("main")
            .param(WasmValType::I32)
            .result(WasmValType::I32)
            .local(WasmValType::I32)
            .body(vec![
                WasmInstr::I32Const(42),
                WasmInstr::LocalSet(0),
                WasmInstr::LocalGet(0),
            ]);

        assert_eq!(f.symbol, "main");
        assert_eq!(f.params, vec![WasmValType::I32]);
        assert_eq!(f.results, vec![WasmValType::I32]);
        assert_eq!(f.locals, vec![WasmValType::I32]);
    }

    #[test]
    fn wasm_module_builder() {
        let imp = WasmImport::func("host", "log", "$host_log", [WasmValType::I32], []);
        let g = WasmGlobal::i32_mut("$counter", 0);
        let mem = WasmMemory::exported(1, 256, "memory");
        let f = WasmFunction::new("start").body(vec![
            WasmInstr::GlobalGet("$counter".to_owned()),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::GlobalSet("$counter".to_owned()),
        ]);
        let e = WasmExport::func("start", "start");

        let module = WasmModule::new()
            .import(imp)
            .global(g)
            .memory(mem)
            .function(f)
            .export(e);

        assert_eq!(module.imports.len(), 1);
        assert_eq!(module.globals.len(), 1);
        assert!(module.memory.is_some());
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.exports.len(), 1);
    }

    #[test]
    fn wasm_module_constructors_keep_wat_inputs_typed() {
        let imp = WasmImport::from_catalog_type_specs(
            "wasi_snapshot_preview1",
            "fd_write",
            "$fd_write",
            "param i32 i32 i32 i32",
            "result i32",
        );
        let mem = WasmMemory::new(1, 2);
        let exported_mem = WasmMemory::exported(1, 4, "memory");
        let data = WasmDataSegment::new(32, b"stdout".to_vec());
        let func_export = WasmExport::func("main", "main");
        let memory_export = WasmExport::memory("memory");

        assert_eq!(imp.module, "wasi_snapshot_preview1");
        assert_eq!(imp.name, "fd_write");
        assert_eq!(imp.func_symbol, "$fd_write");
        assert_eq!(imp.params, vec![WasmValType::I32; 4]);
        assert_eq!(imp.results, vec![WasmValType::I32]);
        assert_eq!(mem.export_name, None);
        assert_eq!(exported_mem.export_name, Some("memory".to_owned()));
        assert_eq!(data.offset, 32);
        assert_eq!(data.data, b"stdout");
        assert!(matches!(func_export.kind, WasmExportKind::Func(ref symbol) if symbol == "main"));
        assert!(matches!(memory_export.kind, WasmExportKind::Memory));
    }
}
