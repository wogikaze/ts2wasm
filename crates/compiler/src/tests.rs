use super::*;
use crate::stages::lower::ModuleExport;
use crate::stages::parse::is_typescript_virtual_section;
use ts2wasm_source::Span;
use ts2wasm_syntax::{Expr, Stmt};

#[test]
fn parses_console_log_string() {
    let program = parse_program("console.log(\"hi\");").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Expr {
            expr: Expr::Call { callee, args, .. },
            ..
        } => {
            assert!(matches!(
                args.as_slice(),
                [Expr::String { value, .. }] if value == "hi"
            ));
            assert!(matches!(
                callee.as_ref(),
                Expr::Member { property, .. } if property == "log"
            ));
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn parses_m2_subset() {
    let source = r#"
        let i = 0;
        let sum = 0;
        while (i < 3) {
            sum = sum + i;
            i = i + 1;
        }
        function add(a, b) { return a + b; }
        if (true) { console.log("sum=" + sum); } else { console.log("bad"); }
        console.log(add(2, 3));
    "#;
    let program = parse_program(source).unwrap();
    assert_eq!(program.len(), 6);
}

#[test]
fn parses_m3_semantics() {
    let source = r#"
        console.log(undefined);
        console.log(null);
        console.log(null === undefined);
        console.log("x" + true);
        if (!0) { console.log("zero false"); }
    "#;
    let program = parse_program(source).unwrap();
    assert_eq!(program.len(), 5);
}

#[test]
fn parses_program_with_utf8_bom() {
    let program = parse_program("\u{feff}console.log(1);").unwrap();
    assert_eq!(program.len(), 1);
}

#[test]
fn classifies_package_json_virtual_section_as_non_typescript() {
    let source = r#"
// @filename: node_modules/typescript/package.json
{
    "name": "typescript",
    "types": "/.ts/typescript.d.ts"
}
// @filename: APISample_transform.ts
console.log("ok");
"#;
    let sections = split_file_name_sections(source);
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].0, "node_modules/typescript/package.json");
    assert!(!is_typescript_virtual_section(Path::new(&sections[0].0)));
    assert!(is_typescript_virtual_section(Path::new(&sections[1].0)));
}

#[test]
fn reports_namespace_only_multi_section_with_section_name() {
    let dir = unique_temp_dir("namespace-only-multi-section");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let input = dir.join("entry.ts");
    let output = dir.join("out.wasm");
    let source = r#"
// @Filename: test.ts
namespace C {
    export class Name {}
}

// @Filename: typings.d.ts
declare namespace A {
    namespace AA {
        function func(): number;
    }
}
"#;
    std::fs::write(&input, source).expect("multi-section source should be written");

    let err = build_file(&input, &output)
        .expect_err("namespace-only multi-section should report focused section diagnostic");
    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(
        err.message.contains("section `test.ts`"),
        "diagnostic should include section name: {err:?}"
    );
    assert!(!err.message.contains("no module bodies"));
    assert_eq!(err.span, Some(Span { start: 0, end: 9 }));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn parses_program_with_line_comment_prefix() {
    let program = parse_program("// lead comment\nconsole.log(1);").unwrap();
    assert_eq!(program.len(), 1);
}

#[test]
fn parses_program_with_block_comment_prefix() {
    let program = parse_program("/*--- metadata ---*/\nconsole.log(1);").unwrap();
    assert_eq!(program.len(), 1);
}

#[test]
fn parses_program_with_dollar_identifier() {
    let program = parse_program("let $done = 1; console.log($done);").unwrap();
    assert_eq!(program.len(), 2);
}

#[test]
fn rejects_unterminated_block_comment() {
    let err = parse_program("/* unterminated").unwrap_err();
    assert_eq!(err.code, DiagCode::SyntaxError);
    assert!(err.message.contains("unterminated block comment"));
    assert!(err.span.is_some());
}

#[test]
fn parses_const_statement() {
    let program = parse_program("const x = 1;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Number { value, .. } => assert_eq!(*value, 1),
                _ => panic!("expected number expression"),
            }
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn parses_var_statement() {
    let program = parse_program("var x = 1;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Number { value, .. } => assert_eq!(*value, 1),
                _ => panic!("expected number expression"),
            }
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn rejects_top_level_return_in_ast_validation() {
    let program = parse_program("return 1;").unwrap();
    let err = validate_ast(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::InvalidTopLevelReturn);
}

#[test]
fn permits_nested_function_in_ast_validation() {
    let program = parse_program("if (true) { function f() { return 1; } }").unwrap();
    validate_ast(&program).expect("nested function lowering handles support diagnostics");
}

#[test]
fn rejects_duplicate_let_in_same_scope() {
    let program = parse_program("let x = 1; let x = 2;").unwrap();
    let err = validate_ast(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::DuplicateLocal);
    assert!(err.span.is_some());
}

#[test]
fn accepts_multiple_empty_binding_patterns() {
    // Empty destructuring patterns use synthetic names "{}" and "[]"
    // and should not trigger DuplicateLocal.
    let program = parse_program("const {} = f(); const [] = f(); const {} = g();").unwrap();
    assert!(validate_ast(&program).is_ok());
}

#[test]
fn m6_3b_1_runtime_gate_permits_read_stdin_bytes_execution_path() {
    let ast = parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    ensure_runtime_feature_gates(&lowered).expect("gate must pass after M6-3b-1 enables runtime");
}

#[test]
fn parses_logical_and_operator() {
    let program = parse_program("let x = 1 && 2;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Binary { op, .. } => {
                    assert!(matches!(op, ts2wasm_frontend::BinaryOp::And));
                }
                _ => panic!("expected binary expression"),
            }
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn parses_logical_or_operator() {
    let program = parse_program("let x = 1 || 2;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Binary { op, .. } => {
                    assert!(matches!(op, ts2wasm_frontend::BinaryOp::Or));
                }
                _ => panic!("expected binary expression"),
            }
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn parses_greater_than_operator() {
    let program = parse_program("let x = 5 > 3;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            match expr {
                Expr::Binary { op, .. } => {
                    assert!(matches!(op, ts2wasm_frontend::BinaryOp::Greater));
                }
                _ => panic!("expected binary expression"),
            }
        }
        other => panic!("unexpected stmt: {other:?}"),
    }
}

#[test]
fn parses_typeof_operator() {
    let program = parse_program("let t = typeof x;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "t");
            assert!(matches!(expr, Expr::TypeOf { .. }));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_typescript_type_annotations_as_syntax_only() {
    let source = r#"
        function add(a: number, b: number): number { return a + b; }
        const limit: number = 4;
        let done: boolean = limit >= 4;
        console.log(add(limit, 2), done);
    "#;
    let program = parse_program(source).unwrap();
    assert_eq!(program.len(), 4);
    match &program[0] {
        Stmt::Function { params, .. } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].0, "a");
            assert_eq!(params[1].0, "b");
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_instanceof_expression() {
    let program = parse_program("let b = x instanceof Array;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "b");
            assert!(matches!(expr, Expr::InstanceOf { .. }));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_ternary_expression() {
    let program = parse_program("let x = a ? b : c;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "x");
            assert!(matches!(expr, Expr::Ternary { .. }));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_arrow_function_single_param() {
    let program = parse_program("let f = x => x + 1;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "f");
            assert!(matches!(expr, Expr::ArrowFn { .. }));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_new_expression() {
    let program = parse_program("let obj = new Array(10);").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "obj");
            assert!(matches!(expr, Expr::New { .. }));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_do_while_loop() {
    let program = parse_program("do { x = 1; } while (x);").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::DoWhile { .. } => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_for_loop_with_init_cond_update() {
    // For loop variant (full traditional for loop)
    // Note: Parser supports for statement dispatch, full expression parsing in for update may be deferred
    let program = parse_program("for (;;) { break; }").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::For { .. } => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_power_operator() {
    let program = parse_program("let p = 2 ** 3;").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Let { expr, .. } => {
            assert!(matches!(
                expr,
                Expr::Binary {
                    op: ts2wasm_frontend::BinaryOp::Power,
                    ..
                }
            ));
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_bitwise_operators() {
    let program = parse_program("let b = (a & b) | (c ^ d) | ~e;").unwrap();
    assert_eq!(program.len(), 1);
    let span = program[0].span();
    assert!(span.start < usize::MAX);
}

#[test]
fn parses_shift_operators() {
    let program = parse_program("let s = (a << 2) | (b >> 1) | (c >>> 3);").unwrap();
    assert_eq!(program.len(), 1);
    let span = program[0].span();
    assert!(span.start < usize::MAX);
}

#[test]
fn parses_throw_statement() {
    let program = parse_program("throw new Error();").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Throw { .. } => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_try_catch_finally() {
    let program = parse_program("try { x = 1; } catch (e) { } finally { cleanup(); }").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::TryCatch { .. } => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn parses_switch_statement() {
    let program = parse_program("switch (x) { case 1: break; default: break; }").unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Stmt::Switch { .. } => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn static_named_import_binding_lowering_uses_source_export_when_importer_shadows_name() {
    let dir = unique_temp_dir("static-binding-shadow");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let entry = dir.join("entry.ts");
    let source_module = dir.join("source.ts");
    let entry_source = r#"
import { value as importedValue } from "./source";
const value = 99;
console.log(importedValue);
"#;
    std::fs::write(&entry, entry_source).expect("entry should be written");
    std::fs::write(&source_module, "export const value = 1;\n")
        .expect("source module should be written");

    let program = parse_program(entry_source).expect("entry should parse");
    validate_ast(&program).expect("entry should validate");
    let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
    let lowering = lower_static_named_import_bindings_for_build(&program, &graph)
        .expect("binding lowering should succeed");

    assert_eq!(lowering.named_imports.len(), 1);
    let binding = &lowering.named_imports[0];
    assert_eq!(binding.source_specifier, "./source");
    assert_eq!(binding.source_module_id, 1);
    assert_eq!(binding.source_path, source_module.canonicalize().unwrap());
    assert_eq!(binding.imported_name, "value");
    assert_eq!(binding.local_name, "importedValue");
    assert_eq!(binding.lowered_statement_index, 0);
    assert!(matches!(binding.initializer, Expr::Number { value: 1, .. }));

    match &lowering.rewritten_program[0] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "importedValue");
            assert!(matches!(expr, Expr::Number { value: 1, .. }));
        }
        other => panic!("unexpected rewritten import stmt: {other:?}"),
    }
    match &lowering.rewritten_program[1] {
        Stmt::Let { name, expr, .. } => {
            assert_eq!(name, "value");
            assert!(matches!(expr, Expr::Number { value: 99, .. }));
        }
        other => panic!("unexpected importer shadow stmt: {other:?}"),
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn static_module_export_lowering_populates_explicit_lowered_module_statements() {
    let dir = unique_temp_dir("static-module-export-ir");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let entry = dir.join("entry.ts");
    let source_module = dir.join("source.ts");
    let entry_source = r#"
import { value } from "./source";
console.log(value);
"#;
    std::fs::write(&entry, entry_source).expect("entry should be written");
    std::fs::write(&source_module, "export const value = 1;\n")
        .expect("source module should be written");

    let program = parse_program(entry_source).expect("entry should parse");
    validate_ast(&program).expect("entry should validate");
    let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
    let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
        .expect("static named import binding should lower");
    let name_resolved =
        ts2wasm_ir::name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("names should resolve");
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&name_resolved)
        .expect("builtins should resolve");
    let lowered_program = lowered::lower_program(&resolved).expect("program should lower");
    let lowered_program = lower_static_named_import_reads_for_build(
        lowered_program,
        &static_module_binding.named_imports,
    )
    .expect("static named import reads should lower through module exports");
    let lowered_program = populate_static_module_exports_for_build(lowered_program, &graph, &[])
        .expect("static module exports should populate lowered metadata");

    match &lowered_program.top_level_statements[0] {
        lowered::LoweredStmt::Let(_, lowered::LoweredExpr::PropertyGet { obj, key, .. }, _) => {
            assert_eq!(key, "value");
            assert!(matches!(
                obj.as_ref(),
                lowered::LoweredExpr::ModuleLoad { module_id: 1, .. }
            ));
        }
        other => panic!("unexpected lowered import read statement: {other:?}"),
    }
    assert_eq!(lowered_program.modules.len(), 1);
    let module = &lowered_program.modules[0];
    assert_eq!(module.id, 1);
    assert_eq!(module.specifier, "./source");
    assert_eq!(module.locals_count, 1);
    assert_eq!(
        module.statements,
        vec![
            lowered::LoweredStmt::Let(
                lowered::LocalId(0),
                lowered::LoweredExpr::Number(1, Span::generated("test")),
                Span::generated("test")
            ),
            lowered::LoweredStmt::Export {
                name: "value".to_owned(),
                expr: lowered::LoweredExpr::Number(1, Span::generated("test")),
                span: Span::generated("test"),
            },
        ]
    );
    lowered::validate_lowered(&lowered_program)
        .expect("module statements should validate as lowered IR");

    let (validated, _diags) =
        ts2wasm_ir::lowered::Validated::new(lowered_program).expect("already validated above");
    let wat =
        backend::emit_wat(&validated).expect("lowered module metadata should remain buildable");
    assert!(wat.contains("$module_require"));
    assert!(wat.contains("$property_get"));
    assert!(wat.contains("$module_exports_set"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn static_default_export_rewrite_uses_unique_synthetic_locals() {
    let dir = unique_temp_dir("static-default-export-unique");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let entry = dir.join("entry.ts");
    let source = r#"
export default 1;
export default 2;
"#;
    std::fs::write(&entry, source).expect("entry should be written");

    let program = parse_program(source).expect("entry should parse");
    let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
    let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
        .expect("static default export binding should lower");
    let name_resolved =
        ts2wasm_ir::name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("synthetic default locals should not collide");

    let names = static_module_binding
        .rewritten_program
        .iter()
        .filter_map(|stmt| match stmt {
            Stmt::Let { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["__ts2wasm_default_0", "__ts2wasm_default_1"]);
    assert_eq!(name_resolved.len(), 2);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn static_function_export_lowering_populates_entry_module_export() {
    let dir = unique_temp_dir("static-function-export-entry");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let entry = dir.join("entry.ts");
    let source = "export function f() { return 1; }\n";
    std::fs::write(&entry, source).expect("entry should be written");

    let program = parse_program(source).expect("entry should parse");
    validate_ast(&program).expect("entry should validate");
    let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
    let static_module_binding = lower_static_named_import_bindings_for_build(&program, &graph)
        .expect("static function export binding should lower");

    assert_eq!(
        static_module_binding.module_exports,
        vec![ModuleExport {
            name: "f".to_owned(),
            lowered_statement_index: 0,
        }]
    );
    match &static_module_binding.rewritten_program[0] {
        Stmt::Let {
            name,
            expr: Expr::FunctionExpr {
                name: expr_name, ..
            },
            ..
        } => {
            assert_eq!(name, "f");
            assert_eq!(expr_name, "f");
        }
        other => panic!("unexpected rewritten export function stmt: {other:?}"),
    }

    let name_resolved =
        ts2wasm_ir::name_resolver::resolve_names(&static_module_binding.rewritten_program)
            .expect("rewritten function export should resolve");
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&name_resolved)
        .expect("builtins should resolve");
    let lowered_program = lowered::lower_program(&resolved).expect("program should lower");
    let lowered_program = populate_static_module_exports_for_build(
        lowered_program,
        &graph,
        &static_module_binding.module_exports,
    )
    .expect("entry function export should populate module metadata");

    assert_eq!(lowered_program.modules.len(), 1);
    let module = &lowered_program.modules[0];
    assert_eq!(module.id, 0);
    assert_eq!(module.specifier, "<entry>");
    assert_eq!(module.locals_count, 1);
    match &module.statements[..] {
        [
            lowered::LoweredStmt::Export {
                name,
                expr:
                    lowered::LoweredExpr::ArrowFn {
                        representation: lowered::ClosureRepresentation::DirectLocalToken,
                        ..
                    },
                span: _,
            },
        ] => assert_eq!(name, "f"),
        other => panic!("unexpected entry module export statements: {other:?}"),
    }
    lowered::validate_lowered(&lowered_program)
        .expect("entry function export metadata should validate as lowered IR");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn static_module_export_lowering_orders_module_metadata_dependency_first() {
    let dir = unique_temp_dir("static-module-export-order");
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let entry = dir.join("entry.ts");
    let source_module = dir.join("source.ts");
    let nested_module = dir.join("nested.ts");
    let entry_source = r#"import { value } from "./source";"#;
    std::fs::write(&entry, entry_source).expect("entry should be written");
    std::fs::write(
        &source_module,
        r#"
import { nested } from "./nested";
export const value = 1;
"#,
    )
    .expect("source module should be written");
    std::fs::write(&nested_module, "export const nested = 2;\n")
        .expect("nested module should be written");

    let program = parse_program(entry_source).expect("entry should parse");
    let graph = build_entry_module_graph(&entry, &program).expect("graph should build");
    let lowered_program = lowered::LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![],
        functions: vec![],
        modules: vec![],
    };
    let lowered_program = populate_static_module_exports_for_build(lowered_program, &graph, &[])
        .expect("static module exports should populate lowered metadata");

    let module_ids = lowered_program
        .modules
        .iter()
        .map(|module| module.id)
        .collect::<Vec<_>>();
    assert_eq!(module_ids, vec![2, 1]);
    assert_eq!(lowered_program.modules[0].specifier, "./nested");
    assert_eq!(lowered_program.modules[1].specifier, "./source");

    let _ = std::fs::remove_dir_all(&dir);
}

fn unique_temp_dir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ts2wasm-compiler-{label}-{unique}"))
}

// ---------------------------------------------------------------------------
// Helpers and tests moved from backend-wasm (issue 303: frontend dep removal)
// ---------------------------------------------------------------------------
use std::fs;
use std::path::Path;
use std::process::Command;
use ts2wasm_frontend::{Lexer, Parser};
use ts2wasm_runtime_abi::ValueTag;

fn lower_fixture(relative_path: &str) -> ts2wasm_ir::lowered::LoweredProgram {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    let source = fs::read_to_string(&path).expect("fixture should be readable");
    let tokens = Lexer::new(&source)
        .tokenize()
        .expect("fixture should tokenize");
    let parsed = Parser::new(tokens, &source)
        .parse_program()
        .expect("fixture should parse");
    let named =
        ts2wasm_ir::name_resolver::resolve_names(&parsed).expect("fixture should resolve names");
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named)
        .expect("fixture should resolve builtins");
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).expect("fixture should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered).expect("fixture lowered IR should validate");
    lowered
}

fn wat_function<'a>(wat: &'a str, symbol: &str) -> &'a str {
    let marker = format!("  (func ${symbol}");
    let start = wat
        .find(&marker)
        .unwrap_or_else(|| panic!("WAT should contain function ${symbol}"));
    let rest = &wat[start..];
    let end = rest[1..]
        .find("\n  (func $")
        .map(|offset| offset + 1)
        .unwrap_or(rest.len());
    &rest[..end]
}

fn assert_binary_imports_fd_write(wasm: &[u8]) {
    assert!(
        wasm.windows(b"wasi_snapshot_preview1".len())
            .any(|window| window == b"wasi_snapshot_preview1")
    );
    assert!(
        wasm.windows(b"fd_write".len())
            .any(|window| window == b"fd_write")
    );
}

fn run_iwasm(wasm_path: &Path) -> String {
    let output = Command::new("iwasm")
        .arg(wasm_path)
        .output()
        .expect("iwasm should run");
    assert!(
        output.status.success(),
        "iwasm failed for {}\nstdout:\n{}\nstderr:\n{}",
        wasm_path.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("iwasm stdout should be UTF-8")
}

#[test]
fn direct_wasm_binary_mvp_runs_basics_hello_like_wat_path() {
    let program = lower_fixture("../../fixtures/basics-hello/hello.ts");
    let (validated, _) =
        ts2wasm_ir::lowered::Validated::new(program).expect("hello fixture should pass validation");
    let direct_wasm = backend::emit_wasm_binary_mvp(&validated)
        .expect("hello fixture should emit direct wasm binary");
    assert_binary_imports_fd_write(&direct_wasm);

    let validated_plan =
        backend::build_validated_runtime_link_plan(validated.as_ref()).expect("valid link plan");
    let manifest: serde_json::Value =
        serde_json::from_str(&backend::emit_canonical_manifest_json(&validated_plan))
            .expect("manifest should be valid JSON");
    assert_eq!(manifest["wasi"]["stdout"], true);
    assert!(
        manifest["capability_reasons"]["wasi.stdout"]
            .as_array()
            .expect("wasi.stdout should record audit reasons")
            .iter()
            .any(|reason| reason == "console.log")
    );

    let wat = backend::emit_wat(&validated).expect("hello fixture should still emit WAT");
    let temp_dir = unique_temp_dir("direct-wasm-binary-mvp");
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let direct_path = temp_dir.join("hello-direct.wasm");
    let wat_path = temp_dir.join("hello-wat.wat");
    let wat_wasm_path = temp_dir.join("hello-wat.wasm");
    fs::write(&direct_path, direct_wasm).expect("direct wasm should be written");
    fs::write(&wat_path, wat).expect("wat should be written");

    let wat2wasm = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(&wat_wasm_path)
        .output()
        .expect("wat2wasm should run");
    assert!(
        wat2wasm.status.success(),
        "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&wat2wasm.stdout),
        String::from_utf8_lossy(&wat2wasm.stderr)
    );

    let direct_out = run_iwasm(&direct_path);
    let wat_out = run_iwasm(&wat_wasm_path);
    assert_eq!(direct_out, "hi\n");
    assert_eq!(direct_out, wat_out);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn heap_closure_allocation_and_dispatch_emit_abi_payload_and_roots() {
    let program =
        lower_fixture("../../fixtures/core-semantics/ordinary-function-closure-make-adder.ts");

    let (v, _) = ts2wasm_ir::lowered::Validated::new(program).expect("should validate");
    let wat = backend::emit_wat(&v).expect("returned closure fixture should emit WAT");

    assert!(wat.contains("(i32.const -2)"));
    assert!(wat.contains("(i32.const 20)"));
    assert!(wat.contains("(i32.const 16)"));
    assert!(wat.contains("(block $heap_closure_dispatch_done (result i32)"));
    assert!(wat.contains("(call $func_1)"));
    assert!(wat.contains(
        "(i32.store (i32.add (global.get $gc_call_frame_current) (i32.const 8)) (local.get 0))"
    ));
}

#[test]
fn gc_mark_object_payload_marks_heap_closure_capture_slots() {
    let program =
        lower_fixture("../../fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts");

    let (v, _) = ts2wasm_ir::lowered::Validated::new(program).expect("should validate");
    let wat = backend::emit_wat(&v).expect("returned closure GC fixture should emit WAT");

    assert!(wat.contains("(func $gc_mark_object_payload"));
    assert!(wat.contains("(i32.const -2)"));
    assert!(wat.contains("(i32.const 8)"));
    assert!(wat.contains("(block $closure_done"));
    assert!(wat.contains("(loop $closure_scan"));
    assert!(wat.contains("(i32.const 16)"));
    assert!(wat.contains("(i32.const 4)"));
    assert!(wat.contains("(call $gc_mark_value (i32.load (local.get $entry_ptr)))"));
    let payload_start = wat
        .find("(func $gc_mark_object_payload")
        .expect("gc payload marker should exist");
    let payload_wat = &wat[payload_start..];
    let closure_done_start = payload_wat
        .find("(block $closure_done")
        .expect("closure scan block should exist");
    let object_scan_start = payload_wat
        .find("(if (i32.eq (local.get $count) (i32.const -1))")
        .expect("ordinary object payload scan should exist");
    let closure_scan_return = payload_wat[closure_done_start..object_scan_start]
        .find("(return)")
        .expect("closure payload scan should return before object scan");
    assert!(
        closure_done_start + closure_scan_return < object_scan_start,
        "closure marking must return before ordinary object payload scanning"
    );
}

#[test]
fn env_cells_are_tagged_array_payloads_for_gc_tracing() {
    let program =
        lower_fixture("../../fixtures/core-semantics/class-method-mutable-outer-capture.ts");
    let (v, _) = ts2wasm_ir::lowered::Validated::new(program).expect("should validate");
    let wat = backend::emit_wat(&v).expect("mutable class method env cell fixture should emit WAT");

    // Env cell: ARRAY_HEADER_SIZE=20 + ENV_CELL_SLOT_COUNT*4=4 = 24 bytes
    assert!(
        wat.contains("(call $alloc_heap (i32.const 24))"),
        "env cells need an array header (20 bytes) plus one captured value slot (4 bytes)"
    );
    // The array length field stores EC (env cell slot count = 1)
    assert!(
        wat.contains("(i32.const 1))"),
        "env cell payload should use array length 1 so GC scans its value slot"
    );
    // The env cell pointer is ORed with the ARRAY tag
    assert!(
        wat.contains(&format!("(i32.const {}))", ValueTag::ARRAY_TAG)),
        "env cell roots/captures must hold a tagged heap value"
    );
    // Env cell load uses HEAP_MASK and ENV_CELL_VALUE_OFFSET (= ARRAY_HEADER_SIZE = 20).
    // We do not hardcode the local index because it depends on the fixture's function
    // parameter layout; any (i32.load ... i32.and (local.get <N>) ... i32.const -8 ... 20)
    // is accepted.
    assert!(
        wat.lines().any(|line| {
            line.contains("(i32.load")
                && line.contains("(i32.and (local.get")
                && line.contains(&format!(
                    "(i32.const {})) (i32.const 20)",
                    ValueTag::HEAP_MASK
                ))
        }),
        "env cell reads should mask the tagged cell before loading the value slot at offset 20"
    );
    // Same for env cell writes.
    assert!(
        wat.lines().any(|line| {
            line.contains("(i32.store")
                && line.contains("(i32.and (local.get")
                && line.contains(&format!(
                    "(i32.const {})) (i32.const 20)",
                    ValueTag::HEAP_MASK
                ))
        }),
        "env cell writes should mask the tagged captured cell before storing the value slot"
    );
    assert!(
        wat.contains("(call $gc_mark_value (i32.load (local.get $elem_ptr)))"),
        "tagged env cells should be traced through the existing array GC scanner"
    );
}

#[test]
fn array_push_grow_emits_dedicated_helper_boundary() {
    let program = lower_fixture("../../fixtures/core-semantics/array-push-recursive-growth.ts");
    let (v, _) = ts2wasm_ir::lowered::Validated::new(program).expect("should validate");
    let wat = backend::emit_wat(&v).expect("array push growth fixture should emit WAT");

    assert!(wat.contains("(func $array_push_grow"));
    assert!(wat.contains("(call $array_push_grow)"));
    assert!(wat.contains("(local $new_capacity i32)"));
    assert!(wat.contains("(call $alloc_heap"));
    assert!(wat.contains("(call $copy"));

    let temp_dir = unique_temp_dir("array-push-grow-helper");
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let wat_path = temp_dir.join("array-push-grow-helper.wat");
    let wasm_path = temp_dir.join("array-push-grow-helper.wasm");
    fs::write(&wat_path, wat).expect("wat should be written");

    let wat2wasm = Command::new("wat2wasm")
        .arg(&wat_path)
        .arg("-o")
        .arg(&wasm_path)
        .output()
        .expect("wat2wasm should run");
    assert!(
        wat2wasm.status.success(),
        "wat2wasm failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&wat2wasm.stdout),
        String::from_utf8_lossy(&wat2wasm.stderr)
    );

    let _ = fs::remove_dir_all(temp_dir);
}
