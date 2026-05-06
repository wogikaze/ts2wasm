use crate::{
    BinaryOp, DiagCode, Diagnostic, ExportNamedSpecifier, Expr, ImportDefaultSpecifier,
    ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp, ModuleSpecifier,
    OBJECT_SPREAD_SENTINEL, ReExportNamedSpecifier, SYMBOL_ITERATOR_OBJECT_KEY, Span, SpannedToken,
    Stmt, Token, TokenKind, UnaryOp,
    ast::{ClassPrivateElement, ClassStaticBlock, ReExportNamespaceSpecifier},
};
use std::collections::HashSet;

pub struct Parser {
    tokens: Vec<SpannedToken>,
    cursor: usize,
    strict_mode: bool,
    typescript_generic_functions: HashSet<String>,
    parenthesized_expr_spans: HashSet<(usize, usize)>,
    pending_statements: Vec<Stmt>,
    possible_eval_shadowing: bool,
    /// For each token, whether it is preceded by a line terminator in the source.
    has_preceding_newline: Vec<bool>,
    /// Whether we are inside an async function body (for context-sensitive `await` parsing).
    in_async_fn: bool,
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

struct StaticEvalFunctionBlock<'a> {
    prefix: &'a str,
    inner_source: &'a str,
    suffix: &'a str,
}

include!("parser/statements.rs");
include!("parser/binding_patterns.rs");
include!("parser/expressions.rs");
include!("parser/tokens.rs");
include!("parser/helpers.rs");
include!("parser/tests.rs");
