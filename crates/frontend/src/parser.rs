use crate::{
    ArrayLiteralElement, BinaryOp, DiagCode, Diagnostic, ExportNamedSpecifier, Expr,
    ImportAttribute, ImportDefaultSpecifier, ImportNamedSpecifier, ImportNamespaceSpecifier,
    ImportPhase, LogicalAssignOp, ModuleSpecifier, OBJECT_SPREAD_SENTINEL, ObjectProp,
    ReExportNamedSpecifier, SYMBOL_ITERATOR_OBJECT_KEY, Span, SpannedToken, Stmt, Token, TokenKind,
    UnaryOp,
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
    possible_eval_shadowing: bool,
    possible_function_shadowing: bool,
    /// For each token, whether it is preceded by a line terminator in the source.
    has_preceding_newline: Vec<bool>,
    /// Whether we are inside an async function body (for context-sensitive `await` parsing).
    in_async_fn: bool,
    /// Whether we are inside a generator function body (for context-sensitive `yield` parsing).
    in_generator_fn: bool,
    fn_depth: u32,
    /// Maps class names to their TypeScript-`private` field names (erased at runtime).
    class_private_fields: HashMap<String, Vec<String>>,
    namespace_names_encountered: HashSet<String>,
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

struct StaticEvalFunctionBlock<'a> {
    prefix: &'a str,
    inner_source: &'a str,
    suffix: &'a str,
}

/// Offset a span by `offset` positions.
fn offset_span(span: &mut Span, offset: usize) {
    span.start += offset;
    span.end += offset;
}

/// Recursively offset all spans in a `Stmt` and its children.
fn offset_stmt_spans(stmt: &mut Stmt, offset: usize) {
    match stmt {
        Stmt::ImportSideEffect {
            specifier,
            attributes,
            span,
        } => {
            offset_span(span, offset);
            specifier.span.start += offset;
            specifier.span.end += offset;
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ImportNamed {
            specifiers,
            source,
            attributes,
            span,
            ..
        } => {
            offset_span(span, offset);
            source.span.start += offset;
            source.span.end += offset;
            for spec in specifiers {
                offset_span(&mut spec.imported_span, offset);
                offset_span(&mut spec.local_span, offset);
                offset_span(&mut spec.span, offset);
            }
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ImportDefault {
            specifier,
            source,
            attributes,
            span,
            ..
        } => {
            offset_span(span, offset);
            offset_span(&mut specifier.local_span, offset);
            offset_span(&mut specifier.span, offset);
            source.span.start += offset;
            source.span.end += offset;
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ImportDefaultNamed {
            default,
            specifiers,
            source,
            attributes,
            span,
        } => {
            offset_span(span, offset);
            offset_span(&mut default.local_span, offset);
            offset_span(&mut default.span, offset);
            for spec in specifiers {
                offset_span(&mut spec.imported_span, offset);
                offset_span(&mut spec.local_span, offset);
                offset_span(&mut spec.span, offset);
            }
            source.span.start += offset;
            source.span.end += offset;
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ImportNamespace {
            specifier,
            source,
            attributes,
            span,
        } => {
            offset_span(span, offset);
            offset_span(&mut specifier.local_span, offset);
            offset_span(&mut specifier.span, offset);
            source.span.start += offset;
            source.span.end += offset;
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ImportDefaultNamespace {
            default,
            namespace,
            source,
            attributes,
            span,
        } => {
            offset_span(span, offset);
            offset_span(&mut default.local_span, offset);
            offset_span(&mut default.span, offset);
            offset_span(&mut namespace.local_span, offset);
            offset_span(&mut namespace.span, offset);
            source.span.start += offset;
            source.span.end += offset;
            for attr in attributes {
                offset_span(&mut attr.key_span, offset);
                offset_span(&mut attr.value_span, offset);
                offset_span(&mut attr.span, offset);
            }
        }
        Stmt::ExportNamed { specifiers, span } => {
            offset_span(span, offset);
            for spec in specifiers {
                offset_span(&mut spec.local_span, offset);
                offset_span(&mut spec.exported_span, offset);
                offset_span(&mut spec.span, offset);
            }
        }
        Stmt::ExportNamedFrom {
            specifiers,
            source,
            span,
        } => {
            offset_span(span, offset);
            for spec in specifiers {
                offset_span(&mut spec.imported_span, offset);
                offset_span(&mut spec.exported_span, offset);
                offset_span(&mut spec.span, offset);
            }
            source.span.start += offset;
            source.span.end += offset;
        }
        Stmt::ExportAllFrom {
            star_span,
            source,
            span,
        } => {
            offset_span(span, offset);
            offset_span(star_span, offset);
            source.span.start += offset;
            source.span.end += offset;
        }
        Stmt::ExportNamespaceFrom {
            namespace,
            source,
            span,
        } => {
            offset_span(span, offset);
            offset_span(&mut namespace.exported_span, offset);
            offset_span(&mut namespace.span, offset);
            source.span.start += offset;
            source.span.end += offset;
        }
        Stmt::ExportDecl {
            declaration,
            specifier,
            span,
        } => {
            offset_span(span, offset);
            offset_stmt_spans(&mut *declaration, offset);
            offset_span(&mut specifier.local_span, offset);
            offset_span(&mut specifier.exported_span, offset);
            offset_span(&mut specifier.span, offset);
        }
        Stmt::ExportDefault {
            expr,
            default_span,
            span,
        } => {
            offset_span(span, offset);
            offset_span(default_span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::ExportAssignment { expr, span } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::Let { expr, span, .. } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::AmbientValueDecl { span, .. } => {
            offset_span(span, offset);
        }
        Stmt::Assign { expr, span, .. } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::Expr { expr, span } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(condition, offset);
            for s in then_body {
                offset_stmt_spans(s, offset);
            }
            for s in else_body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::While {
            condition,
            body,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(condition, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::Function {
            body, params, span, ..
        } => {
            offset_span(span, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
            for (_, default, _) in params {
                if let Some(expr) = default {
                    offset_expr_spans(expr, offset);
                }
            }
        }
        Stmt::Return { expr, span } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::ClassDecl {
            body,
            extends,
            static_blocks,
            private_elements,
            interface_heritage,
            span,
            ..
        } => {
            offset_span(span, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
            if let Some(extends) = extends {
                offset_expr_spans(&mut *extends, offset);
            }
            for block in static_blocks {
                offset_span(&mut block.span, offset);
                for s in &mut block.body {
                    offset_stmt_spans(s, offset);
                }
            }
            for elem in private_elements {
                match elem {
                    ClassPrivateElement::Field {
                        name_span,
                        value,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        if let Some(expr) = value {
                            offset_expr_spans(expr, offset);
                        }
                    }
                    ClassPrivateElement::Method {
                        name_span,
                        params,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for (_, default, _) in params {
                            if let Some(expr) = default {
                                offset_expr_spans(expr, offset);
                            }
                        }
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                    ClassPrivateElement::Getter {
                        name_span,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                    ClassPrivateElement::Setter {
                        name_span,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                }
            }
            for expr in interface_heritage {
                offset_expr_spans(expr, offset);
            }
        }
        Stmt::EnumDecl { span, .. } => {
            offset_span(span, offset);
        }
        Stmt::TryCatch {
            try_block,
            catch_param: _,
            catch_block,
            finally_block,
            span,
        } => {
            offset_span(span, offset);
            for s in try_block {
                offset_stmt_spans(s, offset);
            }
            if let Some(block) = catch_block {
                for s in block {
                    offset_stmt_spans(s, offset);
                }
            }
            if let Some(block) = finally_block {
                for s in block {
                    offset_stmt_spans(s, offset);
                }
            }
        }
        Stmt::Throw { expr, span } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
        }
        Stmt::Switch { expr, cases, span } => {
            offset_span(span, offset);
            offset_expr_spans(expr, offset);
            for (opt_expr, stmts) in cases {
                if let Some(expr) = opt_expr {
                    offset_expr_spans(expr, offset);
                }
                for s in stmts {
                    offset_stmt_spans(s, offset);
                }
            }
        }
        Stmt::DoWhile {
            body,
            condition,
            span,
        } => {
            offset_span(span, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
            offset_expr_spans(condition, offset);
        }
        Stmt::For {
            init,
            condition,
            update,
            body,
            span,
        } => {
            offset_span(span, offset);
            if let Some(init) = init {
                offset_stmt_spans(&mut *init, offset);
            }
            if let Some(condition) = condition {
                offset_expr_spans(condition, offset);
            }
            if let Some(update) = update {
                offset_expr_spans(update, offset);
            }
            for s in body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::ForIn {
            var: _,
            iter,
            body,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(iter, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::ForOf {
            var: _,
            iter,
            body,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(iter, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::ForAwaitOf {
            var: _,
            iter,
            body,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(iter, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
        }
        Stmt::Labeled {
            label: _,
            body,
            span,
        } => {
            offset_span(span, offset);
            offset_stmt_spans(&mut *body, offset);
        }
        Stmt::Break { span, .. } => {
            offset_span(span, offset);
        }
        Stmt::Continue { span, .. } => {
            offset_span(span, offset);
        }
        Stmt::Block { statements, span } => {
            offset_span(span, offset);
            for s in statements {
                offset_stmt_spans(s, offset);
            }
        }
    }
}

/// Recursively offset all spans in an `Expr` and its children.
fn offset_expr_spans(expr: &mut Expr, offset: usize) {
    match expr {
        Expr::Number { span, .. }
        | Expr::DecimalNumber { span, .. }
        | Expr::BigInt { span, .. }
        | Expr::String { span, .. }
        | Expr::Bool { span, .. }
        | Expr::Null { span }
        | Expr::Undefined { span }
        | Expr::Ident { span, .. }
        | Expr::This { span }
        | Expr::NewTarget { span }
        | Expr::ImportMeta { span } => {
            offset_span(span, offset);
        }
        Expr::Await { expr: inner, span } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::Yield {
            expr: inner, span, ..
        } => {
            offset_span(span, offset);
            if let Some(inner) = inner {
                offset_expr_spans(&mut *inner, offset);
            }
        }
        Expr::Unary {
            expr: inner, span, ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::Binary {
            left, right, span, ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *left, offset);
            offset_expr_spans(&mut *right, offset);
        }
        Expr::Member { object, span, .. } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
        }
        Expr::OptionalMember { object, span, .. } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
        }
        Expr::Call { callee, args, span } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *callee, offset);
            for arg in args {
                offset_expr_spans(arg, offset);
            }
        }
        Expr::OptionalCall { callee, args, span } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *callee, offset);
            for arg in args {
                offset_expr_spans(arg, offset);
            }
        }
        Expr::Assign {
            expr: inner, span, ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::LogicalAssign {
            expr: inner, span, ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::LogicalPropertyAssign {
            object_expr,
            expr: inner,
            span,
            ..
        } => {
            offset_span(span, offset);
            if let Some(obj_expr) = object_expr {
                offset_expr_spans(&mut *obj_expr, offset);
            }
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::Array { elements, span } => {
            offset_span(span, offset);
            for elem in elements {
                match elem {
                    ArrayLiteralElement::Present(expr) | ArrayLiteralElement::Spread(expr) => {
                        offset_expr_spans(expr, offset);
                    }
                    ArrayLiteralElement::Hole(s) => offset_span(s, offset),
                }
            }
        }
        Expr::Object { props, span } => {
            offset_span(span, offset);
            for prop in props {
                match prop {
                    ObjectProp::KeyValue { value, .. }
                    | ObjectProp::Shorthand { value, .. }
                    | ObjectProp::MethodShorthand { value, .. } => {
                        offset_expr_spans(value, offset);
                    }
                    ObjectProp::ComputedKey { key, value } => {
                        offset_expr_spans(&mut *key, offset);
                        offset_expr_spans(value, offset);
                    }
                }
            }
        }
        Expr::Index {
            object,
            index,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
            offset_expr_spans(&mut *index, offset);
        }
        Expr::OptionalIndex {
            object,
            index,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
            offset_expr_spans(&mut *index, offset);
        }
        Expr::New {
            expr: inner,
            args,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
            for arg in args {
                offset_expr_spans(arg, offset);
            }
        }
        Expr::TypeOf { expr: inner, span } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::InstanceOf {
            expr: inner,
            type_expr,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
            offset_expr_spans(&mut *type_expr, offset);
        }
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *condition, offset);
            offset_expr_spans(&mut *then_expr, offset);
            offset_expr_spans(&mut *else_expr, offset);
        }
        Expr::ArrowFn {
            body,
            body_stmts,
            span,
            ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *body, offset);
            for s in body_stmts {
                offset_stmt_spans(s, offset);
            }
        }
        Expr::FunctionExpr {
            body, params, span, ..
        } => {
            offset_span(span, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
            for (_, default, _) in params {
                if let Some(expr) = default {
                    offset_expr_spans(expr, offset);
                }
            }
        }
        Expr::Spread { expr: inner, span } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *inner, offset);
        }
        Expr::PropertyAssign {
            object,
            value,
            span,
            ..
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
            offset_expr_spans(&mut *value, offset);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            span,
        } => {
            offset_span(span, offset);
            offset_expr_spans(&mut *object, offset);
            offset_expr_spans(&mut *index, offset);
            offset_expr_spans(&mut *value, offset);
        }
        Expr::ClassExpr {
            body,
            extends,
            static_blocks,
            private_elements,
            interface_heritage,
            span,
            ..
        } => {
            offset_span(span, offset);
            for s in body {
                offset_stmt_spans(s, offset);
            }
            if let Some(extends) = extends {
                offset_expr_spans(&mut *extends, offset);
            }
            for block in static_blocks {
                offset_span(&mut block.span, offset);
                for s in &mut block.body {
                    offset_stmt_spans(s, offset);
                }
            }
            for elem in private_elements {
                match elem {
                    ClassPrivateElement::Field {
                        name_span,
                        value,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        if let Some(expr) = value {
                            offset_expr_spans(expr, offset);
                        }
                    }
                    ClassPrivateElement::Method {
                        name_span,
                        params,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for (_, default, _) in params {
                            if let Some(expr) = default {
                                offset_expr_spans(expr, offset);
                            }
                        }
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                    ClassPrivateElement::Getter {
                        name_span,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                    ClassPrivateElement::Setter {
                        name_span,
                        body,
                        span,
                        ..
                    } => {
                        offset_span(name_span, offset);
                        offset_span(span, offset);
                        for s in body {
                            offset_stmt_spans(s, offset);
                        }
                    }
                }
            }
            for expr in interface_heritage {
                offset_expr_spans(expr, offset);
            }
        }
        Expr::Sequence { exprs, span } => {
            offset_span(span, offset);
            for e in exprs {
                offset_expr_spans(e, offset);
            }
        }
    }
}

include!("parser/statements.rs");
include!("parser/binding_patterns.rs");
include!("parser/expressions.rs");
include!("parser/tokens.rs");
include!("parser/helpers.rs");
include!("parser/eval_expand.rs");
include!("parser/tests.rs");
