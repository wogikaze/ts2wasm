//! Static facts — inferred properties of lowered expressions.
//!
//! This module defines `StaticFacts`, which collects all the static analysis
//! facts that the Resolver tracks about locals during lowering. These facts
//! enable optimizations such as:
//! - Identifying known array locals for fast-path runtime calls
//! - Tracking bigint locals for division/remainder precision
//! - Tracking static object literals for copy-safe property access
//! - Identifying nullish, string literal, and regexp locals
//!
//! Current scope:
//! - `Facts` struct from resolver/mod.rs
//! - `Captures` struct from resolver/mod.rs (env_cell/heap_closure tracking)

use std::collections::{HashMap, HashSet};

use crate::builtin_resolved::{
    ResolvedArrayElement, ResolvedExpr, ResolvedObjectProp, ResolvedStmt,
};
use crate::lowered::{ClosureRepresentation, FuncId, LocalId, LoweredExpr};
use ts2wasm_source::Span;

/// Static analysis facts tracked during lowering.
///
/// These are the inferred properties of locals that enable optimization
/// decisions (e.g., using ArrayPush for known arrays, selecting BigInt
/// division/remainder precision based on control flow).
pub struct StaticFacts {
    // ------------------------------------------------------------------
    // From resolver/mod.rs `Facts`:
    // ------------------------------------------------------------------
    /// Heap-allocated closure locals (for heap-closure calling convention).
    pub heap_closure_locals: HashSet<LocalId>,
    /// Nullish locals (known to be null or undefined).
    pub nullish_locals: HashSet<LocalId>,
    /// Locals holding RegExp literal strings.
    pub regexp_literal_locals: HashSet<LocalId>,
    /// Locals holding invalid Date instances.
    pub invalid_date_locals: HashSet<LocalId>,
    /// Locals holding Date instances (valid or invalid).
    pub date_locals: HashSet<LocalId>,
    /// Locals holding bigint values.
    pub bigint_locals: HashSet<LocalId>,
    /// Locals holding bigint values that need division/remainder precision tracking.
    pub control_flow_bigint_div_rem_locals: HashSet<LocalId>,
    /// Locals with mixed bigint precision from branch merging.
    pub control_flow_mixed_bigint_locals: HashSet<LocalId>,
    /// Locals known to be dense arrays.
    pub array_locals: HashSet<LocalId>,
    /// Static array slot contents: local → slots.
    pub static_array_slots: HashMap<LocalId, Vec<ResolvedArrayElement>>,
    /// Locals with Symbol.iterator property (for custom iteration).
    pub symbol_iterator_object_locals: HashSet<LocalId>,
    /// Locals known to hold symbol values.
    pub symbol_value_locals: HashSet<LocalId>,
    /// Static Symbol descriptions for symbol locals; `None` means Symbol().
    pub symbol_description_locals: HashMap<LocalId, Option<String>>,
    /// Locals holding Array iterator objects returned by values/keys/entries.
    pub array_iterator_locals: HashSet<LocalId>,
    /// Locals holding generator iterator objects returned by generator calls.
    pub generator_iterator_locals: HashSet<LocalId>,
    /// Statically collected yield values for simple top-level generator functions.
    pub generator_function_yields: HashMap<String, Vec<ResolvedExpr>>,
    /// Statically collected lazy resume steps for simple top-level generator functions.
    pub generator_function_steps: HashMap<String, Vec<GeneratorYieldStep>>,
    /// Statically collected statements that run when a generator resumes to completion.
    pub generator_function_completion_steps: HashMap<String, Vec<ResolvedStmt>>,
    /// Specialized static plans for generator object literals whose computed keys resume from yields.
    pub generator_function_object_resume_plans: HashMap<String, GeneratorObjectResumePlan>,
    /// Locals holding statically visible generator iterators with a runtime state local.
    pub generator_iterator_bindings: HashMap<LocalId, GeneratorIteratorBinding>,
    /// Locals holding statically visible object generator method iterators.
    pub generator_method_iterator_bindings: HashMap<LocalId, GeneratorMethodIteratorBinding>,
    /// Static object literal contents: local → property records.
    pub static_object_literal_locals: HashMap<LocalId, Vec<ResolvedObjectProp>>,
    /// Alias source tracking for static object literals: alias → source_ids.
    pub static_object_literal_alias_sources: HashMap<LocalId, HashSet<LocalId>>,
    /// Locals that are function-like arrays (e.g., `arguments` based on function params).
    pub static_function_array_like_locals: HashMap<LocalId, StaticFunctionArrayLike>,
    /// Locals known to be string literals (with their value).
    pub string_literal_locals: HashMap<LocalId, String>,
    /// Locals known to be number literals as exact source decimal strings.
    pub number_literal_locals: HashMap<LocalId, String>,
    /// Locals that are native Set.add method references.
    pub native_set_add_locals: HashSet<LocalId>,
    /// Generator function names (for call-site resolution).
    pub generator_function_names: HashSet<String>,

    // ------------------------------------------------------------------
    // From resolver/mod.rs `Captures`:
    // ------------------------------------------------------------------
    /// Names that require environment cell wrappers (for mutable captures).
    pub env_cell_names: HashSet<String>,
    /// Locals that have been wrapped in environment cells.
    pub env_cell_locals: HashSet<LocalId>,
    /// Environment-cell locals whose cell object has been initialized.
    pub initialized_env_cell_locals: HashSet<LocalId>,
    /// Names that use heap-allocated closure representation.
    pub heap_closure_names: HashSet<String>,
    /// Arrow closure locals: local_id → ArrowClosure (for inline arrow fn expansion).
    pub arrow_locals: HashMap<LocalId, ArrowClosure>,
    /// Static ECMAScript `name` metadata for function-token locals when it differs from the binding.
    pub function_metadata_name_locals: HashMap<LocalId, String>,
    /// Locals holding static `Function(...)` generated functions that are constructable.
    pub constructable_function_locals: HashSet<LocalId>,
    /// Static `Function.prototype.bind` locals that can be expanded at call sites.
    pub bound_function_locals: HashMap<LocalId, BoundFunction>,
    /// Static `Function.prototype.call/apply.bind(fn)` locals expanded at call sites.
    pub function_method_locals: HashMap<LocalId, FunctionMethodBinding>,
    /// Locals holding host function handles returned by dynamic `Function(...)`.
    pub host_function_handle_locals: HashSet<LocalId>,
    /// Locals holding values returned from host function handles.
    pub host_external_object_locals: HashSet<LocalId>,
    /// Static bound class constructor locals created with `ClassName.bind(...)`.
    pub bound_constructor_locals: HashMap<LocalId, BoundConstructor>,
    /// Static Proxy locals created as `new Proxy(target, handler)`.
    pub proxy_locals: HashMap<LocalId, ProxyBinding>,
    /// Static Intl.NumberFormat locals with constructor options visible at compile time.
    pub intl_number_format_locals: HashMap<LocalId, IntlNumberFormatOptions>,
    /// Static Intl.DateTimeFormat locals with constructor options visible at compile time.
    pub intl_date_time_format_locals: HashMap<LocalId, IntlDateTimeFormatOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostExternalKind {
    FunctionHandle,
    Object,
}

impl StaticFacts {
    pub fn mark_host_external(&mut self, local_id: LocalId, kind: HostExternalKind, present: bool) {
        let set = match kind {
            HostExternalKind::FunctionHandle => &mut self.host_function_handle_locals,
            HostExternalKind::Object => &mut self.host_external_object_locals,
        };
        if present {
            set.insert(local_id);
        } else {
            set.remove(&local_id);
        }
    }

    pub fn is_host_external(&self, local_id: LocalId, kind: HostExternalKind) -> bool {
        match kind {
            HostExternalKind::FunctionHandle => {
                self.host_function_handle_locals.contains(&local_id)
            }
            HostExternalKind::Object => self.host_external_object_locals.contains(&local_id),
        }
    }
}

/// Tracks the known elements of a function-parameter-based array-like value
/// (e.g., `function(a, b, c) { ... }` where we track assignments to indices).
#[derive(Debug, Clone)]
pub struct StaticFunctionArrayLike {
    pub elements: Vec<Option<ResolvedArrayElement>>,
}

/// A straight-line chunk of generator body that executes before one yield.
#[derive(Debug, Clone)]
pub struct GeneratorYieldStep {
    pub statements: Vec<ResolvedStmt>,
    pub value: ResolvedExpr,
}

/// A generator body of the form `target = { [yield]: ... }` whose object
/// construction can resume with `.next(value)` arguments.
#[derive(Debug, Clone)]
pub struct GeneratorObjectResumePlan {
    pub target: String,
    pub props: Vec<ResolvedObjectProp>,
    pub yield_values: Vec<ResolvedExpr>,
}

/// Compile-time state for a statically visible generator iterator local.
#[derive(Debug, Clone)]
pub struct GeneratorIteratorBinding {
    pub func_name: String,
    pub state_local: LocalId,
    pub resume_args: Vec<ResolvedExpr>,
}

/// Compile-time state for a statically visible object generator method iterator local.
#[derive(Debug, Clone)]
pub struct GeneratorMethodIteratorBinding {
    pub func_id: FuncId,
    pub receiver_local: LocalId,
    pub args: Vec<ResolvedExpr>,
    pub state_local: LocalId,
}

/// Tracks an arrow function closure that can be inlined or heap-allocated.
#[derive(Debug, Clone)]
pub struct ArrowClosure {
    pub func_id: FuncId,
    pub captures: Vec<LocalId>,
}

/// Tracks a statically known bound function local.
#[derive(Debug, Clone)]
pub struct BoundFunction {
    pub func_id: FuncId,
    pub receiver: ResolvedExpr,
    pub bound_args: Vec<ResolvedExpr>,
}

/// Tracks a statically known bound class constructor.
#[derive(Debug, Clone)]
pub struct BoundConstructor {
    pub class_name: String,
    pub bound_args: Vec<ResolvedExpr>,
}

/// Tracks a statically known Function.prototype method bound to a target function.
#[derive(Debug, Clone)]
pub struct FunctionMethodBinding {
    pub func_id: FuncId,
    pub kind: FunctionMethodKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionMethodKind {
    Call,
    Apply,
}

/// Tracks the statically visible target and handler for a Proxy local.
#[derive(Debug, Clone)]
pub struct ProxyBinding {
    pub target: ResolvedExpr,
    pub handler: ResolvedExpr,
}

/// Statically visible Intl.NumberFormat constructor options.
#[derive(Debug, Clone)]
pub struct IntlNumberFormatOptions {
    pub locale: String,
    pub style: String,
    pub currency: String,
    pub notation: String,
    pub compact_display: String,
    pub sign_display: String,
}

/// Statically visible Intl.DateTimeFormat constructor options.
#[derive(Debug, Clone)]
pub struct IntlDateTimeFormatOptions {
    pub locale: String,
    pub time_zone: String,
    pub locale_matcher: String,
}

/// Static Proxy trap lowering kinds for the supported MVP trap slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyTrapKind {
    ProxyGet,
    ProxySet,
    ProxyHas,
    ProxyDeleteProperty,
    ProxyConstruct,
    ProxyApply,
    ProxyGetPrototypeOf,
    ProxySetPrototypeOf,
    ProxyIsExtensible,
    ProxyPreventExtensions,
    ProxyGetOwnPropertyDescriptor,
    ProxyDefineProperty,
    ProxyOwnKeys,
    Named(&'static str),
}

impl ProxyTrapKind {
    pub fn method_name(self) -> &'static str {
        match self {
            Self::ProxyGet => "get",
            Self::ProxySet => "set",
            Self::ProxyHas => "has",
            Self::ProxyDeleteProperty => "deleteProperty",
            Self::ProxyConstruct => "construct",
            Self::ProxyApply => "apply",
            Self::ProxyGetPrototypeOf => "getPrototypeOf",
            Self::ProxySetPrototypeOf => "setPrototypeOf",
            Self::ProxyIsExtensible => "isExtensible",
            Self::ProxyPreventExtensions => "preventExtensions",
            Self::ProxyGetOwnPropertyDescriptor => "getOwnPropertyDescriptor",
            Self::ProxyDefineProperty => "defineProperty",
            Self::ProxyOwnKeys => "ownKeys",
            Self::Named(name) => name,
        }
    }
}

impl ArrowClosure {
    /// Convert this closure to a lowered expression with the given representation.
    pub fn to_expr(&self, representation: ClosureRepresentation) -> LoweredExpr {
        LoweredExpr::ArrowFn {
            func_id: self.func_id,
            captures: self.captures.clone(),
            representation,
            span: Span::generated("arrow_fn"),
        }
    }
}

impl StaticFacts {
    /// Create a new empty StaticFacts.
    pub fn new() -> Self {
        Self {
            heap_closure_locals: HashSet::new(),
            nullish_locals: HashSet::new(),
            regexp_literal_locals: HashSet::new(),
            invalid_date_locals: HashSet::new(),
            date_locals: HashSet::new(),
            bigint_locals: HashSet::new(),
            control_flow_bigint_div_rem_locals: HashSet::new(),
            control_flow_mixed_bigint_locals: HashSet::new(),
            array_locals: HashSet::new(),
            static_array_slots: HashMap::new(),
            symbol_iterator_object_locals: HashSet::new(),
            symbol_value_locals: HashSet::new(),
            symbol_description_locals: HashMap::new(),
            array_iterator_locals: HashSet::new(),
            generator_iterator_locals: HashSet::new(),
            generator_function_yields: HashMap::new(),
            generator_function_steps: HashMap::new(),
            generator_function_completion_steps: HashMap::new(),
            generator_function_object_resume_plans: HashMap::new(),
            generator_iterator_bindings: HashMap::new(),
            generator_method_iterator_bindings: HashMap::new(),
            static_object_literal_locals: HashMap::new(),
            static_object_literal_alias_sources: HashMap::new(),
            static_function_array_like_locals: HashMap::new(),
            string_literal_locals: HashMap::new(),
            number_literal_locals: HashMap::new(),
            native_set_add_locals: HashSet::new(),
            generator_function_names: HashSet::new(),
            env_cell_names: HashSet::new(),
            env_cell_locals: HashSet::new(),
            initialized_env_cell_locals: HashSet::new(),
            heap_closure_names: HashSet::new(),
            arrow_locals: HashMap::new(),
            function_metadata_name_locals: HashMap::new(),
            constructable_function_locals: HashSet::new(),
            bound_function_locals: HashMap::new(),
            function_method_locals: HashMap::new(),
            host_function_handle_locals: HashSet::new(),
            host_external_object_locals: HashSet::new(),
            bound_constructor_locals: HashMap::new(),
            proxy_locals: HashMap::new(),
            intl_number_format_locals: HashMap::new(),
            intl_date_time_format_locals: HashMap::new(),
        }
    }

    /// Create a StaticFacts with pre-populated capture and generator names.
    pub fn with_captures(
        env_cell_names: HashSet<String>,
        heap_closure_names: HashSet<String>,
        generator_function_names: HashSet<String>,
    ) -> Self {
        let mut facts = Self::new();
        facts.env_cell_names = env_cell_names;
        facts.heap_closure_names = heap_closure_names;
        facts.generator_function_names = generator_function_names;
        facts
    }

    /// Create a StaticFacts with pre-populated capture, generator, and arrow closure data.
    pub fn with_facts(
        env_cell_names: HashSet<String>,
        heap_closure_names: HashSet<String>,
        generator_function_names: HashSet<String>,
        arrow_locals: HashMap<LocalId, ArrowClosure>,
    ) -> Self {
        let mut facts = Self::new();
        facts.env_cell_names = env_cell_names;
        facts.heap_closure_names = heap_closure_names;
        facts.generator_function_names = generator_function_names;
        facts.arrow_locals = arrow_locals;
        facts
    }

    /// Check if a local is a known array.
    pub fn is_array(&self, local: LocalId) -> bool {
        self.array_locals.contains(&local)
    }

    /// Check if a local is a known bigint.
    pub fn is_bigint(&self, local: LocalId) -> bool {
        self.bigint_locals.contains(&local)
    }

    /// Check if a local is nullish (null or undefined).
    pub fn is_nullish(&self, local: LocalId) -> bool {
        self.nullish_locals.contains(&local)
    }

    /// Check if a local is an environment cell.
    pub fn is_env_cell(&self, local: LocalId) -> bool {
        self.env_cell_locals.contains(&local)
    }

    /// Check if a local is a heap closure.
    pub fn is_heap_closure(&self, local: LocalId) -> bool {
        self.heap_closure_locals.contains(&local)
    }

    /// Get the known string value of a string literal local.
    pub fn string_value(&self, local: LocalId) -> Option<&String> {
        self.string_literal_locals.get(&local)
    }

    /// Check if a name is in the env_cell_names set.
    pub fn needs_env_cell(&self, name: &str) -> bool {
        self.env_cell_names.contains(name)
    }

    /// Check if a name is in the heap_closure_names set.
    pub fn needs_heap_closure(&self, name: &str) -> bool {
        self.heap_closure_names.contains(name)
    }
}

impl Default for StaticFacts {
    fn default() -> Self {
        Self::new()
    }
}
