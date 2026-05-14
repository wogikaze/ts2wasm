use super::types::{MirArraySlot, MirExpr, MirFunction, MirProgram, MirStmt};
use crate::lowered::{LoweredArraySlot, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt};

pub(super) fn mir_expr_to_lower(expr: &MirExpr) -> LoweredExpr {
    match expr {
        MirExpr::Number(v, span) => LoweredExpr::Number(*v, *span),
        MirExpr::DecimalNumber(v, span) => LoweredExpr::DecimalNumber(v.clone(), *span),
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
            slots: slots.iter().map(mir_array_slot_to_lower).collect(),
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
            cause: None,
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
        MirExpr::ModuleLoad {
            module_id,
            kind,
            span,
        } => LoweredExpr::ModuleLoad {
            module_id: *module_id,
            kind: *kind,
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

fn mir_array_slot_to_lower(slot: &MirArraySlot) -> LoweredArraySlot {
    match slot {
        MirArraySlot::Present(expr) => LoweredArraySlot::Present(mir_expr_to_lower(expr)),
        MirArraySlot::Hole => LoweredArraySlot::Hole,
    }
}

pub(super) fn mir_stmt_to_lower(stmt: &MirStmt) -> LoweredStmt {
    match stmt {
        MirStmt::Block(stmts, span) => {
            LoweredStmt::Block(stmts.iter().map(mir_stmt_to_lower).collect(), *span)
        }
        MirStmt::Let(local, expr, span) => LoweredStmt::Let(*local, mir_expr_to_lower(expr), *span),
        MirStmt::Assign(local, expr, span) => {
            LoweredStmt::Assign(*local, mir_expr_to_lower(expr), *span)
        }
        MirStmt::Expr(expr, span) => LoweredStmt::Expr(mir_expr_to_lower(expr), *span),
        MirStmt::Yield(expr, span) => LoweredStmt::Yield(mir_expr_to_lower(expr), *span),
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
        MirStmt::ForAwaitOfLower {
            var,
            iter,
            async_iter_local,
            next_result_local,
            done_local,
            value_local,
            body,
            span,
        } => LoweredStmt::ForAwaitOfLower {
            var: *var,
            iter: mir_expr_to_lower(iter),
            async_iter_local: *async_iter_local,
            next_result_local: *next_result_local,
            done_local: *done_local,
            value_local: *value_local,
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
        MirStmt::ModuleExportsUpdate { name, local, span } => LoweredStmt::ModuleExportsUpdate {
            name: name.clone(),
            local: *local,
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
        is_generator: func.is_generator,
        generator_state: func.generator_state.clone(),
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
