use std::fmt::Write as _;

use ts2wasm_syntax::{
    ArrayLiteralElement, BinaryOp, ClassPrivateElement, Expr, LogicalAssignOp,
    OBJECT_SPREAD_SENTINEL, Stmt, UnaryOp,
};

pub(crate) fn unparse_program(program: &[Stmt]) -> String {
    let mut out = String::new();
    for stmt in program {
        unparse_stmt(&mut out, stmt, 0);
    }
    out
}

pub(crate) fn unparse_stmt(out: &mut String, stmt: &Stmt, indent: usize) {
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
            let _ = if matches!(
                stmt,
                Stmt::ImportDefault {
                    phase: ts2wasm_syntax::ImportPhase::Source,
                    ..
                }
            ) {
                writeln!(
                    out,
                    "import source {} from '{}';",
                    specifier.local, source.value
                )
            } else {
                writeln!(out, "import {} from '{}';", specifier.local, source.value)
            };
        }
        Stmt::ImportDefaultNamed {
            default,
            specifiers,
            source,
            ..
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
            let _ = writeln!(
                out,
                "import {}, {{ {specifiers} }} from '{}';",
                default.local, source.value
            );
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
        Stmt::ImportDefaultNamespace {
            default,
            namespace,
            source,
            ..
        } => {
            let _ = writeln!(
                out,
                "import {}, * as {} from '{}';",
                default.local, namespace.local, source.value
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
        Stmt::ExportNamedFrom {
            specifiers, source, ..
        } => {
            let specifiers = specifiers
                .iter()
                .map(|specifier| {
                    if specifier.imported == specifier.exported {
                        specifier.imported.clone()
                    } else {
                        format!("{} as {}", specifier.imported, specifier.exported)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "export {{ {specifiers} }} from '{}';", source.value);
        }
        Stmt::ExportAllFrom { source, .. } => {
            let _ = writeln!(out, "export * from '{}';", source.value);
        }
        Stmt::ExportNamespaceFrom {
            namespace, source, ..
        } => {
            let _ = writeln!(
                out,
                "export * as {} from '{}';",
                namespace.exported, source.value
            );
        }
        Stmt::ExportDecl { declaration, .. } => {
            let _ = write!(out, "export ");
            unparse_stmt(out, declaration, 0);
        }
        Stmt::ExportDefault { expr, .. } => {
            let _ = writeln!(out, "export default {};", unparse_expr(expr));
        }
        Stmt::ExportAssignment { expr, .. } => {
            let _ = writeln!(out, "export = {};", unparse_expr(expr));
        }
        Stmt::Let { name, expr, .. } => {
            let _ = writeln!(out, "let {name} = {};", unparse_expr(expr));
        }
        Stmt::Using {
            name,
            expr,
            is_async,
            ..
        } => {
            let keyword = if *is_async { "await using" } else { "using" };
            let _ = writeln!(out, "{keyword} {name} = {};", unparse_expr(expr));
        }
        Stmt::AmbientValueDecl { .. } => {}
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
            name,
            params,
            body,
            is_ambient,
            ..
        } => {
            let params = if *is_ambient {
                String::new()
            } else {
                params
                    .iter()
                    .map(|(name, default, rest)| {
                        let prefix = if *rest { "..." } else { "" };
                        match default {
                            Some(expr) => format!("{prefix}{name} = {}", unparse_expr(expr)),
                            None => format!("{prefix}{name}"),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let _ = writeln!(out, "function {name}({params}) {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        Stmt::Return { expr, .. } => {
            let _ = writeln!(out, "return {};", unparse_expr(expr));
        }
        Stmt::EnumDecl { name, .. } => {
            let _ = writeln!(out, "enum {name} {{}}");
        }
        Stmt::TypeAlias {
            name,
            underlying_type,
            ..
        } => {
            let _ = writeln!(out, "type {name} = {underlying_type:?};");
        }
        Stmt::InterfaceDecl { name, .. } => {
            let _ = writeln!(out, "interface {name} {{}}");
        }
        Stmt::ClassDecl {
            name,
            extends,
            body,
            static_blocks,
            private_elements,
            ..
        } => {
            let extends = extends
                .as_ref()
                .map(|expr| format!(" extends {}", unparse_expr(expr)))
                .unwrap_or_default();
            let _ = writeln!(out, "class {name}{extends} {{");
            unparse_block(out, body, indent + 1);
            for static_block in static_blocks {
                write_indent(out, indent + 1);
                let _ = writeln!(out, "static {{");
                unparse_block(out, &static_block.body, indent + 2);
                write_indent(out, indent + 1);
                let _ = writeln!(out, "}}");
            }
            for element in private_elements {
                unparse_private_class_element(out, element, indent + 1);
            }
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
        Stmt::ForAwaitOf {
            var, iter, body, ..
        } => {
            let _ = writeln!(out, "for await (let {var} of {}) {{", unparse_expr(iter));
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
        Stmt::Block { statements, .. } => unparse_block(out, statements, indent),
    }
}

pub(crate) fn unparse_block(out: &mut String, body: &[Stmt], indent: usize) {
    for stmt in body {
        unparse_stmt(out, stmt, indent);
    }
}

pub(crate) fn unparse_private_class_element(
    out: &mut String,
    element: &ClassPrivateElement,
    indent: usize,
) {
    match element {
        ClassPrivateElement::Field {
            name,
            value,
            is_static,
            ..
        } => {
            write_indent(out, indent);
            let prefix = if *is_static { "static " } else { "" };
            match value {
                Some(value) => {
                    let _ = writeln!(out, "{prefix}#{name} = {};", unparse_expr(value));
                }
                None => {
                    let _ = writeln!(out, "{prefix}#{name};");
                }
            }
        }
        ClassPrivateElement::Method {
            name,
            params,
            body,
            is_static,
            ..
        } => {
            write_indent(out, indent);
            let prefix = if *is_static { "static " } else { "" };
            let params = params
                .iter()
                .map(|(name, default, is_rest)| {
                    let prefix = if *is_rest { "..." } else { "" };
                    match default {
                        Some(expr) => format!("{prefix}{name} = {}", unparse_expr(expr)),
                        None => format!("{prefix}{name}"),
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "{prefix}#{name}({params}) {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        ClassPrivateElement::Getter {
            name,
            body,
            is_static,
            ..
        } => {
            write_indent(out, indent);
            let prefix = if *is_static { "static " } else { "" };
            let _ = writeln!(out, "{prefix}get #{name}() {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
        ClassPrivateElement::Setter {
            name,
            param,
            body,
            is_static,
            ..
        } => {
            write_indent(out, indent);
            let prefix = if *is_static { "static " } else { "" };
            let _ = writeln!(out, "{prefix}set #{name}({param}) {{");
            unparse_block(out, body, indent + 1);
            write_indent(out, indent);
            let _ = writeln!(out, "}}");
        }
    }
}

pub(crate) fn unparse_for_init(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Let { name, expr, .. } => format!("let {name} = {}", unparse_expr(expr)),
        Stmt::Assign { name, expr, .. } => format!("{name} = {}", unparse_expr(expr)),
        Stmt::Expr { expr, .. } => unparse_expr(expr),
        other => format!("{other:?}"),
    }
}

pub(crate) fn unparse_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number { value, .. } => value.to_string(),
        Expr::DecimalNumber { value, .. } => value.clone(),
        Expr::BigInt { raw, .. } => raw.clone(),
        Expr::String { value, .. } => format!("{:?}", value),
        Expr::Bool { value, .. } => value.to_string(),
        Expr::Null { .. } => "null".to_owned(),
        Expr::Undefined { .. } => "undefined".to_owned(),
        Expr::Await { expr, .. } => format!("await {}", unparse_expr(expr)),
        Expr::Yield { expr, delegate, .. } => {
            let keyword = if *delegate { "yield*" } else { "yield" };
            match expr {
                Some(expr) => format!("{keyword} {}", unparse_expr(expr)),
                None => keyword.to_owned(),
            }
        }
        Expr::Ident { name, .. } => name.clone(),
        Expr::Unary { op, expr, .. } => match op {
            UnaryOp::Not => format!("!{}", unparse_expr(expr)),
            UnaryOp::Plus => format!("+{}", unparse_expr(expr)),
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
        Expr::OptionalMember {
            object, property, ..
        } => format!("{}?.{}", unparse_expr(object), property),
        Expr::Call { callee, args, .. } => {
            format!("{}({})", unparse_expr(callee), unparse_expr_list(args))
        }
        Expr::OptionalCall { callee, args, .. } => {
            format!("{}?.({})", unparse_expr(callee), unparse_expr_list(args))
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
        Expr::Array { elements, .. } => format!(
            "[{}]",
            elements
                .iter()
                .map(|element| match element {
                    ArrayLiteralElement::Present(expr) => unparse_expr(expr),
                    ArrayLiteralElement::Spread(expr) => unparse_expr(expr),
                    ArrayLiteralElement::Hole(_) => String::new(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::Object { props, .. } => {
            let props = props
                .iter()
                .map(|prop| {
                    let value = prop.value();
                    if prop.static_key() == Some(OBJECT_SPREAD_SENTINEL) {
                        format!("...{}", unparse_expr(value))
                    } else if let Some(key) = prop.static_key() {
                        format!("{key}: {}", unparse_expr(value))
                    } else {
                        let key = prop.computed_key().map(unparse_expr).unwrap_or_default();
                        format!("[{key}]: {}", unparse_expr(value))
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{props}}}")
        }
        Expr::Index { object, index, .. } => {
            format!("{}[{}]", unparse_expr(object), unparse_expr(index))
        }
        Expr::OptionalIndex { object, index, .. } => {
            format!("{}?.[{}]", unparse_expr(object), unparse_expr(index))
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
        Expr::ArrowFn {
            params,
            body,
            body_stmts,
            ..
        } => unparse_arrow_fn_expr(params, body, body_stmts),
        Expr::FunctionExpr {
            name, params, body, ..
        } => unparse_function_expr(name, params, body),
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
        Expr::ClassExpr { name, .. } => {
            if name.is_empty() {
                "class { ... }".to_owned()
            } else {
                format!("class {name} {{ ... }}")
            }
        }
        Expr::This { .. } => "this".to_owned(),
        Expr::NewTarget { .. } => "new.target".to_owned(),
        Expr::ImportMeta { .. } => "import.meta".to_owned(),
        Expr::PrivateIdent { name, .. } => format!("#{name}"),
        Expr::Sequence { exprs, .. } => {
            let parts: Vec<String> = exprs.iter().map(unparse_expr).collect();
            parts.join(", ")
        }
        Expr::Topic { .. } => "%".to_owned(),
    }
}

pub(crate) fn unparse_expr_list(exprs: &[Expr]) -> String {
    exprs
        .iter()
        .map(unparse_expr)
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn unparse_arrow_fn_expr(params: &[String], body: &Expr, body_stmts: &[Stmt]) -> String {
    if body_stmts.is_empty() {
        format!("({}) => {}", params.join(", "), unparse_expr(body))
    } else {
        let stmts: Vec<String> = body_stmts
            .iter()
            .map(|s| {
                let mut buf = String::new();
                unparse_stmt(&mut buf, s, 0);
                buf
            })
            .collect();
        format!(
            "({}) => {{ {} return {}; }}",
            params.join(", "),
            stmts.join("; "),
            unparse_expr(body)
        )
    }
}

pub(crate) fn unparse_function_expr(
    name: &str,
    params: &[(String, Option<Expr>, bool)],
    body: &[Stmt],
) -> String {
    let params = params
        .iter()
        .map(|(name, _, is_rest)| {
            if *is_rest {
                format!("...{name}")
            } else {
                name.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let mut out = format!("function {name}({params}) {{\n");
    unparse_block(&mut out, body, 1);
    out.push('}');
    out
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
        BinaryOp::NullishCoalesce => "??",
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
        BinaryOp::Pipeline => "|>",
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

pub(crate) fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}
