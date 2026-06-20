//! SpecAlgoIR step types and completion model.

use crate::SpecOp;

/// A typed local variable or value reference in SpecAlgoIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecLocal(pub u32);

/// A control-flow block identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecBlockId(pub u32);

/// ECMAScript completion kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Normal,
    Throw,
    Break,
    Continue,
    Return,
}

/// A completion record carrying a value local.
#[derive(Debug, Clone)]
pub struct Completion {
    pub kind: CompletionKind,
    pub value: SpecLocal,
}

impl Completion {
    pub fn normal(value: SpecLocal) -> Self {
        Self { kind: CompletionKind::Normal, value }
    }
    pub fn throw(value: SpecLocal) -> Self {
        Self { kind: CompletionKind::Throw, value }
    }
}

/// A mapping from old completion kinds to a handler block.
/// Used for try/finally, iterator close, and error recovery.
#[derive(Debug, Clone)]
pub struct CompletionMap {
    pub from: Vec<CompletionKind>,
    pub to: CompletionKind,
    pub handler: SpecBlockId,
}

/// Every ECMAScript algorithm step.
#[derive(Debug, Clone)]
pub enum SpecAlgoStep {
    // ── Property storage primitives ───────────────────────────────────
    OwnPropertyLookup { object: SpecLocal, key: SpecLocal, result_desc: SpecLocal },
    OwnPropertyInsert { object: SpecLocal, key: SpecLocal, desc: SpecLocal },
    OwnPropertyUpdate { object: SpecLocal, key: SpecLocal, desc: SpecLocal },
    OwnPropertyDelete { object: SpecLocal, key: SpecLocal, result: SpecLocal },
    OwnPropertyKeysRaw { object: SpecLocal, result: SpecLocal },
    GetPrototypeSlot { object: SpecLocal, result_proto: SpecLocal },
    SetPrototypeSlot { object: SpecLocal, proto: SpecLocal, result: SpecLocal },
    IsExtensibleBit { object: SpecLocal, result: SpecLocal },
    PreventExtensionsBit { object: SpecLocal },

    // ── Descriptor operations ────────────────────────────────────────
    GetDescriptorValue { desc: SpecLocal, result: SpecLocal },
    SetDescriptorValue { desc: SpecLocal, value: SpecLocal },
    GetDescriptorGetter { desc: SpecLocal, result: SpecLocal },
    GetDescriptorSetter { desc: SpecLocal, result: SpecLocal },
    CreateDataDescriptor {
        value: SpecLocal, writable: bool, enumerable: bool, configurable: bool,
        result: SpecLocal,
    },
    CreateAccessorDescriptor {
        get: SpecLocal, set: SpecLocal, enumerable: bool, configurable: bool,
        result: SpecLocal,
    },

    // ── Type queries ─────────────────────────────────────────────────
    IsCallable { value: SpecLocal, result: SpecLocal },
    IsConstructor { value: SpecLocal, result: SpecLocal },
    IsPropertyKey { value: SpecLocal, result: SpecLocal },
    SameValue { x: SpecLocal, y: SpecLocal, result: SpecLocal },
    SameValueZero { x: SpecLocal, y: SpecLocal, result: SpecLocal },
    IsDataDescriptor { desc: SpecLocal, result: SpecLocal },
    IsAccessorDescriptor { desc: SpecLocal, result: SpecLocal },
    IsGenericDescriptor { desc: SpecLocal, result: SpecLocal },
    IsWritable { desc: SpecLocal, result: SpecLocal },
    IsConfigurable { desc: SpecLocal, result: SpecLocal },
    IsEnumerable { desc: SpecLocal, result: SpecLocal },
    TypeOf { value: SpecLocal, result: SpecLocal },

    // ── Allocation ───────────────────────────────────────────────────
    AllocateObject { result: SpecLocal },
    AllocateArray { result: SpecLocal },
    AllocateFunction { result: SpecLocal },

    // ── Context / environment ────────────────────────────────────────
    EnterExecutionContext { realm: SpecLocal, env: SpecLocal },
    LeaveExecutionContext,
    GetBindingValue { env: SpecLocal, name: String, result: SpecLocal },
    SetMutableBinding { env: SpecLocal, name: String, value: SpecLocal },
    CreateBinding { env: SpecLocal, name: String, mutable: bool },
    InitializeBinding { env: SpecLocal, name: String, value: SpecLocal },
    ResolveBinding { name: String, env: SpecLocal, result: SpecLocal },
    DeleteBinding { env: SpecLocal, name: String, result: SpecLocal },
    HasBinding { env: SpecLocal, name: String, result: SpecLocal },

    // ── Realm / intrinsics / host ────────────────────────────────────
    GetRealmIntrinsic { intrinsic: super::IntrinsicId, result: SpecLocal },
    GetActiveScriptOrModule { result: SpecLocal },
    HostResolveImportedModule {
        referencing_module: SpecLocal, specifier: String, result: SpecLocal,
    },

    // ── Completion / control flow ──────────────────────────────────────
    /// Return with a given completion kind.
    ReturnCompletion { completion: Completion },
    /// Return a normal completion with a value.
    ReturnNormal { value: SpecLocal },
    /// Return a throw completion with an error.
    ReturnThrow { value: SpecLocal },
    /// If the given completion is throw, propagate it. Otherwise unwrap normal.
    ReturnIfAbrupt { completion: SpecLocal, result_normal: SpecLocal },
    /// Branch on condition.
    BranchOnCondition { cond: SpecLocal, then_block: SpecBlockId, else_block: SpecBlockId },
    /// Unconditional jump to block.
    Jump { block: SpecBlockId },
    /// Loop header (back edge target).
    Loop { header: SpecBlockId },

    // ── Try / finally / cleanup ──────────────────────────────────────
    TryBlock { body: SpecBlockId, catch: Option<SpecBlockId>, finally: Option<SpecBlockId> },
    CompletionMapInstall { map: CompletionMap },
    IteratorClose { iterator: SpecLocal, completion: Completion },

    // ── Dispatch ─────────────────────────────────────────────────────
    CallSpecOp { op: SpecOp, args: Vec<SpecLocal>, result: SpecLocal },
    CallBuiltinAlgorithm {
        algorithm: super::BuiltinId, args: Vec<SpecLocal>, result: SpecLocal,
    },
    CallRuntimePrimitive { symbol: String, args: Vec<SpecLocal>, result: SpecLocal },
    CallFunction { callee: SpecLocal, this_arg: SpecLocal, args: Vec<SpecLocal>, result: SpecLocal },

    // ── Iterator ─────────────────────────────────────────────────────
    GetIterator { object: SpecLocal, method: SpecLocal, result: SpecLocal },
    IteratorNext { iterator: SpecLocal, result: SpecLocal },
    IteratorComplete { iter_result: SpecLocal, result: SpecLocal },
    IteratorValue { iter_result: SpecLocal, result: SpecLocal },
    CreateIterResultObject { value: SpecLocal, done: bool, result: SpecLocal },
}
