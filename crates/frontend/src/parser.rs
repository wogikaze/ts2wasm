use crate::{
    BinaryOp, DiagCode, Diagnostic, ExportNamedSpecifier, Expr, ImportAttribute,
    ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier, ImportPhase,
    LogicalAssignOp, ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ObjectProp, ReExportNamedSpecifier,
    SYMBOL_ITERATOR_OBJECT_KEY, Span, SpannedToken, Stmt, Token, TokenKind, TypeRef, UnaryOp,
    ast::{ClassPrivateElement, ClassStaticBlock, ReExportNamespaceSpecifier},
};
use std::collections::{HashMap, HashSet};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    cursor: usize,
    strict_mode: bool,
    typescript_generic_functions: HashSet<String>,
    parenthesized_expr_spans: HashSet<(usize, usize)>,
    pending_statements: Vec<Stmt>,
    /// For each token, whether it is preceded by a line terminator in the source.
    has_preceding_newline: Vec<bool>,
    /// Whether we are inside an async function body (for context-sensitive `await` parsing).
    in_async_fn: bool,
    /// Whether we are inside a generator function body (for context-sensitive `yield` parsing).
    in_generator_fn: bool,
    fn_depth: u32,
    /// How many loop levels we're nested in (for `break`/`continue` validation).
    loop_depth: u32,
    /// How many switch statement levels we're nested in (for `break` validation inside switch).
    switch_depth: u32,
    /// Whether we've already seen a `export default` declaration in this module.
    has_default_export: bool,
    /// Whether we are inside a class field initializer expression.
    in_class_field_init: bool,
    /// Maps class names to their TypeScript-`private` field names (erased at runtime).
    class_private_fields: HashMap<String, Vec<String>>,
    namespace_names_encountered: HashSet<String>,
    /// Counter for generating unique namespace stub names.
    namespace_stub_counter: u32,
    /// The original source text (used for Function.prototype.toString etc.).
    source: String,
}

/// For each token, check if there is a line terminator between the previous
/// token's end and this token's start. The first token is always `false`.
fn compute_newline_flags(source: &str, tokens: &[SpannedToken]) -> Vec<bool> {
    let mut flags = Vec::with_capacity(tokens.len());
    let mut prev_end = 0usize;
    for token in tokens {
        let nl = if token.span.start > prev_end {
            source[prev_end..token.span.start].contains('\n')
        } else {
            false
        };
        flags.push(nl);
        prev_end = token.span.end;
    }
    flags
}

struct ParsedParam {
    name: String,
    default: Option<Expr>,
    is_rest: bool,
    is_parameter_property: bool,
    is_this_parameter: bool,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedBindingPattern {
    text: String,
    span: Span,
    is_identifier: bool,
}

fn desugar_destructured_params(
    params: &mut Vec<(String, Option<Expr>, bool)>,
    body: &mut Vec<Stmt>,
) {
    let mut stmts_to_add: Vec<Stmt> = Vec::new();
    for (i, (name, _default, _is_rest)) in params.iter_mut().enumerate() {
        if name.starts_with('{') || name.starts_with('[') {
            let pattern_text = name.clone();
            let temp_name = format!("_p{i}");
            *name = temp_name.clone();
            stmts_to_add.push(Stmt::Let {
                name: pattern_text,
                expr: Expr::Ident {
                    name: temp_name,
                    span: Span::generated("param_destructure"),
                },
                span: Span::generated("param_destructure"),
                is_var: false,
            });
        }
    }
    for stmt in stmts_to_add.into_iter().rev() {
        body.insert(0, stmt);
    }
}

impl Parser {
    fn check_duplicate_params(
        &self,
        params: &[(String, Option<Expr>, bool)],
    ) -> Result<(), Diagnostic> {
        if !self.strict_mode {
            return Ok(());
        }
        let mut seen = HashSet::new();
        for (name, _, _) in params {
            if !seen.insert(name.clone()) {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: format!("duplicate parameter '{name}' not allowed in strict mode"),
                    span: None,
                    phase: Some("parser"),
                });
            }
        }
        Ok(())
    }

    fn validate_strict_mode_fn_params(
        &self,
        name: &str,
        name_span: Span,
        params: &[(String, Option<Expr>, bool)],
    ) -> Result<(), Diagnostic> {
        if !self.strict_mode {
            return Ok(());
        }
        let mut seen = HashSet::new();
        for (param_name, default, is_rest) in params {
            if *is_rest || default.is_some() {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: format!(
                        "function `{}` has non-simple parameters in strict mode",
                        if name.is_empty() { "<anonymous>" } else { name }
                    ),
                    span: Some(name_span),
                    phase: Some("parser"),
                });
            }
            if !seen.insert(param_name.clone()) {
                return Err(Diagnostic {
                    code: DiagCode::SyntaxError,
                    message: format!(
                        "duplicate parameter `{param_name}` not allowed in strict mode"
                    ),
                    span: Some(name_span),
                    phase: Some("parser"),
                });
            }
        }
        Ok(())
    }
}

include!("parser/statements.rs");
include!("parser/binding_patterns.rs");
include!("parser/expressions.rs");
include!("parser/tokens.rs");
include!("parser/helpers.rs");
include!("parser/tests.rs");
