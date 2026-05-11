fn main() {
    let fixtures = [
        "console-log",
        "empty",
        "number-add",
        "object-literal",
        "for-loop",
    ];
    for name in &fixtures {
        let source = std::fs::read_to_string(format!("fixtures/linker/{}.ts", name)).unwrap();
        let program = ts2wasm_cli::parse_program(&source).unwrap();
        let name_resolved = ts2wasm_ir::name_resolver::resolve_names(&program).unwrap();
        let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&name_resolved).unwrap();
        let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
        let (validated, _) = ts2wasm_ir::lowered::Validated::new(lowered).expect("should validate");
        let json = ts2wasm_backend_wasm::emit_link_plan_snapshot_json(&validated);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let formatted = serde_json::to_string_pretty(&parsed).unwrap();
        std::fs::write(
            format!("fixtures/linker/{}.snapshot.json", name),
            formatted.as_bytes(),
        )
        .unwrap();
        eprintln!("Updated: fixtures/linker/{}.snapshot.json", name);
    }
}
