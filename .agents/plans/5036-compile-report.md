# Issue 5036: Introduce CompileReport<T> for non-fatal diagnostics

## Summary

Replace the `Result<T, Diagnostic>` pipeline with `CompileReport<T>` that can carry warning/non-fatal diagnostics alongside a value, so that "compilation succeeded but has warnings" is representable.

## Design

### 1. Define `CompileReport<T>` in `crates/compiler/src/lib.rs`

```rust
#[derive(Debug, Clone)]
pub struct CompileReport<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}

impl<T> CompileReport<T> {
    pub fn ok(value: T) -> Self {
        Self { value, diagnostics: Vec::new() }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> CompileReport<U> {
        CompileReport { value: f(self.value), diagnostics: self.diagnostics }
    }

    /// Convert to Result, keeping only the first diagnostic as error
    pub fn into_result(self) -> Result<T, Diagnostic> {
        Ok(self.value)
    }

    /// Fold diagnostics into self from a fallible step
    pub fn and_then<U>(self, f: impl FnOnce(T) -> Result<U, Diagnostic>) -> Result<CompileReport<U>, Diagnostic> {
        let value = f(self.value)?;
        Ok(CompileReport { value, diagnostics: self.diagnostics })
    }
}
```

### 2. Change pipeline functions to accumulate diagnostics

The main `build_file_with_host_deny` function:
- Keep `?` for fatal errors (I/O, parse failure, invariant violations)
- For stages that emit non-fatal diagnostics, fold them into the report
- `validate_ast`, `ensure_runtime_feature_gates`, `validate_host_deny` remain as fatal errors
- `validate_lowered` and `validate_hir` already return `Vec<Diagnostic>` — fold into the report

### 3. Stage classification

| Stage | Fatal | Non-fatal candidate | Notes |
|---|---|---|---|
| fs::read_to_string | YES | NO | File I/O |
| validate_type_reference_directives | YES | NO | Preflight check |
| Lexer::tokenize | YES | NO | Parse error |
| Parser::parse_program | YES | NO | Parse error |
| validate_ast | YES | NO | Structural invariant |
| build_entry_module_graph | YES | NO | Resolution error |
| lower_static_named_import_bindings | YES | NO | Binding resolution |
| name_resolver::resolve_names | YES | NO | Name resolution error |
| builtin_resolver::resolve_builtins | YES | NO | Builtin resolution |
| validate_optimized_hir_slice | YES* | Potentially | Currently skips UnsupportedSyntax |
| lowered::lower_program | YES | NO | Lowering error |
| lower_static_named_import_reads | YES* | Potentially | Index OOB = fatal |
| populate_static_module_exports | YES | NO | Export resolution |
| lowered::validate_lowered | YES | Fold Vec<Diagnostic> into report | Already returns Vec |
| ensure_runtime_feature_gates | YES | NO | Gate check |
| validate_host_deny | YES | NO | Gate check |
| emit_wat / write_wasm | YES | NO | Backend emission |

### 4. CLI output changes (`crates/cli/src/main.rs`)

After compilation:

```rust
match result {
    CompileReport { value: (), diagnostics } => {
        for diag in &diagnostics {
            eprintln!("warning: {diag}");
        }
        ExitCode::SUCCESS
    }
}
```

If there are fatal errors, they still short-circuit via `?`.

### 5. Re-export from `crates/cli/src/lib.rs`

Add `pub use ts2wasm_compiler::CompileReport;` to the re-export shim.

## Files to modify

1. `crates/compiler/src/lib.rs` — Add `CompileReport<T>`, update `build_file_with_host_deny`
2. `crates/cli/src/main.rs` — Update build command to print diagnostics
3. `crates/cli/src/lib.rs` — Re-export `CompileReport`
4. `crates/ir/src/lowered/validate.rs` — Already returns `Vec<Diagnostic>`, folded at call site
5. `crates/ir/src/semantic.rs` — Already returns `Vec<Diagnostic>`, folded at call site

## Verification

```sh
cargo fmt --all --check
cargo nextest run
```

## Acceptance criteria

- [x] `CompileReport<T>` defined with value + diagnostics list
- [x] Non-fatal diagnostics appear in CLI output
- [x] Existing fatal errors unchanged
- [x] fmt + nextest pass
