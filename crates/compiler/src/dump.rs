use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ts2wasm_frontend::{
    BinaryOp, DiagCode, Diagnostic, Expr, Lexer, Parser, SpannedToken, Stmt, UnaryOp,
};
use ts2wasm_ir::builtin_resolved::ResolvedStmt;
use ts2wasm_ir::lowered::LoweredProgram;

use super::{backend, builtin_resolver, lowered, name_resolver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DumpPhase {
    All,
    Tokens,
    Ast,
    Resolved,
    Lowered,
    Wat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DumpOptions {
    pub phase: DumpPhase,
    pub unparse: bool,
}

impl Default for DumpOptions {
    fn default() -> Self {
        Self {
            phase: DumpPhase::All,
            unparse: false,
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
}

struct DumpPipeline {
    tokens: Vec<SpannedToken>,
    ast: Vec<Stmt>,
    resolved: Vec<ResolvedStmt>,
    lowered: LoweredProgram,
}

pub fn dump_file_with_options(input: &Path, options: DumpOptions) -> Result<String, Diagnostic> {
    if options.unparse && options.phase != DumpPhase::Ast {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "--unparse is currently supported only with --ast".to_owned(),
            span: None,
        });
    }

    let source = fs::read_to_string(input).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", input.display()),
        span: None,
    })?;

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

    let pipeline = build_dump_pipeline(&source)?;
    let mut out = String::new();

    match options.phase {
        DumpPhase::All => {
            push_section(&mut out, "tokens", &format!("{:#?}", pipeline.tokens));
            push_section(&mut out, "ast", &format!("{:#?}", pipeline.ast));
            push_section(&mut out, "resolved", &format!("{:#?}", pipeline.resolved));
            push_section(&mut out, "lowered", &format!("{:#?}", pipeline.lowered));
            let wat = backend::emit_wat(&pipeline.lowered)?;
            push_section(&mut out, "wat", &wat);
        }
        DumpPhase::Resolved => {
            push_section(&mut out, "resolved", &format!("{:#?}", pipeline.resolved));
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

fn build_dump_pipeline(source: &str) -> Result<DumpPipeline, Diagnostic> {
    let tokens = Lexer::new(source).tokenize()?;
    let ast = Parser::new(tokens.clone()).parse_program()?;
    super::validate_ast(&ast)?;
    let name_resolved = name_resolver::resolve_names(&ast)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
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
        lowered,
    })
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

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}
