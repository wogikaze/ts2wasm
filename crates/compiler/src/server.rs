use std::fs;
use std::io::{self, BufRead, Write};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use ts2wasm_frontend::{Diagnostic, Lexer, Parser};
use ts2wasm_ir::OptimizationLevel;
use ts2wasm_ir::builtin_resolver;
use ts2wasm_ir::lowered;
use ts2wasm_ir::lowered::LoweredProgram;
use ts2wasm_ir::name_resolver;

use crate::module_graph;
use crate::write_wasm_from_wat;
use crate::{
    ensure_runtime_feature_gates, lower_static_named_import_bindings_for_build,
    lower_static_named_import_reads_for_build, populate_static_module_exports_for_build,
    validate_ast, validate_optimized_hir_slice,
};

/// Default timeout for batch compilation (in seconds). After this, remaining
/// items are reported as timed out rather than processed.
const BATCH_TIMEOUT_SECS: u64 = 30;

/// Default upper bound for batch workers when no environment override is set.
///
/// The Python coverage runner passes its `--jobs` value to the server.  The old
/// fixed cap of 8 made large reference batches artificially slow on 16+ core
/// machines, so the cap is now configurable while still bounded by CPU count.
const DEFAULT_MAX_WORKERS_CAP: usize = 32;
const DEFAULT_SERVER_WORKER_STACK_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone, Copy)]
struct BatchLimits {
    timeout_secs: u64,
    max_workers_cap: usize,
    worker_stack_bytes: usize,
}

impl BatchLimits {
    fn from_env(available_workers: usize) -> Self {
        let max_workers_cap = std::env::var("TS2WASM_SERVER_MAX_WORKERS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_MAX_WORKERS_CAP)
            .min(available_workers)
            .max(1);
        let worker_stack_bytes = std::env::var("TS2WASM_SERVER_WORKER_STACK_BYTES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_SERVER_WORKER_STACK_BYTES);
        Self {
            timeout_secs: BATCH_TIMEOUT_SECS,
            max_workers_cap,
            worker_stack_bytes,
        }
    }
}

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
    /// Optional batch/single output mode. "wasm" writes a wasm file and
    /// includes its path in the response; any other value is compile-only.
    #[serde(default)]
    emit: Option<String>,
    /// Optional worker count hint for batch mode.
    #[serde(default)]
    jobs: Option<usize>,
}

/// An item in a batch request.
#[derive(Debug, Deserialize, Clone)]
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
    phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wasm_path: Option<String>,
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
            let results = process_batch(&tmpdir, &req.items, request_emit_mode(&req), req.jobs);
            let json =
                serde_json::to_string(&results).map_err(|e| format!("serialization error: {e}"))?;
            let mut out = stdout.lock();
            writeln!(out, "{json}").map_err(|e| format!("stdout write error: {e}"))?;
            out.flush()
                .map_err(|e| format!("stdout flush error: {e}"))?;
        } else {
            // Single-file mode.  Keep a stable virtual path for diagnostics and
            // relative module graph bookkeeping, but avoid writing then reading
            // the source back from disk for every server request.
            let virtual_path = tmpdir.join(format!("{}.js", req.id));
            let result = compile_source_text_with_emit(
                &req.source,
                &virtual_path,
                &tmpdir,
                req.id,
                request_emit_mode(&req),
            );
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmitMode {
    Check,
    Wasm,
}

fn request_emit_mode(req: &ServerRequest) -> EmitMode {
    match req.emit.as_deref() {
        Some("wasm") => EmitMode::Wasm,
        _ => EmitMode::Check,
    }
}

fn process_batch(
    tmpdir: &Path,
    items: &[BatchItem],
    emit_mode: EmitMode,
    requested_workers: Option<usize>,
) -> Vec<ServerResponse> {
    let available_workers = std::thread::available_parallelism()
        .map(|c| c.get())
        .unwrap_or(4)
        .max(1);
    let limits = BatchLimits::from_env(available_workers);
    process_batch_with_compiler(tmpdir, items, emit_mode, requested_workers, limits)
}

fn process_batch_with_compiler(
    tmpdir: &Path,
    items: &[BatchItem],
    emit_mode: EmitMode,
    requested_workers: Option<usize>,
    limits: BatchLimits,
) -> Vec<ServerResponse> {
    process_batch_with_compile(
        tmpdir,
        items,
        requested_workers,
        limits,
        move |item, virtual_path, tmpdir| {
            compile_source_text_with_emit(&item.source, virtual_path, tmpdir, item.id, emit_mode)
        },
    )
}

fn process_batch_with_compile<F>(
    tmpdir: &Path,
    items: &[BatchItem],
    requested_workers: Option<usize>,
    limits: BatchLimits,
    compile_item: F,
) -> Vec<ServerResponse>
where
    F: Fn(&BatchItem, &Path, &Path) -> Result<Option<PathBuf>, Diagnostic>
        + Send
        + Sync
        + UnwindSafe
        + RefUnwindSafe
        + 'static,
{
    let n = items.len();
    if n == 0 {
        return vec![];
    }

    // Create a slot for each result, pre-filled with a placeholder.
    let results: Arc<Mutex<Vec<Option<ServerResponse>>>> =
        Arc::new(Mutex::new((0..n).map(|_| None).collect()));

    // Atomic counter for work stealing across threads.
    let next_idx = Arc::new(AtomicUsize::new(0));

    // Timeout tracking: once set, workers stop claiming new items.
    let timed_out = AtomicBool::new(false);
    let timed_out = Arc::new(timed_out);
    let deadline = Instant::now();
    let items: Arc<Vec<BatchItem>> = Arc::new(items.to_vec());
    let shared_tmpdir = Arc::new(tmpdir.to_path_buf());

    let worker_limit = requested_workers
        .unwrap_or(limits.max_workers_cap)
        .clamp(1, limits.max_workers_cap);
    let n_workers = worker_limit.min(n).max(1);
    let compile_item = Arc::new(compile_item);

    let mut handles = Vec::with_capacity(n_workers);
    for worker_id in 0..n_workers {
        let items = Arc::clone(&items);
        let results = Arc::clone(&results);
        let next_idx = Arc::clone(&next_idx);
        let timed_out = Arc::clone(&timed_out);
        let tmpdir = Arc::clone(&shared_tmpdir);
        let compile_item = Arc::clone(&compile_item);

        let handle = std::thread::Builder::new()
            .name(format!("ts2wasm-server-worker-{worker_id}"))
            .stack_size(limits.worker_stack_bytes)
            .spawn(move || {
                loop {
                    // Check timeout before claiming next item.
                    if timed_out.load(Ordering::Relaxed) {
                        break;
                    }
                    if deadline.elapsed().as_secs() >= limits.timeout_secs {
                        timed_out.store(true, Ordering::Relaxed);
                        break;
                    }

                    let idx = next_idx.fetch_add(1, Ordering::Relaxed);
                    if idx >= n {
                        break;
                    }

                    let item = &items[idx];
                    let virtual_path = tmpdir.join(format!("b_{}_{}.js", idx, item.id));

                    // Catch panics per-item so one bad compilation can't crash
                    // the entire batch.  Compile directly from the JSON payload;
                    // the previous implementation wrote every source to a temp
                    // file and then read it back in `lower_source`, which adds a
                    // large amount of filesystem churn for test262 batches.
                    let compile_result =
                        std::panic::catch_unwind(|| compile_item(item, &virtual_path, &tmpdir));

                    let resp = match compile_result {
                        Ok(Ok(wasm_path)) => make_response(item.id, Ok(wasm_path)),
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

                                    phase: None,
                                }),
                            )
                        }
                    };
                    let mut guard = results.lock().unwrap();
                    guard[idx] = Some(resp);
                }
            })
            .expect("failed to spawn server worker thread");

        handles.push(handle);
    }

    for handle in handles {
        if handle.join().is_err() {
            timed_out.store(true, Ordering::Relaxed);
        }
    }

    // Fill any remaining unprocessed slots (timeout or skipped) with error
    // responses so the caller always gets exactly n results.
    let mut final_results = Arc::try_unwrap(results)
        .expect("worker threads should be joined and dropped")
        .into_inner()
        .unwrap();
    for (idx, slot) in final_results.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(make_response(
                items[idx].id,
                Err(Diagnostic {
                    code: ts2wasm_frontend::DiagCode::InvariantViolation,
                    message: if timed_out.load(Ordering::Relaxed) {
                        format!(
                            "batch timed out after {}s (item {})",
                            limits.timeout_secs, idx
                        )
                    } else {
                        format!("internal error: item {idx} was skipped")
                    },
                    span: None,

                    phase: None,
                }),
            ));
        }
    }

    final_results
        .into_iter()
        .map(|r| r.expect("all batch slots filled after timeout fill"))
        .collect()
}

fn make_response(id: i64, result: Result<Option<PathBuf>, Diagnostic>) -> ServerResponse {
    match result {
        Ok(wasm_path) => ServerResponse {
            id,
            status: "ok".into(),
            code: None,
            phase: None,
            message: None,
            wasm_path: wasm_path.map(|path| path.display().to_string()),
        },
        Err(diag) => ServerResponse {
            id,
            status: "error".into(),
            code: Some(format!("{:?}", diag.code)),
            phase: diag.phase.map(|p| p.to_string()),
            message: Some(diag.message),
            wasm_path: None,
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
#[allow(dead_code)]
fn compile_source_with_emit(
    path: &Path,
    tmpdir: &Path,
    id: i64,
    emit_mode: EmitMode,
) -> Result<Option<PathBuf>, Diagnostic> {
    use ts2wasm_frontend::DiagCode;

    let source = fs::read_to_string(path).map_err(|error| Diagnostic {
        code: DiagCode::BackendIo,
        message: format!("failed to read {}: {error}", path.display()),
        span: None,

        phase: None,
    })?;
    compile_source_text_with_emit(&source, path, tmpdir, id, emit_mode)
}

fn compile_source_text_with_emit(
    source: &str,
    path: &Path,
    tmpdir: &Path,
    id: i64,
    emit_mode: EmitMode,
) -> Result<Option<PathBuf>, Diagnostic> {
    // `module_graph::build_entry_module_graph` canonicalizes the entry path.
    // The server compiles from the in-memory JSON payload, but the virtual
    // entry still has to exist on disk for canonicalization and relative
    // import bookkeeping.  Create an empty placeholder instead of writing the
    // full source and reading it back.
    fs::File::create(path).map_err(|error| Diagnostic {
        code: ts2wasm_frontend::DiagCode::BackendIo,
        message: format!("failed to create virtual entry {}: {error}", path.display()),
        span: None,
        phase: None,
    })?;

    let lowered = lower_source_text(path, source)?;
    ensure_runtime_feature_gates(&lowered).map_err(|d| d.with_phase("runtime-gate"))?;

    match emit_mode {
        EmitMode::Check => Ok(None),
        EmitMode::Wasm => {
            let output = tmpdir.join(format!("{}.wasm", id));
            ts2wasm_backend_wasm::emit_wat(&lowered)
                .map_err(|d| d.with_phase("backend"))
                .and_then(|wat| write_wasm_from_wat(&wat, &output))
                .map_err(|d| d.with_phase("backend"))?;
            Ok(Some(output))
        }
    }
}

fn lower_source_text(path: &Path, source: &str) -> Result<LoweredProgram, Diagnostic> {
    use ts2wasm_frontend::DiagCode;

    let tokens = Lexer::new(source)
        .tokenize()
        .map_err(|d| d.with_phase("lexer"))?;
    let program = Parser::new(tokens, source)
        .parse_program()
        .map_err(|d| d.with_phase("parser"))?;
    validate_ast(&program).map_err(|d| d.with_phase("ast-validator"))?;
    let module_graph = module_graph::build_entry_module_graph(path, &program)
        .map_err(|d| d.with_phase("module-resolver"))?;
    // Surface cycle diagnostics: report first cycle diagnostic as error.
    if let Some(cycle_diag) = module_graph.cycle_diagnostics().first() {
        return Err(cycle_diag.clone().with_phase("module-resolver"));
    }
    // Validate dependency-first initialization order.
    module_graph::validate_init_order(&module_graph)
        .map_err(|d| d.with_phase("module-resolver"))?;
    let static_module_binding =
        lower_static_named_import_bindings_for_build(&program, &module_graph)
            .map_err(|d| d.with_phase("module-resolver"))?;
    let name_resolved = name_resolver::resolve_names(&static_module_binding.rewritten_program)
        .map_err(|d| d.with_phase("name-resolver"))?;
    let resolved = builtin_resolver::resolve_builtins(&name_resolved)
        .map_err(|d| d.with_phase("builtin-resolver"))?;
    super::validate_typescript_semantics_for_path(path, &resolved)
        .map_err(|d| d.with_phase("semantic-validator"))?;
    validate_optimized_hir_slice(&resolved, OptimizationLevel::O0)
        .map_err(|d| d.with_phase("hir-validator"))?;
    let lowered = lowered::lower_program(&resolved).map_err(|d| d.with_phase("lowering"))?;
    let lowered =
        lower_static_named_import_reads_for_build(lowered, &static_module_binding.named_imports)
            .map_err(|d| d.with_phase("module-resolver"))?;
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
            phase: None,
        })
    })?;

    Ok(lowered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    fn test_tmpdir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "ts2wasm_server_test_{}_{}",
            std::process::id(),
            name
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test temp dir should be created");
        path
    }

    fn test_limits(timeout_secs: u64, max_workers_cap: usize) -> BatchLimits {
        BatchLimits {
            timeout_secs,
            max_workers_cap,
            worker_stack_bytes: 2 * 1024 * 1024,
        }
    }

    fn batch_items(count: usize) -> Vec<BatchItem> {
        (0..count)
            .map(|idx| BatchItem {
                id: idx as i64,
                source: format!("item-{idx}"),
            })
            .collect()
    }

    #[test]
    fn batch_timeout_reports_unprocessed_items() {
        let tmpdir = test_tmpdir("timeout");
        let items = batch_items(3);

        let results = process_batch_with_compile(
            &tmpdir,
            &items,
            Some(2),
            test_limits(0, 2),
            |_item, _path, _tmpdir| panic!("timeout should stop before compiling"),
        );

        assert_eq!(results.len(), items.len());
        for (idx, response) in results.iter().enumerate() {
            assert_eq!(response.id, idx as i64);
            assert_eq!(response.status, "error");
            assert_eq!(response.code.as_deref(), Some("InvariantViolation"));
            assert!(
                response
                    .message
                    .as_deref()
                    .unwrap_or_default()
                    .contains("batch timed out after 0s"),
                "unexpected timeout response: {:?}",
                response.message
            );
        }

        let _ = fs::remove_dir_all(tmpdir);
    }

    #[test]
    fn batch_worker_count_is_capped() {
        let tmpdir = test_tmpdir("worker-cap");
        let items = batch_items(6);
        let active = Arc::new(AtomicUsize::new(0));
        let max_active = Arc::new(AtomicUsize::new(0));
        let active_for_compile = Arc::clone(&active);
        let max_for_compile = Arc::clone(&max_active);

        let results = process_batch_with_compile(
            &tmpdir,
            &items,
            Some(99),
            test_limits(30, 2),
            move |_item, _path, _tmpdir| {
                let current = active_for_compile.fetch_add(1, Ordering::SeqCst) + 1;
                max_for_compile.fetch_max(current, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(20));
                active_for_compile.fetch_sub(1, Ordering::SeqCst);
                Ok(None)
            },
        );

        assert_eq!(results.len(), items.len());
        assert!(results.iter().all(|response| response.status == "ok"));
        assert!(
            max_active.load(Ordering::SeqCst) <= 2,
            "batch exceeded worker cap: max_active={}",
            max_active.load(Ordering::SeqCst)
        );

        let _ = fs::remove_dir_all(tmpdir);
    }

    #[test]
    fn batch_panic_is_reported_per_item() {
        let tmpdir = test_tmpdir("panic");
        let items = vec![
            BatchItem {
                id: 1,
                source: "ok".to_owned(),
            },
            BatchItem {
                id: 2,
                source: "panic".to_owned(),
            },
            BatchItem {
                id: 3,
                source: "ok".to_owned(),
            },
        ];

        let results = process_batch_with_compile(
            &tmpdir,
            &items,
            Some(1),
            test_limits(30, 1),
            |item, _path, _tmpdir| {
                if item.source == "panic" {
                    panic!("synthetic batch panic");
                }
                Ok(None)
            },
        );

        assert_eq!(results.len(), items.len());
        assert_eq!(results[0].status, "ok");
        assert_eq!(results[1].id, 2);
        assert_eq!(results[1].status, "error");
        assert_eq!(results[1].code.as_deref(), Some("InvariantViolation"));
        assert!(
            results[1]
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("synthetic batch panic"),
            "unexpected panic response: {:?}",
            results[1].message
        );
        assert_eq!(results[2].status, "ok");

        let _ = fs::remove_dir_all(tmpdir);
    }
}
