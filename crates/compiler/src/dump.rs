use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ts2wasm_frontend::{
    BinaryOp, DiagCode, Diagnostic, Expr, Lexer, LogicalAssignOp, Parser, SpannedToken, Stmt,
    UnaryOp, validate_type_reference_directives,
};
use ts2wasm_ir::builtin::BuiltinId;
use ts2wasm_ir::builtin_resolved::ResolvedStmt;
use ts2wasm_ir::lowered::LoweredProgram;
use ts2wasm_ir::optimizer::{OptimizationLevel, OptimizedHirProgram};
use ts2wasm_ir::semantic::{HirExpr, HirProgram, HirRelationalOp, HirStmt};

use super::{backend, builtin_resolver, lowered, name_resolver};

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
            code: DiagCode::UnsupportedSyntax,
            message: "--unparse is currently supported only with --ast, --tir, or --optimize"
                .to_owned(),
            span: None,
        });
    }

    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,
    })?;
    validate_type_reference_directives(&source)?;

    if matches!(options.phase, DumpPhase::Tokens) {
        let tokens = Lexer::new(&source).tokenize()?;
        return Ok(format_section("tokens", &format!("{tokens:#?}")));
    }

    if options.phase == DumpPhase::Ast && options.unparse {
        let ast = parse_ast(&source)?;
        return Ok(unparse_program(&ast));
    }

    if options.phase == DumpPhase::Ast {
        let ast = parse_ast(&source)?;
        return Ok(format_section("ast", &format!("{ast:#?}")));
    }

    let pipeline = build_dump_pipeline(&source, options.optimization_level)?;
    let mut out = String::new();

    match options.phase {
        DumpPhase::All => {
            push_section(&mut out, "tokens", &format!("{:#?}", pipeline.tokens));
            push_section(&mut out, "ast", &format!("{:#?}", pipeline.ast));
            push_section(&mut out, "resolved", &format!("{:#?}", pipeline.resolved));
            push_optional_typed_ir_section(&mut out, &pipeline.typed_ir)?;
            push_optional_optimized_ir_section(&mut out, &pipeline.optimized_ir)?;
            push_section(&mut out, "lowered", &format!("{:#?}", pipeline.lowered));
            let wat = backend::emit_wat(&pipeline.lowered)?;
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
            let wat = backend::emit_wat(&pipeline.lowered)?;
            push_section(&mut out, "wat", &wat);
        }
        DumpPhase::Tokens | DumpPhase::Ast => unreachable!("handled before full pipeline"),
    }

    Ok(out)
}

fn parse_ast(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    Parser::new(tokens).parse_program()
}

fn build_dump_pipeline(
    source: &str,
    optimization_level: OptimizationLevel,
) -> Result<DumpPipeline, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    let ast = Parser::new(tokens.clone()).parse_program()?;
    super::validate_ast(&ast)?;
    let name_resolved = name_resolver::resolve_names(&ast)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    let typed_ir = build_typed_ir(&resolved);
    let optimized_ir = typed_ir
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|typed_ir| optimize_typed_ir(typed_ir, optimization_level));
    let lowered = lowered::lower_program(&resolved)?;
    lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
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
        out.push_str(&unparse_hir_program(typed_ir));
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
        out.push_str(&unparse_hir_program(&optimized_ir.hir));
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

fn unparse_program(program: &[Stmt]) -> String {
    let mut out = String::new();
    for stmt in program {
        unparse_stmt(&mut out, stmt, 0);
    }
    out
}

fn unparse_stmt(out: &mut String, stmt: &Stmt, indent: usize) {
    write_indent(out, indent);
    match stmt {
        Stmt::ImportSideEffect { specifier, .. } => {
            let _ = writeln!(out, "import '{}';", specifier.value);
        }
        Stmt::ImportNamed {
            specifiers, source, ..
        } => {
            let specifiers = specifiers
                .iter()
                .map(|specifier| {
                    if specifier.imported == specifier.local {
                        specifier.imported.clone()
                    } else {
                        format!("{} as {}", specifier.imported, specifier.local)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "import {{ {specifiers} }} from '{}';", source.value);
        }
        Stmt::ImportDefault {
            specifier, source, ..
        } => {
            let _ = writeln!(out, "import {} from '{}';", specifier.local, source.value);
        }
        Stmt::ImportNamespace {
            specifier, source, ..
        } => {
            let _ = writeln!(
                out,
                "import * as {} from '{}';",
                specifier.local, source.value
            );
        }
        Stmt::ExportNamed { specifiers, .. } => {
            let specifiers = specifiers
                .iter()
                .map(|specifier| {
                    if specifier.local == specifier.exported {
                        specifier.local.clone()
                    } else {
                        format!("{} as {}", specifier.local, specifier.exported)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "export {{ {specifiers} }};");
        }
        Stmt::Let { name, expr, .. } => {
            let _ = writeln!(out, "let {name} = {};", unparse_expr(expr));
        }
        Stmt::Assign { name, expr, .. } => {
            let _ = writeln!(out, "{name} = {};", unparse_expr(expr));
        }
        Stmt::Expr { expr, .. } => {
            let _ = writeln!(out, "{};", unparse_expr(expr));
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let _ = writeln!(out, "if ({}) {{", unparse_expr(condition));
            unparse_block(out, then_body, indent + 1);
            write_indent(out, indent);
            if else_body.is_empty() {
                let _ = writeln!(out, "}}");
            } else {
                let _ = writeln!(out, "}} else {{");
                unparse_block(out, else_body, indent + 1);
                write_indent(out, indent);
                let _ = writeln!(out, "}}");
            }
        }
        Stmt::While {
            condition, body, ..
        } => {
            let _ = writeln!(out, "while ({}) {{", unparse_expr(condition));
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::Function {
            name, params, body, ..
        } => {
            let params = params
                .iter()
                .map(|(name, default, rest)| {
                    let prefix = if *rest { "..." } else { "" };
                    match default {
                        Some(expr) => format!("{prefix}{name} = {}", unparse_expr(expr)),
                        None => format!("{prefix}{name}"),
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "function {name}({params}) {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::Return { expr, .. } => {
            let _ = writeln!(out, "return {};", unparse_expr(expr));
        }
        Stmt::ClassDecl {
            name,
            extends,
            body,
            ..
        } => {
            let extends = extends
                .as_ref()
                .map(|expr| format!(" extends {}", unparse_expr(expr)))
                .unwrap_or_default();
            let _ = writeln!(out, "class {name}{extends} {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::TryCatch {
            try_block,
            catch_param,
            catch_block,
            finally_block,
            ..
        } => {
            let _ = writeln!(out, "try {{");
            unparse_block(out, try_block, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
            if let Some(catch_block) = catch_block {
                write_indent(out, indent);
                let param = catch_param
                    .as_ref()
                    .map(|name| format!(" ({name})"))
                    .unwrap_or_default();
                let _ = writeln!(out, "catch{param} {{");
                unparse_block(out, catch_block, indent + 1);
                write_indent(out, indent);
                let _ = writeln!(out, "}}");
            }
            if let Some(finally_block) = finally_block {
                write_indent(out, indent);
                let _ = writeln!(out, "finally {{");
                unparse_block(out, finally_block, indent + 1);
                write_indent(out, indent);
                let _ = writeln!(out, "}}");
            }
        }
        Stmt::Throw { expr, .. } => {
            let _ = writeln!(out, "throw {};", unparse_expr(expr));
        }
        Stmt::Switch { expr, cases, .. } => {
            let _ = writeln!(out, "switch ({}) {{", unparse_expr(expr));
            for (case_expr, body) in cases {
                write_indent(out, indent + 1);
                match case_expr {
                    Some(expr) => {
                        let _ = writeln!(out, "case {}:", unparse_expr(expr));
                    }
                    None => {
                        let _ = writeln!(out, "default:");
                    }
                }
                unparse_block(out, body, indent + 2);
            }
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::DoWhile {
            body, condition, ..
        } => {
            let _ = writeln!(out, "do {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}} while ({});", unparse_expr(condition));
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            ..
        } => {
            let init = init.as_deref().map(unparse_for_init).unwrap_or_default();
            let condition = condition.as_ref().map(unparse_expr).unwrap_or_default();
            let update = update.as_ref().map(unparse_expr).unwrap_or_default();
            let _ = writeln!(out, "for ({init}; {condition}; {update}) {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::ForIn {
            var, iter, body, ..
        } => {
            let _ = writeln!(out, "for (let {var} in {}) {{", unparse_expr(iter));
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::ForOf {
            var, iter, body, ..
        } => {
            let _ = writeln!(out, "for (let {var} of {}) {{", unparse_expr(iter));
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::Labeled { label, body, .. } => {
            let _ = writeln!(out, "{label}:");
            unparse_stmt(out, body, indent + 1);
        }
        Stmt::Break { label, .. } => {
            let suffix = label
                .as_ref()
                .map(|label| format!(" {label}"))
                .unwrap_or_default();
            let _ = writeln!(out, "break{suffix};");
        }
        Stmt::Continue { label, .. } => {
            let suffix = label
                .as_ref()
                .map(|label| format!(" {label}"))
                .unwrap_or_default();
            let _ = writeln!(out, "continue{suffix};");
        }
    }
}

fn unparse_block(out: &mut String, body: &[Stmt], indent: usize) {
    for stmt in body {
        unparse_stmt(out, stmt, indent);
    }
}

fn unparse_for_init(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Let { name, expr, .. } => format!("let {name} = {}", unparse_expr(expr)),
        Stmt::Assign { name, expr, .. } => format!("{name} = {}", unparse_expr(expr)),
        Stmt::Expr { expr, .. } => unparse_expr(expr),
        other => format!("{other:?}"),
    }
}

fn unparse_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number { value, .. } => value.to_string(),
        Expr::String { value, .. } => format!("{:?}", value),
        Expr::Bool { value, .. } => value.to_string(),
        Expr::Null { .. } => "null".to_owned(),
        Expr::Undefined { .. } => "undefined".to_owned(),
        Expr::Ident { name, .. } => name.clone(),
        Expr::Unary { op, expr, .. } => match op {
            UnaryOp::Not => format!("!{}", unparse_expr(expr)),
            UnaryOp::Negate => format!("-{}", unparse_expr(expr)),
            UnaryOp::Increment => format!("{}++", unparse_expr(expr)),
            UnaryOp::Decrement => format!("{}--", unparse_expr(expr)),
            UnaryOp::PreIncrement => format!("++{}", unparse_expr(expr)),
            UnaryOp::PreDecrement => format!("--{}", unparse_expr(expr)),
            UnaryOp::TypeOf => format!("typeof {}", unparse_expr(expr)),
            UnaryOp::BitwiseNot => format!("~{}", unparse_expr(expr)),
            UnaryOp::Delete => format!("delete {}", unparse_expr(expr)),
            UnaryOp::Void => format!("void {}", unparse_expr(expr)),
        },
        Expr::Binary {
            left, op, right, ..
        } => format!(
            "({} {} {})",
            unparse_expr(left),
            binary_op_text(*op),
            unparse_expr(right)
        ),
        Expr::Member {
            object, property, ..
        } => format!("{}.{}", unparse_expr(object), property),
        Expr::Call { callee, args, .. } => {
            format!("{}({})", unparse_expr(callee), unparse_expr_list(args))
        }
        Expr::Assign { name, expr, .. } => format!("{name} = {}", unparse_expr(expr)),
        Expr::LogicalAssign { name, op, expr, .. } => {
            format!(
                "{name} {} {}",
                logical_assign_op_text(*op),
                unparse_expr(expr)
            )
        }
        Expr::LogicalPropertyAssign {
            object,
            property,
            op,
            expr,
            ..
        } => {
            format!(
                "{object}.{property} {} {}",
                logical_assign_op_text(*op),
                unparse_expr(expr)
            )
        }
        Expr::Array { elements, .. } => format!("[{}]", unparse_expr_list(elements)),
        Expr::Object { props, .. } => {
            let props = props
                .iter()
                .map(|(key, value)| format!("{key}: {}", unparse_expr(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{props}}}")
        }
        Expr::Index { object, index, .. } => {
            format!("{}[{}]", unparse_expr(object), unparse_expr(index))
        }
        Expr::New { expr, args, .. } => {
            format!("new {}({})", unparse_expr(expr), unparse_expr_list(args))
        }
        Expr::TypeOf { expr, .. } => format!("typeof {}", unparse_expr(expr)),
        Expr::InstanceOf {
            expr, type_expr, ..
        } => format!(
            "({} instanceof {})",
            unparse_expr(expr),
            unparse_expr(type_expr)
        ),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => format!(
            "({} ? {} : {})",
            unparse_expr(condition),
            unparse_expr(then_expr),
            unparse_expr(else_expr)
        ),
        Expr::ArrowFn { params, body, .. } => {
            format!("({}) => {}", params.join(", "), unparse_expr(body))
        }
        Expr::Spread { expr, .. } => format!("...{}", unparse_expr(expr)),
        Expr::PropertyAssign {
            object,
            property,
            value,
            ..
        } => format!(
            "{}.{} = {}",
            unparse_expr(object),
            property,
            unparse_expr(value)
        ),
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => format!(
            "{}[{}] = {}",
            unparse_expr(object),
            unparse_expr(index),
            unparse_expr(value)
        ),
        Expr::This { .. } => "this".to_owned(),
    }
}

fn unparse_expr_list(exprs: &[Expr]) -> String {
    exprs
        .iter()
        .map(unparse_expr)
        .collect::<Vec<_>>()
        .join(", ")
}

fn binary_op_text(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Subtract => "-",
        BinaryOp::Less => "<",
        BinaryOp::LessEqual => "<=",
        BinaryOp::Greater => ">",
        BinaryOp::GreaterEqual => ">=",
        BinaryOp::StrictEqual => "===",
        BinaryOp::EqualEqual => "==",
        BinaryOp::BangEqual => "!=",
        BinaryOp::StrictNotEqual => "!==",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::Multiply => "*",
        BinaryOp::Divide => "/",
        BinaryOp::Modulo => "%",
        BinaryOp::Power => "**",
        BinaryOp::BitwiseAnd => "&",
        BinaryOp::BitwiseOr => "|",
        BinaryOp::BitwiseXor => "^",
        BinaryOp::LeftShift => "<<",
        BinaryOp::RightShift => ">>",
        BinaryOp::UnsignedRightShift => ">>>",
        BinaryOp::In => "in",
        BinaryOp::InstanceOf => "instanceof",
    }
}

fn logical_assign_op_text(op: LogicalAssignOp) -> &'static str {
    match op {
        LogicalAssignOp::And => "&&=",
        LogicalAssignOp::Or => "||=",
        LogicalAssignOp::Nullish => "??=",
    }
}

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn unparse_hir_program(program: &HirProgram) -> String {
    let mut out = String::new();
    for stmt in &program.body {
        unparse_hir_stmt(&mut out, stmt, 0);
    }
    for function in &program.functions {
        let params = function
            .params
            .iter()
            .map(|local| format!("local${}", local.0))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(out, "function fn${}({params}) {{", function.id.0);
        for stmt in &function.body {
            unparse_hir_stmt(&mut out, stmt, 1);
        }
        let _ = writeln!(out, "}}");
    }
    out
}

fn unparse_hir_stmt(out: &mut String, stmt: &HirStmt, indent: usize) {
    write_indent(out, indent);
    match stmt {
        HirStmt::Let { local, init } => {
            let _ = writeln!(out, "let local${} = {};", local.0, unparse_hir_expr(init));
        }
        HirStmt::StoreLocal { local, value } => {
            let _ = writeln!(out, "local${} = {};", local.0, unparse_hir_expr(value));
        }
        HirStmt::Expr(expr) => {
            let _ = writeln!(out, "{};", unparse_hir_expr(expr));
        }
        HirStmt::BranchIfTruthy {
            condition,
            then_body,
            else_body,
        } => {
            let _ = writeln!(out, "if ({}) {{", unparse_hir_expr(condition));
            for stmt in then_body {
                unparse_hir_stmt(out, stmt, indent + 1);
            }
            write_indent(out, indent);
            if else_body.is_empty() {
                let _ = writeln!(out, "}}");
            } else {
                let _ = writeln!(out, "}} else {{");
                for stmt in else_body {
                    unparse_hir_stmt(out, stmt, indent + 1);
                }
                write_indent(out, indent);
                let _ = writeln!(out, "}}");
            }
        }
        HirStmt::LoopWhile { condition, body } => {
            let _ = writeln!(out, "while ({}) {{", unparse_hir_expr(condition));
            for stmt in body {
                unparse_hir_stmt(out, stmt, indent + 1);
            }
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        HirStmt::Return(expr) => {
            let _ = writeln!(out, "return {};", unparse_hir_expr(expr));
        }
    }
}

fn unparse_hir_expr(expr: &HirExpr) -> String {
    match expr {
        HirExpr::ConstUndefined => "undefined".to_owned(),
        HirExpr::ConstNull => "null".to_owned(),
        HirExpr::ConstBool(value) => value.to_string(),
        HirExpr::ConstNumber(value) => value.to_string(),
        HirExpr::ConstString(value) => format!("{value:?}"),
        HirExpr::LoadLocal(local) => format!("local${}", local.0),
        HirExpr::LoadBuiltin(name) => format!("builtin::{name}"),
        HirExpr::ToBoolean(expr) => format!("ToBoolean({})", unparse_hir_expr(expr)),
        HirExpr::JsUnaryNot(expr) => format!("!{}", unparse_hir_expr(expr)),
        HirExpr::JsAdd { left, right } => format!(
            "JsAdd({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsStrictEqual { left, right } => format!(
            "JsStrictEqual({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsAbstractEqual { left, right } => format!(
            "JsAbstractEqual({}, {})",
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::JsRelational { op, left, right } => format!(
            "JsRelational({}, {}, {})",
            hir_relational_op_text(*op),
            unparse_hir_expr(left),
            unparse_hir_expr(right)
        ),
        HirExpr::GetProp { object, key } => {
            format!("GetProp({}, {key:?})", unparse_hir_expr(object))
        }
        HirExpr::GetIndex { object, index } => format!(
            "GetIndex({}, {})",
            unparse_hir_expr(object),
            unparse_hir_expr(index)
        ),
        HirExpr::ArrayLength(expr) => format!("ArrayLength({})", unparse_hir_expr(expr)),
        HirExpr::CallBuiltin { builtin, args } => format!(
            "{}({})",
            hir_builtin_name(*builtin),
            unparse_hir_expr_list(args)
        ),
        HirExpr::CallFunction { function, args } => {
            format!("fn${}({})", function.0, unparse_hir_expr_list(args))
        }
        HirExpr::CallMethod {
            receiver,
            method,
            args,
        } => format!(
            "CallMethod({}, {method:?}, [{}])",
            unparse_hir_expr(receiver),
            unparse_hir_expr_list(args)
        ),
    }
}

fn unparse_hir_expr_list(exprs: &[HirExpr]) -> String {
    exprs
        .iter()
        .map(unparse_hir_expr)
        .collect::<Vec<_>>()
        .join(", ")
}

fn hir_relational_op_text(op: HirRelationalOp) -> &'static str {
    match op {
        HirRelationalOp::Less => "<",
        HirRelationalOp::LessEqual => "<=",
        HirRelationalOp::Greater => ">",
        HirRelationalOp::GreaterEqual => ">=",
    }
}

fn hir_builtin_name(builtin: BuiltinId) -> &'static str {
    match builtin {
        BuiltinId::ConsoleLog => "console.log",
        BuiltinId::ReadStdinUtf8 => "readStdinUtf8",
        BuiltinId::FsReadFileSync => "fs.readFileSync",
        BuiltinId::FsWriteFileSync => "fs.writeFileSync",
        BuiltinId::FsAppendFileSync => "fs.appendFileSync",
        BuiltinId::ProcessArgv => "process.argv",
        BuiltinId::ProcessEnv => "process.env",
        BuiltinId::ProcessExit => "process.exit",
        BuiltinId::PathJoin => "path.join",
        BuiltinId::PathResolve => "path.resolve",
        BuiltinId::PathBasename => "path.basename",
        BuiltinId::PathDirname => "path.dirname",
        BuiltinId::CryptoRandomBytes => "crypto.randomBytes",
        BuiltinId::InstanceOf => "instanceof",
    }
}
