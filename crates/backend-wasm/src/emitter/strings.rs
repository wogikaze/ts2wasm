use crate::emitter::WatEmitter;
use crate::runtime_fn::StringOrigin;
use ts2wasm_ir::lowered::{LoweredExpr, LoweredStmt};

impl WatEmitter<'_> {
    pub(super) fn intern_required_runtime_strings(&mut self) {
        let runtime_strings: Vec<_> = self
            .link_plan
            .required_runtime_strings()
            .iter()
            .copied()
            .collect();
        for value in runtime_strings {
            // Use the first origin from the RuntimeFn catalog as the representative origin.
            let origin = self
                .link_plan
                .string_origins()
                .get(value)
                .and_then(|origins| origins.first())
                .map(|rf| StringOrigin::Runtime(*rf))
                .unwrap_or(StringOrigin::UserLiteral);
            self.intern_string_with_origin(value, origin);
        }
    }

    pub(super) fn collect_program_strings(&mut self, statements: &[LoweredStmt]) {
        for statement in statements {
            self.collect_statement_strings(statement);
        }
    }

    fn collect_statement_strings(&mut self, statement: &LoweredStmt) {
        match statement {
            LoweredStmt::Block(statements, _) => {
                self.collect_program_strings(statements);
            }
            LoweredStmt::Let(_, expr, _)
            | LoweredStmt::Assign(_, expr, _)
            | LoweredStmt::Expr(expr, _)
            | LoweredStmt::Return(expr, _)
            | LoweredStmt::Throw(expr, _) => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(then_body);
                self.collect_program_strings(else_body);
            }
            LoweredStmt::While {
                condition, body, ..
            } => {
                self.collect_expr_strings(condition);
                self.collect_program_strings(body);
            }
            LoweredStmt::TryFinally {
                try_body,
                finally_body,
                ..
            } => {
                self.collect_program_strings(try_body);
                self.collect_program_strings(finally_body);
            }
            LoweredStmt::TryCatch {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                self.collect_program_strings(try_body);
                if let Some(body) = catch_body {
                    self.collect_program_strings(body);
                }
                if let Some(body) = finally_body {
                    self.collect_program_strings(body);
                }
            }
            LoweredStmt::Switch { expr, cases, .. } => {
                self.collect_expr_strings(expr);
                for (cond, body) in cases {
                    if let Some(cond_expr) = cond {
                        self.collect_expr_strings(cond_expr);
                    }
                    self.collect_program_strings(body);
                }
            }
            LoweredStmt::DoWhile {
                body, condition, ..
            } => {
                self.collect_program_strings(body);
                self.collect_expr_strings(condition);
            }
            LoweredStmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(init_stmt) = init {
                    self.collect_statement_strings(init_stmt);
                }
                if let Some(cond) = condition {
                    self.collect_expr_strings(cond);
                }
                if let Some(upd) = update {
                    self.collect_expr_strings(upd);
                }
                self.collect_program_strings(body);
            }
            LoweredStmt::ForIn {
                var: _, iter, body, ..
            } => {
                self.collect_expr_strings(iter);
                self.collect_program_strings(body);
            }
            LoweredStmt::ForOf {
                var: _, iter, body, ..
            } => {
                self.collect_expr_strings(iter);
                self.collect_program_strings(body);
            }
            LoweredStmt::Labeled { body, .. } => self.collect_statement_strings(body),
            LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
            LoweredStmt::Export { name, expr, .. } => {
                self.intern_string(name);
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ModuleExportsAssign { expr, .. } => {
                self.collect_expr_strings(expr);
            }
            LoweredStmt::ClassDecl {
                methods,
                static_methods,
                ..
            } => {
                for (name, _) in methods.iter().chain(static_methods.iter()) {
                    self.intern_string(name);
                }
            }
        }
    }

    fn collect_expr_strings(&mut self, expr: &LoweredExpr) {
        match expr {
            LoweredExpr::String(value, _) => {
                self.intern_string(value);
            }
            LoweredExpr::BigIntLiteral { decimal, .. } => {
                self.intern_string(decimal);
            }
            LoweredExpr::Number(_, _)
            | LoweredExpr::Bool(_, _)
            | LoweredExpr::Null(..)
            | LoweredExpr::Undefined(..)
            | LoweredExpr::This(..)
            | LoweredExpr::Local(_, _) => {}
            LoweredExpr::PromiseGetValue { promise, .. } => {
                self.collect_expr_strings(promise);
            }
            LoweredExpr::ArrowFn { .. } => {}
            LoweredExpr::Unary { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::Assign { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::EnvCellNew(expr, _) => self.collect_expr_strings(expr),
            LoweredExpr::EnvCellGet(_, _) => {}
            LoweredExpr::EnvCellSet { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::LogicalAssign { expr, .. } => self.collect_expr_strings(expr),
            LoweredExpr::LogicalPropertyAssign { key, expr, .. } => {
                self.intern_string(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalMemberAssign {
                object, key, expr, ..
            } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                self.collect_expr_strings(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(key);
                self.collect_expr_strings(expr);
            }
            LoweredExpr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            LoweredExpr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::PropertyDelete { object, key, .. } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
            }
            LoweredExpr::PropertyDeleteDynamic { object, key, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(key);
            }
            LoweredExpr::PropertyIn { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::PropertyInDynamic { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::ArrayNew { elements, .. } => {
                for elem in elements {
                    self.collect_expr_strings(elem);
                }
            }
            LoweredExpr::ArrayNewSparse { slots, .. } => {
                for slot in slots {
                    if let ts2wasm_ir::lowered::LoweredArraySlot::Present(elem) = slot {
                        self.collect_expr_strings(elem);
                    }
                }
            }
            LoweredExpr::ArrayGet { arr, index, .. } => {
                self.collect_expr_strings(arr);
                self.collect_expr_strings(index);
            }
            LoweredExpr::Index { object, index, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
            }
            LoweredExpr::GetLength(inner, _) => {
                self.collect_expr_strings(inner);
            }
            LoweredExpr::ObjectNew { props, .. } => {
                for (key, val) in props {
                    self.intern_string(key);
                    self.collect_expr_strings(val);
                }
            }
            LoweredExpr::ErrorNew {
                constructor,
                message,
                ..
            } => {
                self.intern_string("message");
                self.intern_string("stack");
                self.intern_string(super::builtin_error_stack_prefix(*constructor));
                self.collect_expr_strings(message);
            }
            LoweredExpr::PropertyGet { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::OptionalPropertyGet { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.intern_string(key);
            }
            LoweredExpr::PropertyGetDynamic { obj, key, .. } => {
                self.collect_expr_strings(obj);
                self.collect_expr_strings(key);
            }
            LoweredExpr::OptionalIndex { object, index, .. } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
            }
            LoweredExpr::OptionalCall { callee, call, .. } => {
                self.collect_expr_strings(callee);
                self.collect_expr_strings(call);
            }
            LoweredExpr::MethodCall { object, .. } => {
                self.collect_expr_strings(object);
            }
            LoweredExpr::PropertySet {
                object, key, value, ..
            } => {
                self.collect_expr_strings(object);
                self.intern_string(key);
                self.collect_expr_strings(value);
            }
            LoweredExpr::PropertySetDynamic {
                object,
                index,
                value,
                ..
            } => {
                self.collect_expr_strings(object);
                self.collect_expr_strings(index);
                self.collect_expr_strings(value);
            }
            LoweredExpr::New { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
            LoweredExpr::ClassPrototype(_, _) | LoweredExpr::BuiltinErrorPrototype(_, _) => {}
            LoweredExpr::Block { stmts, result, .. } => {
                for stmt in stmts {
                    self.collect_statement_strings(stmt);
                }
                self.collect_expr_strings(result);
            }
            LoweredExpr::ModuleLoad { .. } => {}
            LoweredExpr::RuntimeCall { args, .. } => {
                for arg in args {
                    self.collect_expr_strings(arg);
                }
            }
        }
    }
}
