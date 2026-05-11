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

use super::runtime_fn::HostImportSpec;

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
    pub fn from_host_spec(spec: &HostImportSpec) -> Self {
        Self {
            module: spec.module.to_owned(),
            name: spec.name.to_owned(),
            func_symbol: spec.wat_symbol.to_owned(),
            params: parse_catalog_type_list(spec.params),
            results: parse_catalog_type_list(spec.result),
        }
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

/// An export declaration.
#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub kind: WasmExportKind,
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
// Runtime signature validation
// ---------------------------------------------------------------------------

use super::runtime_fn::{RuntimeFn, RuntimeSignature};

/// Validate that a runtime call's expected stack effect matches the
/// function's declared signature.
///
/// # Panics
///
/// Panics if `expected_params` or `expected_results` differ from
/// `runtime_fn.stack_effect()`.
pub fn check_runtime_signature(
    runtime_fn: RuntimeFn,
    expected_params: usize,
    expected_results: usize,
) {
    let sig = runtime_fn.stack_effect();
    assert_eq!(
        sig.params, expected_params,
        "RuntimeFn {:?}: expected {} params, declared {}",
        runtime_fn, expected_params, sig.params,
    );
    assert_eq!(
        sig.results, expected_results,
        "RuntimeFn {:?}: expected {} results, declared {}",
        runtime_fn, expected_results, sig.results,
    );
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
                WasmInstr::LocalGet(0),
                WasmInstr::I32Const(42),
                WasmInstr::I32Add,
                WasmInstr::Return,
            ]);

        assert_eq!(f.symbol, "main");
        assert_eq!(f.params.len(), 1);
        assert_eq!(f.results.len(), 1);
        assert_eq!(f.locals.len(), 1);
        assert_eq!(f.body.len(), 4);
    }

    #[test]
    fn wasm_module_builder() {
        let imp = WasmImport {
            module: "host".to_owned(),
            name: "log".to_owned(),
            func_symbol: "$host_log".to_owned(),
            params: vec![WasmValType::I32],
            results: vec![],
        };
        let g = WasmGlobal::i32_mut("$counter", 0);
        let mem = WasmMemory {
            min_pages: 1,
            max_pages: 256,
            export_name: Some("memory".to_owned()),
        };
        let f = WasmFunction::new("start").body(vec![
            WasmInstr::GlobalGet("$counter".to_owned()),
            WasmInstr::I32Const(1),
            WasmInstr::I32Add,
            WasmInstr::GlobalSet("$counter".to_owned()),
        ]);
        let e = WasmExport {
            name: "start".to_owned(),
            kind: WasmExportKind::Func("start".to_owned()),
        };

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
    fn wasm_import_from_host_spec() {
        let spec = HostImportSpec {
            module: "wasi_snapshot_preview1",
            name: "fd_write",
            wat_symbol: "$fd_write",
            abi: crate::runtime_fn::HostAbi::WasiPreview1,
            params: "param i32 i32 i32 i32",
            result: "result i32",
        };
        let imp = WasmImport::from_host_spec(&spec);
        assert_eq!(imp.module, "wasi_snapshot_preview1");
        assert_eq!(imp.name, "fd_write");
        assert_eq!(imp.func_symbol, "$fd_write");
        assert_eq!(imp.params, vec![WasmValType::I32; 4]);
        assert_eq!(imp.results, vec![WasmValType::I32]);
    }

    #[test]
    fn runtime_signature_stack_effect_basics() {
        // 1 param, 1 result (common case)
        assert_eq!(
            RuntimeFn::TruthyBool.stack_effect(),
            RuntimeSignature {
                params: 1,
                results: 1
            },
        );
        // 0 params, 1 result
        assert_eq!(
            RuntimeFn::PrivateBrandTypeError.stack_effect(),
            RuntimeSignature {
                params: 0,
                results: 1
            },
        );
        // 2 params, 1 result
        assert_eq!(
            RuntimeFn::ArrayGet.stack_effect(),
            RuntimeSignature {
                params: 2,
                results: 1
            },
        );
        // 3 params, 1 result
        assert_eq!(
            RuntimeFn::PropertyGet.stack_effect(),
            RuntimeSignature {
                params: 3,
                results: 1
            },
        );
        // 4 params, 1 result
        assert_eq!(
            RuntimeFn::PropertySet.stack_effect(),
            RuntimeSignature {
                params: 4,
                results: 1
            },
        );
        // 6 params, 1 result
        assert_eq!(
            RuntimeFn::MakeBigIntLiteral.stack_effect(),
            RuntimeSignature {
                params: 6,
                results: 1
            },
        );
        // 1 param, 0 results
        assert_eq!(
            RuntimeFn::ModuleExportsAssign.stack_effect(),
            RuntimeSignature {
                params: 1,
                results: 0
            },
        );
        // 3 params, 0 results
        assert_eq!(
            RuntimeFn::ModuleExportsSet.stack_effect(),
            RuntimeSignature {
                params: 3,
                results: 0
            },
        );
    }

    #[test]
    fn runtime_signature_check_pass() {
        check_runtime_signature(RuntimeFn::TruthyBool, 1, 1);
        check_runtime_signature(RuntimeFn::ArrayGet, 2, 1);
        check_runtime_signature(RuntimeFn::PropertySet, 4, 1);
        check_runtime_signature(RuntimeFn::ModuleExportsAssign, 1, 0);
    }

    #[test]
    #[should_panic(expected = "PropertySet: expected 3 params, declared 4")]
    fn runtime_signature_check_fail_params() {
        check_runtime_signature(RuntimeFn::PropertySet, 3, 1);
    }

    #[test]
    #[should_panic(expected = "TruthyBool: expected 0 results, declared 1")]
    fn runtime_signature_check_fail_results() {
        check_runtime_signature(RuntimeFn::TruthyBool, 1, 0);
    }
}
