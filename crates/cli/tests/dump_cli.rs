use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn write_temp_source(name: &str, source: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ts2wasm-dump-{name}-{}-{}.ts",
        std::process::id(),
        unique_suffix()
    ));
    fs::write(&path, source).expect("source fixture should be written");
    path
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
}

fn run_dump(args: &[&str], source: &str) -> String {
    let path = write_temp_source("cli", source);
    let output = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("dump")
        .args(args)
        .arg(&path)
        .output()
        .expect("ts2wasm dump should execute");

    assert!(
        output.status.success(),
        "dump failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("dump output should be valid UTF-8")
}

fn run_dump_error(args: &[&str], source: &str) -> String {
    let path = write_temp_source("cli-error", source);
    let output = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("dump")
        .args(args)
        .arg(&path)
        .output()
        .expect("ts2wasm dump should execute");

    assert!(
        !output.status.success(),
        "dump unexpectedly succeeded:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    String::from_utf8(output.stderr).expect("dump stderr should be valid UTF-8")
}

fn fixture_path(fixture: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(fixture)
}

fn build_fixture(fixture: &str) {
    let input = fixture_path(fixture);
    assert!(input.exists(), "fixture should exist: {:?}", input);

    let output = std::env::temp_dir().join(format!(
        "ts2wasm-dump-build-{}-{}.wasm",
        fixture.replace(['/', '.'], "_"),
        unique_suffix()
    ));
    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("ts2wasm build should execute");

    assert!(
        build.status.success(),
        "build failed for {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn dump_ast_unparse_emits_pseudo_source() {
    let output = run_dump(&["--ast", "--unparse"], "let x = 1 + 2;");

    assert_eq!(output, "let x = (1 + 2);\n");
}

#[test]
fn dump_ast_unparse_normalizes_numeric_literal_separators() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let decimal = 1_000; let binary = 0b1010_0101; let octal = 0o7_7; let hex = 0xF_F;",
    );

    assert_eq!(
        output,
        "let decimal = 1000;\nlet binary = 165;\nlet octal = 63;\nlet hex = 255;\n"
    );
}

#[test]
fn dump_ast_reports_invalid_numeric_literal_separator() {
    let stderr = run_dump_error(&["--ast", "--unparse"], "let value = 1__0;");

    assert!(stderr.contains("[UnsupportedSyntax]"), "{stderr}");
    assert!(stderr.contains("numeric separator"), "{stderr}");
}

#[test]
fn dump_ast_unparse_preserves_optional_chaining_forms() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let a = obj?.x; let b = obj?.[key]; let c = fn?.(1);",
    );

    assert_eq!(
        output,
        "let a = obj?.x;\nlet b = obj?.[key];\nlet c = fn?.(1);\n"
    );
}

#[test]
fn dump_ast_reports_invalid_optional_chaining_assignment_target() {
    let stderr = run_dump_error(&["--ast"], "obj?.x = 1;");

    assert!(stderr.contains("[UnsupportedSyntax]"), "{stderr}");
    assert!(stderr.contains("issue-246"), "{stderr}");
    assert!(stderr.contains("assignment or update target"), "{stderr}");
}

#[test]
fn dump_ast_class_static_block_preserves_block_and_statement_spans() {
    let output = run_dump(&["--ast"], "class C { static { console.log(1); } }");

    assert!(output.contains("static_blocks"), "{output}");
    assert!(output.contains("ClassStaticBlock"), "{output}");
    assert!(output.contains("start: 10,"), "{output}");
    assert!(output.contains("end: 36,"), "{output}");
    assert!(output.contains("start: 19,"), "{output}");
    assert!(output.contains("end: 34,"), "{output}");
}

#[test]
fn dump_ast_private_class_field_preserves_private_identifier_span() {
    let output = run_dump(&["--ast"], "class C { #x = 1; }");

    assert!(output.contains("private_elements"), "{output}");
    assert!(output.contains("Field"), "{output}");
    assert!(output.contains("name: \"x\""), "{output}");
    assert!(output.contains("name_span"), "{output}");
    assert!(output.contains("start: 10,"), "{output}");
    assert!(output.contains("end: 12,"), "{output}");
}

#[test]
fn dump_ast_classifies_bigint_literals() {
    let output = run_dump(
        &["--ast"],
        "let dec = 1n; let bin = 0b101n; let oct = 0o77n; let hex = 0xFFn;",
    );

    assert!(output.contains("BigInt"), "{output}");
    for raw in ["1n", "0b101n", "0o77n", "0xFFn"] {
        assert!(output.contains(raw), "{output}");
    }
    assert!(!output.contains("Ident(\n            \"n\""), "{output}");
}

#[test]
fn dump_ast_unparse_preserves_bigint_literals() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let dec = 1n; let bin = 0b101n; let oct = 0o77n; let hex = 0xFFn;",
    );

    assert_eq!(
        output,
        "let dec = 1n;\nlet bin = 0b101n;\nlet oct = 0o77n;\nlet hex = 0xFFn;\n"
    );
}

#[test]
fn dump_reports_stable_invalid_bigint_diagnostics() {
    for source in [
        "let value = 1.0n;",
        "let value = 1e2n;",
        "let value = 0b2n;",
        "let value = 01n;",
    ] {
        let stderr = run_dump_error(&["--ast"], source);

        assert!(stderr.contains("[UnsupportedSyntax]"), "{stderr}");
        assert!(stderr.contains("issue-244"), "{stderr}");
        assert!(!stderr.contains("expected Semicolon"), "{stderr}");
        assert!(!stderr.contains("Ident(\"n\")"), "{stderr}");
    }
}

#[test]
fn dump_ast_unparse_erases_typescript_type_annotations() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let value: number;\nfunction add(a: number, b: number): number { return a + b; }\n",
    );

    assert_eq!(
        output,
        "let value = undefined;\nfunction add(a, b) {\n  return (a + b);\n}\n"
    );
}

#[test]
fn dump_ast_unparse_accepts_destructuring_binding_patterns() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let arr = [1, 2];\nlet [a, b] = arr;\nlet { x } = obj;\nfunction f([c], { y }) {}\nlet g = ([d]) => d;\n",
    );

    assert_eq!(
        output,
        "let arr = [1, 2];\nlet [a, b] = arr;\nlet {x} = obj;\nfunction f([c], {y}) {\n}\nlet g = ([d]) => d;\n"
    );
}

#[test]
fn dump_ast_reports_explicit_invalid_destructuring_rest() {
    let stderr = run_dump_error(&["--ast", "--unparse"], "let [...a, b] = arr;");

    assert!(stderr.contains("issue-247"), "{stderr}");
    assert!(
        stderr.contains("rest binding must be the final element"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_covers_destructuring_assignment_patterns() {
    let output = run_dump(
        &["--ast"],
        "({ x, y: target.value = 3, nested: [a, , b], ...rest } = obj); [first, , second = fallback, ...tail] = arr;",
    );

    assert!(output.contains("Assign"), "{output}");
    assert!(
        output.contains("\"{x, y: target.value = 3, nested: [a, , b], ...rest}\""),
        "{output}"
    );
    assert!(
        output.contains("\"[first, , second = fallback, ...tail]\""),
        "{output}"
    );
}

#[test]
fn dump_ast_reports_invalid_destructuring_assignment_rest() {
    let stderr = run_dump_error(&["--ast"], "[...a, b] = arr;");

    assert!(stderr.contains("issue-252"), "{stderr}");
    assert!(
        stderr.contains("rest assignment target must be the final element"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_unparse_erases_typescript_interface_declarations() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "interface Point { x: number; y: number; }\nexport interface NamedPoint extends Point { name: string; }\nlet x: Point = { x: 1, y: 2 };\n",
    );

    assert_eq!(output, "let x = {x: 1, y: 2};\n");
}

#[test]
fn dump_ast_unparse_erases_typescript_type_alias_declarations() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "type Id = number;\ntype Box<T> = { value: T };\nexport type Point<T extends string | number = number> = { x: number; y: number; format: (value: T) => string; };\ntype EndAlias<T extends Missing> = {}\ntype InlineAlias = { value: number }\nlet x: Point = { x: 1, y: 2 };\n",
    );

    assert_eq!(output, "let x = {x: 1, y: 2};\n");
}

#[test]
fn dump_ast_unparse_erases_ambient_declarations() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "declare class Ambient { value: number; }\ndeclare function read(value: Ambient): number;\ndeclare const ambientName: string;\ndeclare enum AmbientEnum { A, B = 2 }\nclass Runtime { declare prop: string; read() { return 1; } }\nlet value: number = 1;\n",
    );

    assert_eq!(
        output,
        "function read() {\n}\nclass Runtime {\n  function read() {\n    return 1;\n  }\n}\nlet value = 1;\n"
    );
}

#[test]
fn dump_ast_reports_ambient_module_as_module_unsupported() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "declare module \"fs\" { export var value: string; }",
    );

    // Ambient module declarations are erased (no runtime impact)
    assert_eq!(output, "");
}

#[test]
fn build_accepts_ambient_namespace_and_module_erasure() {
    build_fixture("basics-types/ambient-namespace-erasure.ts");
}

#[test]
fn dump_ast_unparse_erases_typescript_generics() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "function id<T>(value: T): T { return value; }\nlet result: number = id<number>(3);\n",
    );

    assert_eq!(
        output,
        "function id(value) {\n  return value;\n}\nlet result = id(3);\n"
    );
}

#[test]
fn dump_ast_unparse_erases_typescript_as_assertions() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let value = 3 as number;\nlet nested = ({ x: value } as { x: number });\nlet chained = [value] as number[] as unknown;\n",
    );

    assert_eq!(
        output,
        "let value = 3;\nlet nested = {x: value};\nlet chained = [value];\n"
    );
}

#[test]
fn dump_ast_unparse_erases_typescript_satisfies_expressions() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let value = { x: 3 } satisfies { x: number };\nconsole.log(value.x);\n",
    );

    assert_eq!(output, "let value = {x: 3};\nconsole.log(value.x);\n");
}

#[test]
fn dump_ast_unparse_erases_typescript_satisfies_and_const_assertions() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "let value = ({ x: 3 } satisfies { x: number }) as const;\nlet angle = <const>{ x: value.x + 4 };\nconsole.log(value.x);\nconsole.log(angle.x);\n",
    );

    assert_eq!(
        output,
        "let value = {x: 3};\nlet angle = {x: (value.x + 4)};\nconsole.log(value.x);\nconsole.log(angle.x);\n"
    );
}

#[test]
fn build_accepts_erasable_typescript_type_annotations() {
    build_fixture("basics-types/type-annotation-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_interface_declarations() {
    build_fixture("basics-types/interface-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_type_alias_declarations() {
    build_fixture("basics-types/type-alias-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_ambient_declarations() {
    build_fixture("basics-types/ambient-declaration-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_ambient_erasure_comprehensive() {
    build_fixture("basics-types/ambient-erasure-comprehensive.ts");
}

#[test]
fn build_accepts_erasable_typescript_generics() {
    build_fixture("basics-types/generic-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_as_assertions() {
    build_fixture("basics-types/as-assertion-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_satisfies_expressions() {
    build_fixture("basics-types/satisfies-erasure.ts");
}

#[test]
fn build_accepts_erasable_typescript_satisfies_and_const_assertions() {
    build_fixture("basics-types/satisfies-const-erasure.ts");
}

#[test]
fn dump_lowered_emits_lowered_program() {
    let output = run_dump(&["--lowered"], "let x = 1 + 2;");

    assert!(output.contains("== lowered =="), "{output}");
    assert!(output.contains("LoweredProgram"), "{output}");
    assert!(output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_tir_emits_semantic_hir() {
    let output = run_dump(&["--tir"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== typed-ir =="), "{output}");
    assert!(output.contains("HirProgram"), "{output}");
    assert!(output.contains("JsAdd"), "{output}");
    assert!(output.contains("CallBuiltin"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_tir_unparse_emits_pseudo_source() {
    let output = run_dump(&["--tir", "--unparse"], "let x = 1 + 2; console.log(x);");

    assert_eq!(
        output,
        "let local$0 = JsAdd(1, 2);\nconsole.log(local$0);\n"
    );
}

#[test]
fn dump_optimize_o0_emits_optimized_hir_without_folding() {
    let output = run_dump(&["--optimize", "-O0"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("OptimizedHirProgram"), "{output}");
    assert!(output.contains("level: O0"), "{output}");
    assert!(output.contains("applied_passes: []"), "{output}");
    assert!(output.contains("JsAdd"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_optimize_accepts_all_optimization_levels() {
    for (flag, level) in [("-O0", "O0"), ("-O1", "O1"), ("-O2", "O2"), ("-O3", "O3")] {
        let output = run_dump(&["--optimize", flag], "let x = 1;");
        assert!(output.contains(&format!("level: {level}")), "{output}");
    }
}

#[test]
fn dump_optimize_o2_uses_real_optimizer_passes() {
    let output = run_dump(&["--optimize", "-O2"], "let x = 1 + 2; console.log(x);");

    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("OptimizedHirProgram"), "{output}");
    assert!(output.contains("level: O2"), "{output}");
    assert!(output.contains("LiteralNumericAddFold"), "{output}");
    assert!(!output.contains("JsAdd"), "{output}");
    assert!(!output.contains("LoweredProgram"), "{output}");
    assert!(!output.contains("top_level_statements"), "{output}");
}

#[test]
fn dump_optimize_unparse_emits_optimized_pseudo_source() {
    let output = run_dump(
        &["--optimize", "--unparse", "-O2"],
        "let x = 1 + 2; console.log(x);",
    );

    assert_eq!(output, "let local$0 = 3;\nconsole.log(local$0);\n");
}

#[test]
fn dump_without_phase_emits_available_phases() {
    let output = run_dump(&[], "let x = 1;");

    assert!(output.contains("== tokens =="), "{output}");
    assert!(output.contains("== ast =="), "{output}");
    assert!(output.contains("== resolved =="), "{output}");
    assert!(output.contains("== typed-ir =="), "{output}");
    assert!(output.contains("== optimized-ir =="), "{output}");
    assert!(output.contains("== lowered =="), "{output}");
    assert!(output.contains("== wat =="), "{output}");
}

// ---- Error recovery and source span tests (issue 5045) ----

#[test]
fn dump_ast_reports_unterminated_ts_interface_declaration() {
    let stderr = run_dump_error(&["--ast"], "interface Point { x: number; ");
    assert!(stderr.contains("[UnsupportedTypeScriptSyntax]"), "{stderr}");
    assert!(stderr.contains("unterminated TypeScript"), "{stderr}");
}

#[test]
fn dump_ast_reports_unterminated_ts_type_alias() {
    let output = run_dump(
        &["--ast", "--unparse"],
        "type Id = number;\nlet x: Id = 1;\n",
    );
    assert_eq!(output, "let x = 1;\n");
}

#[test]
fn dump_ast_reports_unterminated_ambient_class_extends() {
    let stderr = run_dump_error(&["--ast"], "declare class C extends ");
    assert!(
        stderr.contains("unterminated ambient class extends"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_reports_unterminated_ambient_function() {
    let stderr = run_dump_error(&["--ast"], "declare function read(");
    assert!(stderr.contains("unterminated ambient function"), "{stderr}");
}

#[test]
fn dump_ast_reports_unterminated_array_binding() {
    let stderr = run_dump_error(&["--ast", "--unparse"], "let [a, = arr;");
    assert!(stderr.contains("issue-247"), "{stderr}");
    assert!(
        stderr.contains("expected binding identifier or pattern"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_reports_unterminated_object_binding() {
    let stderr = run_dump_error(&["--ast", "--unparse"], "let {a, = obj;");
    assert!(stderr.contains("issue-247"), "{stderr}");
    assert!(
        stderr.contains("expected object binding property key"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_reports_destructuring_rest_must_be_final() {
    let stderr = run_dump_error(&["--ast", "--unparse"], "let [...a, b] = arr;");
    assert!(stderr.contains("issue-247"), "{stderr}");
    assert!(
        stderr.contains("rest binding must be the final element"),
        "{stderr}"
    );
}

#[test]
fn dump_ast_reports_optional_chaining_assignment_target() {
    let stderr = run_dump_error(&["--ast"], "obj?.x = 1;");
    assert!(stderr.contains("[UnsupportedSyntax]"), "{stderr}");
    assert!(stderr.contains("issue-246"), "{stderr}");
}

#[test]
fn dump_ast_accepts_regexp_flags() {
    let output = run_dump(&["--ast"], "let r = /hello/g;");
    assert!(output.contains('"'), "{output}");
    assert!(output.contains("/hello/g"), "{output}");
}
