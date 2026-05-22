mod ast;
mod hir;

use std::{fmt::Write as _, fs, path::Path};

use super::{
    backend, build_multi_section_file, lowered, split_file_name_sections, test262_preprocessor,
};
use crate::stages::eval_expand::expand_static_eval_fragments;
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_frontend::{Lexer, Parser, validate_type_reference_directives};
use ts2wasm_ir::builtin_resolved::ResolvedStmt;
use ts2wasm_ir::lowered::{LoweredProgram, Validated};
use ts2wasm_ir::optimizer::{OptimizationLevel, OptimizedHirProgram};
use ts2wasm_ir::semantic::HirProgram;
use ts2wasm_syntax::{SpannedToken, Stmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DumpPhase {
    All,
    Tokens,
    Ast,
    Resolved,
    TypedIr,
    OptimizedIr,
    Lowered,
    Wat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DumpOptions {
    pub phase: DumpPhase,
    pub unparse: bool,
    pub optimization_level: OptimizationLevel,
}

impl Default for DumpOptions {
    fn default() -> Self {
        Self {
            phase: DumpPhase::All,
            unparse: false,
            optimization_level: OptimizationLevel::O0,
        }
    }
}

impl DumpOptions {
    pub fn set_phase(&mut self, phase: DumpPhase) -> Result<(), String> {
        if self.phase != DumpPhase::All {
            return Err("dump accepts only one phase flag".to_owned());
        }
        self.phase = phase;
        Ok(())
    }

    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }
}

struct DumpPipeline {
    tokens: Vec<SpannedToken>,
    ast: Vec<Stmt>,
    resolved: Vec<ResolvedStmt>,
    typed_ir: Result<HirProgram, Diagnostic>,
    optimized_ir: Result<OptimizedHirProgram, Diagnostic>,
    lowered: LoweredProgram,
}

pub fn dump_file_with_options(input: &Path, options: DumpOptions) -> Result<String, Diagnostic> {
    if options.unparse
        && !matches!(
            options.phase,
            DumpPhase::Ast | DumpPhase::TypedIr | DumpPhase::OptimizedIr
        )
    {
        return Err(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "--unparse is currently supported only with --ast, --tir, or --optimize"
                .to_owned(),
            span: None,

            phase: None,
        });
    }

    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,

        phase: None,
    })?;
    let source = test262_preprocessor::process_test262_includes(input, &source)?;
    validate_type_reference_directives(&source)?;

    if matches!(options.phase, DumpPhase::Tokens) {
        let tokens = Lexer::new(&source).tokenize()?;
        return Ok(format_section("tokens", &format!("{tokens:#?}")));
    }

    if options.phase == DumpPhase::Ast && options.unparse {
        let ast = parse_ast(&source)?;
        return Ok(ast::unparse_program(&ast));
    }

    if options.phase == DumpPhase::Ast {
        let ast = parse_ast(&source)?;
        return Ok(format_section("ast", &format!("{ast:#?}")));
    }

    let sections = split_file_name_sections(&source);
    if !sections.is_empty()
        && matches!(
            options.phase,
            DumpPhase::All | DumpPhase::Lowered | DumpPhase::Wat
        )
    {
        let cr = build_multi_section_file(input, &sections, Path::new("/dev/null"), None, false)?;
        if options.phase == DumpPhase::All {
            return Ok(format!(
                "\n== multi-section build ==\n{}\nbuild: ok\n",
                cr.diagnostics
                    .iter()
                    .map(|d| format!("warning: {d}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
        return Ok("multi-section build: ok".to_owned());
    }

    let pipeline = build_dump_pipeline(input, &source, options.optimization_level)?;
    let mut out = String::new();
    match options.phase {
        DumpPhase::All => {
            push_section(&mut out, "tokens", &format!("{:#?}", pipeline.tokens));
            push_section(&mut out, "ast", &format!("{:#?}", pipeline.ast));
            push_section(&mut out, "resolved", &format!("{:#?}", pipeline.resolved));
            push_optional_typed_ir_section(&mut out, &pipeline.typed_ir)?;
            push_optional_optimized_ir_section(&mut out, &pipeline.optimized_ir)?;
            push_section(&mut out, "lowered", &format!("{:#?}", pipeline.lowered));
            let (validated, _) =
                Validated::new(pipeline.lowered.clone()).map_err(|d| Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: d.message,
                    span: d.span,
                    phase: None,
                })?;
            let wat = backend::emit_wat(&validated)?;
            push_section(&mut out, "wat", &wat);
        }
        DumpPhase::Resolved => {
            push_section(&mut out, "resolved", &format!("{:#?}", pipeline.resolved));
        }
        DumpPhase::TypedIr => {
            push_typed_ir_section(&mut out, &pipeline.typed_ir, options.unparse)?;
        }
        DumpPhase::OptimizedIr => {
            push_optimized_ir_section(&mut out, &pipeline.optimized_ir, options.unparse)?;
        }
        DumpPhase::Lowered => {
            push_section(&mut out, "lowered", &format!("{:#?}", pipeline.lowered));
        }
        DumpPhase::Wat => {
            let (validated, _) =
                Validated::new(pipeline.lowered.clone()).map_err(|d| Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: d.message,
                    span: d.span,
                    phase: None,
                })?;
            let wat = backend::emit_wat(&validated)?;
            push_section(&mut out, "wat", &wat);
        }
        DumpPhase::Tokens | DumpPhase::Ast => unreachable!("handled before full pipeline"),
    }

    Ok(out)
}

fn parse_ast(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens, source).parse_program()
}

fn build_dump_pipeline(
    input: &Path,
    source: &str,
    optimization_level: OptimizationLevel,
) -> Result<DumpPipeline, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    let ast = Parser::new(tokens.clone(), source).parse_program()?;
    eprintln!("[pipeline] validate_ast");
    super::validate_ast(&ast)?;
    eprintln!("[pipeline] module_graph");
    super::module_graph::validate_entry_module_graph(input, &ast)?;
    eprintln!("[pipeline] resolve_names");
    let name_resolved = ts2wasm_ir::name_resolver::resolve_names(&ast)?;
    eprintln!("[pipeline] resolve_builtins");
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&name_resolved)?;
    let resolved = expand_static_eval_fragments(resolved)?;
    super::validate_typescript_semantics_for_path(input, &resolved)?;
    eprintln!("[pipeline] build_typed_ir");
    let typed_ir = build_typed_ir(&resolved);
    let optimized_ir = typed_ir
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|typed_ir| optimize_typed_ir(typed_ir, optimization_level));
    eprintln!("[pipeline] lower_program");
    let lowered = lowered::lower_program(&resolved)?;
    eprintln!("[pipeline] validate_lowered");
    lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,

            phase: None,
        })
    })?;
    super::ensure_runtime_feature_gates(&lowered)?;

    Ok(DumpPipeline {
        tokens,
        ast,
        resolved,
        typed_ir,
        optimized_ir,
        lowered,
    })
}

fn build_typed_ir(resolved: &[ResolvedStmt]) -> Result<HirProgram, Diagnostic> {
    let typed_ir = ts2wasm_ir::semantic::lower_to_hir(resolved)?;
    ts2wasm_ir::semantic::validate_hir(&typed_ir).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_hir failed with empty diagnostic list".to_owned(),
            span: None,

            phase: None,
        })
    })?;
    Ok(typed_ir)
}

pub(crate) fn optimize_typed_ir(
    typed_ir: &HirProgram,
    level: OptimizationLevel,
) -> Result<OptimizedHirProgram, Diagnostic> {
    ts2wasm_ir::optimizer::optimize_hir(typed_ir, level)
}

fn format_section(name: &str, body: &str) -> String {
    let mut out = String::new();
    push_section(&mut out, name, body);
    out
}

fn push_section(out: &mut String, name: &str, body: &str) {
    let _ = writeln!(out, "== {name} ==");
    out.push_str(body.trim_end());
    out.push('\n');
}

fn push_typed_ir_section(
    out: &mut String,
    typed_ir: &Result<HirProgram, Diagnostic>,
    unparse: bool,
) -> Result<(), Diagnostic> {
    let typed_ir = typed_ir.as_ref().map_err(Clone::clone)?;
    if unparse {
        out.push_str(&hir::unparse_hir_program(typed_ir));
    } else {
        push_section(out, "typed-ir", &format!("{typed_ir:#?}"));
    }
    Ok(())
}

fn push_optional_typed_ir_section(
    out: &mut String,
    typed_ir: &Result<HirProgram, Diagnostic>,
) -> Result<(), Diagnostic> {
    match typed_ir {
        Ok(typed_ir) => {
            push_section(out, "typed-ir", &format!("{typed_ir:#?}"));
            Ok(())
        }
        Err(error) if error.code == DiagCode::UnsupportedSyntax => {
            push_section(
                out,
                "typed-ir",
                &format!("unsupported by initial HIR slice: {}", error.message),
            );
            Ok(())
        }
        Err(error) => Err(error.clone()),
    }
}

fn push_optimized_ir_section(
    out: &mut String,
    optimized_ir: &Result<OptimizedHirProgram, Diagnostic>,
    unparse: bool,
) -> Result<(), Diagnostic> {
    let optimized_ir = optimized_ir.as_ref().map_err(Clone::clone)?;
    if unparse {
        out.push_str(&hir::unparse_hir_program(&optimized_ir.hir));
    } else {
        push_section(out, "optimized-ir", &format!("{optimized_ir:#?}"));
    }
    Ok(())
}

fn push_optional_optimized_ir_section(
    out: &mut String,
    optimized_ir: &Result<OptimizedHirProgram, Diagnostic>,
) -> Result<(), Diagnostic> {
    match optimized_ir {
        Ok(optimized_ir) => {
            push_section(out, "optimized-ir", &format!("{optimized_ir:#?}"));
            Ok(())
        }
        Err(error) if error.code == DiagCode::UnsupportedSyntax => {
            push_section(
                out,
                "optimized-ir",
                &format!("unsupported by initial optimizer slice: {}", error.message),
            );
            Ok(())
        }
        Err(error) => Err(error.clone()),
    }
}
