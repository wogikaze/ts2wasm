use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

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

/// Default timeout for batch compilation (in seconds). After this, remaining
/// items are reported as timed out rather than processed.
const BATCH_TIMEOUT_SECS: u64 = 30;

/// Maximum number of worker threads for batch processing. Capped at CPU count.
const MAX_WORKERS: usize = 8;

/// A single build request from the client (one JSON line on stdin).
#[derive(Debug, Deserialize)]
struct ServerRequest {
    /// Opaque id echoed back in the response. Use -1 to signal shutdown.
    id: i64,
    /// Source for single-file request (empty for batch).
    #[serde(default)]
    source: String,
    /// Batch of files for parallel processing (empty for single-file).
    #[serde(default)]
    items: Vec<BatchItem>,
}

/// An item in a batch request.
#[derive(Debug, Deserialize)]
struct BatchItem {
    id: i64,
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
/// to stdout. Supports two request formats:
///   - Single: `{"id": N, "source": "..."}`
///   - Batch:  `{"id": -1, "items": [{"id": 1, "source": "..."}, ...]}`
///
/// Batch requests process all files in parallel using a thread pool with
/// a configurable timeout and max concurrency. Panics are caught and
/// reported per-item rather than crashing the server.
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

        if req.id == -1 && req.items.is_empty() {
            break; // shutdown signal (single-file with id=-1, not a batch)
        }

        if !req.items.is_empty() {
            // Batch mode: process all items in parallel using a thread pool
            let results = process_batch(&tmpdir, &req.items);
            let json =
                serde_json::to_string(&results).map_err(|e| format!("serialization error: {e}"))?;
            let mut out = stdout.lock();
            writeln!(out, "{json}").map_err(|e| format!("stdout write error: {e}"))?;
            out.flush()
                .map_err(|e| format!("stdout flush error: {e}"))?;
        } else {
            // Single-file mode
            let tmpfile = tmpdir.join(format!("{}.js", req.id));
            if let Err(e) = fs::write(&tmpfile, &req.source) {
                let resp = make_response(
                    req.id,
                    Err(Diagnostic {
                        code: ts2wasm_frontend::DiagCode::BackendIo,
                        message: format!("failed to write temp file: {e}"),
                        span: None,
                    }),
                );
                emit_response(&stdout, &resp)?;
                continue;
            }

            let result = compile_source(&tmpfile);
            let _ = fs::remove_file(&tmpfile);
            emit_response(&stdout, &make_response(req.id, result))?;
        }
    }

    // Clean up the per-process temp directory.
    let _ = fs::remove_dir_all(&tmpdir);
    Ok(())
}

/// Process a batch of files in parallel using `std::thread::scope`.
///
/// Features:
/// - **Timeout**: if `BATCH_TIMEOUT_SECS` elapses, remaining items get a
///   `timed out` error response and workers stop.
/// - **Max concurrency**: worker count is capped at `MAX_WORKERS` and CPU
///   count, whichever is lower.
/// - **Panic aggregation**: per-worker panics are caught via
///   `std::panic::catch_unwind` and reported as internal error responses.
///
/// Returns one `ServerResponse` per item, in the same order as `items`.
fn process_batch(tmpdir: &Path, items: &[BatchItem]) -> Vec<ServerResponse> {
    let n = items.len();
    if n == 0 {
        return vec![];
    }

    // Create a slot for each result, pre-filled with a placeholder.
    let results: Mutex<Vec<Option<ServerResponse>>> = Mutex::new((0..n).map(|_| None).collect());

    // Atomic counter for work stealing across threads.
    let next_idx = AtomicUsize::new(0);

    // Timeout tracking: once set, workers stop claiming new items.
    let timed_out = AtomicBool::new(false);
    let deadline = Instant::now();

    // Number of worker threads: up to CPU count and MAX_WORKERS, capped at batch size.
    let n_workers = std::thread::available_parallelism()
        .map(|c| c.get())
        .unwrap_or(4)
        .min(MAX_WORKERS)
        .min(n)
        .max(1);

    std::thread::scope(|s| {
        for _ in 0..n_workers {
            s.spawn(|| {
                loop {
                    // Check timeout before claiming next item.
                    if timed_out.load(Ordering::Relaxed) {
                        break;
                    }
                    if deadline.elapsed().as_secs() >= BATCH_TIMEOUT_SECS {
                        timed_out.store(true, Ordering::Relaxed);
                        break;
                    }

                    let idx = next_idx.fetch_add(1, Ordering::Relaxed);
                    if idx >= n {
                        break;
                    }

                    let item = &items[idx];
                    let tmpfile = tmpdir.join(format!("b_{}_{}.js", idx, item.id));

                    // Catch panics per-item so one bad compilation can't crash
                    // the entire batch.
                    let compile_result = std::panic::catch_unwind(|| {
                        // Write source, compile, clean up.
                        let write_result = fs::write(&tmpfile, &item.source);
                        match write_result {
                            Ok(()) => {
                                let r = compile_source(&tmpfile);
                                let _ = fs::remove_file(&tmpfile);
                                r
                            }
                            Err(err) => Err(Diagnostic {
                                code: ts2wasm_frontend::DiagCode::BackendIo,
                                message: format!("failed to write temp file: {err}"),
                                span: None,
                            }),
                        }
                    });

                    let resp = match compile_result {
                        Ok(Ok(())) => make_response(item.id, Ok(())),
                        Ok(Err(diag)) => make_response(item.id, Err(diag)),
                        Err(panic_payload) => {
                            let msg = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                                format!("internal error: {s}")
                            } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                                format!("internal error: {s}")
                            } else {
                                "internal error: unknown panic".to_owned()
                            };
                            make_response(
                                item.id,
                                Err(Diagnostic {
                                    code: ts2wasm_frontend::DiagCode::InvariantViolation,
                                    message: msg,
                                    span: None,
                                }),
                            )
                        }
                    };
                    let mut guard = results.lock().unwrap();
                    guard[idx] = Some(resp);
                }
            });
        }
    });

    // Fill any remaining unprocessed slots (timeout or skipped) with error
    // responses so the caller always gets exactly n results.
    let mut final_results = results.into_inner().unwrap();
    for (idx, slot) in final_results.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(make_response(
                items[idx].id,
                Err(Diagnostic {
                    code: ts2wasm_frontend::DiagCode::InvariantViolation,
                    message: if timed_out.load(Ordering::Relaxed) {
                        format!(
                            "batch timed out after {}s (item {})",
                            BATCH_TIMEOUT_SECS, idx
                        )
                    } else {
                        format!("internal error: item {idx} was skipped")
                    },
                    span: None,
                }),
            ));
        }
    }

    final_results
        .into_iter()
        .map(|r| r.expect("all batch slots filled after timeout fill"))
        .collect()
}

fn make_response(id: i64, result: Result<(), Diagnostic>) -> ServerResponse {
    match result {
        Ok(()) => ServerResponse {
            id,
            status: "ok".into(),
            code: None,
            message: None,
        },
        Err(diag) => ServerResponse {
            id,
            status: "error".into(),
            code: Some(format!("{:?}", diag.code)),
            message: Some(diag.message),
        },
    }
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
    let lowered = populate_static_module_exports_for_build(
        lowered,
        &module_graph,
        &static_module_binding.module_exports,
    )?;

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
