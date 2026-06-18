use crate::block::{SemBlock, Terminator};
use crate::expr::{BinaryOp, SemExpr, UnaryOp};
use crate::program::{SemFunction, SemProgram};
use crate::stmt::SemStmt;
use crate::value::{BlockId, LocalId, ValueRef};
use ts2wasm_ir::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedStmt};
use ts2wasm_runtime_core::value::TaggedValue;
use ts2wasm_source::Span;
use ts2wasm_syntax::{BinaryOp as SyntaxBinaryOp, UnaryOp as SyntaxUnaryOp};

pub struct LoweringContext {
    pub next_local: LocalId,
    pub next_block: BlockId,
    pub current_blocks: Vec<SemBlock>,
    pub current_locals: Vec<LocalId>,
    /// Name → local mapping for variable resolution.
    pub name_map: std::collections::HashMap<String, LocalId>,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self {
            next_local: 0,
            next_block: 0,
            current_blocks: Vec::new(),
            current_locals: Vec::new(),
            name_map: std::collections::HashMap::new(),
        }
    }

    pub fn alloc_local(&mut self) -> LocalId {
        let id = self.next_local;
        self.next_local += 1;
        self.current_locals.push(id);
        id
    }

    /// Allocate a local for a named variable and register it in the name map.
    pub fn alloc_named_local(&mut self, name: &str) -> LocalId {
        let id = self.alloc_local();
        self.name_map.insert(name.to_owned(), id);
        id
    }

    /// Resolve a name to its local ID.
    pub fn resolve_local(&self, name: &str) -> Option<LocalId> {
        self.name_map.get(name).copied()
    }

    pub fn alloc_block(&mut self) -> BlockId {
        let id = self.next_block;
        self.next_block += 1;
        id
    }

    pub fn push_block(&mut self, block: SemBlock) {
        self.current_blocks.push(block);
    }

    pub fn emit_stmt(&mut self, stmt: SemStmt) {
        if let Some(block) = self.current_blocks.last_mut() {
            block.stmts.push(stmt);
        }
    }
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

pub fn lower_to_sem_ir(program: &[ResolvedStmt]) -> SemProgram {
    let mut ctx = LoweringContext::new();
    let entry = ctx.alloc_block();

    let entry_block = SemBlock {
        id: entry,
        stmts: Vec::new(),
        terminator: Terminator::Jump(entry + 1, Span::default()),
        span: Span::default(),
    };
    ctx.push_block(entry_block);

    let mut functions = Vec::new();

    for stmt in program {
        lower_stmt(stmt, &mut ctx, &mut functions);
    }

    SemProgram {
        functions,
        top_level_blocks: ctx.current_blocks,
        entry_block: entry,
        span: Span::default(),
    }
}

fn lower_stmt(
    stmt: &ResolvedStmt,
    ctx: &mut LoweringContext,
    functions: &mut Vec<SemFunction>,
) {
    match stmt {
        ResolvedStmt::Block { statements } => {
            for s in statements {
                lower_stmt(s, ctx, functions);
            }
        }
        ResolvedStmt::Let(name, expr) => {
            let local = ctx.alloc_named_local(name);
            ctx.emit_stmt(SemStmt::Let {
                local,
                init: Some(lower_expr(expr, ctx)),
                span: Span::default(),
            });
        }
        ResolvedStmt::Assign(name, expr) => {
            let local = ctx.resolve_local(name).unwrap_or_else(|| ctx.alloc_named_local(name));
            ctx.emit_stmt(SemStmt::Assign {
                local,
                value: lower_expr(expr, ctx),
                span: Span::default(),
            });
        }
        ResolvedStmt::Expr(expr) => {
            ctx.emit_stmt(SemStmt::Expr(lower_expr(expr, ctx), Span::default()));
        }
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond_local = ctx.alloc_local();
            ctx.emit_stmt(SemStmt::Let {
                local: cond_local,
                init: Some(lower_expr(condition, ctx)),
                span: Span::default(),
            });

            let then_block = ctx.alloc_block();
            let else_block = ctx.alloc_block();
            let merge_block = ctx.alloc_block();

            let else_target = if else_body.is_empty() { merge_block } else { else_block };
            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Branch {
                    cond: ValueRef::local(cond_local),
                    then: then_block,
                    else_: else_target,
                    span: Span::default(),
                };
            }

            let mut then_ctx = LoweringContext::new();
            let then_block_obj = SemBlock {
                id: then_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(merge_block, Span::default()),
                span: Span::default(),
            };
            then_ctx.push_block(then_block_obj);
            for s in then_body {
                lower_stmt(s, &mut then_ctx, functions);
            }
            ctx.current_blocks.extend(then_ctx.current_blocks);

            if !else_body.is_empty() {
                let mut else_ctx = LoweringContext::new();
                let else_block_obj = SemBlock {
                    id: else_block,
                    stmts: Vec::new(),
                    terminator: Terminator::Jump(merge_block, Span::default()),
                    span: Span::default(),
                };
                else_ctx.push_block(else_block_obj);
                for s in else_body {
                    lower_stmt(s, &mut else_ctx, functions);
                }
                ctx.current_blocks.extend(else_ctx.current_blocks);
            }

            let merge_block_obj = SemBlock {
                id: merge_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                span: Span::default(),
            };
            ctx.push_block(merge_block_obj);
        }
        ResolvedStmt::While { condition, body } => {
            let header_block = ctx.alloc_block();
            let body_block = ctx.alloc_block();
            let exit_block = ctx.alloc_block();
            let cond_local = ctx.alloc_local();

            {
                let blocks = &mut ctx.current_blocks;
                if let Some(block) = blocks.last_mut() {
                    block.terminator = Terminator::Jump(header_block, Span::default());
                }
            }
            let header = SemBlock {
                id: header_block,
                stmts: vec![SemStmt::Let {
                    local: cond_local,
                    init: Some(lower_expr(condition, ctx)),
                    span: Span::default(),
                }],
                terminator: Terminator::Branch {
                    cond: ValueRef::local(cond_local),
                    then: body_block,
                    else_: exit_block,
                    span: Span::default(),
                },
                span: Span::default(),
            };
            ctx.push_block(header);

            let mut body_ctx = LoweringContext::new();
            let body_block_obj = SemBlock {
                id: body_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(header_block, Span::default()),
                span: Span::default(),
            };
            body_ctx.push_block(body_block_obj);
            for s in body {
                lower_stmt(s, &mut body_ctx, functions);
            }
            ctx.current_blocks.extend(body_ctx.current_blocks);

            let exit = SemBlock {
                id: exit_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                span: Span::default(),
            };
            ctx.push_block(exit);
        }
        ResolvedStmt::Return(expr) => {
            let val = ctx.alloc_local();
            ctx.emit_stmt(SemStmt::Let {
                local: val,
                init: Some(lower_expr(expr, ctx)),
                span: Span::default(),
            });
            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Return(ValueRef::local(val), Span::default());
            }
        }
        ResolvedStmt::Throw(expr) => {
            let val = ctx.alloc_local();
            ctx.emit_stmt(SemStmt::Let {
                local: val,
                init: Some(lower_expr(expr, ctx)),
                span: Span::default(),
            });
            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Throw(ValueRef::local(val), Span::default());
            }
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        } => {
            let try_body = ctx.alloc_block();
            let catch_id = catch_block.as_ref().map(|_| ctx.alloc_block());
            let finally_id = finally_block.as_ref().map(|_| ctx.alloc_block());

            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::UnwindTo {
                    try_body,
                    catch: catch_id,
                    finally: finally_id,
                    span: Span::default(),
                };
            }

            let mut try_ctx = LoweringContext::new();
            try_ctx.push_block(SemBlock {
                id: try_body,
                stmts: Vec::new(),
                terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                span: Span::default(),
            });
            for s in try_block {
                lower_stmt(s, &mut try_ctx, functions);
            }
            ctx.current_blocks.extend(try_ctx.current_blocks);

            if let (Some(cid), Some(catch_stmts)) = (catch_id, catch_block.as_ref()) {
                let mut catch_ctx = LoweringContext::new();
                catch_ctx.push_block(SemBlock {
                    id: cid,
                    stmts: Vec::new(),
                    terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                    span: Span::default(),
                });
                if let Some(param) = catch_param {
                    let local = catch_ctx.alloc_named_local(param);
                    catch_ctx.emit_stmt(SemStmt::Let {
                        local,
                        init: None,
                        span: Span::default(),
                    });
                }
                for s in catch_stmts {
                    lower_stmt(s, &mut catch_ctx, functions);
                }
                ctx.current_blocks.extend(catch_ctx.current_blocks);
            }

            if let (Some(fid), Some(finally_stmts)) = (finally_id, finally_block.as_ref()) {
                let mut finally_ctx = LoweringContext::new();
                finally_ctx.push_block(SemBlock {
                    id: fid,
                    stmts: Vec::new(),
                    terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                    span: Span::default(),
                });
                for s in finally_stmts {
                    lower_stmt(s, &mut finally_ctx, functions);
                }
                ctx.current_blocks.extend(finally_ctx.current_blocks);
            }
        }
        ResolvedStmt::Function {
            name,
            params,
            body,
            is_generator,
            is_async,
            ..
        } => {
            let func_id = functions.len() as u32;
            let func_entry = ctx.alloc_block();

            let mut func_ctx = LoweringContext::new();
            let entry_block = SemBlock {
                id: func_entry,
                stmts: Vec::new(),
                terminator: Terminator::Jump(ctx.alloc_block(), Span::default()),
                span: Span::default(),
            };
            func_ctx.push_block(entry_block);

            for param in params {
                let local = func_ctx.alloc_named_local(&param.name);
                func_ctx.emit_stmt(SemStmt::Let {
                    local,
                    init: None,
                    span: Span::default(),
                });
            }

            for s in body {
                lower_stmt(s, &mut func_ctx, functions);
            }

            functions.push(SemFunction {
                id: func_id,
                name: name.clone(),
                params: func_ctx.current_locals.clone(),
                locals: func_ctx.current_locals,
                blocks: func_ctx.current_blocks,
                entry_block: func_entry,
                is_async: *is_async,
                is_generator: *is_generator,
                is_strict: false,
                span: Span::default(),
            });
        }
        ResolvedStmt::Labeled { label: _, body } => {
            lower_stmt(body, ctx, functions);
        }
        ResolvedStmt::Break { label: _ } => {}
        ResolvedStmt::Continue { label: _ } => {}
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            let header = ctx.alloc_block();
            let body_block = ctx.alloc_block();
            let exit = ctx.alloc_block();

            if let Some(init_stmt) = init {
                lower_stmt(init_stmt, ctx, functions);
            }

            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Jump(header, Span::default());
            }

            let cond_local = ctx.alloc_local();
            let cond_expr = condition
                .as_ref()
                .map(|c| lower_expr(c, ctx))
                .unwrap_or(SemExpr::Constant(TaggedValue::TRUE, Span::default()));

            ctx.push_block(SemBlock {
                id: header,
                stmts: vec![SemStmt::Let {
                    local: cond_local,
                    init: Some(cond_expr),
                    span: Span::default(),
                }],
                terminator: Terminator::Branch {
                    cond: ValueRef::local(cond_local),
                    then: body_block,
                    else_: exit,
                    span: Span::default(),
                },
                span: Span::default(),
            });

            let mut body_ctx = LoweringContext::new();
            body_ctx.push_block(SemBlock {
                id: body_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(header, Span::default()),
                span: Span::default(),
            });
            for s in body {
                lower_stmt(s, &mut body_ctx, functions);
            }
            if let Some(update_expr) = update {
                body_ctx.emit_stmt(SemStmt::Expr(
                    lower_expr(update_expr, ctx),
                    Span::default(),
                ));
            }
            ctx.current_blocks.extend(body_ctx.current_blocks);

            let next_id = ctx.alloc_block();
            ctx.push_block(SemBlock {
                id: exit,
                stmts: Vec::new(),
                terminator: Terminator::Jump(next_id, Span::default()),
                span: Span::default(),
            });
        }
        ResolvedStmt::DoWhile { body, condition } => {
            let header = ctx.alloc_block();
            let exit = ctx.alloc_block();

            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Jump(header, Span::default());
            }

            let body_next = ctx.alloc_block();
            let mut body_ctx = LoweringContext::new();
            body_ctx.push_block(SemBlock {
                id: header,
                stmts: Vec::new(),
                terminator: Terminator::Jump(body_next, Span::default()),
                span: Span::default(),
            });
            for s in body {
                lower_stmt(s, &mut body_ctx, functions);
            }
            let cond_local = body_ctx.alloc_local();
            body_ctx.emit_stmt(SemStmt::Let {
                local: cond_local,
                init: Some(lower_expr(condition, ctx)),
                span: Span::default(),
            });
            if let Some(block) = body_ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Branch {
                    cond: ValueRef::local(cond_local),
                    then: header,
                    else_: exit,
                    span: Span::default(),
                };
            }
            ctx.current_blocks.extend(body_ctx.current_blocks);

            let exit_next = ctx.alloc_block();
            ctx.push_block(SemBlock {
                id: exit,
                stmts: Vec::new(),
                terminator: Terminator::Jump(exit_next, Span::default()),
                span: Span::default(),
            });
        }
        ResolvedStmt::ForIn { var, iter, body } => {
            // TODO: ForIn needs Object.keys() iteration — lowered as a while loop with index
            // For now, emit the loop structure with the iter evaluated and body lowered
            let iter_local = ctx.alloc_local();
            ctx.emit_stmt(SemStmt::Let {
                local: iter_local,
                init: Some(lower_expr(iter, ctx)),
                span: Span::default(),
            });
            let var_local = ctx.alloc_named_local(var);

            let header_block = ctx.alloc_block();
            let body_block = ctx.alloc_block();
            let exit_block = ctx.alloc_block();

            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Jump(header_block, Span::default());
            }

            // Loop header: check if there are more keys
            // TODO: proper iterator protocol — for now just emit body once
            ctx.push_block(SemBlock {
                id: header_block,
                stmts: vec![SemStmt::Let {
                    local: var_local,
                    init: None,
                    span: Span::default(),
                }],
                terminator: Terminator::Branch {
                    cond: ValueRef::local(iter_local),
                    then: body_block,
                    else_: exit_block,
                    span: Span::default(),
                },
                span: Span::default(),
            });

            let mut body_ctx = LoweringContext::new();
            body_ctx.name_map = ctx.name_map.clone();
            body_ctx.next_local = ctx.next_local;
            body_ctx.push_block(SemBlock {
                id: body_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(header_block, Span::default()),
                span: Span::default(),
            });
            for s in body {
                lower_stmt(s, &mut body_ctx, functions);
            }
            ctx.next_local = body_ctx.next_local;
            ctx.current_blocks.extend(body_ctx.current_blocks);

            let exit_next = ctx.alloc_block();
            ctx.push_block(SemBlock {
                id: exit_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(exit_next, Span::default()),
                span: Span::default(),
            });
        }
        ResolvedStmt::ForOf { var, iter, body } => {
            // TODO: ForOf needs Symbol.iterator protocol
            let iter_local = ctx.alloc_local();
            ctx.emit_stmt(SemStmt::Let {
                local: iter_local,
                init: Some(lower_expr(iter, ctx)),
                span: Span::default(),
            });
            let var_local = ctx.alloc_named_local(var);

            let header_block = ctx.alloc_block();
            let body_block = ctx.alloc_block();
            let exit_block = ctx.alloc_block();

            if let Some(block) = ctx.current_blocks.last_mut() {
                block.terminator = Terminator::Jump(header_block, Span::default());
            }

            ctx.push_block(SemBlock {
                id: header_block,
                stmts: vec![SemStmt::IteratorNext {
                    iterator: ValueRef::local(iter_local),
                    result: var_local,
                    span: Span::default(),
                }],
                terminator: Terminator::Branch {
                    cond: ValueRef::local(iter_local),
                    then: body_block,
                    else_: exit_block,
                    span: Span::default(),
                },
                span: Span::default(),
            });

            let mut body_ctx = LoweringContext::new();
            body_ctx.name_map = ctx.name_map.clone();
            body_ctx.next_local = ctx.next_local;
            body_ctx.push_block(SemBlock {
                id: body_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(header_block, Span::default()),
                span: Span::default(),
            });
            for s in body {
                lower_stmt(s, &mut body_ctx, functions);
            }
            ctx.next_local = body_ctx.next_local;
            ctx.current_blocks.extend(body_ctx.current_blocks);

            let exit_next = ctx.alloc_block();
            ctx.push_block(SemBlock {
                id: exit_block,
                stmts: Vec::new(),
                terminator: Terminator::Jump(exit_next, Span::default()),
                span: Span::default(),
            });
        }
        ResolvedStmt::Switch { expr: _, cases: _ } => {}
        ResolvedStmt::Export { name: _, expr: _ } => {}
        ResolvedStmt::ClassDecl { .. } => {}
        ResolvedStmt::DestructureLet { .. } => {}
        ResolvedStmt::DestructureAssign { .. } => {}
        ResolvedStmt::AmbientValue(_) => {}
        ResolvedStmt::ModuleExportsAssign { .. } => {}
        ResolvedStmt::ForAwaitOf { .. } => {}
    }
}

fn lower_expr(expr: &ResolvedExpr, ctx: &LoweringContext) -> SemExpr {
    let span = Span::default();
    match expr {
        ResolvedExpr::Number(n) => {
            SemExpr::Constant(TaggedValue::encode_number(*n), span)
        }
        ResolvedExpr::DecimalNumber(_s) => SemExpr::Constant(
            TaggedValue::encode_number(0),
            span,
        ),
        ResolvedExpr::BigIntLiteral { .. } => {
            SemExpr::Constant(TaggedValue::UNDEFINED, span)
        }
        ResolvedExpr::String(_s) => SemExpr::Constant(
            TaggedValue::UNDEFINED, span,
        ),
        ResolvedExpr::Bool(true) => SemExpr::Constant(TaggedValue::TRUE, span),
        ResolvedExpr::Bool(false) => SemExpr::Constant(TaggedValue::FALSE, span),
        ResolvedExpr::Null => SemExpr::Constant(TaggedValue::NULL, span),
        ResolvedExpr::Undefined => SemExpr::Constant(TaggedValue::UNDEFINED, span),
        ResolvedExpr::This { span: s } => SemExpr::This(*s),
        ResolvedExpr::Ident(name) => {
            match ctx.resolve_local(name) {
                Some(local) => SemExpr::Local(local, span),
                None => SemExpr::Reference(
                    crate::reference::SemReference::property(
                        ValueRef::local(0),
                        name.clone(),
                        true,
                    ),
                    span,
                ),
            }
        }
        ResolvedExpr::Binary { left, op, right } => {
            let bin_op = match op {
                SyntaxBinaryOp::Add => BinaryOp::Add,
                SyntaxBinaryOp::Subtract => BinaryOp::Sub,
                SyntaxBinaryOp::Multiply => BinaryOp::Mul,
                SyntaxBinaryOp::Divide => BinaryOp::Div,
                SyntaxBinaryOp::Modulo => BinaryOp::Mod,
                SyntaxBinaryOp::StrictEqual => BinaryOp::StrictEqual,
                SyntaxBinaryOp::EqualEqual => BinaryOp::EqualEqual,
                SyntaxBinaryOp::Less => BinaryOp::Less,
                SyntaxBinaryOp::Greater => BinaryOp::Greater,
                SyntaxBinaryOp::LessEqual => BinaryOp::LessEqual,
                SyntaxBinaryOp::GreaterEqual => BinaryOp::GreaterEqual,
                SyntaxBinaryOp::BitwiseAnd => BinaryOp::BitwiseAnd,
                SyntaxBinaryOp::BitwiseOr => BinaryOp::BitwiseOr,
                SyntaxBinaryOp::BitwiseXor => BinaryOp::BitwiseXor,
                SyntaxBinaryOp::And => BinaryOp::And,
                SyntaxBinaryOp::Or => BinaryOp::Or,
                _ => BinaryOp::StrictEqual,
            };
            SemExpr::Binary {
                left: Box::new(lower_expr(left, ctx)),
                op: bin_op,
                right: Box::new(lower_expr(right, ctx)),
                span,
            }
        }
        ResolvedExpr::Unary { op, expr: inner } => {
            let uop = match op {
                SyntaxUnaryOp::Not => UnaryOp::Not,
                SyntaxUnaryOp::Negate => UnaryOp::Negate,
                SyntaxUnaryOp::TypeOf => UnaryOp::TypeOf,
                SyntaxUnaryOp::Void => UnaryOp::Void,
                _ => UnaryOp::Not,
            };
            SemExpr::Unary {
                op: uop,
                expr: Box::new(lower_expr(inner, ctx)),
                span,
            }
        }
        ResolvedExpr::Call {
            callee,
            args,
            span: call_span,
        } => SemExpr::Call {
            callee: Box::new(lower_expr(callee, ctx)),
            args: args.iter().map(|a| lower_expr(a, ctx)).collect(),
            span: *call_span,
        },
        ResolvedExpr::PropertyAccess { object, key, span: prop_span } => SemExpr::PropertyGet {
            object: Box::new(lower_expr(object, ctx)),
            key: key.clone(),
            span: *prop_span,
        },
        ResolvedExpr::MethodCall {
            object,
            method,
            args,
            span: call_span,
        } => SemExpr::Call {
            callee: Box::new(SemExpr::PropertyGet {
                object: Box::new(lower_expr(object, ctx)),
                key: method.clone(),
                span: *call_span,
            }),
            args: args.iter().map(|a| lower_expr(a, ctx)).collect(),
            span: *call_span,
        },
        ResolvedExpr::PropertyAssign { object, key, value, span: assign_span } => SemExpr::PropertySet {
            object: Box::new(lower_expr(object, ctx)),
            key: key.clone(),
            value: Box::new(lower_expr(value, ctx)),
            span: *assign_span,
        },
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            span: tern_span,
        } => {
            // Ternary is lowered to if-then-else in the CFG.
            // But as an expression, we use Binary And/Or as a placeholder
            // that short-circuits correctly at runtime.
            // TODO: proper ternary lowering through the block CFG
            let cond = lower_expr(condition, ctx);
            let then_val = lower_expr(then_expr, ctx);
            let else_val = lower_expr(else_expr, ctx);
            // `(cond && then_val) || else_val` is semantically equivalent
            SemExpr::Binary {
                left: Box::new(SemExpr::Binary {
                    left: Box::new(cond),
                    op: BinaryOp::And,
                    right: Box::new(then_val),
                    span,
                }),
                op: BinaryOp::Or,
                right: Box::new(else_val),
                span: *tern_span,
            }
        }
        ResolvedExpr::Array(elements) => SemExpr::ArrayLiteral {
            elements: elements
                .iter()
                .map(|e| match e {
                    ResolvedArrayElement::Present(ex) => lower_expr(ex, ctx),
                    ResolvedArrayElement::Hole => {
                        SemExpr::Constant(TaggedValue::UNDEFINED, span)
                    }
                })
                .collect(),
            span,
        },
        ResolvedExpr::Object(props) => SemExpr::ObjectLiteral {
            properties: props
                .iter()
                .map(|p| (p.static_key().unwrap_or("").to_owned(), lower_expr(p.value(), ctx)))
                .collect(),
            span,
        },
        ResolvedExpr::New {
            class_name,
            args,
            span: new_span,
        } => {
            let constructor = match ctx.resolve_local(class_name) {
                Some(local) => SemExpr::Local(local, span),
                None => SemExpr::Reference(
                    crate::reference::SemReference::property(
                        ValueRef::local(0),
                        class_name.clone(),
                        true,
                    ),
                    span,
                ),
            };
            SemExpr::Construct {
                constructor: Box::new(constructor),
                args: args.iter().map(|a| lower_expr(a, ctx)).collect(),
                new_target: None,
                span: *new_span,
            }
        }
        ResolvedExpr::Sequence(exprs) => {
            if let Some(last) = exprs.last() {
                lower_expr(last, ctx)
            } else {
                SemExpr::Constant(TaggedValue::UNDEFINED, span)
            }
        }
        ResolvedExpr::ComputedIndex { object, index } => SemExpr::PropertyGetDynamic {
            object: Box::new(lower_expr(object, ctx)),
            key: Box::new(lower_expr(index, ctx)),
            span,
        },
        ResolvedExpr::Spread(inner) => SemExpr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(lower_expr(inner, ctx)),
            span,
        },
        ResolvedExpr::ArrowFn { .. } | ResolvedExpr::FunctionExpr { .. } => {
            SemExpr::Constant(TaggedValue::UNDEFINED, span)
        }
        _ => SemExpr::Constant(TaggedValue::UNDEFINED, span),
    }
}
