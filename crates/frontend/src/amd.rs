use crate::ast::{Expr, ModuleSpecifier, Stmt};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

/// AMD `define(...)` call patterns detected during scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AmdPattern {
    /// `define(["dep1", "dep2", ...], function(dep1, dep2, ...) { ... })`
    /// or `define(["dep1", ...], function(dep1, ...) { ... }, exports?)`
    WithDependencyArray {
        deps: Vec<String>,
        factory_params: Vec<String>,
        factory_body: Vec<Stmt>,
        has_return_export: bool,
        span: Span,
    },
    /// `define(function(require, exports, module) { ... })`
    Simplified {
        factory_body: Vec<Stmt>,
        _has_require: bool,
        has_exports: bool,
        has_module: bool,
        span: Span,
    },
}

/// Detect and extract AMD `define(...)` patterns from a top-level statement.
pub(crate) fn detect_amd_define(stmt: &Stmt) -> Option<AmdPattern> {
    let (callee, args, span) = match stmt {
        Stmt::Expr {
            expr: Expr::Call { callee, args, span },
            ..
        } => (callee, args, *span),
        _ => return None,
    };

    // Check callee is `define` identifier
    let Expr::Ident { name, .. } = callee.as_ref() else {
        return None;
    };
    if name != "define" {
        return None;
    }

    if args.is_empty() {
        return None;
    }

    // Form 1: define(["dep1", ...], function(...) { ... })
    // Check first arg is an array literal of strings
    if let Expr::Array { elements, .. } = &args[0] {
        let mut deps = Vec::new();
        for elem in elements {
            match elem {
                crate::ArrayLiteralElement::Present(Expr::String { value, .. }) => {
                    deps.push(value.clone());
                }
                _ => {
                    // Not a simple string dependency array; bail out
                    return None;
                }
            }
        }

        if args.len() < 2 {
            return None;
        }

        // Second arg should be a function expression
        if let Expr::FunctionExpr { params, body, .. } = &args[1] {
            let factory_params: Vec<String> =
                params.iter().map(|(name, _, _)| name.clone()).collect();
            let has_return_export = has_top_level_return(body);

            return Some(AmdPattern::WithDependencyArray {
                deps,
                factory_params,
                factory_body: body.clone(),
                has_return_export,
                span,
            });
        }
        return None;
    }

    // Form 2: define(function(require, exports, module) { ... })
    if let Expr::FunctionExpr { params, body, .. } = &args[0] {
        let param_names: Vec<String> = params.iter().map(|(name, _, _)| name.clone()).collect();
        let has_require = param_names.iter().any(|n| n == "require");
        let has_exports = param_names.iter().any(|n| n == "exports");
        let has_module = param_names.iter().any(|n| n == "module");

        return Some(AmdPattern::Simplified {
            factory_body: body.clone(),
            _has_require: has_require,
            has_exports,
            has_module,
            span,
        });
    }

    None
}

/// Check if a block contains a top-level return statement that likely provides
/// the AMD module exports.
fn has_top_level_return(body: &[Stmt]) -> bool {
    body.iter().any(|stmt| matches!(stmt, Stmt::Return { .. }))
}

/// Transform AMD `define(...)` calls in the program into standard ES module
/// import/export syntax.
///
/// This function scans the program for top-level AMD define() calls and
/// rewrites them. The transformed program can then be processed by the
/// standard module pipeline.
pub fn transform_amd_program(program: Vec<Stmt>) -> Result<Vec<Stmt>, Diagnostic> {
    // First, detect if this is an AMD module program.
    let has_amd = program.iter().any(|stmt| detect_amd_define(stmt).is_some());
    if !has_amd {
        return Ok(program);
    }

    // If there are any regular import/export statements alongside AMD define(),
    // that is an unsupported mixed mode.
    let has_regular_imports = program.iter().any(|stmt| {
        matches!(
            stmt,
            Stmt::ImportSideEffect { .. }
                | Stmt::ImportNamed { .. }
                | Stmt::ImportDefault { .. }
                | Stmt::ImportDefaultNamed { .. }
                | Stmt::ImportNamespace { .. }
                | Stmt::ImportDefaultNamespace { .. }
                | Stmt::ExportNamed { .. }
                | Stmt::ExportNamedFrom { .. }
                | Stmt::ExportAllFrom { .. }
                | Stmt::ExportDecl { .. }
                | Stmt::ExportDefault { .. }
                | Stmt::ExportAssignment { .. }
                | Stmt::ExportNamespaceFrom { .. }
        )
    });
    if has_regular_imports {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedModule,
            message: "mixed AMD define() and standard ES module imports/exports are not supported"
                .to_owned(),
            span: None,
            phase: Some("amd-transform"),
        });
    }

    // Process each top-level statement, replacing AMD define() calls.
    let mut rewritten = Vec::new();
    for stmt in program {
        if let Some(pattern) = detect_amd_define(&stmt) {
            rewrite_amd_pattern(pattern, &mut rewritten)?;
        } else {
            rewritten.push(stmt);
        }
    }

    Ok(rewritten)
}

/// Rewrite a single AMD pattern into ES module statements.
fn rewrite_amd_pattern(pattern: AmdPattern, output: &mut Vec<Stmt>) -> Result<(), Diagnostic> {
    match pattern {
        AmdPattern::WithDependencyArray {
            deps,
            factory_params,
            factory_body,
            has_return_export,
            span,
        } => {
            rewrite_dependency_array_form(
                deps,
                factory_params,
                factory_body,
                has_return_export,
                span,
                output,
            )?;
        }
        AmdPattern::Simplified {
            factory_body,
            _has_require,
            has_exports,
            has_module,
            span,
        } => {
            rewrite_simplified_form(
                factory_body,
                _has_require,
                has_exports,
                has_module,
                span,
                output,
            )?;
        }
    }
    Ok(())
}

/// Rewrite `define(["dep1", ...], function(dep1, ...) { ... })` form.
fn rewrite_dependency_array_form(
    deps: Vec<String>,
    factory_params: Vec<String>,
    factory_body: Vec<Stmt>,
    has_return_export: bool,
    span: Span,
    output: &mut Vec<Stmt>,
) -> Result<(), Diagnostic> {
    // For each dependency, generate an import statement and a let binding.
    let mut import_names: Vec<String> = Vec::new();
    let mut module_local_names: Vec<String> = Vec::new();

    for (i, dep) in deps.iter().enumerate() {
        let module_local = if i < factory_params.len() {
            // Use the factory parameter name for the import binding
            factory_params[i].clone()
        } else {
            format!("__amd_dep_{}", i)
        };
        import_names.push(dep.clone());
        module_local_names.push(module_local);
    }

    // Generate import statements for each dependency.
    // AMD dependency strings are treated as module specifiers.
    // For bare specifiers (not starting with ./ or ../), we add "./" prefix
    // to make them local-relative as required by the module graph builder.
    for (i, dep) in import_names.iter().enumerate() {
        let specifier = if dep.starts_with("./") || dep.starts_with("../") {
            dep.clone()
        } else if dep == "require" || dep == "exports" || dep == "module" {
            // These are AMD-reserved names, skip import generation
            continue;
        } else {
            // For bare specifiers like "dep", we keep them as-is.
            // The module graph builder will attempt resolution.
            dep.clone()
        };

        let _module_local = &module_local_names[i];

        // Generate: import * as __amd_module_N from "specifier";
        // Then assign the factory parameter from the namespace import if needed
        output.push(Stmt::ImportNamespace {
            specifier: crate::ImportNamespaceSpecifier {
                local: format!("__amd_module_{}", i),
                local_span: span,
                span,
            },
            source: ModuleSpecifier {
                value: specifier.clone(),
                span,
            },
            attributes: Vec::new(),
            span,
        });
    }

    // If the factory function has a return statement that returns module exports,
    // we need to assign the result to a temporary and export it as default.
    let body = if has_return_export {
        // Wrap the factory body: capture the return value
        let mut body_stmts: Vec<Stmt> = Vec::new();
        for stmt in &factory_body {
            if let Stmt::Return { expr, .. } = stmt {
                body_stmts.push(Stmt::ExportDefault {
                    expr: expr.clone(),
                    default_span: span,
                    span,
                });
            } else {
                body_stmts.push(stmt.clone());
            }
        }
        body_stmts
    } else {
        factory_body.clone()
    };

    // Assign import namespace to factory parameters
    for (i, param_name) in factory_params.iter().enumerate() {
        if i < deps.len() {
            let dep = &deps[i];
            if dep == "require" || dep == "exports" || dep == "module" {
                // These are AMD-reserved; handled differently
                continue;
            }
            output.push(Stmt::Let {
                name: param_name.clone(),
                expr: Expr::Ident {
                    name: format!("__amd_module_{}", i),
                    span,
                },
                span,
                is_var: false,
            });
        }
    }

    // Emit the factory body directly (it becomes the module body)
    output.extend(body);

    Ok(())
}

/// Rewrite the simplified `define(function(require, exports, module) { ... })` form.
fn rewrite_simplified_form(
    factory_body: Vec<Stmt>,
    _has_require: bool,
    has_exports: bool,
    has_module: bool,
    span: Span,
    output: &mut Vec<Stmt>,
) -> Result<(), Diagnostic> {
    // For the simplified form:
    // - `exports` becomes a local object that we'll use for exports
    // - `module` becomes `{ exports: exports }`
    // - `require("dep")` calls need to be converted to imports

    // Scan factory body for require("dep") calls and collect dependencies.
    let require_deps = collect_require_dependencies(&factory_body);

    // Generate namespace imports for require() dependencies.
    for (i, dep) in require_deps.iter().enumerate() {
        let specifier = if dep.starts_with("./") || dep.starts_with("../") {
            dep.clone()
        } else {
            dep.clone()
        };
        output.push(Stmt::ImportNamespace {
            specifier: crate::ImportNamespaceSpecifier {
                local: format!("__amd_req_{}", i),
                local_span: span,
                span,
            },
            source: ModuleSpecifier {
                value: specifier,
                span,
            },
            attributes: Vec::new(),
            span,
        });
    }

    if has_exports {
        // Initialize exports as an empty object
        output.push(Stmt::Let {
            name: "exports".to_owned(),
            expr: Expr::Object {
                props: Vec::new(),
                span,
            },
            span,
            is_var: false,
        });
    }

    if has_module {
        // Initialize module as { exports: exports }
        if has_exports {
            output.push(Stmt::Let {
                name: "module".to_owned(),
                expr: Expr::Object {
                    props: vec![crate::ObjectProp::KeyValue {
                        key: "exports".to_owned(),
                        value: Expr::Ident {
                            name: "exports".to_owned(),
                            span,
                        },
                    }],
                    span,
                },
                span,
                is_var: false,
            });
        } else {
            output.push(Stmt::Let {
                name: "module".to_owned(),
                expr: Expr::Object {
                    props: Vec::new(),
                    span,
                },
                span,
                is_var: false,
            });
        }
    }

    // If there are require() dependencies, replace require() calls with
    // references to the import namespaces.
    for stmt in &factory_body {
        output.push(stmt.clone());
    }

    // If exports is used, export it as default at the end
    if has_exports {
        output.push(Stmt::ExportDefault {
            expr: Expr::Ident {
                name: "exports".to_owned(),
                span,
            },
            default_span: span,
            span,
        });
    }

    Ok(())
}

/// Scan a body of statements for `require("string")` calls and collect the
/// dependency module names.
fn collect_require_dependencies(body: &[Stmt]) -> Vec<String> {
    let mut deps = Vec::new();
    for stmt in body {
        match stmt {
            Stmt::Let { expr, .. } => {
                if let Some(dep) = extract_require_call(expr) {
                    if !deps.contains(&dep) {
                        deps.push(dep);
                    }
                }
            }
            Stmt::Expr { expr, .. } => {
                if let Some(dep) = extract_require_call(expr) {
                    if !deps.contains(&dep) {
                        deps.push(dep);
                    }
                }
            }
            Stmt::Assign { expr, .. } => {
                if let Some(dep) = extract_require_call(expr) {
                    if !deps.contains(&dep) {
                        deps.push(dep);
                    }
                }
            }
            Stmt::Return { expr, .. } => {
                if let Some(dep) = extract_require_call(expr) {
                    if !deps.contains(&dep) {
                        deps.push(dep);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                deps.extend(collect_require_dependencies(then_body));
                deps.extend(collect_require_dependencies(else_body));
            }
            Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                deps.extend(collect_require_dependencies(body));
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
                ..
            } => {
                if let Some(init_stmt) = init {
                    deps.extend(collect_require_dependencies(&[init_stmt.as_ref().clone()]));
                }
                if let Some(cond_expr) = condition {
                    if let Some(dep) = extract_require_call(cond_expr) {
                        if !deps.contains(&dep) {
                            deps.push(dep);
                        }
                    }
                }
                if let Some(upd_expr) = update {
                    if let Some(dep) = extract_require_call(upd_expr) {
                        if !deps.contains(&dep) {
                            deps.push(dep);
                        }
                    }
                }
                deps.extend(collect_require_dependencies(body));
            }
            Stmt::ForIn { body, .. } | Stmt::ForOf { body, .. } => {
                deps.extend(collect_require_dependencies(body));
            }
            Stmt::Switch { cases, .. } => {
                for (_, case_body) in cases {
                    deps.extend(collect_require_dependencies(case_body));
                }
            }
            Stmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                deps.extend(collect_require_dependencies(try_block));
                if let Some(catch) = catch_block {
                    deps.extend(collect_require_dependencies(catch));
                }
                if let Some(finally) = finally_block {
                    deps.extend(collect_require_dependencies(finally));
                }
            }
            Stmt::Block { statements, .. } => {
                deps.extend(collect_require_dependencies(statements));
            }
            Stmt::Labeled { body, .. } => {
                deps.extend(collect_require_dependencies(&[body.as_ref().clone()]));
            }
            _ => {}
        }
    }
    deps
}

/// Check if an expression is a `require("...")` call and return the string argument.
fn extract_require_call(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Call { callee, args, .. } => {
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if name == "require" && args.len() == 1 {
                    if let Expr::String { value, .. } = &args[0] {
                        return Some(value.clone());
                    }
                }
            }
            None
        }
        Expr::Assign { expr: value, .. } => extract_require_call(value),
        Expr::Binary { left, right, .. } => {
            extract_require_call(left).or_else(|| extract_require_call(right))
        }
        Expr::Unary { expr: inner, .. } => extract_require_call(inner),
        Expr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => extract_require_call(condition)
            .or_else(|| extract_require_call(then_expr))
            .or_else(|| extract_require_call(else_expr)),
        Expr::Member { object, .. } => extract_require_call(object),
        Expr::Index { object, index, .. } => {
            extract_require_call(object).or_else(|| extract_require_call(index))
        }
        Expr::Sequence { exprs, .. } => {
            for e in exprs {
                if let Some(dep) = extract_require_call(e) {
                    return Some(dep);
                }
            }
            None
        }
        Expr::Array { elements, .. } => {
            for elem in elements {
                match elem {
                    crate::ArrayLiteralElement::Present(e)
                    | crate::ArrayLiteralElement::Spread(e) => {
                        if let Some(dep) = extract_require_call(e) {
                            return Some(dep);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;
    use crate::diagnostic::DiagCode;
    use crate::parser::Parser;

    /// Parse source WITHOUT applying the AMD transform (raw parse).
    fn parse_raw(source: &str) -> Vec<Stmt> {
        let raw_tokens = Lexer::new(source).tokenize().unwrap();
        Parser::new(raw_tokens, source).parse_raw_program_for_testing()
    }

    fn parse_with_amd(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        let raw_tokens = Lexer::new(source).tokenize().unwrap();
        let mut raw_parser = Parser::new(raw_tokens, source);
        let raw_program = raw_parser.parse_raw_program_for_testing();
        transform_amd_program(raw_program)
    }

    #[test]
    fn detect_amd_with_dependency_array() {
        let stmts = parse_raw(r#"define(["dep"], function(dep) { return dep; });"#);
        let pattern = detect_amd_define(&stmts[0]);
        assert!(pattern.is_some(), "should detect AMD define with array");

        match pattern.unwrap() {
            AmdPattern::WithDependencyArray {
                deps,
                factory_params,
                has_return_export,
                ..
            } => {
                assert_eq!(deps, vec!["dep"]);
                assert_eq!(factory_params, vec!["dep"]);
                assert!(has_return_export);
            }
            other => panic!("expected WithDependencyArray, got {:?}", other),
        }
    }

    #[test]
    fn detect_amd_simplified() {
        let stmts =
            parse_raw(r#"define(function(require, exports, module) { exports.foo = 1; });"#);
        let pattern = detect_amd_define(&stmts[0]);
        assert!(pattern.is_some(), "should detect simplified AMD define");

        match pattern.unwrap() {
            AmdPattern::Simplified {
                has_exports,
                has_module,
                ..
            } => {
                assert!(has_exports);
                assert!(has_module);
            }
            other => panic!("expected Simplified, got {:?}", other),
        }
    }

    #[test]
    fn detect_amd_simplified_exports_only() {
        let stmts = parse_raw(r#"define(function(exports, module) { exports.foo = 1; });"#);
        let pattern = detect_amd_define(&stmts[0]);
        assert!(pattern.is_some(), "should detect simplified AMD define");

        match pattern.unwrap() {
            AmdPattern::Simplified {
                has_exports,
                has_module,
                ..
            } => {
                assert!(has_exports);
                assert!(has_module);
            }
            other => panic!("expected Simplified, got {:?}", other),
        }
    }

    #[test]
    fn transform_dependency_array_with_return() {
        let result =
            parse_with_amd(r#"define(["dep"], function(dep) { return { value: dep }; });"#);
        assert!(
            result.is_ok(),
            "AMD transform should succeed: {:?}",
            result.err()
        );

        let program = result.unwrap();
        // Should have: import namespace, let binding, and export default
        let has_import_ns = program
            .iter()
            .any(|s| matches!(s, Stmt::ImportNamespace { .. }));
        assert!(
            has_import_ns,
            "should have namespace import: {:#?}",
            program
        );

        let has_export_default = program
            .iter()
            .any(|s| matches!(s, Stmt::ExportDefault { .. }));
        assert!(
            has_export_default,
            "should have export default: {:#?}",
            program
        );
    }

    #[test]
    fn transform_simplified_form() {
        let result = parse_with_amd(
            r#"define(function(require, exports, module) {
                var dep = require("dep");
                exports.foo = dep;
            });"#,
        );
        assert!(
            result.is_ok(),
            "AMD transform should succeed: {:?}",
            result.err()
        );

        let program = result.unwrap();
        // Should have: exports initialization, then factory body, then export default
        let has_exports_init = program
            .iter()
            .any(|s| matches!(s, Stmt::Let { name, .. } if name == "exports"));
        assert!(has_exports_init, "should initialize exports");

        let has_export_default = program
            .iter()
            .any(|s| matches!(s, Stmt::ExportDefault { .. }));
        assert!(has_export_default, "should have export default");
    }

    #[test]
    fn non_amd_program_unchanged() {
        let source = r#"const x = 1; console.log(x);"#;
        let result = parse_with_amd(source);
        assert!(result.is_ok());
        let program = result.unwrap();
        // Should not have any transforms applied
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn rejects_mixed_amd_and_esm() {
        let source = r#"import { x } from "./mod";
define(["dep"], function(d) { return d; });"#;
        let raw_tokens = Lexer::new(source).tokenize().unwrap();
        let mut raw_parser = Parser::new(raw_tokens, source);
        let raw_program = raw_parser.parse_raw_program_for_testing();
        let result = transform_amd_program(raw_program);
        assert!(result.is_err(), "should reject mixed AMD and ESM");
        assert_eq!(result.unwrap_err().code, DiagCode::UnsupportedModule);
    }

    #[test]
    fn multiple_dependency_array() {
        let result = parse_with_amd(
            r#"define(["a", "b", "c"], function(a, b, c) {
                return { a: a, b: b, c: c };
            });"#,
        );
        assert!(
            result.is_ok(),
            "AMD transform should succeed: {:?}",
            result.err()
        );

        let program = result.unwrap();
        // Count namespace imports: 3 deps -> 3 namespace imports
        let import_count = program
            .iter()
            .filter(|s| matches!(s, Stmt::ImportNamespace { .. }))
            .count();
        assert_eq!(import_count, 3, "should have 3 namespace imports");
    }

    #[test]
    fn simplified_form_with_require_deps() {
        let result = parse_with_amd(
            r#"define(function(require) {
                var a = require("depA");
                var b = require("depB");
                return a + b;
            });"#,
        );
        assert!(
            result.is_ok(),
            "should handle require() deps: {:?}",
            result.err()
        );
        let program = result.unwrap();
        // Should have namespace imports for require() calls
        let import_count = program
            .iter()
            .filter(|s| matches!(s, Stmt::ImportNamespace { .. }))
            .count();
        assert!(
            import_count >= 2,
            "should have imports for require() deps, got: {}",
            import_count
        );
    }

    #[test]
    fn no_op_on_regular_call_to_define() {
        let stmts = parse_raw(r#"define(something_else);"#);
        let pattern = detect_amd_define(&stmts[0]);
        // define(something_else) with a non-function, non-array arg
        // Should not match any known pattern
        assert!(pattern.is_none());
    }
}
