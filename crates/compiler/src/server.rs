use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use ts2wasm_frontend::{Diagnostic, Lexer, Parser};
use ts2wasm_ir::OptimizationLevel;
use ts2wasm_ir::builtin_resolver;
use ts2wasm_ir::lowered;
use ts2wasm_ir::name_resolver;

use crate::module_graph;
use crate::{
    ensure_runtime_feature_gates, lower_static_named_import_bindings_for_build,
    lower_static_named_import_reads_for_build, populate_static_module_exports_for_build,
    validate_ast, validate_optimized_hir_slice,
};

/// A single build request from the client (one JSON line on stdin).
#[derive(Debug, Deserialize)]
struct ServerRequest {
    /// Opaque id echoed back in the response. Use -1 to signal shutdown.
    id: i64,
    /// The full TypeScript/JavaScript source to compile (already preprocessed).
    source: String,
}

/// The response sent back as one JSON line on stdout.
#[derive(Debug, Serialize)]
struct ServerResponse {
    id: i64,
    status: String, // "ok" or "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

/// Run the ts2wasm build server.
///
/// Reads newline-delimited JSON from stdin and writes newline-delimited JSON
/// to stdout. Each input line is a `ServerRequest`; each output line is a
/// `ServerResponse`.
///
/// Send `{"id": -1, "source": ""}` (or EOF) to shut down cleanly.
pub fn run_server() -> Result<(), String> {
    // One shared temp directory for the lifetime of this server.
    let tmpdir = std::env::temp_dir().join(format!("ts2wasm_srv_{}", std::process::id()));
    fs::create_dir_all(&tmpdir).map_err(|e| format!("failed to create temp dir: {e}"))?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut line_buf = String::new();

    loop {
        line_buf.clear();
        let n = stdin
            .lock()
            .read_line(&mut line_buf)
            .map_err(|e| format!("stdin read error: {e}"))?;
        if n == 0 {
            break; // EOF
        }

        let line = line_buf.trim();
        if line.is_empty() {
            continue;
        }

        let req: ServerRequest =
            serde_json::from_str(line).map_err(|e| format!("bad request JSON: {e}"))?;

        if req.id == -1 {
            break; // shutdown signal
        }

        // The server writes the source to a temp file so the module-graph
        // builder (which requires on-disk canonicalization) can find it.
        let tmpfile = tmpdir.join(format!("{}.js", req.id));
        if let Err(e) = fs::write(&tmpfile, &req.source) {
            let resp = ServerResponse {
                id: req.id,
                status: "error".into(),
                code: Some("BackendIo".into()),
                message: Some(format!("failed to write temp file: {e}")),
            };
            emit_response(&stdout, &resp)?;
            continue;
        }

        let result = compile_source(&tmpfile);
        let _ = fs::remove_file(&tmpfile);

        let resp = match result {
            Ok(()) => ServerResponse {
                id: req.id,
                status: "ok".into(),
                code: None,
                message: None,
            },
            Err(diag) => ServerResponse {
                id: req.id,
                status: "error".into(),
                code: Some(format!("{:?}", diag.code)),
                message: Some(diag.message),
            },
        };

        emit_response(&stdout, &resp)?;
    }

    // Clean up the per-process temp directory.
    let _ = fs::remove_dir_all(&tmpdir);
    Ok(())
}

fn emit_response(stdout: &io::Stdout, resp: &ServerResponse) -> Result<(), String> {
    let json = serde_json::to_string(resp).map_err(|e| format!("serialization error: {e}"))?;
    let mut out = stdout.lock();
    writeln!(out, "{json}").map_err(|e| format!("stdout write error: {e}"))?;
    out.flush()
        .map_err(|e| format!("stdout flush error: {e}"))?;
    Ok(())
}

/// Run the full compiler pipeline for a single file, but stop before emitting
/// WAT/WASM. Returns `Ok(())` if compilation succeeds, or the first
/// `Diagnostic` if it fails.
///
/// The file at `path` must already have been preprocessed by the client (i.e.
/// test262 `// includes:` directives already resolved). We skip
/// `test262_preprocessor::process_test262_includes` and
/// `validate_type_reference_directives`.
fn compile_source(path: &Path) -> Result<(), Diagnostic> {
    use ts2wasm_frontend::DiagCode;

    let source = fs::read_to_string(path).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", path.display()),
        span: None,
    })?;
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    validate_ast(&program)?;
    let module_graph = module_graph::build_entry_module_graph(path, &program)?;
    let static_module_binding =
        lower_static_named_import_bindings_for_build(&program, &module_graph)?;
    let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)?;
    let lowered = lowered::lower_program(&resolved)?;
    let lowered =
        lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)?;
    let lowered = populate_static_module_exports_for_build(lowered, &module_graph)?;

    lowered::validate_lowered(&lowered).map_err(|errs| {
        errs.into_iter().next().unwrap_or(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: "validate_lowered failed with empty diagnostic list".to_owned(),
            span: None,
        })
    })?;

    ensure_runtime_feature_gates(&lowered)?;
    Ok(())
}
