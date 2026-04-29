use crate::{
    BinaryOp, DiagCode, Diagnostic, ExportNamedSpecifier, Expr, ImportDefaultSpecifier,
    ImportNamedSpecifier, ImportNamespaceSpecifier, LogicalAssignOp, ModuleSpecifier,
    ReExportNamedSpecifier, Span, SpannedToken, Stmt, Token, TokenKind, UnaryOp,
    ast::{ClassPrivateElement, ClassStaticBlock, ReExportNamespaceSpecifier},
};
use std::collections::HashSet;

pub struct Parser {
    tokens: Vec<SpannedToken>,
    cursor: usize,
    strict_mode: bool,
    typescript_generic_functions: HashSet<String>,
    parenthesized_expr_spans: HashSet<(usize, usize)>,
}

struct ParsedParam {
    name: String,
    default: Option<Expr>,
    is_rest: bool,
    is_parameter_property: bool,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedBindingPattern {
    text: String,
    span: Span,
    is_identifier: bool,
}

include!("parser/statements.rs");
include!("parser/expressions.rs");
include!("parser/tokens.rs");
include!("parser/helpers.rs");
include!("parser/tests.rs");
