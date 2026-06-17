//! MIR (Mid-level IR) to WAT emission.
//!
//! This module provides a feature-gated backend emission path that accepts
//! `Validated<MirProgram>` instead of `Validated<LoweredProgram>`.
//! It emits a small native MIR subset directly and routes the rest through an
//! explicitly named compatibility fallback.
//!
//! Feature gate: This path is explicitly named (`emit_mir_wat` / `emit_mir_wat_validated`)
//! and is not the default. The existing `Validated<LoweredProgram>` path remains
//! unchanged until MIR parity is proven.

use ts2wasm_ir::lowered::{
    FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredProgram, MirExpr, MirFunction,
    MirProgram, MirStmt, RuntimeFn, Validated,
};
use ts2wasm_runtime_abi::ValueTag;

use crate::wat_writer::WatWriter;
use crate::{DiagCode, Diagnostic, emitter};

/// Emit WAT from a `MirProgram`.
///
/// The direct MIR emitter is intentionally narrow while MIR parity is still
/// gated. Unsupported MIR nodes use the named compatibility fallback so callers
/// can distinguish native coverage from legacy LoweredProgram emission in tests
/// and follow-up work.
pub fn emit_mir_wat(program: &MirProgram) -> Result<String, Diagnostic> {
    match emit_mir_wat_native_subset(program) {
        Ok(wat) => Ok(wat),
        Err(_) => emit_mir_wat_via_lowered_compat(program),
    }
}

/// Emit WAT from a `MirProgram` through the legacy LoweredProgram emitter.
pub fn emit_mir_wat_via_lowered_compat(program: &MirProgram) -> Result<String, Diagnostic> {
    let lowered: LoweredProgram = program.clone().into();
    emitter::emit_wat(&lowered)
}

/// Emit WAT from a `Validated<MirProgram>`.
pub fn emit_mir_wat_validated(program: &Validated<MirProgram>) -> Result<String, Diagnostic> {
    emit_mir_wat(program.as_ref())
}

fn emit_mir_wat_native_subset(program: &MirProgram) -> Result<String, Diagnostic> {
    NativeMirWatEmitter::new(program).emit()
}

struct NativeMirWatEmitter<'a> {
    program: &'a MirProgram,
    writer: WatWriter,
    labels: usize,
}

impl<'a> NativeMirWatEmitter<'a> {
    fn new(program: &'a MirProgram) -> Self {
        Self {
            program,
            writer: WatWriter::new(),
            labels: 0,
        }
    }

    fn emit(mut self) -> Result<String, Diagnostic> {
        if !self.program.modules.is_empty() {
            return Err(unsupported(
                None,
                "native MIR WAT subset does not emit modules",
            ));
        }
        self.writer.open_module();
        for function in &self.program.functions {
            self.emit_function(function)?;
        }
        self.emit_start()?;
        self.writer.close_module();
        Ok(self.writer.into_string())
    }

    fn emit_function(&mut self, function: &MirFunction) -> Result<(), Diagnostic> {
        if function.is_async || function.is_generator || function.generator_state.is_some() {
            return Err(unsupported(
                None,
                "native MIR WAT subset does not emit async or generator functions",
            ));
        }
        if function.uses_receiver || function.rest_param_index.is_some() {
            return Err(unsupported(
                None,
                "native MIR WAT subset does not emit receiver or rest-parameter functions",
            ));
        }
        let sig = self.writer.output_mut();
        sig.push_str(&format!("  (func ${} ", function_symbol(function.id)));
        for _ in &function.params {
            sig.push_str("(param i32) ");
        }
        sig.push_str("(result i32)\n");
        for _ in &function.locals {
            self.writer.line(4, "(local i32)");
        }
        self.emit_statements(&function.body, 4, true)?;
        self.writer
            .line_fmt(4, format_args!("(i32.const {})", ValueTag::UNDEFINED));
        self.writer.line(2, ")");
        Ok(())
    }

    fn emit_start(&mut self) -> Result<(), Diagnostic> {
        self.writer.line(2, "(func $_start (export \"_start\")");
        for _ in &self.program.top_level_locals {
            self.writer.line(4, "(local i32)");
        }
        self.emit_statements(&self.program.top_level_statements, 4, false)?;
        self.writer.line(2, ")");
        Ok(())
    }

    fn emit_statements(
        &mut self,
        statements: &[MirStmt],
        indent: usize,
        allow_return: bool,
    ) -> Result<(), Diagnostic> {
        for statement in statements {
            self.emit_statement(statement, indent, allow_return)?;
        }
        Ok(())
    }

    fn emit_statement(
        &mut self,
        statement: &MirStmt,
        indent: usize,
        allow_return: bool,
    ) -> Result<(), Diagnostic> {
        match statement {
            MirStmt::Block(statements, _) => self.emit_statements(statements, indent, allow_return),
            MirStmt::Let(local, expr, _) | MirStmt::Assign(local, expr, _) => {
                self.emit_expr(expr, indent)?;
                self.writer.local_set(indent, local_index(*local));
                Ok(())
            }
            MirStmt::Expr(expr, _) | MirStmt::Yield(expr, _) => {
                self.emit_expr(expr, indent)?;
                if expr_produces_value(expr) {
                    self.writer.drop(indent);
                }
                Ok(())
            }
            MirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.emit_condition(condition, indent)?;
                self.writer.r#if(indent);
                self.writer.then(indent);
                self.emit_statements(then_body, indent + 4, allow_return)?;
                self.writer.end(indent);
                if !else_body.is_empty() {
                    self.writer.r#else(indent);
                    self.emit_statements(else_body, indent + 4, allow_return)?;
                    self.writer.end(indent);
                }
                self.writer.end(indent);
                Ok(())
            }
            MirStmt::While {
                condition, body, ..
            } => {
                let exit_label = self.next_label("mir_while_exit");
                let loop_label = self.next_label("mir_while_loop");
                self.writer.block(indent, &exit_label);
                self.writer.r#loop(indent + 2, &loop_label);
                self.emit_condition(condition, indent + 4)?;
                self.writer.i32_eqz(indent + 4);
                self.writer.br_if(indent + 4, &exit_label);
                self.emit_statements(body, indent + 4, allow_return)?;
                self.writer.r#br(indent + 4, &loop_label);
                self.writer.end(indent + 2);
                self.writer.end(indent);
                Ok(())
            }
            MirStmt::Return(expr, span) => {
                if !allow_return {
                    return Err(unsupported(
                        Some(*span),
                        "native MIR WAT subset does not emit top-level return",
                    ));
                }
                self.emit_expr(expr, indent)?;
                self.writer.return_(indent);
                Ok(())
            }
            MirStmt::Throw(_, span) => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit throw",
            )),
            MirStmt::TryFinally { span, .. } | MirStmt::TryCatch { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit try/catch/finally",
            )),
            MirStmt::Switch { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit switch",
            )),
            MirStmt::DoWhile { span, .. }
            | MirStmt::For { span, .. }
            | MirStmt::ForIn { span, .. }
            | MirStmt::ForOf { span, .. }
            | MirStmt::ForAwaitOfLower { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit this loop form",
            )),
            MirStmt::Labeled { span, .. }
            | MirStmt::Break { span, .. }
            | MirStmt::Continue { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit labels, break, or continue",
            )),
            MirStmt::Export { span, .. }
            | MirStmt::ModuleExportsUpdate { span, .. }
            | MirStmt::ModuleExportsAssign { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit module exports",
            )),
            MirStmt::ClassDecl { span, .. } => Err(unsupported(
                Some(*span),
                "native MIR WAT subset does not emit class declarations",
            )),
        }
    }

    fn emit_expr(&mut self, expr: &MirExpr, indent: usize) -> Result<(), Diagnostic> {
        match expr {
            MirExpr::Number(value, span) => {
                if ValueTag::can_encode_number(*value) {
                    self.writer.line_fmt(
                        indent,
                        format_args!("(i32.const {})", ValueTag::encode_number(*value)),
                    );
                    Ok(())
                } else {
                    Err(unsupported(
                        Some(*span),
                        "native MIR WAT subset only emits tagged small-int numbers",
                    ))
                }
            }
            MirExpr::Bool(true, _) => {
                self.writer.i32_const(indent, ValueTag::TRUE);
                Ok(())
            }
            MirExpr::Bool(false, _) => {
                self.writer.i32_const(indent, ValueTag::FALSE);
                Ok(())
            }
            MirExpr::Null(_) => {
                self.writer.i32_const(indent, ValueTag::NULL);
                Ok(())
            }
            MirExpr::Undefined(_) => {
                self.writer.i32_const(indent, ValueTag::UNDEFINED);
                Ok(())
            }
            MirExpr::Local(local, _) => {
                self.writer.local_get(indent, local_index(*local));
                Ok(())
            }
            MirExpr::Assign { local, expr, .. } => {
                self.emit_expr(expr, indent)?;
                self.writer.local_tee(indent, local_index(*local));
                Ok(())
            }
            MirExpr::Binary {
                left,
                op,
                right,
                span,
            } => self.emit_binary_expr(left, *op, right, *span, indent),
            MirExpr::Call { kind, args, span } => self.emit_call_expr(*kind, args, *span, indent),
            MirExpr::RuntimeCall {
                intrinsic,
                args,
                span,
            } => self.emit_runtime_call_expr(*intrinsic, args, *span, indent),
            MirExpr::Block {
                stmts,
                result,
                span: _,
            } => {
                self.emit_statements(stmts, indent, false)?;
                self.emit_expr(result, indent)
            }
            _ => Err(unsupported(
                expr_span(expr),
                "native MIR WAT subset does not emit this expression",
            )),
        }
    }

    fn emit_binary_expr(
        &mut self,
        left: &MirExpr,
        op: LoweredBinaryOp,
        right: &MirExpr,
        span: ts2wasm_source::Span,
        indent: usize,
    ) -> Result<(), Diagnostic> {
        match op {
            LoweredBinaryOp::Add
            | LoweredBinaryOp::Subtract
            | LoweredBinaryOp::Multiply
            | LoweredBinaryOp::Less
            | LoweredBinaryOp::LessEqual
            | LoweredBinaryOp::Greater
            | LoweredBinaryOp::GreaterEqual
            | LoweredBinaryOp::StrictEqual
            | LoweredBinaryOp::EqualEqual
            | LoweredBinaryOp::BangEqual
            | LoweredBinaryOp::StrictNotEqual => {
                self.emit_small_int_payload(left, indent)?;
                self.emit_small_int_payload(right, indent)?;
                match op {
                    LoweredBinaryOp::Add => {
                        self.writer.i32_add(indent);
                        self.emit_encode_small_int_payload(indent);
                    }
                    LoweredBinaryOp::Subtract => {
                        self.writer.i32_sub(indent);
                        self.emit_encode_small_int_payload(indent);
                    }
                    LoweredBinaryOp::Multiply => {
                        self.writer.i32_mul(indent);
                        self.emit_encode_small_int_payload(indent);
                    }
                    LoweredBinaryOp::Less => self.emit_bool_from_i32_compare(indent, "lt_s"),
                    LoweredBinaryOp::LessEqual => self.emit_bool_from_i32_compare(indent, "le_s"),
                    LoweredBinaryOp::Greater => self.emit_bool_from_i32_compare(indent, "gt_s"),
                    LoweredBinaryOp::GreaterEqual => {
                        self.emit_bool_from_i32_compare(indent, "ge_s")
                    }
                    LoweredBinaryOp::StrictEqual | LoweredBinaryOp::EqualEqual => {
                        self.emit_bool_from_i32_compare(indent, "eq")
                    }
                    LoweredBinaryOp::BangEqual | LoweredBinaryOp::StrictNotEqual => {
                        self.emit_bool_from_i32_compare(indent, "ne")
                    }
                    _ => unreachable!(),
                }
                Ok(())
            }
            _ => Err(unsupported(
                Some(span),
                format!("native MIR WAT subset does not emit binary op {op:?}"),
            )),
        }
    }

    fn emit_call_expr(
        &mut self,
        kind: FunctionCallKind,
        args: &[MirExpr],
        span: ts2wasm_source::Span,
        indent: usize,
    ) -> Result<(), Diagnostic> {
        match kind {
            FunctionCallKind::User(func_id) => {
                let Some(function) = self.program.functions.get(func_id.0) else {
                    return Err(unsupported(
                        Some(span),
                        format!("native MIR WAT subset cannot resolve user call {func_id:?}"),
                    ));
                };
                if args.len() != function.params.len() {
                    return Err(unsupported(
                        Some(span),
                        "native MIR WAT subset only emits exact-arity user calls",
                    ));
                }
                for arg in args {
                    self.emit_expr(arg, indent)?;
                }
                self.writer
                    .line_fmt(indent, format_args!("(call ${})", function_symbol(func_id)));
                Ok(())
            }
            FunctionCallKind::Builtin(builtin) => Err(unsupported(
                Some(span),
                format!(
                    "native MIR WAT subset uses compatibility fallback for builtin {builtin:?}"
                ),
            )),
        }
    }

    fn emit_runtime_call_expr(
        &mut self,
        intrinsic: RuntimeFn,
        args: &[MirExpr],
        span: ts2wasm_source::Span,
        indent: usize,
    ) -> Result<(), Diagnostic> {
        match intrinsic {
            RuntimeFn::TruthyBool if args.len() == 1 => self.emit_condition(&args[0], indent),
            _ => Err(unsupported(
                Some(span),
                format!(
                    "native MIR WAT subset uses compatibility fallback for runtime call {intrinsic:?}"
                ),
            )),
        }
    }

    fn emit_condition(&mut self, expr: &MirExpr, indent: usize) -> Result<(), Diagnostic> {
        match expr {
            MirExpr::Bool(value, _) => {
                self.writer.i32_const(indent, i32::from(*value));
                Ok(())
            }
            MirExpr::Undefined(_) | MirExpr::Null(_) => {
                self.writer.i32_const(indent, 0);
                Ok(())
            }
            MirExpr::Number(value, span) if *value == 0 => {
                if ValueTag::can_encode_number(*value) {
                    self.writer.i32_const(indent, 0);
                    Ok(())
                } else {
                    Err(unsupported(
                        Some(*span),
                        "native MIR WAT subset only emits tagged small-int conditions",
                    ))
                }
            }
            MirExpr::Number(value, span) => {
                if ValueTag::can_encode_number(*value) {
                    self.writer.i32_const(indent, 1);
                    Ok(())
                } else {
                    Err(unsupported(
                        Some(*span),
                        "native MIR WAT subset only emits tagged small-int conditions",
                    ))
                }
            }
            _ => Err(unsupported(
                expr_span(expr),
                "native MIR WAT subset only emits literal conditions",
            )),
        }
    }

    fn emit_small_int_payload(&mut self, expr: &MirExpr, indent: usize) -> Result<(), Diagnostic> {
        match expr {
            MirExpr::Number(value, _) if ValueTag::can_encode_number(*value) => {
                self.writer.i32_const(indent, *value);
                Ok(())
            }
            _ => {
                self.emit_expr(expr, indent)?;
                self.writer.line_fmt(
                    indent,
                    format_args!("(i32.const {})", ValueTag::NUMBER_SHIFT),
                );
                self.writer.line_fmt(indent, format_args!("(i32.shr_s)"));
                Ok(())
            }
        }
    }

    fn emit_encode_small_int_payload(&mut self, indent: usize) {
        self.writer.line_fmt(
            indent,
            format_args!("(i32.const {})", ValueTag::NUMBER_SHIFT),
        );
        self.writer.line_fmt(indent, format_args!("(i32.shl)"));
        self.writer
            .line_fmt(indent, format_args!("(i32.const {})", ValueTag::NUMBER));
        self.writer.i32_or(indent);
    }

    fn emit_bool_from_i32_compare(&mut self, indent: usize, compare: &str) {
        self.writer
            .line_fmt(indent, format_args!("(i32.{compare})"));
        self.writer.line_fmt(
            indent,
            format_args!(
                "(select (i32.const {}) (i32.const {}))",
                ValueTag::TRUE,
                ValueTag::FALSE
            ),
        );
    }

    fn next_label(&mut self, prefix: &str) -> String {
        let id = self.labels;
        self.labels += 1;
        format!("{prefix}_{id}")
    }
}

fn expr_produces_value(expr: &MirExpr) -> bool {
    !matches!(
        expr,
        MirExpr::Call {
            kind: FunctionCallKind::Builtin(_),
            ..
        }
    )
}

fn local_index(local: LocalId) -> usize {
    local.0
}

fn function_symbol(id: FuncId) -> String {
    format!("func_{}", id.0)
}

fn unsupported(span: Option<ts2wasm_source::Span>, message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedRuntimeSubset,
        message: message.into(),
        span,
        phase: Some("mir-wat"),
    }
}

fn expr_span(expr: &MirExpr) -> Option<ts2wasm_source::Span> {
    match expr {
        MirExpr::Number(_, span)
        | MirExpr::DecimalNumber(_, span)
        | MirExpr::String(_, span)
        | MirExpr::Bool(_, span)
        | MirExpr::Null(span)
        | MirExpr::Undefined(span)
        | MirExpr::Local(_, span)
        | MirExpr::EnvCellNew(_, span)
        | MirExpr::EnvCellGet(_, span)
        | MirExpr::EnvCellSet { span, .. }
        | MirExpr::Unary { span, .. }
        | MirExpr::Binary { span, .. }
        | MirExpr::PropertyIn { span, .. }
        | MirExpr::PropertyInDynamic { span, .. }
        | MirExpr::Call { span, .. }
        | MirExpr::Assign { span, .. }
        | MirExpr::LogicalAssign { span, .. }
        | MirExpr::LogicalPropertyAssign { span, .. }
        | MirExpr::LogicalComputedPropertyAssign { span, .. }
        | MirExpr::LogicalComputedMemberAssign { span, .. }
        | MirExpr::LogicalMemberAssign { span, .. }
        | MirExpr::ArrayNew { span, .. }
        | MirExpr::ArrayNewSparse { span, .. }
        | MirExpr::ArrayGet { span, .. }
        | MirExpr::Index { span, .. }
        | MirExpr::GetLength(_, span)
        | MirExpr::ObjectNew { span, .. }
        | MirExpr::ErrorNew { span, .. }
        | MirExpr::PropertyGet { span, .. }
        | MirExpr::OptionalPropertyGet { span, .. }
        | MirExpr::PropertyGetDynamic { span, .. }
        | MirExpr::OptionalIndex { span, .. }
        | MirExpr::OptionalCall { span, .. }
        | MirExpr::MethodCall { span, .. }
        | MirExpr::PromiseGetValue { span, .. }
        | MirExpr::RuntimeCall { span, .. }
        | MirExpr::PropertySet { span, .. }
        | MirExpr::PropertyDelete { span, .. }
        | MirExpr::PropertyDeleteDynamic { span, .. }
        | MirExpr::PropertySetDynamic { span, .. }
        | MirExpr::New { span, .. }
        | MirExpr::ClassPrototype(_, span)
        | MirExpr::BuiltinErrorPrototype(_, span)
        | MirExpr::BuiltinConstructor(_, span)
        | MirExpr::ModuleLoad { span, .. }
        | MirExpr::Block { span, .. }
        | MirExpr::This(span)
        | MirExpr::ArrowFn { span, .. }
        | MirExpr::BigIntLiteral { span, .. } => Some(*span),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use ts2wasm_ir::lowered::{MirFunction, MirProgram};
    use ts2wasm_source::Span;

    fn span() -> Span {
        Span::generated("mir-wat-test")
    }

    fn simple_native_program() -> MirProgram {
        MirProgram {
            top_level_statements: vec![
                MirStmt::Let(LocalId(0), MirExpr::Number(2, span()), span()),
                MirStmt::Expr(
                    MirExpr::Call {
                        kind: FunctionCallKind::User(FuncId(0)),
                        args: vec![MirExpr::Local(LocalId(0), span())],
                        span: span(),
                    },
                    span(),
                ),
            ],
            top_level_locals: vec![LocalId(0)],
            functions: vec![MirFunction {
                id: FuncId(0),
                params: vec![LocalId(0)],
                uses_receiver: false,
                min_required_params: 1,
                rest_param_index: None,
                locals: vec![LocalId(1)],
                body: vec![
                    MirStmt::Let(
                        LocalId(1),
                        MirExpr::Binary {
                            left: Box::new(MirExpr::Local(LocalId(0), span())),
                            op: LoweredBinaryOp::Add,
                            right: Box::new(MirExpr::Number(1, span())),
                            span: span(),
                        },
                        span(),
                    ),
                    MirStmt::If {
                        condition: MirExpr::Bool(true, span()),
                        then_body: vec![MirStmt::Assign(
                            LocalId(1),
                            MirExpr::Number(4, span()),
                            span(),
                        )],
                        else_body: vec![MirStmt::Assign(
                            LocalId(1),
                            MirExpr::Number(5, span()),
                            span(),
                        )],
                        span: span(),
                    },
                    MirStmt::While {
                        condition: MirExpr::Bool(false, span()),
                        body: vec![MirStmt::Assign(
                            LocalId(1),
                            MirExpr::Number(6, span()),
                            span(),
                        )],
                        span: span(),
                    },
                    MirStmt::Return(MirExpr::Local(LocalId(1), span()), span()),
                ],
                recursion_depth: 0,
                is_async: false,
                is_generator: false,
                generator_state: None,
                induction_vars: vec![],
                escape_status: vec![],
                value_reps: vec![],
                optimization_hints: vec![],
            }],
            modules: vec![],
            escape_status: vec![],
        }
    }

    fn assert_wat2wasm(wat: &str) {
        let mut child = Command::new("wat2wasm")
            .arg("-")
            .arg("-o")
            .arg("/dev/null")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("wat2wasm should run");
        child
            .stdin
            .as_mut()
            .expect("wat2wasm stdin")
            .write_all(wat.as_bytes())
            .expect("write WAT to wat2wasm");
        let output = child.wait_with_output().expect("wait for wat2wasm");
        assert!(
            output.status.success(),
            "wat2wasm failed\nWAT:\n{wat}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn native_mir_wat_subset_emits_without_lowered_runtime_shell() {
        let wat = emit_mir_wat_native_subset(&simple_native_program()).expect("native MIR WAT");

        assert!(wat.contains("(func $func_0"));
        assert!(wat.contains("(call $func_0)"));
        assert!(wat.contains("(i32.add)"));
        assert!(wat.contains("(block $mir_while_exit_"));
        assert!(!wat.contains("$wasi_proc_exit"));
        assert_wat2wasm(&wat);
    }

    #[test]
    fn public_mir_wat_uses_native_subset_when_supported() {
        let (program, diagnostics) =
            Validated::new_mir(simple_native_program()).expect("native MIR should validate");
        assert!(diagnostics.is_empty());
        let wat = emit_mir_wat_validated(&program).expect("native MIR WAT");

        assert!(wat.contains("(func $_start"));
        assert!(!wat.contains("$heap"));
    }

    #[test]
    fn unsupported_native_mir_subset_has_precise_diagnostic() {
        let program = MirProgram {
            top_level_statements: vec![MirStmt::Expr(
                MirExpr::String("x".to_owned(), span()),
                span(),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
            escape_status: vec![],
        };

        let err = emit_mir_wat_native_subset(&program).expect_err("string needs compat fallback");
        assert_eq!(err.code, DiagCode::UnsupportedRuntimeSubset);
        assert_eq!(err.phase, Some("mir-wat"));
        assert!(err.message.contains("native MIR WAT subset"));
    }

    #[test]
    fn compatibility_fallback_remains_explicit_for_unsupported_mir() {
        let program = MirProgram {
            top_level_statements: vec![MirStmt::Expr(
                MirExpr::String("x".to_owned(), span()),
                span(),
            )],
            top_level_locals: vec![],
            functions: vec![],
            modules: vec![],
            escape_status: vec![],
        };

        let wat = emit_mir_wat(&program).expect("compatibility fallback WAT");
        assert!(wat.contains("$wasi_proc_exit"));
        assert!(wat.contains("(memory (export \"memory\")"));
    }
}
