use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{DiagCode, Diagnostic};
use ts2wasm_ir::lowered::{
    BuiltinErrorConstructor, ClassPrototypeRef, FuncId, LocalId, LoweredProgram,
};
use ts2wasm_runtime_abi::Layout;

use super::runtime_fn::{RuntimeFn, StringOrigin};
use super::runtime_link_plan::RuntimeLinkPlan;
use super::wat_writer::{WatModuleBuilder, WatWriter};

mod class_prototypes;
mod functions;
mod gc_roots;
mod initializers;
mod strings;

pub(crate) fn emit_wat(program: &LoweredProgram) -> Result<String, Diagnostic> {
    WatEmitter::new(program).emit()
}

pub(super) struct WatEmitter<'a> {
    pub(super) program: &'a LoweredProgram,
    pub(super) link_plan: RuntimeLinkPlan,
    pub(super) strings: HashMap<String, u32>,
    pub(super) string_data: Vec<(u32, String, StringOrigin)>,
    pub(super) next_data_offset: u32,
    /// Set of strings that were interned as runtime-originated (from RuntimeFn spec).
    pub(super) runtime_string_set: HashSet<String>,
    class_name_to_ctor: HashMap<String, FuncId>,
    method_counts: HashMap<FuncId, usize>,
    /// Tracks whether we are currently emitting an async function body.
    pub(super) current_is_async: Cell<bool>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LocalFrame {
    pub(super) user_local_count: usize,
    pub(super) backend_base: usize,
    gc_roots: GcRootStorage,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum GcRootStorage {
    Disabled,
    StaticTable { base_slot: usize },
    ActivationFrame,
}

impl LocalFrame {
    const BACKEND_TEMP_GROUP_SIZE: usize = 3;
    const BACKEND_TEMP_GROUP_COUNT: usize = 4;
    /// Number of additional locals for Completion Record fields
    /// (cr_status, cr_value, cr_target, cr_saved_status, cr_saved_value).
    /// These are placed after backend temps.
    /// Saved slots (+3, +4) are used only in try-finally for CR save/restore.
    const CR_LOCAL_COUNT: usize = 5;

    pub(super) fn new(user_local_count: usize, gc_root_base_slot: Option<usize>) -> Self {
        Self {
            user_local_count,
            backend_base: user_local_count,
            gc_roots: gc_root_base_slot
                .map(|base_slot| GcRootStorage::StaticTable { base_slot })
                .unwrap_or(GcRootStorage::Disabled),
        }
    }

    pub(super) fn activation(user_local_count: usize, enabled: bool) -> Self {
        Self {
            user_local_count,
            backend_base: user_local_count,
            gc_roots: if enabled {
                GcRootStorage::ActivationFrame
            } else {
                GcRootStorage::Disabled
            },
        }
    }

    pub(super) const fn total_local_count(self) -> usize {
        self.user_local_count + self.backend_local_count() + Self::CR_LOCAL_COUNT
    }

    pub(super) const fn backend_local_count(self) -> usize {
        Self::BACKEND_TEMP_GROUP_SIZE * Self::BACKEND_TEMP_GROUP_COUNT
    }

    /// Base local index for Completion Record locals: cr_status, cr_value, cr_target.
    ///
    /// These are placed after all backend temporaries (outside the rotating temp pool).
    /// Indices: cr_status=[base], cr_value=[base+1], cr_target=[base+2].
    pub(super) const fn cr_local_base(self) -> usize {
        self.user_local_count + self.backend_local_count()
    }

    /// Base local index for saved Completion Record slots (saved_status, saved_value).
    /// Used only in try-finally to save CR before the finally block runs.
    /// Indices: saved_status=[base+3], saved_value=[base+4].
    pub(super) const fn cr_save_local_base(self) -> usize {
        self.cr_local_base() + 3
    }

    pub(super) const fn heap_base_tmp(self) -> usize {
        self.backend_base
    }

    pub(super) const fn heap_value_tmp(self) -> usize {
        self.backend_base + 1
    }

    pub(super) const fn switch_value_tmp(self) -> usize {
        self.backend_base + 2
    }

    pub(super) fn child_temp_frame(self) -> Self {
        let next_base = self.backend_base + Self::BACKEND_TEMP_GROUP_SIZE;
        let max_base =
            self.user_local_count + self.backend_local_count() - Self::BACKEND_TEMP_GROUP_SIZE;
        Self {
            backend_base: next_base.min(max_base),
            ..self
        }
    }

    pub(super) fn gc_root_slot(self, local_id: LocalId) -> Option<usize> {
        self.gc_root_slot_for_index(local_id.0)
    }

    pub(super) fn gc_root_slot_for_index(self, local_index: usize) -> Option<usize> {
        if local_index >= self.total_local_count() {
            return None;
        }
        match self.gc_roots {
            GcRootStorage::Disabled | GcRootStorage::ActivationFrame => None,
            GcRootStorage::StaticTable { base_slot } => Some(base_slot + local_index),
        }
    }

    pub(super) fn gc_root_slots(self) -> impl Iterator<Item = (usize, usize)> {
        let base_slot = match self.gc_roots {
            GcRootStorage::StaticTable { base_slot } => Some(base_slot),
            GcRootStorage::Disabled | GcRootStorage::ActivationFrame => None,
        };
        base_slot.into_iter().flat_map(move |base| {
            (0..self.total_local_count()).map(move |local| (local, base + local))
        })
    }

    pub(super) fn uses_activation_roots(self) -> bool {
        matches!(self.gc_roots, GcRootStorage::ActivationFrame)
    }
}

impl<'a> WatEmitter<'a> {
    pub(super) fn new(program: &'a LoweredProgram) -> Self {
        let link_plan = RuntimeLinkPlan::from_program(program);
        let mut class_name_to_ctor = HashMap::new();
        let mut method_counts = HashMap::new();
        Self::compute_class_decl_metadata(
            &program.top_level_statements,
            &mut class_name_to_ctor,
            &mut method_counts,
        );
        let mut emitter = Self {
            program,
            link_plan,
            strings: HashMap::new(),
            string_data: Vec::new(),
            runtime_string_set: HashSet::new(),
            next_data_offset: Layout::DATA_START,
            class_name_to_ctor,
            method_counts,
            current_is_async: Cell::new(false),
        };
        emitter.intern_required_runtime_strings();
        emitter.collect_program_strings(&program.top_level_statements);
        for function in &program.functions {
            emitter.collect_program_strings(&function.body);
        }
        for module in &program.modules {
            emitter.collect_program_strings(&module.statements);
        }
        emitter
    }

    fn emit(mut self) -> Result<String, Diagnostic> {
        self.validate_memory_layout()?;
        let _required_capabilities = self.link_plan.required_capabilities();
        let mut writer = WatWriter::new();
        writer.open_module();

        // Buffer for methods that still output to &mut String (cross-file API).
        let mut buf = String::new();

        // emit_imports_from_catalog uses WatModuleBuilder internally.
        self.emit_imports_from_catalog(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        writer.line_fmt(
            2,
            format_args!(
                "(memory (export \"memory\") {} {})",
                Layout::MEMORY_MIN_PAGES,
                Layout::MEMORY_MAX_PAGES,
            ),
        );
        writer.line_fmt(
            2,
            format_args!(
                "(global $heap (mut i32) (i32.const {}))",
                Layout::HEAP_START,
            ),
        );

        // emit_required_globals uses WatModuleBuilder internally.
        self.emit_required_globals(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        self.emit_class_prototype_globals(&mut writer);
        self.emit_builtin_error_prototype_globals(&mut writer);

        self.emit_data_segments(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        self.emit_runtime(&mut buf);
        writer.push_str(&buf);
        buf.clear();

        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::SetFromArray)
        {
            self.emit_set_add_dispatcher(&mut writer);
        }
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::JsonStringify)
        {
            self.emit_json_replacer_dispatcher(&mut writer);
        }

        self.emit_functions(&mut writer);

        self.emit_module_initializers(&mut writer);

        self.emit_start(&mut writer);

        writer.close_module();
        Ok(writer.into_string())
    }

    fn validate_memory_layout(&self) -> Result<(), Diagnostic> {
        if self.next_data_offset > Layout::SCRATCH_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "static data segment ({}) overlaps scratch buffer ({})",
                    self.next_data_offset,
                    Layout::SCRATCH_OFFSET
                ),
                span: None,

                phase: None,
            });
        }
        if self.next_data_offset < Layout::DATA_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "static data end ({}) is below data start ({})",
                    self.next_data_offset,
                    Layout::DATA_START
                ),
                span: None,

                phase: None,
            });
        }
        let scratch_end = Layout::SCRATCH_OFFSET
            .checked_add(Layout::SCRATCH_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "scratch range overflow while validating memory layout".to_owned(),
                span: None,

                phase: None,
            })?;
        if scratch_end > Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch range [{}..{}) overlaps heap start ({})",
                    Layout::SCRATCH_OFFSET,
                    scratch_end,
                    Layout::HEAP_START
                ),
                span: None,

                phase: None,
            });
        }
        if scratch_end > Layout::STDIN_BUFFER_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch range [{}..{}) overlaps stdin buffer ({})",
                    Layout::SCRATCH_OFFSET,
                    scratch_end,
                    Layout::STDIN_BUFFER_OFFSET
                ),
                span: None,

                phase: None,
            });
        }
        let stdin_end = Layout::STDIN_BUFFER_OFFSET
            .checked_add(Layout::STDIN_BUFFER_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "stdin range overflow while validating memory layout".to_owned(),
                span: None,

                phase: None,
            })?;
        if stdin_end > Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin range [{}..{}) overlaps heap start ({})",
                    Layout::STDIN_BUFFER_OFFSET,
                    stdin_end,
                    Layout::HEAP_START
                ),
                span: None,

                phase: None,
            });
        }
        if Layout::SCRATCH_OFFSET >= Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "scratch buffer ({}) must be below heap start ({})",
                    Layout::SCRATCH_OFFSET,
                    Layout::HEAP_START
                ),
                span: None,

                phase: None,
            });
        }
        if Layout::STDIN_BUFFER_OFFSET >= Layout::HEAP_START {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin buffer ({}) must be below heap start ({})",
                    Layout::STDIN_BUFFER_OFFSET,
                    Layout::HEAP_START
                ),
                span: None,

                phase: None,
            });
        }
        let stdin_nread_end =
            Layout::STDIN_NREAD_OFFSET
                .checked_add(4)
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "stdin nread region overflow while validating memory layout"
                        .to_owned(),
                    span: None,

                    phase: None,
                })?;
        if stdin_nread_end > Layout::STDIN_BUFFER_OFFSET {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "stdin iovec/nread region [{}..{}) overlaps stdin buffer ({})",
                    Layout::STDIN_IOVEC_OFFSET,
                    stdin_nread_end,
                    Layout::STDIN_BUFFER_OFFSET
                ),
                span: None,

                phase: None,
            });
        }
        if !Layout::HEAP_START.is_multiple_of(Layout::ALIGN) {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "HEAP_START ({}) must be {}-byte aligned for RawValue heap tags",
                    Layout::HEAP_START,
                    Layout::ALIGN
                ),
                span: None,

                phase: None,
            });
        }
        let max_stdin_heap_allocation = Layout::HEAP_START
            .checked_add(Layout::STRING_HEADER_SIZE)
            .and_then(|base| base.checked_add(Layout::STDIN_READ_LIMIT))
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "stdin heap allocation overflow while validating memory layout".to_owned(),
                span: None,

                phase: None,
            })?;
        let initial_memory_bytes = Layout::MEMORY_MIN_PAGES
            .checked_mul(Layout::WASM_PAGE_SIZE)
            .ok_or_else(|| Diagnostic {
                code: DiagCode::InvariantViolation,
                message: "memory page byte size overflow while validating memory layout".to_owned(),
                span: None,

                phase: None,
            })?;
        if max_stdin_heap_allocation > initial_memory_bytes {
            return Err(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "single max stdin heap allocation from HEAP_START ({max_stdin_heap_allocation}) exceeds initial memory bytes ({initial_memory_bytes})"
                ),
                span: None,

                phase: None,
            });
        }
        Ok(())
    }

    fn emit_imports_from_catalog(&self, wat: &mut String) {
        let mut writer = WatModuleBuilder::new();
        for import in self.link_plan.required_imports() {
            let spec = import.spec();
            writer.push_import_func(&spec);
        }
        wat.push_str(&writer.into_inner());
    }

    fn emit_required_globals(&self, wat: &mut String) {
        let mut writer = WatModuleBuilder::new();
        for global in self.link_plan.required_globals() {
            writer.push_global_i32(global.symbol(), global.initial_value());
        }
        wat.push_str(&writer.into_inner());
    }
}

pub(super) fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}

pub(super) fn module_init_symbol(module_id: usize) -> String {
    format!("module_init_{module_id}")
}

pub(super) fn class_prototype_global(id: FuncId) -> String {
    format!("class_proto_{}", id.0)
}

pub(super) fn builtin_error_prototype_global(constructor: BuiltinErrorConstructor) -> &'static str {
    match constructor {
        BuiltinErrorConstructor::Error => "error_proto_error",
        BuiltinErrorConstructor::RangeError => "error_proto_range_error",
        BuiltinErrorConstructor::TypeError => "error_proto_type_error",
        BuiltinErrorConstructor::ReferenceError => "error_proto_reference_error",
        BuiltinErrorConstructor::SyntaxError => "error_proto_syntax_error",
    }
}

pub(super) fn builtin_error_stack_prefix(constructor: BuiltinErrorConstructor) -> &'static str {
    match constructor {
        BuiltinErrorConstructor::Error => "Error: ",
        BuiltinErrorConstructor::RangeError => "RangeError: ",
        BuiltinErrorConstructor::TypeError => "TypeError: ",
        BuiltinErrorConstructor::ReferenceError => "ReferenceError: ",
        BuiltinErrorConstructor::SyntaxError => "SyntaxError: ",
    }
}

fn add_builtin_error_prototype_ref(
    constructor: BuiltinErrorConstructor,
    prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
) {
    let mut current = Some(constructor);
    while let Some(constructor) = current {
        prototypes.insert(constructor);
        current = constructor.parent();
    }
}

fn add_class_prototype_ref(
    prototype: &ClassPrototypeRef,
    prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
) {
    let mut current = prototype.constructor;
    for parent in &prototype.parent_constructors {
        prototypes
            .entry(current)
            .and_modify(|existing| {
                if existing.is_none() {
                    *existing = Some(*parent);
                }
            })
            .or_insert(Some(*parent));
        prototypes.entry(*parent).or_insert(None);
        current = *parent;
    }
    prototypes.entry(current).or_insert(None);
}

fn class_prototype_depth(
    constructor: FuncId,
    prototypes: &BTreeMap<FuncId, Option<FuncId>>,
) -> usize {
    let mut depth = 0;
    let mut current = constructor;
    while let Some(Some(parent)) = prototypes.get(&current) {
        depth += 1;
        current = *parent;
        if depth > prototypes.len() {
            break;
        }
    }
    depth
}
