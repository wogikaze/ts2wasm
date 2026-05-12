//! MIR (Mid-level IR) to WAT emission bridge.
//!
//! This module provides a feature-gated backend emission path that accepts
//! `Validated<MirProgram>` instead of `Validated<LoweredProgram>`.
//! It converts MirProgram to LoweredProgram and delegates to the standard
//! emitter. When the HIR/MIR migration matures and the backend is refactored,
//! this bridge should be replaced with a native MIR emitter.
//!
//! Feature gate: This path is explicitly named (`emit_mir_wat` / `emit_mir_wat_validated`)
//! and is not the default. The existing `Validated<LoweredProgram>` path remains
//! unchanged until MIR parity is proven.

use ts2wasm_ir::lowered::{
    FunctionCallKind, LoweredExpr, LoweredFunction, LoweredProgram, LoweredStmt, RuntimeFn,
    Validated,
    mir::{MirExpr, MirFunction, MirProgram, MirStmt},
};
use ts2wasm_source::Span;

use crate::{Diagnostic, emitter};

/// Emit WAT from a `MirProgram` by converting it to `LoweredProgram` and
/// delegating to the standard emitter.
pub fn emit_mir_wat(program: &MirProgram) -> Result<String, Diagnostic> {
    let lowered = mir_program_to_lowered(program);
    emitter::emit_wat(&lowered)
}

/// Emit WAT from a `Validated<MirProgram>`.
pub fn emit_mir_wat_validated(program: &Validated<MirProgram>) -> Result<String, Diagnostic> {
    emit_mir_wat(program.as_ref())
}

// ---------------------------------------------------------------------------
// Conversion: MirProgram → LoweredProgram
// ---------------------------------------------------------------------------

fn mir_program_to_lowered(mir: &MirProgram) -> LoweredProgram {
    LoweredProgram {
        top_level_statements: mir
            .top_level_statements
            .iter()
            .map(mir_stmt_to_lowered)
            .collect(),
        top_level_locals: mir.top_level_locals.clone(),
        functions: mir.functions.iter().map(mir_function_to_lowered).collect(),
        modules: mir.modules.clone(),
    }
}

fn mir_function_to_lowered(mf: &MirFunction) -> LoweredFunction {
    LoweredFunction {
        id: mf.id,
        params: mf.params.clone(),
        uses_receiver: mf.uses_receiver,
        min_required_params: mf.min_required_params,
        rest_param_index: mf.rest_param_index,
        locals: mf.locals.clone(),
        body: mf.body.iter().map(mir_stmt_to_lowered).collect(),
        recursion_depth: mf.recursion_depth,
        is_async: mf.is_async,
    }
}

fn mir_stmt_to_lowered(s: &MirStmt) -> LoweredStmt {
    let span = Span::generated("mir");
    match s {
        MirStmt::Let { local, init } => LoweredStmt::Let(*local, mir_expr_to_lowered(init), span),
        MirStmt::Assign { local, init } => {
            LoweredStmt::Assign(*local, mir_expr_to_lowered(init), span)
        }
        MirStmt::Expr(e) => LoweredStmt::Expr(mir_expr_to_lowered(e), span),
        MirStmt::If {
            condition,
            then_body,
            else_body,
        } => LoweredStmt::If {
            condition: mir_expr_to_lowered(condition),
            then_body: then_body.iter().map(mir_stmt_to_lowered).collect(),
            else_body: else_body.iter().map(mir_stmt_to_lowered).collect(),
            span,
        },
        MirStmt::While { condition, body } => LoweredStmt::While {
            condition: mir_expr_to_lowered(condition),
            body: body.iter().map(mir_stmt_to_lowered).collect(),
            span,
        },
        MirStmt::Return(e) => LoweredStmt::Return(mir_expr_to_lowered(e), span),
        MirStmt::Throw(e) => LoweredStmt::Throw(mir_expr_to_lowered(e), span),
        MirStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        } => LoweredStmt::TryCatch {
            try_body: try_body.iter().map(mir_stmt_to_lowered).collect(),
            catch_var: *catch_var,
            catch_body: catch_body
                .as_ref()
                .map(|b| b.iter().map(mir_stmt_to_lowered).collect()),
            finally_body: finally_body
                .as_ref()
                .map(|b| b.iter().map(mir_stmt_to_lowered).collect()),
            span,
        },
        MirStmt::Switch { expr, cases } => LoweredStmt::Switch {
            expr: mir_expr_to_lowered(expr),
            cases: cases
                .iter()
                .map(|(cond, body)| {
                    (
                        cond.as_ref().map(|c| mir_expr_to_lowered(c)),
                        body.iter().map(mir_stmt_to_lowered).collect(),
                    )
                })
                .collect(),
            span,
        },
        MirStmt::Labeled { label, body } => LoweredStmt::Labeled {
            label: label.clone(),
            body: Box::new(mir_stmt_to_lowered(body)),
            span,
        },
        MirStmt::Break { label } => LoweredStmt::Break {
            label: label.clone(),
            span,
        },
        MirStmt::Continue { label } => LoweredStmt::Continue {
            label: label.clone(),
            span,
        },
        MirStmt::ClassDecl {
            name,
            extends,
            constructor,
            methods,
            static_methods,
            private_fields,
        } => LoweredStmt::ClassDecl {
            name: name.clone(),
            extends: extends.clone(),
            constructor: *constructor,
            methods: methods.clone(),
            static_methods: static_methods.clone(),
            private_fields: private_fields.clone(),
            span,
        },
        MirStmt::Export { name, expr } => LoweredStmt::Export {
            name: name.clone(),
            expr: mir_expr_to_lowered(expr),
            span,
        },
        MirStmt::ModuleExportsAssign { expr } => LoweredStmt::ModuleExportsAssign {
            expr: mir_expr_to_lowered(expr),
            span,
        },
    }
}

fn mir_expr_to_lowered(e: &MirExpr) -> LoweredExpr {
    let span = Span::generated("mir");
    match e {
        MirExpr::I32Const(v) => LoweredExpr::Number(*v, span),
        MirExpr::StringConst(s) => LoweredExpr::String(s.clone(), span),
        MirExpr::Local(l) => LoweredExpr::Local(*l, span),
        MirExpr::CallRuntime { intrinsic, args } => LoweredExpr::RuntimeCall {
            intrinsic: *intrinsic,
            args: args.iter().map(mir_expr_to_lowered).collect(),
            span,
        },
        MirExpr::CallFunction { func, args } => LoweredExpr::Call {
            kind: FunctionCallKind::User(*func),
            args: args.iter().map(mir_expr_to_lowered).collect(),
            span,
        },
        MirExpr::CallClosure { closure, args } => {
            // Closures are represented in LoweredExpr as ArrowFn + User call.
            // The closure receiver is resolved at MIR level; bridge via RuntimeCall.
            let mut bridge_args = vec![mir_expr_to_lowered(closure)];
            bridge_args.extend(args.iter().map(mir_expr_to_lowered));
            LoweredExpr::RuntimeCall {
                intrinsic: RuntimeFn::HeapClosureCall,
                args: bridge_args,
                span,
            }
        }
        MirExpr::NewObject { props } => LoweredExpr::ObjectNew {
            props: props
                .iter()
                .map(|(k, v)| (k.clone(), mir_expr_to_lowered(v)))
                .collect(),
            non_enumerable: 0,
            span,
        },
        MirExpr::NewArray { elements } => LoweredExpr::ArrayNew {
            elements: elements.iter().map(mir_expr_to_lowered).collect(),
            span,
        },
        MirExpr::LoadModule { module_id } => LoweredExpr::ModuleLoad {
            module_id: *module_id,
            span,
        },
        MirExpr::Block { stmts, result } => LoweredExpr::Block {
            stmts: stmts.iter().map(mir_stmt_to_lowered).collect(),
            result: Box::new(mir_expr_to_lowered(result)),
            span,
        },
    }
}
