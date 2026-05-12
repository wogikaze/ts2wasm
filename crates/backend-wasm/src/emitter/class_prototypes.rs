use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::emitter::{self, WatEmitter};
use crate::runtime_fn::RuntimeFn;
use crate::wat_writer::WatWriter;
use ts2wasm_ir::lowered::{BuiltinErrorConstructor, FuncId, LoweredExpr, LoweredStmt};
use ts2wasm_runtime_abi::Layout;

impl WatEmitter<'_> {
    pub(super) fn emit_class_prototype_globals(&self, writer: &mut WatWriter) {
        for constructor in self.class_prototypes().keys() {
            writer.line_fmt(
                2,
                format_args!(
                    "(global ${} (mut i32) (i32.const 0))",
                    emitter::class_prototype_global(*constructor),
                ),
            );
        }
    }

    pub(super) fn emit_builtin_error_prototype_globals(&self, writer: &mut WatWriter) {
        for constructor in self.builtin_error_prototypes() {
            writer.line_fmt(
                2,
                format_args!(
                    "(global ${} (mut i32) (i32.const 0))",
                    emitter::builtin_error_prototype_global(constructor),
                ),
            );
        }
    }

    pub(super) fn emit_class_prototype_initializers(&self, wat: &mut String, indent: usize) {
        let pad = " ".repeat(indent);
        for (constructor, parent) in self.ordered_class_prototypes() {
            let global = emitter::class_prototype_global(constructor);
            let method_count = self.method_counts.get(&constructor).copied().unwrap_or(0);
            let size = Layout::OBJECT_HEADER_SIZE + method_count as u32 * Layout::OBJECT_ENTRY_SIZE;
            wat.push_str(&format!(
                "{pad}(if (i32.eqz (global.get ${global}))\n{pad}  (then\n"
            ));
            wat.push_str(&format!(
                "{pad}    (global.set ${global} (call {} (i32.const {size})))\n",
                RuntimeFn::AllocHeap.symbol(),
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (global.get ${global}) (i32.const 0))\n"
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) (i32.const 0))\n",
                Layout::OBJECT_FLAGS_OFFSET,
            ));
            let parent_expr = parent
                .map(|id| format!("global.get ${}", emitter::class_prototype_global(id)))
                .unwrap_or_else(|| "i32.const 0".to_owned());
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) ({parent_expr}))\n",
                Layout::OBJECT_PROTOTYPE_OFFSET,
            ));
            wat.push_str(&format!("{pad}  )\n{pad})\n"));
        }
    }

    pub(super) fn emit_builtin_error_prototype_initializers(
        &self,
        wat: &mut String,
        indent: usize,
    ) {
        let pad = " ".repeat(indent);
        for constructor in self.builtin_error_prototypes() {
            let global = emitter::builtin_error_prototype_global(constructor);
            wat.push_str(&format!(
                "{pad}(if (i32.eqz (global.get ${global}))\n{pad}  (then\n"
            ));
            wat.push_str(&format!(
                "{pad}    (global.set ${global} (call {} (i32.const {})))\n",
                RuntimeFn::AllocHeap.symbol(),
                Layout::OBJECT_HEADER_SIZE,
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (global.get ${global}) (i32.const 0))\n"
            ));
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) (i32.const 0))\n",
                Layout::OBJECT_FLAGS_OFFSET,
            ));
            let parent_expr = constructor
                .parent()
                .map(|parent| {
                    format!(
                        "global.get ${}",
                        emitter::builtin_error_prototype_global(parent)
                    )
                })
                .unwrap_or_else(|| "i32.const 0".to_owned());
            wat.push_str(&format!(
                "{pad}    (i32.store (i32.add (global.get ${global}) (i32.const {})) ({parent_expr}))\n",
                Layout::OBJECT_PROTOTYPE_OFFSET,
            ));
            wat.push_str(&format!("{pad}  )\n{pad})\n"));
        }
    }

    fn ordered_class_prototypes(&self) -> Vec<(FuncId, Option<FuncId>)> {
        let prototypes = self.class_prototypes();
        let mut ordered = prototypes
            .iter()
            .map(|(constructor, parent)| {
                (
                    *constructor,
                    *parent,
                    emitter::class_prototype_depth(*constructor, &prototypes),
                )
            })
            .collect::<Vec<_>>();
        ordered.sort_by_key(|(constructor, _, depth)| (*depth, constructor.0));
        ordered
            .into_iter()
            .map(|(constructor, parent, _)| (constructor, parent))
            .collect()
    }

    pub(crate) fn class_prototypes(&self) -> BTreeMap<FuncId, Option<FuncId>> {
        let mut prototypes = BTreeMap::new();
        Self::collect_class_decl_prototypes(
            &self.program.top_level_statements,
            &mut prototypes,
            &self.class_name_to_ctor,
        );
        Self::collect_class_prototypes_from_stmts(
            &self.program.top_level_statements,
            &mut prototypes,
        );
        for function in &self.program.functions {
            Self::collect_class_decl_prototypes(
                &function.body,
                &mut prototypes,
                &self.class_name_to_ctor,
            );
            Self::collect_class_prototypes_from_stmts(&function.body, &mut prototypes);
        }
        prototypes
    }

    pub(crate) fn builtin_error_prototypes(&self) -> BTreeSet<BuiltinErrorConstructor> {
        let mut prototypes = BTreeSet::new();
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::BigIntDivisionByZeroRangeError)
        {
            emitter::add_builtin_error_prototype_ref(
                BuiltinErrorConstructor::RangeError,
                &mut prototypes,
            );
        }
        if self
            .link_plan
            .required_runtime_functions()
            .contains(&RuntimeFn::BigIntMixedArithmeticTypeError)
            || self
                .link_plan
                .required_runtime_functions()
                .contains(&RuntimeFn::PrivateBrandTypeError)
        {
            emitter::add_builtin_error_prototype_ref(
                BuiltinErrorConstructor::TypeError,
                &mut prototypes,
            );
        }
        Self::collect_builtin_error_prototypes_from_stmts(
            &self.program.top_level_statements,
            &mut prototypes,
        );
        for function in &self.program.functions {
            Self::collect_builtin_error_prototypes_from_stmts(&function.body, &mut prototypes);
        }
        prototypes
    }

    fn collect_class_prototypes_from_stmts(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::Block(statements, _) => {
                    Self::collect_class_prototypes_from_stmts(statements, prototypes);
                }
                LoweredStmt::Let(_, expr, _)
                | LoweredStmt::Assign(_, expr, _)
                | LoweredStmt::Expr(expr, _)
                | LoweredStmt::Yield(expr, _)
                | LoweredStmt::Return(expr, _)
                | LoweredStmt::Throw(expr, _)
                | LoweredStmt::Export { expr, .. }
                | LoweredStmt::ModuleExportsAssign { expr, .. } => {
                    Self::collect_class_prototypes_from_expr(expr, prototypes);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    Self::collect_class_prototypes_from_expr(condition, prototypes);
                    Self::collect_class_prototypes_from_stmts(then_body, prototypes);
                    Self::collect_class_prototypes_from_stmts(else_body, prototypes);
                }
                LoweredStmt::While {
                    condition, body, ..
                }
                | LoweredStmt::DoWhile {
                    body, condition, ..
                } => {
                    Self::collect_class_prototypes_from_expr(condition, prototypes);
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::TryFinally {
                    try_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_class_prototypes_from_stmts(try_body, prototypes);
                    Self::collect_class_prototypes_from_stmts(finally_body, prototypes);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_class_prototypes_from_stmts(try_body, prototypes);
                    if let Some(body) = catch_body {
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                    if let Some(body) = finally_body {
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::Switch { expr, cases, .. } => {
                    Self::collect_class_prototypes_from_expr(expr, prototypes);
                    for (case_expr, body) in cases {
                        if let Some(case_expr) = case_expr {
                            Self::collect_class_prototypes_from_expr(case_expr, prototypes);
                        }
                        Self::collect_class_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                    ..
                } => {
                    if let Some(init) = init {
                        Self::collect_class_prototypes_from_stmts(
                            std::slice::from_ref(init.as_ref()),
                            prototypes,
                        );
                    }
                    if let Some(condition) = condition {
                        Self::collect_class_prototypes_from_expr(condition, prototypes);
                    }
                    if let Some(update) = update {
                        Self::collect_class_prototypes_from_expr(update, prototypes);
                    }
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
                    Self::collect_class_prototypes_from_expr(iter, prototypes);
                    Self::collect_class_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::Labeled { body, .. } => Self::collect_class_prototypes_from_stmts(
                    std::slice::from_ref(body.as_ref()),
                    prototypes,
                ),
                LoweredStmt::Break { .. }
                | LoweredStmt::Continue { .. }
                | LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_builtin_error_prototypes_from_stmts(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::Block(statements, _) => {
                    Self::collect_builtin_error_prototypes_from_stmts(statements, prototypes);
                }
                LoweredStmt::Let(_, expr, _)
                | LoweredStmt::Assign(_, expr, _)
                | LoweredStmt::Expr(expr, _)
                | LoweredStmt::Yield(expr, _)
                | LoweredStmt::Return(expr, _)
                | LoweredStmt::Throw(expr, _)
                | LoweredStmt::Export { expr, .. }
                | LoweredStmt::ModuleExportsAssign { expr, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
                }
                LoweredStmt::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(then_body, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(else_body, prototypes);
                }
                LoweredStmt::While {
                    condition, body, ..
                }
                | LoweredStmt::DoWhile {
                    body, condition, ..
                } => {
                    Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::TryFinally {
                    try_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_builtin_error_prototypes_from_stmts(try_body, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(finally_body, prototypes);
                }
                LoweredStmt::TryCatch {
                    try_body,
                    catch_body,
                    finally_body,
                    ..
                } => {
                    Self::collect_builtin_error_prototypes_from_stmts(try_body, prototypes);
                    if let Some(body) = catch_body {
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                    if let Some(body) = finally_body {
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::Switch { expr, cases, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
                    for (case_expr, body) in cases {
                        if let Some(case_expr) = case_expr {
                            Self::collect_builtin_error_prototypes_from_expr(case_expr, prototypes);
                        }
                        Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                    }
                }
                LoweredStmt::For {
                    init,
                    condition,
                    update,
                    body,
                    ..
                } => {
                    if let Some(stmt) = init {
                        Self::collect_builtin_error_prototypes_from_stmts(
                            std::slice::from_ref(stmt.as_ref()),
                            prototypes,
                        );
                    }
                    if let Some(condition) = condition {
                        Self::collect_builtin_error_prototypes_from_expr(condition, prototypes);
                    }
                    if let Some(update) = update {
                        Self::collect_builtin_error_prototypes_from_expr(update, prototypes);
                    }
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::ForIn { iter, body, .. } | LoweredStmt::ForOf { iter, body, .. } => {
                    Self::collect_builtin_error_prototypes_from_expr(iter, prototypes);
                    Self::collect_builtin_error_prototypes_from_stmts(body, prototypes);
                }
                LoweredStmt::Labeled { body, .. } => {
                    Self::collect_builtin_error_prototypes_from_stmts(
                        std::slice::from_ref(body.as_ref()),
                        prototypes,
                    )
                }
                LoweredStmt::Break { .. }
                | LoweredStmt::Continue { .. }
                | LoweredStmt::ClassDecl { .. } => {}
            }
        }
    }

    fn collect_class_prototypes_from_expr(
        expr: &LoweredExpr,
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
    ) {
        match expr {
            LoweredExpr::ClassPrototype(prototype, _) => {
                emitter::add_class_prototype_ref(prototype, prototypes);
            }
            LoweredExpr::BuiltinErrorPrototype(_, _) | LoweredExpr::ErrorNew { .. } => {}
            LoweredExpr::Block { stmts, result, .. } => {
                Self::collect_class_prototypes_from_stmts(stmts, prototypes);
                Self::collect_class_prototypes_from_expr(result, prototypes);
            }
            LoweredExpr::New {
                prototype, args, ..
            } => {
                emitter::add_class_prototype_ref(prototype, prototypes);
                for arg in args {
                    Self::collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::Unary { expr, .. }
            | LoweredExpr::GetLength(expr, _)
            | LoweredExpr::PropertyGet { obj: expr, .. }
            | LoweredExpr::OptionalPropertyGet { obj: expr, .. }
            | LoweredExpr::MethodCall { object: expr, .. }
            | LoweredExpr::PropertyDelete { object: expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Binary { left, right, .. } => {
                Self::collect_class_prototypes_from_expr(left, prototypes);
                Self::collect_class_prototypes_from_expr(right, prototypes);
            }
            LoweredExpr::PropertyIn { obj, .. } => {
                Self::collect_class_prototypes_from_expr(obj, prototypes);
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. }
            | LoweredExpr::ArrayGet {
                arr: obj,
                index: key,
                ..
            }
            | LoweredExpr::Index {
                object: obj,
                index: key,
                ..
            }
            | LoweredExpr::OptionalIndex {
                object: obj,
                index: key,
                ..
            }
            | LoweredExpr::PropertyGetDynamic { obj, key, .. }
            | LoweredExpr::PropertyDeleteDynamic {
                object: obj, key, ..
            } => {
                Self::collect_class_prototypes_from_expr(obj, prototypes);
                Self::collect_class_prototypes_from_expr(key, prototypes);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                Self::collect_class_prototypes_from_expr(callee, prototypes);
                Self::collect_class_prototypes_from_expr(call, prototypes);
            }
            LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    Self::collect_class_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    Self::collect_class_prototypes_from_expr(elem, prototypes);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        Self::collect_class_prototypes_from_expr(elem, prototypes);
                    }
                }
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (_, value) in props {
                    Self::collect_class_prototypes_from_expr(value, prototypes);
                }
            }
            LoweredExpr::PropertySet { object, value, .. } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(index, prototypes);
                Self::collect_class_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::Assign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellNew(expr, _) => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::EnvCellSet { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalAssign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalPropertyAssign { expr, .. } => {
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                Self::collect_class_prototypes_from_expr(key, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                Self::collect_class_prototypes_from_expr(object, prototypes);
                Self::collect_class_prototypes_from_expr(key, prototypes);
                Self::collect_class_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::String(_, _)
            | LoweredExpr::BigIntLiteral { .. }
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::Local(_, _)
            | LoweredExpr::ModuleLoad { .. }
            | LoweredExpr::This(..)
            | LoweredExpr::PromiseGetValue { .. } => {}
            LoweredExpr::ArrowFn { .. } => {}
        }
    }

    fn collect_builtin_error_prototypes_from_expr(
        expr: &LoweredExpr,
        prototypes: &mut BTreeSet<BuiltinErrorConstructor>,
    ) {
        match expr {
            LoweredExpr::BuiltinErrorPrototype(constructor, _) => {
                emitter::add_builtin_error_prototype_ref(*constructor, prototypes);
            }
            LoweredExpr::Block { stmts, result, .. } => {
                Self::collect_builtin_error_prototypes_from_stmts(stmts, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(result, prototypes);
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
                ..
            } => {
                emitter::add_builtin_error_prototype_ref(*constructor, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(message, prototypes);
            }
            LoweredExpr::Unary { expr, .. }
            | LoweredExpr::GetLength(expr, _)
            | LoweredExpr::PropertyGet { obj: expr, .. }
            | LoweredExpr::OptionalPropertyGet { obj: expr, .. }
            | LoweredExpr::MethodCall { object: expr, .. }
            | LoweredExpr::PropertyDelete { object: expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::Binary { left, right, .. }
            | LoweredExpr::PropertyGetDynamic {
                obj: left,
                key: right,
                ..
            }
            | LoweredExpr::Index {
                object: left,
                index: right,
                ..
            }
            | LoweredExpr::OptionalIndex {
                object: left,
                index: right,
                ..
            }
            | LoweredExpr::ArrayGet {
                arr: left,
                index: right,
                ..
            }
            | LoweredExpr::PropertyDeleteDynamic {
                object: left,
                key: right,
                ..
            }
            | LoweredExpr::PropertyInDynamic {
                obj: left,
                key: right,
                ..
            } => {
                Self::collect_builtin_error_prototypes_from_expr(left, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(right, prototypes);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(callee, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(call, prototypes);
            }
            LoweredExpr::PropertySet { object, value, .. }
            | LoweredExpr::PropertySetDynamic { object, value, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(value, prototypes);
            }
            LoweredExpr::Call { args, .. } | LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    Self::collect_builtin_error_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    Self::collect_builtin_error_prototypes_from_expr(elem, prototypes);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        Self::collect_builtin_error_prototypes_from_expr(elem, prototypes);
                    }
                }
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (_, value) in props {
                    Self::collect_builtin_error_prototypes_from_expr(value, prototypes);
                }
            }
            LoweredExpr::New { args, .. } => {
                for arg in args {
                    Self::collect_builtin_error_prototypes_from_expr(arg, prototypes);
                }
            }
            LoweredExpr::Assign { expr, .. }
            | LoweredExpr::EnvCellNew(expr, _)
            | LoweredExpr::EnvCellSet { expr, .. }
            | LoweredExpr::LogicalAssign { expr, .. }
            | LoweredExpr::LogicalPropertyAssign { expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::LogicalMemberAssign { object, expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(key, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                Self::collect_builtin_error_prototypes_from_expr(object, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(key, prototypes);
                Self::collect_builtin_error_prototypes_from_expr(expr, prototypes);
            }
            LoweredExpr::PropertyIn { obj, .. } => {
                Self::collect_builtin_error_prototypes_from_expr(obj, prototypes);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::String(_, _)
            | LoweredExpr::BigIntLiteral { .. }
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::Local(_, _)
            | LoweredExpr::ClassPrototype(_, _)
            | LoweredExpr::ModuleLoad { .. }
            | LoweredExpr::This(..)
            | LoweredExpr::PromiseGetValue { .. } => {}
            LoweredExpr::ArrowFn { .. } => {}
        }
    }

    fn collect_class_decl_prototypes(
        stmts: &[LoweredStmt],
        prototypes: &mut BTreeMap<FuncId, Option<FuncId>>,
        class_name_to_ctor: &HashMap<String, FuncId>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::ClassDecl {
                    constructor: Some(ctor_id),
                    extends,
                    ..
                } => {
                    let parent = extends
                        .as_ref()
                        .and_then(|name| class_name_to_ctor.get(name))
                        .copied();
                    prototypes.entry(*ctor_id).or_insert(parent);
                    if let Some(parent_id) = parent {
                        prototypes.entry(parent_id).or_insert(None);
                    }
                }
                LoweredStmt::Block(statements, _) => {
                    Self::collect_class_decl_prototypes(statements, prototypes, class_name_to_ctor);
                }
                _ => {}
            }
        }
    }

    pub(super) fn compute_class_decl_metadata(
        stmts: &[LoweredStmt],
        class_name_to_ctor: &mut HashMap<String, FuncId>,
        method_counts: &mut HashMap<FuncId, usize>,
    ) {
        for stmt in stmts {
            match stmt {
                LoweredStmt::ClassDecl {
                    name,
                    constructor: Some(ctor_id),
                    methods,
                    ..
                } => {
                    class_name_to_ctor.insert(name.clone(), *ctor_id);
                    method_counts.insert(*ctor_id, methods.len());
                }
                LoweredStmt::Block(statements, _) => {
                    Self::compute_class_decl_metadata(
                        statements,
                        class_name_to_ctor,
                        method_counts,
                    );
                }
                _ => {}
            }
        }
    }
}
