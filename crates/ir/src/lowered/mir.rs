// Native MIR types — independent data model with LoweredProgram compatibility bridge.
//
// These four core types (MirProgram, MirFunction, MirStmt, MirExpr) are native
// Rust types, not type aliases. They carry the same structure as the Lowered*
// equivalents so that From bridges can convert losslessly.
//
// MirExpr uses LoweredBinaryOp, LoweredUnaryOp, etc. directly for its field
// types rather than creating separate Mir* aliases. The original type names
// are already publicly available from `crate::lowered::*`.

use super::{
    BuiltinErrorConstructor, ClassPrototypeRef, ClosureRepresentation, FuncId, FunctionCallKind,
    LocalId, LoweredArraySlot, LoweredBinaryOp, LoweredExpr, LoweredFunction,
    LoweredLogicalAssignOp, LoweredProgram, LoweredStmt, LoweredUnaryOp, ModuleInfo, RuntimeFn,
};
use ts2wasm_source::Span;

pub type MirArraySlot = LoweredArraySlot;
pub type MirBinaryOp = LoweredBinaryOp;
pub type MirBuiltinErrorConstructor = BuiltinErrorConstructor;
pub type MirClassPrototypeRef = ClassPrototypeRef;
pub type MirClosureRepresentation = ClosureRepresentation;
pub type MirFunctionCallKind = FunctionCallKind;
pub type MirLogicalAssignOp = LoweredLogicalAssignOp;
pub type MirModuleInfo = ModuleInfo;
pub type MirUnaryOp = LoweredUnaryOp;

// ---------------------------------------------------------------------------
// MirExpr — native expression type
// ---------------------------------------------------------------------------

/// A MIR expression node. Structurally equivalent to LoweredExpr.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirExpr {
    Number(i32, Span),
    BigIntLiteral {
        decimal: String,
        sign: i32,
        limb_low: u32,
        limb_high: u32,
        span: Span,
    },
    String(String, Span),
    Bool(bool, Span),
    Null(Span),
    Undefined(Span),
    Local(LocalId, Span),
    EnvCellNew(Box<MirExpr>, Span),
    EnvCellGet(LocalId, Span),
    EnvCellSet {
        cell: LocalId,
        expr: Box<MirExpr>,
        span: Span,
    },
    Unary {
        op: LoweredUnaryOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    Binary {
        left: Box<MirExpr>,
        op: LoweredBinaryOp,
        right: Box<MirExpr>,
        span: Span,
    },
    PropertyIn {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyInDynamic {
        obj: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<MirExpr>,
        span: Span,
    },
    Assign {
        local: LocalId,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalAssign {
        local: LocalId,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalPropertyAssign {
        object: LocalId,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalComputedPropertyAssign {
        object: LocalId,
        key: Box<MirExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalComputedMemberAssign {
        object: Box<MirExpr>,
        key: Box<MirExpr>,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    LogicalMemberAssign {
        object: Box<MirExpr>,
        key: String,
        op: LoweredLogicalAssignOp,
        expr: Box<MirExpr>,
        span: Span,
    },
    ArrayNew {
        elements: Vec<MirExpr>,
        span: Span,
    },
    ArrayNewSparse {
        slots: Vec<LoweredArraySlot>,
        span: Span,
    },
    ArrayGet {
        arr: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    Index {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    GetLength(Box<MirExpr>, Span),
    ObjectNew {
        props: Vec<(String, MirExpr)>,
        non_enumerable: u32,
        span: Span,
    },
    ErrorNew {
        constructor: BuiltinErrorConstructor,
        message: Box<MirExpr>,
        span: Span,
    },
    PropertyGet {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    OptionalPropertyGet {
        obj: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyGetDynamic {
        obj: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    OptionalIndex {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<MirExpr>,
        call: Box<MirExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<MirExpr>,
        method: String,
        span: Span,
    },
    /// Extract the resolved value from a fulfilled Promise.
    PromiseGetValue {
        promise: Box<MirExpr>,
        span: Span,
    },
    RuntimeCall {
        intrinsic: RuntimeFn,
        args: Vec<MirExpr>,
        span: Span,
    },
    PropertySet {
        object: Box<MirExpr>,
        key: String,
        value: Box<MirExpr>,
        span: Span,
    },
    PropertyDelete {
        object: Box<MirExpr>,
        key: String,
        span: Span,
    },
    PropertyDeleteDynamic {
        object: Box<MirExpr>,
        key: Box<MirExpr>,
        span: Span,
    },
    PropertySetDynamic {
        object: Box<MirExpr>,
        index: Box<MirExpr>,
        value: Box<MirExpr>,
        span: Span,
    },
    New {
        constructor: FuncId,
        prototype: ClassPrototypeRef,
        args: Vec<MirExpr>,
        base_local: LocalId,
        private_brand: Option<u32>,
        private_slot_count: usize,
        span: Span,
    },
    ClassPrototype(ClassPrototypeRef, Span),
    BuiltinErrorPrototype(BuiltinErrorConstructor, Span),
    ModuleLoad {
        module_id: usize,
        span: Span,
    },
    Block {
        stmts: Vec<MirStmt>,
        result: Box<MirExpr>,
        span: Span,
    },
    This(Span),
    ArrowFn {
        func_id: FuncId,
        captures: Vec<LocalId>,
        representation: ClosureRepresentation,
        span: Span,
    },
}

// ---------------------------------------------------------------------------
// MirStmt — native statement type
// ---------------------------------------------------------------------------

/// A MIR statement node. Structurally equivalent to LoweredStmt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirStmt {
    Block(Vec<MirStmt>, Span),
    Let(LocalId, MirExpr, Span),
    Assign(LocalId, MirExpr, Span),
    Expr(MirExpr, Span),
    If {
        condition: MirExpr,
        then_body: Vec<MirStmt>,
        else_body: Vec<MirStmt>,
        span: Span,
    },
    While {
        condition: MirExpr,
        body: Vec<MirStmt>,
        span: Span,
    },
    Return(MirExpr, Span),
    Throw(MirExpr, Span),
    TryFinally {
        try_body: Vec<MirStmt>,
        finally_body: Vec<MirStmt>,
        span: Span,
    },
    TryCatch {
        try_body: Vec<MirStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<MirStmt>>,
        finally_body: Option<Vec<MirStmt>>,
        span: Span,
    },
    Switch {
        expr: MirExpr,
        cases: Vec<(Option<MirExpr>, Vec<MirStmt>)>,
        span: Span,
    },
    DoWhile {
        body: Vec<MirStmt>,
        condition: MirExpr,
        span: Span,
    },
    For {
        init: Option<Box<MirStmt>>,
        condition: Option<MirExpr>,
        update: Option<MirExpr>,
        body: Vec<MirStmt>,
        span: Span,
    },
    ForIn {
        var: LocalId,
        iter: MirExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<MirStmt>,
        span: Span,
    },
    ForOf {
        var: LocalId,
        iter: MirExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<MirStmt>,
        span: Span,
    },
    Labeled {
        label: String,
        body: Box<MirStmt>,
        span: Span,
    },
    Break {
        label: Option<String>,
        span: Span,
    },
    Continue {
        label: Option<String>,
        span: Span,
    },
    Export {
        name: String,
        expr: MirExpr,
        span: Span,
    },
    ModuleExportsAssign {
        expr: MirExpr,
        span: Span,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<FuncId>,
        methods: Vec<(String, FuncId)>,
        static_methods: Vec<(String, FuncId)>,
        private_fields: Vec<String>,
        span: Span,
    },
}

// ---------------------------------------------------------------------------
// MirFunction — native function type
// ---------------------------------------------------------------------------

/// A MIR function definition. Structurally equivalent to LoweredFunction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub uses_receiver: bool,
    pub min_required_params: usize,
    pub rest_param_index: Option<usize>,
    pub locals: Vec<LocalId>,
    pub body: Vec<MirStmt>,
    /// Static recursion depth: 0 = not recursive, 1+ = part of a recursive cycle.
    pub recursion_depth: usize,
    pub is_async: bool,
}

// ---------------------------------------------------------------------------
// MirProgram — native program type
// ---------------------------------------------------------------------------

/// A MIR (mid-level IR) program. Structurally equivalent to LoweredProgram.
///
/// The `modules` field uses `ModuleInfo` (the original type) because ModuleInfo
/// carries its own statement type (Vec<LoweredStmt>). A future native MIR
/// pass may replace this with a fully native ModuleInfo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirProgram {
    /// Top-level execution statements (excluding function definitions).
    pub top_level_statements: Vec<MirStmt>,
    /// Local variable IDs used at the top level.
    pub top_level_locals: Vec<LocalId>,
    /// User-defined function bodies.
    pub functions: Vec<MirFunction>,
    /// Module graph lowering information.
    pub modules: Vec<ModuleInfo>,
}

// ---------------------------------------------------------------------------
// Bridge: LoweredProgram -> MirProgram
// ---------------------------------------------------------------------------

fn lower_expr_to_mir(expr: &LoweredExpr) -> MirExpr {
    match expr {
        LoweredExpr::Number(v, span) => MirExpr::Number(*v, *span),
        LoweredExpr::BigIntLiteral {
            decimal,
            sign,
            limb_low,
            limb_high,
            span,
        } => MirExpr::BigIntLiteral {
            decimal: decimal.clone(),
            sign: *sign,
            limb_low: *limb_low,
            limb_high: *limb_high,
            span: *span,
        },
        LoweredExpr::String(s, span) => MirExpr::String(s.clone(), *span),
        LoweredExpr::Bool(b, span) => MirExpr::Bool(*b, *span),
        LoweredExpr::Null(span) => MirExpr::Null(*span),
        LoweredExpr::Undefined(span) => MirExpr::Undefined(*span),
        LoweredExpr::Local(id, span) => MirExpr::Local(*id, *span),
        LoweredExpr::EnvCellNew(expr, span) => {
            MirExpr::EnvCellNew(Box::new(lower_expr_to_mir(expr)), *span)
        }
        LoweredExpr::EnvCellGet(cell, span) => MirExpr::EnvCellGet(*cell, *span),
        LoweredExpr::EnvCellSet { cell, expr, span } => MirExpr::EnvCellSet {
            cell: *cell,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::Unary { op, expr, span } => MirExpr::Unary {
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::Binary {
            left,
            op,
            right,
            span,
        } => MirExpr::Binary {
            left: Box::new(lower_expr_to_mir(left)),
            op: *op,
            right: Box::new(lower_expr_to_mir(right)),
            span: *span,
        },
        LoweredExpr::PropertyIn { obj, key, span } => MirExpr::PropertyIn {
            obj: Box::new(lower_expr_to_mir(obj)),
            key: key.clone(),
            span: *span,
        },
        LoweredExpr::PropertyInDynamic { obj, key, span } => MirExpr::PropertyInDynamic {
            obj: Box::new(lower_expr_to_mir(obj)),
            key: Box::new(lower_expr_to_mir(key)),
            span: *span,
        },
        LoweredExpr::Call { kind, args, span } => MirExpr::Call {
            kind: *kind,
            args: args.iter().map(lower_expr_to_mir).collect(),
            span: *span,
        },
        LoweredExpr::Assign { local, expr, span } => MirExpr::Assign {
            local: *local,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::LogicalAssign {
            local,
            op,
            expr,
            span,
        } => MirExpr::LogicalAssign {
            local: *local,
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr,
            span,
        } => MirExpr::LogicalPropertyAssign {
            object: *object,
            key: key.clone(),
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr,
            span,
        } => MirExpr::LogicalComputedPropertyAssign {
            object: *object,
            key: Box::new(lower_expr_to_mir(key)),
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr,
            span,
        } => MirExpr::LogicalComputedMemberAssign {
            object: Box::new(lower_expr_to_mir(object)),
            key: Box::new(lower_expr_to_mir(key)),
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr,
            span,
        } => MirExpr::LogicalMemberAssign {
            object: Box::new(lower_expr_to_mir(object)),
            key: key.clone(),
            op: *op,
            expr: Box::new(lower_expr_to_mir(expr)),
            span: *span,
        },
        LoweredExpr::ArrayNew { elements, span } => MirExpr::ArrayNew {
            elements: elements.iter().map(lower_expr_to_mir).collect(),
            span: *span,
        },
        LoweredExpr::ArrayNewSparse { slots, span } => MirExpr::ArrayNewSparse {
            slots: slots.clone(),
            span: *span,
        },
        LoweredExpr::ArrayGet { arr, index, span } => MirExpr::ArrayGet {
            arr: Box::new(lower_expr_to_mir(arr)),
            index: Box::new(lower_expr_to_mir(index)),
            span: *span,
        },
        LoweredExpr::Index {
            object,
            index,
            span,
        } => MirExpr::Index {
            object: Box::new(lower_expr_to_mir(object)),
            index: Box::new(lower_expr_to_mir(index)),
            span: *span,
        },
        LoweredExpr::GetLength(expr, span) => {
            MirExpr::GetLength(Box::new(lower_expr_to_mir(expr)), *span)
        }
        LoweredExpr::ObjectNew {
            props,
            non_enumerable,
            span,
        } => {
            let mir_props: Vec<(String, MirExpr)> = props
                .iter()
                .map(|(k, v)| (k.clone(), lower_expr_to_mir(v)))
                .collect();
            MirExpr::ObjectNew {
                props: mir_props,
                non_enumerable: *non_enumerable,
                span: *span,
            }
        }
        LoweredExpr::ErrorNew {
            constructor,
            message,
            span,
        } => MirExpr::ErrorNew {
            constructor: *constructor,
            message: Box::new(lower_expr_to_mir(message)),
            span: *span,
        },
        LoweredExpr::PropertyGet { obj, key, span } => MirExpr::PropertyGet {
            obj: Box::new(lower_expr_to_mir(obj)),
            key: key.clone(),
            span: *span,
        },
        LoweredExpr::OptionalPropertyGet { obj, key, span } => MirExpr::OptionalPropertyGet {
            obj: Box::new(lower_expr_to_mir(obj)),
            key: key.clone(),
            span: *span,
        },
        LoweredExpr::PropertyGetDynamic { obj, key, span } => MirExpr::PropertyGetDynamic {
            obj: Box::new(lower_expr_to_mir(obj)),
            key: Box::new(lower_expr_to_mir(key)),
            span: *span,
        },
        LoweredExpr::OptionalIndex {
            object,
            index,
            span,
        } => MirExpr::OptionalIndex {
            object: Box::new(lower_expr_to_mir(object)),
            index: Box::new(lower_expr_to_mir(index)),
            span: *span,
        },
        LoweredExpr::OptionalCall { callee, call, span } => MirExpr::OptionalCall {
            callee: Box::new(lower_expr_to_mir(callee)),
            call: Box::new(lower_expr_to_mir(call)),
            span: *span,
        },
        LoweredExpr::MethodCall {
            object,
            method,
            span,
        } => MirExpr::MethodCall {
            object: Box::new(lower_expr_to_mir(object)),
            method: method.clone(),
            span: *span,
        },
        LoweredExpr::PromiseGetValue { promise, span } => MirExpr::PromiseGetValue {
            promise: Box::new(lower_expr_to_mir(promise)),
            span: *span,
        },
        LoweredExpr::RuntimeCall {
            intrinsic,
            args,
            span,
        } => MirExpr::RuntimeCall {
            intrinsic: *intrinsic,
            args: args.iter().map(lower_expr_to_mir).collect(),
            span: *span,
        },
        LoweredExpr::PropertySet {
            object,
            key,
            value,
            span,
        } => MirExpr::PropertySet {
            object: Box::new(lower_expr_to_mir(object)),
            key: key.clone(),
            value: Box::new(lower_expr_to_mir(value)),
            span: *span,
        },
        LoweredExpr::PropertyDelete { object, key, span } => MirExpr::PropertyDelete {
            object: Box::new(lower_expr_to_mir(object)),
            key: key.clone(),
            span: *span,
        },
        LoweredExpr::PropertyDeleteDynamic { object, key, span } => {
            MirExpr::PropertyDeleteDynamic {
                object: Box::new(lower_expr_to_mir(object)),
                key: Box::new(lower_expr_to_mir(key)),
                span: *span,
            }
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
            span,
        } => MirExpr::PropertySetDynamic {
            object: Box::new(lower_expr_to_mir(object)),
            index: Box::new(lower_expr_to_mir(index)),
            value: Box::new(lower_expr_to_mir(value)),
            span: *span,
        },
        LoweredExpr::New {
            constructor,
            prototype,
            args,
            base_local,
            private_brand,
            private_slot_count,
            span,
        } => MirExpr::New {
            constructor: *constructor,
            prototype: prototype.clone(),
            args: args.iter().map(lower_expr_to_mir).collect(),
            base_local: *base_local,
            private_brand: *private_brand,
            private_slot_count: *private_slot_count,
            span: *span,
        },
        LoweredExpr::ClassPrototype(proto, span) => MirExpr::ClassPrototype(proto.clone(), *span),
        LoweredExpr::BuiltinErrorPrototype(ctor, span) => {
            MirExpr::BuiltinErrorPrototype(*ctor, *span)
        }
        LoweredExpr::ModuleLoad { module_id, span } => MirExpr::ModuleLoad {
            module_id: *module_id,
            span: *span,
        },
        LoweredExpr::Block {
            stmts,
            result,
            span,
        } => MirExpr::Block {
            stmts: stmts.iter().map(lower_stmt_to_mir).collect(),
            result: Box::new(lower_expr_to_mir(result)),
            span: *span,
        },
        LoweredExpr::This(span) => MirExpr::This(*span),
        LoweredExpr::ArrowFn {
            func_id,
            captures,
            representation,
            span,
        } => MirExpr::ArrowFn {
            func_id: *func_id,
            captures: captures.clone(),
            representation: *representation,
            span: *span,
        },
    }
}

fn lower_stmt_to_mir(stmt: &LoweredStmt) -> MirStmt {
    match stmt {
        LoweredStmt::Block(stmts, span) => {
            MirStmt::Block(stmts.iter().map(lower_stmt_to_mir).collect(), *span)
        }
        LoweredStmt::Let(local, expr, span) => MirStmt::Let(*local, lower_expr_to_mir(expr), *span),
        LoweredStmt::Assign(local, expr, span) => {
            MirStmt::Assign(*local, lower_expr_to_mir(expr), *span)
        }
        LoweredStmt::Expr(expr, span) => MirStmt::Expr(lower_expr_to_mir(expr), *span),
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => MirStmt::If {
            condition: lower_expr_to_mir(condition),
            then_body: then_body.iter().map(lower_stmt_to_mir).collect(),
            else_body: else_body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::While {
            condition,
            body,
            span,
        } => MirStmt::While {
            condition: lower_expr_to_mir(condition),
            body: body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::Return(expr, span) => MirStmt::Return(lower_expr_to_mir(expr), *span),
        LoweredStmt::Throw(expr, span) => MirStmt::Throw(lower_expr_to_mir(expr), *span),
        LoweredStmt::TryFinally {
            try_body,
            finally_body,
            span,
        } => MirStmt::TryFinally {
            try_body: try_body.iter().map(lower_stmt_to_mir).collect(),
            finally_body: finally_body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
            span,
        } => MirStmt::TryCatch {
            try_body: try_body.iter().map(lower_stmt_to_mir).collect(),
            catch_var: *catch_var,
            catch_body: catch_body
                .as_ref()
                .map(|b| b.iter().map(lower_stmt_to_mir).collect()),
            finally_body: finally_body
                .as_ref()
                .map(|b| b.iter().map(lower_stmt_to_mir).collect()),
            span: *span,
        },
        LoweredStmt::Switch { expr, cases, span } => {
            let mir_cases: Vec<(Option<MirExpr>, Vec<MirStmt>)> = cases
                .iter()
                .map(|(cond, body)| {
                    (
                        cond.as_ref().map(lower_expr_to_mir),
                        body.iter().map(lower_stmt_to_mir).collect(),
                    )
                })
                .collect();
            MirStmt::Switch {
                expr: lower_expr_to_mir(expr),
                cases: mir_cases,
                span: *span,
            }
        }
        LoweredStmt::DoWhile {
            body,
            condition,
            span,
        } => MirStmt::DoWhile {
            body: body.iter().map(lower_stmt_to_mir).collect(),
            condition: lower_expr_to_mir(condition),
            span: *span,
        },
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
            span,
        } => MirStmt::For {
            init: init.as_ref().map(|i| Box::new(lower_stmt_to_mir(i))),
            condition: condition.as_ref().map(lower_expr_to_mir),
            update: update.as_ref().map(lower_expr_to_mir),
            body: body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => MirStmt::ForIn {
            var: *var,
            iter: lower_expr_to_mir(iter),
            iter_local: *iter_local,
            index_local: *index_local,
            len_local: *len_local,
            body: body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => MirStmt::ForOf {
            var: *var,
            iter: lower_expr_to_mir(iter),
            iter_local: *iter_local,
            index_local: *index_local,
            len_local: *len_local,
            body: body.iter().map(lower_stmt_to_mir).collect(),
            span: *span,
        },
        LoweredStmt::Labeled { label, body, span } => MirStmt::Labeled {
            label: label.clone(),
            body: Box::new(lower_stmt_to_mir(body)),
            span: *span,
        },
        LoweredStmt::Break { label, span } => MirStmt::Break {
            label: label.clone(),
            span: *span,
        },
        LoweredStmt::Continue { label, span } => MirStmt::Continue {
            label: label.clone(),
            span: *span,
        },
        LoweredStmt::Export { name, expr, span } => MirStmt::Export {
            name: name.clone(),
            expr: lower_expr_to_mir(expr),
            span: *span,
        },
        LoweredStmt::ModuleExportsAssign { expr, span } => MirStmt::ModuleExportsAssign {
            expr: lower_expr_to_mir(expr),
            span: *span,
        },
        LoweredStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_methods,
            private_fields,
            span,
        } => MirStmt::ClassDecl {
            name: name.clone(),
            extends: extends.clone(),
            constructor: *constructor,
            methods: methods.clone(),
            static_methods: static_methods.clone(),
            private_fields: private_fields.clone(),
            span: *span,
        },
    }
}

fn lower_function_to_mir(func: &LoweredFunction) -> MirFunction {
    MirFunction {
        id: func.id,
        params: func.params.clone(),
        uses_receiver: func.uses_receiver,
        min_required_params: func.min_required_params,
        rest_param_index: func.rest_param_index,
        locals: func.locals.clone(),
        body: func.body.iter().map(lower_stmt_to_mir).collect(),
        recursion_depth: func.recursion_depth,
        is_async: func.is_async,
    }
}

impl From<LoweredProgram> for MirProgram {
    fn from(program: LoweredProgram) -> Self {
        MirProgram {
            top_level_statements: program
                .top_level_statements
                .iter()
                .map(lower_stmt_to_mir)
                .collect(),
            top_level_locals: program.top_level_locals,
            functions: program
                .functions
                .iter()
                .map(lower_function_to_mir)
                .collect(),
            modules: program.modules,
        }
    }
}

// ---------------------------------------------------------------------------
// Bridge: MirProgram -> LoweredProgram
// ---------------------------------------------------------------------------

fn mir_expr_to_lower(expr: &MirExpr) -> LoweredExpr {
    match expr {
        MirExpr::Number(v, span) => LoweredExpr::Number(*v, *span),
        MirExpr::BigIntLiteral {
            decimal,
            sign,
            limb_low,
            limb_high,
            span,
        } => LoweredExpr::BigIntLiteral {
            decimal: decimal.clone(),
            sign: *sign,
            limb_low: *limb_low,
            limb_high: *limb_high,
            span: *span,
        },
        MirExpr::String(s, span) => LoweredExpr::String(s.clone(), *span),
        MirExpr::Bool(b, span) => LoweredExpr::Bool(*b, *span),
        MirExpr::Null(span) => LoweredExpr::Null(*span),
        MirExpr::Undefined(span) => LoweredExpr::Undefined(*span),
        MirExpr::Local(id, span) => LoweredExpr::Local(*id, *span),
        MirExpr::EnvCellNew(expr, span) => {
            LoweredExpr::EnvCellNew(Box::new(mir_expr_to_lower(expr)), *span)
        }
        MirExpr::EnvCellGet(cell, span) => LoweredExpr::EnvCellGet(*cell, *span),
        MirExpr::EnvCellSet { cell, expr, span } => LoweredExpr::EnvCellSet {
            cell: *cell,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::Unary { op, expr, span } => LoweredExpr::Unary {
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::Binary {
            left,
            op,
            right,
            span,
        } => LoweredExpr::Binary {
            left: Box::new(mir_expr_to_lower(left)),
            op: *op,
            right: Box::new(mir_expr_to_lower(right)),
            span: *span,
        },
        MirExpr::PropertyIn { obj, key, span } => LoweredExpr::PropertyIn {
            obj: Box::new(mir_expr_to_lower(obj)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::PropertyInDynamic { obj, key, span } => LoweredExpr::PropertyInDynamic {
            obj: Box::new(mir_expr_to_lower(obj)),
            key: Box::new(mir_expr_to_lower(key)),
            span: *span,
        },
        MirExpr::Call { kind, args, span } => LoweredExpr::Call {
            kind: *kind,
            args: args.iter().map(mir_expr_to_lower).collect(),
            span: *span,
        },
        MirExpr::Assign { local, expr, span } => LoweredExpr::Assign {
            local: *local,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::LogicalAssign {
            local,
            op,
            expr,
            span,
        } => LoweredExpr::LogicalAssign {
            local: *local,
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::LogicalPropertyAssign {
            object,
            key,
            op,
            expr,
            span,
        } => LoweredExpr::LogicalPropertyAssign {
            object: *object,
            key: key.clone(),
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::LogicalComputedPropertyAssign {
            object,
            key,
            op,
            expr,
            span,
        } => LoweredExpr::LogicalComputedPropertyAssign {
            object: *object,
            key: Box::new(mir_expr_to_lower(key)),
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::LogicalComputedMemberAssign {
            object,
            key,
            op,
            expr,
            span,
        } => LoweredExpr::LogicalComputedMemberAssign {
            object: Box::new(mir_expr_to_lower(object)),
            key: Box::new(mir_expr_to_lower(key)),
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::LogicalMemberAssign {
            object,
            key,
            op,
            expr,
            span,
        } => LoweredExpr::LogicalMemberAssign {
            object: Box::new(mir_expr_to_lower(object)),
            key: key.clone(),
            op: *op,
            expr: Box::new(mir_expr_to_lower(expr)),
            span: *span,
        },
        MirExpr::ArrayNew { elements, span } => LoweredExpr::ArrayNew {
            elements: elements.iter().map(mir_expr_to_lower).collect(),
            span: *span,
        },
        MirExpr::ArrayNewSparse { slots, span } => LoweredExpr::ArrayNewSparse {
            slots: slots.clone(),
            span: *span,
        },
        MirExpr::ArrayGet { arr, index, span } => LoweredExpr::ArrayGet {
            arr: Box::new(mir_expr_to_lower(arr)),
            index: Box::new(mir_expr_to_lower(index)),
            span: *span,
        },
        MirExpr::Index {
            object,
            index,
            span,
        } => LoweredExpr::Index {
            object: Box::new(mir_expr_to_lower(object)),
            index: Box::new(mir_expr_to_lower(index)),
            span: *span,
        },
        MirExpr::GetLength(expr, span) => {
            LoweredExpr::GetLength(Box::new(mir_expr_to_lower(expr)), *span)
        }
        MirExpr::ObjectNew {
            props,
            non_enumerable,
            span,
        } => {
            let lowered_props: Vec<(String, LoweredExpr)> = props
                .iter()
                .map(|(k, v)| (k.clone(), mir_expr_to_lower(v)))
                .collect();
            LoweredExpr::ObjectNew {
                props: lowered_props,
                non_enumerable: *non_enumerable,
                span: *span,
            }
        }
        MirExpr::ErrorNew {
            constructor,
            message,
            span,
        } => LoweredExpr::ErrorNew {
            constructor: *constructor,
            message: Box::new(mir_expr_to_lower(message)),
            span: *span,
        },
        MirExpr::PropertyGet { obj, key, span } => LoweredExpr::PropertyGet {
            obj: Box::new(mir_expr_to_lower(obj)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::OptionalPropertyGet { obj, key, span } => LoweredExpr::OptionalPropertyGet {
            obj: Box::new(mir_expr_to_lower(obj)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::PropertyGetDynamic { obj, key, span } => LoweredExpr::PropertyGetDynamic {
            obj: Box::new(mir_expr_to_lower(obj)),
            key: Box::new(mir_expr_to_lower(key)),
            span: *span,
        },
        MirExpr::OptionalIndex {
            object,
            index,
            span,
        } => LoweredExpr::OptionalIndex {
            object: Box::new(mir_expr_to_lower(object)),
            index: Box::new(mir_expr_to_lower(index)),
            span: *span,
        },
        MirExpr::OptionalCall { callee, call, span } => LoweredExpr::OptionalCall {
            callee: Box::new(mir_expr_to_lower(callee)),
            call: Box::new(mir_expr_to_lower(call)),
            span: *span,
        },
        MirExpr::MethodCall {
            object,
            method,
            span,
        } => LoweredExpr::MethodCall {
            object: Box::new(mir_expr_to_lower(object)),
            method: method.clone(),
            span: *span,
        },
        MirExpr::PromiseGetValue { promise, span } => LoweredExpr::PromiseGetValue {
            promise: Box::new(mir_expr_to_lower(promise)),
            span: *span,
        },
        MirExpr::RuntimeCall {
            intrinsic,
            args,
            span,
        } => LoweredExpr::RuntimeCall {
            intrinsic: *intrinsic,
            args: args.iter().map(mir_expr_to_lower).collect(),
            span: *span,
        },
        MirExpr::PropertySet {
            object,
            key,
            value,
            span,
        } => LoweredExpr::PropertySet {
            object: Box::new(mir_expr_to_lower(object)),
            key: key.clone(),
            value: Box::new(mir_expr_to_lower(value)),
            span: *span,
        },
        MirExpr::PropertyDelete { object, key, span } => LoweredExpr::PropertyDelete {
            object: Box::new(mir_expr_to_lower(object)),
            key: key.clone(),
            span: *span,
        },
        MirExpr::PropertyDeleteDynamic { object, key, span } => {
            LoweredExpr::PropertyDeleteDynamic {
                object: Box::new(mir_expr_to_lower(object)),
                key: Box::new(mir_expr_to_lower(key)),
                span: *span,
            }
        }
        MirExpr::PropertySetDynamic {
            object,
            index,
            value,
            span,
        } => LoweredExpr::PropertySetDynamic {
            object: Box::new(mir_expr_to_lower(object)),
            index: Box::new(mir_expr_to_lower(index)),
            value: Box::new(mir_expr_to_lower(value)),
            span: *span,
        },
        MirExpr::New {
            constructor,
            prototype,
            args,
            base_local,
            private_brand,
            private_slot_count,
            span,
        } => LoweredExpr::New {
            constructor: *constructor,
            prototype: prototype.clone(),
            args: args.iter().map(mir_expr_to_lower).collect(),
            base_local: *base_local,
            private_brand: *private_brand,
            private_slot_count: *private_slot_count,
            span: *span,
        },
        MirExpr::ClassPrototype(proto, span) => LoweredExpr::ClassPrototype(proto.clone(), *span),
        MirExpr::BuiltinErrorPrototype(ctor, span) => {
            LoweredExpr::BuiltinErrorPrototype(*ctor, *span)
        }
        MirExpr::ModuleLoad { module_id, span } => LoweredExpr::ModuleLoad {
            module_id: *module_id,
            span: *span,
        },
        MirExpr::Block {
            stmts,
            result,
            span,
        } => LoweredExpr::Block {
            stmts: stmts.iter().map(mir_stmt_to_lower).collect(),
            result: Box::new(mir_expr_to_lower(result)),
            span: *span,
        },
        MirExpr::This(span) => LoweredExpr::This(*span),
        MirExpr::ArrowFn {
            func_id,
            captures,
            representation,
            span,
        } => LoweredExpr::ArrowFn {
            func_id: *func_id,
            captures: captures.clone(),
            representation: *representation,
            span: *span,
        },
    }
}

fn mir_stmt_to_lower(stmt: &MirStmt) -> LoweredStmt {
    match stmt {
        MirStmt::Block(stmts, span) => {
            LoweredStmt::Block(stmts.iter().map(mir_stmt_to_lower).collect(), *span)
        }
        MirStmt::Let(local, expr, span) => LoweredStmt::Let(*local, mir_expr_to_lower(expr), *span),
        MirStmt::Assign(local, expr, span) => {
            LoweredStmt::Assign(*local, mir_expr_to_lower(expr), *span)
        }
        MirStmt::Expr(expr, span) => LoweredStmt::Expr(mir_expr_to_lower(expr), *span),
        MirStmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => LoweredStmt::If {
            condition: mir_expr_to_lower(condition),
            then_body: then_body.iter().map(mir_stmt_to_lower).collect(),
            else_body: else_body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::While {
            condition,
            body,
            span,
        } => LoweredStmt::While {
            condition: mir_expr_to_lower(condition),
            body: body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::Return(expr, span) => LoweredStmt::Return(mir_expr_to_lower(expr), *span),
        MirStmt::Throw(expr, span) => LoweredStmt::Throw(mir_expr_to_lower(expr), *span),
        MirStmt::TryFinally {
            try_body,
            finally_body,
            span,
        } => LoweredStmt::TryFinally {
            try_body: try_body.iter().map(mir_stmt_to_lower).collect(),
            finally_body: finally_body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
            span,
        } => LoweredStmt::TryCatch {
            try_body: try_body.iter().map(mir_stmt_to_lower).collect(),
            catch_var: *catch_var,
            catch_body: catch_body
                .as_ref()
                .map(|b| b.iter().map(mir_stmt_to_lower).collect()),
            finally_body: finally_body
                .as_ref()
                .map(|b| b.iter().map(mir_stmt_to_lower).collect()),
            span: *span,
        },
        MirStmt::Switch { expr, cases, span } => {
            let lowered_cases: Vec<(Option<LoweredExpr>, Vec<LoweredStmt>)> = cases
                .iter()
                .map(|(cond, body)| {
                    (
                        cond.as_ref().map(mir_expr_to_lower),
                        body.iter().map(mir_stmt_to_lower).collect(),
                    )
                })
                .collect();
            LoweredStmt::Switch {
                expr: mir_expr_to_lower(expr),
                cases: lowered_cases,
                span: *span,
            }
        }
        MirStmt::DoWhile {
            body,
            condition,
            span,
        } => LoweredStmt::DoWhile {
            body: body.iter().map(mir_stmt_to_lower).collect(),
            condition: mir_expr_to_lower(condition),
            span: *span,
        },
        MirStmt::For {
            init,
            condition,
            update,
            body,
            span,
        } => LoweredStmt::For {
            init: init.as_ref().map(|i| Box::new(mir_stmt_to_lower(i))),
            condition: condition.as_ref().map(mir_expr_to_lower),
            update: update.as_ref().map(mir_expr_to_lower),
            body: body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => LoweredStmt::ForIn {
            var: *var,
            iter: mir_expr_to_lower(iter),
            iter_local: *iter_local,
            index_local: *index_local,
            len_local: *len_local,
            body: body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
            span,
        } => LoweredStmt::ForOf {
            var: *var,
            iter: mir_expr_to_lower(iter),
            iter_local: *iter_local,
            index_local: *index_local,
            len_local: *len_local,
            body: body.iter().map(mir_stmt_to_lower).collect(),
            span: *span,
        },
        MirStmt::Labeled { label, body, span } => LoweredStmt::Labeled {
            label: label.clone(),
            body: Box::new(mir_stmt_to_lower(body)),
            span: *span,
        },
        MirStmt::Break { label, span } => LoweredStmt::Break {
            label: label.clone(),
            span: *span,
        },
        MirStmt::Continue { label, span } => LoweredStmt::Continue {
            label: label.clone(),
            span: *span,
        },
        MirStmt::Export { name, expr, span } => LoweredStmt::Export {
            name: name.clone(),
            expr: mir_expr_to_lower(expr),
            span: *span,
        },
        MirStmt::ModuleExportsAssign { expr, span } => LoweredStmt::ModuleExportsAssign {
            expr: mir_expr_to_lower(expr),
            span: *span,
        },
        MirStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_methods,
            private_fields,
            span,
        } => LoweredStmt::ClassDecl {
            name: name.clone(),
            extends: extends.clone(),
            constructor: *constructor,
            methods: methods.clone(),
            static_methods: static_methods.clone(),
            private_fields: private_fields.clone(),
            span: *span,
        },
    }
}

fn mir_function_to_lower(func: &MirFunction) -> LoweredFunction {
    LoweredFunction {
        id: func.id,
        params: func.params.clone(),
        uses_receiver: func.uses_receiver,
        min_required_params: func.min_required_params,
        rest_param_index: func.rest_param_index,
        locals: func.locals.clone(),
        body: func.body.iter().map(mir_stmt_to_lower).collect(),
        recursion_depth: func.recursion_depth,
        is_async: func.is_async,
    }
}

impl From<MirProgram> for LoweredProgram {
    fn from(program: MirProgram) -> Self {
        LoweredProgram {
            top_level_statements: program
                .top_level_statements
                .iter()
                .map(mir_stmt_to_lower)
                .collect(),
            top_level_locals: program.top_level_locals,
            functions: program
                .functions
                .iter()
                .map(mir_function_to_lower)
                .collect(),
            modules: program.modules,
        }
    }
}

impl From<&MirProgram> for LoweredProgram {
    fn from(program: &MirProgram) -> Self {
        program.clone().into()
    }
}

impl From<&LoweredProgram> for LoweredProgram {
    fn from(program: &LoweredProgram) -> Self {
        program.clone()
    }
}

// ---------------------------------------------------------------------------
// MirExpr helper
// ---------------------------------------------------------------------------

impl MirExpr {
    /// Infer a conservative type for this MIR expression.
    /// Mirrors `LoweredExpr::inferred_type`.
    pub fn inferred_type(&self) -> super::InferredType {
        match self {
            Self::Number(_, _) => super::InferredType::Number,
            Self::BigIntLiteral { .. } => super::InferredType::Unknown,
            Self::String(_, _) => super::InferredType::String,
            Self::Bool(_, _) => super::InferredType::Boolean,
            Self::Unary { op, expr, .. } => match op {
                LoweredUnaryOp::Plus if expr.inferred_type() == super::InferredType::Number => {
                    super::InferredType::Number
                }
                LoweredUnaryOp::Negate if expr.inferred_type() == super::InferredType::Number => {
                    super::InferredType::Number
                }
                LoweredUnaryOp::Not => super::InferredType::Boolean,
                _ => super::InferredType::Unknown,
            },
            Self::Binary {
                left, op, right, ..
            } => match op {
                LoweredBinaryOp::Add => match (left.inferred_type(), right.inferred_type()) {
                    (super::InferredType::Number, super::InferredType::Number) => {
                        super::InferredType::Number
                    }
                    (super::InferredType::String, super::InferredType::String) => {
                        super::InferredType::String
                    }
                    _ => super::InferredType::Unknown,
                },
                LoweredBinaryOp::Subtract
                | LoweredBinaryOp::Multiply
                | LoweredBinaryOp::Power
                | LoweredBinaryOp::Divide
                | LoweredBinaryOp::Modulo
                | LoweredBinaryOp::BitwiseAnd
                | LoweredBinaryOp::BitwiseXor
                | LoweredBinaryOp::BitwiseOr => {
                    if left.inferred_type() == super::InferredType::Number
                        && right.inferred_type() == super::InferredType::Number
                    {
                        super::InferredType::Number
                    } else {
                        super::InferredType::Unknown
                    }
                }
                LoweredBinaryOp::Less
                | LoweredBinaryOp::LessEqual
                | LoweredBinaryOp::Greater
                | LoweredBinaryOp::GreaterEqual
                | LoweredBinaryOp::StrictEqual
                | LoweredBinaryOp::EqualEqual
                | LoweredBinaryOp::BangEqual
                | LoweredBinaryOp::StrictNotEqual => super::InferredType::Boolean,
                LoweredBinaryOp::And | LoweredBinaryOp::Or | LoweredBinaryOp::NullishCoalesce => {
                    super::InferredType::Unknown
                }
            },
            Self::Assign { expr, .. } => expr.inferred_type(),
            Self::LogicalAssign { .. }
            | Self::LogicalPropertyAssign { .. }
            | Self::LogicalMemberAssign { .. }
            | Self::LogicalComputedMemberAssign { .. }
            | Self::LogicalComputedPropertyAssign { .. } => super::InferredType::Unknown,
            _ => super::InferredType::Unknown,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests: bridge conversions preserve structure
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_span() -> Span {
        Span { start: 0, end: 0 }
    }

    fn sample_lowered_program() -> LoweredProgram {
        LoweredProgram {
            top_level_statements: vec![
                LoweredStmt::Let(
                    LocalId(0),
                    LoweredExpr::Number(42, make_span()),
                    make_span(),
                ),
                LoweredStmt::Expr(
                    LoweredExpr::Call {
                        kind: FunctionCallKind::Builtin(crate::builtin::BuiltinId::ConsoleLog),
                        args: vec![LoweredExpr::Bool(true, make_span())],
                        span: make_span(),
                    },
                    make_span(),
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![LoweredFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: false,
                min_required_params: 1,
                rest_param_index: None,
                locals: vec![LocalId(0)],
                body: vec![LoweredStmt::Return(
                    LoweredExpr::Local(LocalId(0), make_span()),
                    make_span(),
                )],
                recursion_depth: 0,
                is_async: false,
            }],
            modules: vec![],
        }
    }

    fn assert_mir_expr_roundtrip(expr: LoweredExpr) {
        let mir: MirExpr = lower_expr_to_mir(&expr);
        let lowered_back: LoweredExpr = mir_expr_to_lower(&mir);
        assert_eq!(
            expr, lowered_back,
            "MirExpr roundtrip failed for {:?}",
            expr
        );
    }

    #[test]
    fn bridge_lowered_to_mir_roundtrip_full_program() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        let lowered_back: LoweredProgram = mir.into();
        assert_eq!(
            lowered, lowered_back,
            "Full program bridge roundtrip should preserve all data"
        );
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_top_level_structure() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        assert_eq!(mir.top_level_locals, lowered.top_level_locals);
        assert_eq!(mir.functions.len(), lowered.functions.len());
        assert_eq!(mir.modules.len(), lowered.modules.len());
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_functions() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        let mir_fn = &mir.functions[0];
        let lowered_fn = &lowered.functions[0];
        assert_eq!(mir_fn.id, lowered_fn.id);
        assert_eq!(mir_fn.params, lowered_fn.params);
        assert_eq!(mir_fn.uses_receiver, lowered_fn.uses_receiver);
        assert_eq!(mir_fn.min_required_params, lowered_fn.min_required_params);
        assert_eq!(mir_fn.rest_param_index, lowered_fn.rest_param_index);
        assert_eq!(mir_fn.locals, lowered_fn.locals);
        assert_eq!(mir_fn.body.len(), lowered_fn.body.len());
        assert_eq!(mir_fn.recursion_depth, lowered_fn.recursion_depth);
        assert_eq!(mir_fn.is_async, lowered_fn.is_async);
    }

    #[test]
    fn bridge_lowered_to_mir_preserves_stmts() {
        let lowered = sample_lowered_program();
        let mir: MirProgram = lowered.clone().into();
        assert_eq!(
            mir.top_level_statements.len(),
            lowered.top_level_statements.len()
        );

        // Let stmt
        let mir_stmt = &mir.top_level_statements[0];
        let lowered_stmt = &lowered.top_level_statements[0];
        match (mir_stmt, lowered_stmt) {
            (
                MirStmt::Let(mir_local, mir_expr, _),
                LoweredStmt::Let(lowered_local, lowered_expr, _),
            ) => {
                assert_eq!(mir_local, lowered_local);
                assert_eq!(mir_expr_to_lower(mir_expr), lowered_expr.clone());
            }
            _ => panic!("Expected Let stmt at index 0"),
        }
    }

    #[test]
    fn bridge_mir_expr_roundtrip_constants() {
        assert_mir_expr_roundtrip(LoweredExpr::Number(42, make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::String("hi".to_string(), make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Bool(true, make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Null(make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Undefined(make_span()));
        assert_mir_expr_roundtrip(LoweredExpr::Local(LocalId(0), make_span()));
    }

    #[test]
    fn bridge_mir_expr_roundtrip_complex() {
        assert_mir_expr_roundtrip(LoweredExpr::Binary {
            left: Box::new(LoweredExpr::Number(1, make_span())),
            op: LoweredBinaryOp::Add,
            right: Box::new(LoweredExpr::Number(2, make_span())),
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::Call {
            kind: FunctionCallKind::User(FuncId(0)),
            args: vec![LoweredExpr::Number(42, make_span())],
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::RuntimeCall {
            intrinsic: crate::lowered::RuntimeFn::ArrayPushGrow,
            args: vec![
                LoweredExpr::Local(LocalId(0), make_span()),
                LoweredExpr::Number(1, make_span()),
            ],
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_object_and_array() {
        assert_mir_expr_roundtrip(LoweredExpr::ObjectNew {
            props: vec![("x".to_string(), LoweredExpr::Number(1, make_span()))],
            non_enumerable: 0,
            span: make_span(),
        });

        assert_mir_expr_roundtrip(LoweredExpr::ArrayNew {
            elements: vec![
                LoweredExpr::Number(1, make_span()),
                LoweredExpr::Number(2, make_span()),
            ],
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_block() {
        assert_mir_expr_roundtrip(LoweredExpr::Block {
            stmts: vec![LoweredStmt::Let(
                LocalId(0),
                LoweredExpr::Number(1, make_span()),
                make_span(),
            )],
            result: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_expr_roundtrip_all_variants() {
        let variants: Vec<LoweredExpr> = vec![
            LoweredExpr::BigIntLiteral {
                decimal: "42".to_string(),
                sign: 1,
                limb_low: 42,
                limb_high: 0,
                span: make_span(),
            },
            LoweredExpr::EnvCellNew(Box::new(LoweredExpr::Number(1, make_span())), make_span()),
            LoweredExpr::EnvCellGet(LocalId(0), make_span()),
            LoweredExpr::Unary {
                op: LoweredUnaryOp::Not,
                expr: Box::new(LoweredExpr::Bool(true, make_span())),
                span: make_span(),
            },
            LoweredExpr::PropertyGet {
                obj: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                key: "x".to_string(),
                span: make_span(),
            },
            LoweredExpr::ArrayGet {
                arr: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                index: Box::new(LoweredExpr::Number(0, make_span())),
                span: make_span(),
            },
            LoweredExpr::MethodCall {
                object: Box::new(LoweredExpr::Local(LocalId(0), make_span())),
                method: "toString".to_string(),
                span: make_span(),
            },
            LoweredExpr::This(make_span()),
            LoweredExpr::ModuleLoad {
                module_id: 1,
                span: make_span(),
            },
        ];
        for expr in variants {
            assert_mir_expr_roundtrip(expr);
        }
    }

    #[test]
    fn bridge_mir_expr_roundtrip_new() {
        assert_mir_expr_roundtrip(LoweredExpr::New {
            constructor: FuncId(0),
            prototype: ClassPrototypeRef {
                constructor: FuncId(0),
                parent_constructors: vec![],
            },
            args: vec![LoweredExpr::Number(1, make_span())],
            base_local: LocalId(0),
            private_brand: Some(1),
            private_slot_count: 1,
            span: make_span(),
        });
    }

    #[test]
    fn bridge_mir_is_independent_type() {
        // Compile-time check: MirProgram is NOT LoweredProgram
        fn takes_mir(_p: MirProgram) {}
        fn takes_lowered(_p: LoweredProgram) {}

        let mir = MirProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };
        let lowered = LoweredProgram {
            top_level_statements: vec![],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
        };

        takes_mir(mir);
        takes_lowered(lowered);
    }
}
