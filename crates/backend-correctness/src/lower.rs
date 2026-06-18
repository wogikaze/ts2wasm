use ts2wasm_semantic_ir::block::{SemBlock, Terminator};
use ts2wasm_semantic_ir::expr::SemExpr;
use ts2wasm_semantic_ir::program::SemProgram;
use ts2wasm_semantic_ir::reference::{RefBase, RefName, SemReference};
use ts2wasm_semantic_ir::stmt::SemStmt;
use ts2wasm_semantic_ir::value::ValueRef;
use ts2wasm_spec_kernel::SpecOp;

pub struct LoweredSpec {
    pub ops: Vec<(SpecOp, ts2wasm_source::Span)>,
    pub locals: usize,
}

pub struct CorrectnessLowering;

impl CorrectnessLowering {
    pub fn lower(program: &SemProgram) -> LoweredSpec {
        let mut ops = Vec::new();
        let mut locals = 0;

        for block in &program.top_level_blocks {
            Self::lower_block(block, &mut ops, &mut locals);
        }

        LoweredSpec { ops, locals }
    }

    fn lower_block(
        block: &SemBlock,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        for stmt in &block.stmts {
            Self::lower_stmt(stmt, ops, locals);
        }
        Self::lower_terminator(&block.terminator, ops, locals);
    }

    fn lower_stmt(
        stmt: &SemStmt,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        match stmt {
            SemStmt::Let { init, .. } => {
                if let Some(init_expr) = init {
                    Self::lower_expr(init_expr, ops, locals);
                }
            }
            SemStmt::Assign { value, .. } | SemStmt::Expr(value, _) => {
                Self::lower_expr(value, ops, locals);
            }
            SemStmt::GetValue {
                reference, span, ..
            } => {
                Self::lower_get_value(reference, *span, ops, locals);
            }
            SemStmt::CreateLexicalBinding { env, name, span } => {
                ops.push((
                    SpecOp::CreateBinding {
                        env: *env,
                        name: name.clone(),
                        mutable: true,
                    },
                    *span,
                ));
            }
            SemStmt::InitializeBinding {
                env,
                name,
                value,
                span,
            } => {
                ops.push((
                    SpecOp::InitializeBinding {
                        env: *env,
                        name: name.clone(),
                        value: Self::value_ref_local(value, locals),
                    },
                    *span,
                ));
            }
            SemStmt::PutValue {
                reference,
                value,
                span,
            } => {
                Self::lower_put_value(reference, value, *span, ops, locals);
            }
            SemStmt::EnterContext { span, .. } => {
                ops.push((
                    SpecOp::Call {
                        callee: Self::allocate_synthetic_local(locals),
                        this: Self::allocate_synthetic_local(locals),
                        args: Self::allocate_synthetic_local(locals),
                    },
                    *span,
                ));
            }
            SemStmt::LeaveContext(_) => {}
            SemStmt::GetBindingValue {
                env, name, span, ..
            } => {
                ops.push((
                    SpecOp::GetBindingValue {
                        env: *env,
                        name: name.clone(),
                    },
                    *span,
                ));
            }
            SemStmt::SetMutableBinding {
                env,
                name,
                value,
                span,
            } => {
                ops.push((
                    SpecOp::SetMutableBinding {
                        env: *env,
                        name: name.clone(),
                        value: Self::value_ref_local(value, locals),
                    },
                    *span,
                ));
            }
            SemStmt::ResolveBinding {
                name, env, span, ..
            } => {
                ops.push((
                    SpecOp::ResolveBinding {
                        name: name.clone(),
                        env: *env,
                    },
                    *span,
                ));
            }
            SemStmt::MakeReference { .. } => {}
            SemStmt::IteratorNext { iterator, span, .. } => {
                ops.push((
                    SpecOp::IteratorNext {
                        iterator: Self::value_ref_local(iterator, locals),
                    },
                    *span,
                ));
            }
            SemStmt::IteratorClose {
                iterator,
                completion,
                span,
            } => {
                ops.push((
                    SpecOp::IteratorClose {
                        iterator: Self::value_ref_local(iterator, locals),
                        completion: Self::value_ref_local(completion, locals),
                    },
                    *span,
                ));
            }
        }
    }

    fn lower_expr(
        expr: &SemExpr,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        match expr {
            SemExpr::Constant(_, _) | SemExpr::Local(_, _) => {}
            SemExpr::Unary { .. } | SemExpr::Binary { .. } => {}
            SemExpr::PropertyGet { object, key, span } => {
                let object = Self::expr_local(object, locals);
                let key = Self::materialize_string_key_local(key, ops, locals, *span);
                ops.push((
                    SpecOp::Get {
                        object,
                        key,
                        receiver: object,
                    },
                    *span,
                ));
            }
            SemExpr::Call { callee, span, .. } => {
                ops.push((
                    SpecOp::Call {
                        callee: Self::expr_local(callee, locals),
                        this: Self::allocate_synthetic_local(locals),
                        args: Self::allocate_synthetic_local(locals),
                    },
                    *span,
                ));
            }
            SemExpr::Construct {
                constructor,
                new_target,
                span,
                ..
            } => {
                let constructor_local = Self::expr_local(constructor, locals);
                let new_target_local = new_target
                    .as_deref()
                    .map(|expr| Self::expr_local(expr, locals))
                    .unwrap_or(constructor_local);
                ops.push((
                    SpecOp::Construct {
                        constructor: constructor_local,
                        args: Self::allocate_synthetic_local(locals),
                        new_target: new_target_local,
                    },
                    *span,
                ));
            }
            SemExpr::PropertyGetDynamic { object, key, span } => {
                let object = Self::expr_local(object, locals);
                ops.push((
                    SpecOp::Get {
                        object,
                        key: Self::expr_local(key, locals),
                        receiver: object,
                    },
                    *span,
                ));
            }
            SemExpr::PropertySet {
                object,
                key,
                value,
                span,
            } => {
                let object = Self::expr_local(object, locals);
                let key = Self::materialize_string_key_local(key, ops, locals, *span);
                let value = Self::expr_local(value, locals);
                ops.push((
                    SpecOp::Set {
                        object,
                        key,
                        value,
                        receiver: object,
                    },
                    *span,
                ));
            }
            SemExpr::ArrayLiteral { .. }
            | SemExpr::ObjectLiteral { .. }
            | SemExpr::FunctionExpr { .. }
            | SemExpr::This(_)
            | SemExpr::Super { .. }
            | SemExpr::Import { .. }
            | SemExpr::Reference(_, _) => {}
        }
    }

    fn lower_get_value(
        reference: &SemReference,
        span: ts2wasm_source::Span,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        match &reference.base {
            RefBase::Value(base) | RefBase::Super(base) => {
                let object = Self::value_ref_local(base, locals);
                let key = Self::reference_name_local(&reference.name, ops, locals, span);
                let receiver = reference
                    .this_value
                    .as_ref()
                    .map(|value| Self::value_ref_local(value, locals))
                    .unwrap_or(object);
                ops.push((
                    SpecOp::Get {
                        object,
                        key,
                        receiver,
                    },
                    span,
                ));
            }
            RefBase::Env(env) => {
                ops.push((
                    SpecOp::GetBindingValue {
                        env: *env,
                        name: Self::reference_name_string(&reference.name),
                    },
                    span,
                ));
            }
            RefBase::Unresolvable => {
                ops.push((
                    SpecOp::ResolveBinding {
                        name: Self::reference_name_string(&reference.name),
                        env: 0,
                    },
                    span,
                ));
            }
        }
    }

    fn lower_put_value(
        reference: &SemReference,
        value: &ValueRef,
        span: ts2wasm_source::Span,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        match &reference.base {
            RefBase::Value(base) | RefBase::Super(base) => {
                let object = Self::value_ref_local(base, locals);
                let key = Self::reference_name_local(&reference.name, ops, locals, span);
                let val = Self::value_ref_local(value, locals);
                let receiver = reference
                    .this_value
                    .as_ref()
                    .map(|this_value| Self::value_ref_local(this_value, locals))
                    .unwrap_or(object);
                ops.push((
                    SpecOp::Set {
                        object,
                        key,
                        value: val,
                        receiver,
                    },
                    span,
                ));
            }
            RefBase::Env(env) => {
                ops.push((
                    SpecOp::SetMutableBinding {
                        env: *env,
                        name: Self::reference_name_string(&reference.name),
                        value: Self::value_ref_local(value, locals),
                    },
                    span,
                ));
            }
            RefBase::Unresolvable => {
                ops.push((
                    SpecOp::SetMutableBinding {
                        env: 0,
                        name: Self::reference_name_string(&reference.name),
                        value: Self::value_ref_local(value, locals),
                    },
                    span,
                ));
            }
        }
    }

    fn reference_name_string(name: &RefName) -> String {
        match name {
            RefName::Id(name) | RefName::PrivateName(name) => name.clone(),
            RefName::Key(_) => String::new(),
        }
    }

    fn reference_name_local(
        name: &RefName,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
        span: ts2wasm_source::Span,
    ) -> u32 {
        match name {
            RefName::Id(name) | RefName::PrivateName(name) => {
                Self::materialize_string_key_local(name, ops, locals, span)
            }
            RefName::Key(value) => Self::value_ref_local(value, locals),
        }
    }

    fn expr_local(expr: &SemExpr, locals: &mut usize) -> u32 {
        match expr {
            SemExpr::Local(local, _) => *local,
            _ => Self::allocate_synthetic_local(locals),
        }
    }

    fn value_ref_local(value: &ValueRef, locals: &mut usize) -> u32 {
        match value {
            ValueRef::Local(local) | ValueRef::Argument(local) => *local,
            ValueRef::Constant(_) => Self::allocate_synthetic_local(locals),
        }
    }

    fn materialize_string_key_local(
        key: &str,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
        span: ts2wasm_source::Span,
    ) -> u32 {
        let local = Self::allocate_synthetic_local(locals);
        ops.push((
            SpecOp::PushStringConstant {
                local,
                value: key.to_owned(),
            },
            span,
        ));
        local
    }

    fn allocate_synthetic_local(locals: &mut usize) -> u32 {
        let local = *locals as u32;
        *locals += 1;
        local
    }

    fn lower_terminator(
        term: &Terminator,
        ops: &mut Vec<(SpecOp, ts2wasm_source::Span)>,
        locals: &mut usize,
    ) {
        match term {
            Terminator::Return(value, span) => {
                let local = Self::value_ref_local(value, locals);
                ops.push((SpecOp::Return { value: local }, *span));
            }
            Terminator::Throw(value, span) => {
                let local = Self::value_ref_local(value, locals);
                ops.push((
                    SpecOp::Throw {
                        value: local,
                        catch: None,
                    },
                    *span,
                ));
            }
            Terminator::Branch { .. }
            | Terminator::Jump(_, _)
            | Terminator::UnwindTo { .. }
            | Terminator::Switch { .. } => {}
        }
    }
}
