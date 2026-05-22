use super::types::{MirArraySlot, MirExpr, MirFunction, MirProgram, MirStmt};
use crate::lowered::{LoweredArraySlot, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt};

pub(super) fn lower_expr_to_mir(expr: &LoweredExpr) -> MirExpr {
    match expr {
        LoweredExpr::Number(v, span) => MirExpr::Number(*v, *span),
        LoweredExpr::DecimalNumber(v, span) => MirExpr::DecimalNumber(v.clone(), *span),
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
            slots: slots.iter().map(lower_array_slot_to_mir).collect(),
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
            cause,
            span,
            ..
        } => MirExpr::ErrorNew {
            constructor: *constructor,
            message: Box::new(lower_expr_to_mir(message)),
            cause: cause
                .as_ref()
                .map(|cause| Box::new(lower_expr_to_mir(cause))),
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
        LoweredExpr::ModuleLoad {
            module_id,
            kind,
            span,
        } => MirExpr::ModuleLoad {
            module_id: *module_id,
            kind: *kind,
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

fn lower_array_slot_to_mir(slot: &LoweredArraySlot) -> MirArraySlot {
    match slot {
        LoweredArraySlot::Present(expr) => MirArraySlot::Present(lower_expr_to_mir(expr)),
        LoweredArraySlot::Hole => MirArraySlot::Hole,
    }
}

pub(super) fn lower_stmt_to_mir(stmt: &LoweredStmt) -> MirStmt {
    match stmt {
        LoweredStmt::Block(stmts, span) => {
            MirStmt::Block(stmts.iter().map(lower_stmt_to_mir).collect(), *span)
        }
        LoweredStmt::Let(local, expr, span) => MirStmt::Let(*local, lower_expr_to_mir(expr), *span),
        LoweredStmt::Assign(local, expr, span) => {
            MirStmt::Assign(*local, lower_expr_to_mir(expr), *span)
        }
        LoweredStmt::Expr(expr, span) => MirStmt::Expr(lower_expr_to_mir(expr), *span),
        LoweredStmt::Yield(expr, span) => MirStmt::Yield(lower_expr_to_mir(expr), *span),
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
        LoweredStmt::ForAwaitOfLower {
            var,
            iter,
            async_iter_local,
            next_result_local,
            done_local,
            value_local,
            body,
            span,
        } => MirStmt::ForAwaitOfLower {
            var: *var,
            iter: lower_expr_to_mir(iter),
            async_iter_local: *async_iter_local,
            next_result_local: *next_result_local,
            done_local: *done_local,
            value_local: *value_local,
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
        LoweredStmt::ModuleExportsUpdate { name, local, span } => MirStmt::ModuleExportsUpdate {
            name: name.clone(),
            local: *local,
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

pub(super) fn lower_function_to_mir(func: &LoweredFunction) -> MirFunction {
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
        is_generator: func.is_generator,
        generator_state: func.generator_state.clone(),
        induction_vars: Vec::new(),
        escape_status: Vec::new(),
        value_reps: Vec::new(),
        optimization_hints: Vec::new(),
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
            escape_status: Vec::new(),
        }
    }
}
