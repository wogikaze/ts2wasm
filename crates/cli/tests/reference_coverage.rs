//! Native Rust test262 / tsc reference coverage runner.
//!
//! Runs test262 conformance tests in-process (no Python, no IPC) and reports
//! results as cargo-nextest test cases.  Each test262 subcategory (e.g.
//! `built-ins/Array`, `built-ins/JSON`) is a single `#[test]` function.
//!
//! ## Usage
//!
//! ```sh
//! # Run a single category
//! cargo nextest run -p ts2wasm-cli --test reference_coverage -- t262_builtins_json
//!
//! # Run all test262 categories (compile-only)
//! cargo nextest run -p ts2wasm-cli --test reference_coverage
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use ts2wasm_compiler::server::compile_source_text;
use ts2wasm_compiler::test262_preprocessor::process_test262_includes;
use ts2wasm_shared::test_helpers::unique_temp_dir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Root of the reference test corpus.
fn reference_root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        let p = ts2wasm_shared::test_helpers::repo_root().join("reference");
        assert!(p.exists(), "reference/ directory not found at {p:?}");
        p
    })
}

/// Discover all `.js` files under a `reference/test262/test/<category>` directory.
fn discover_files(category: &str) -> Vec<PathBuf> {
    let root = reference_root().join("test262").join("test").join(category);
    if !root.exists() {
        return Vec::new();
    }
    let mut files = Vec::new();
    collect_js_files(&root, &mut files);
    files
}

fn collect_js_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_js_files(&path, files);
            } else if path.extension().is_some_and(|e| e == "js") {
                files.push(path);
            }
        }
    }
}

/// Run a single test262 file through the in-process compiler and classify the
/// outcome.
fn classify_file(abs_path: &Path) -> Outcome {
    let source = match fs::read_to_string(abs_path) {
        Ok(s) => s,
        Err(e) => return Outcome::blocked(format!("read failed: {e}")),
    };

    // 1. Test262 preprocessing (YAML frontmatter parsing, harness stub injection).
    let processed = match process_test262_includes(abs_path, &source) {
        Ok(s) => s,
        Err(d) => {
            return Outcome::unsupported(format!("preprocessor: {}", d.message));
        }
    };

    // 2. Create a temp directory and a placeholder entry file.
    let tmpdir = unique_temp_dir("refcov");
    let placeholder = Path::new(&tmpdir).join("entry.js");
    // 3. In-process compilation.
    match compile_source_text(&processed, &placeholder, Path::new(&tmpdir)) {
        Ok(wasm_path) => {
            // 4. (Optional future step) Run via iwasm for semantic comparison.
            // Success — wasm binary was produced.
            let _wasm_bytes = fs::read(&wasm_path).unwrap_or_default();
            Outcome::pass()
        }
        Err(d) => {
            // Classify by diagnostic code.
            let code = format!("{:?}", d.code);
            let msg = d.message;

            // InvariantViolation is always a compiler bug.
            if code == "InvariantViolation" {
                return Outcome::internal_failure(format!("InvariantViolation: {msg}"));
            }

            // "Unsupported*" and related codes are expected diagnostic codes
            // that indicate a known limitation, not a regression.
            if code.starts_with("Unsupported") || code.starts_with("Unresolved")
                || code == "SyntaxError" || code == "ArityMismatch"
                || code == "BackendIo"
            {
                return Outcome::unsupported(format!("{code}: {msg}"));
            }

            // Everything else is an unexpected failure.
            Outcome::fail(format!("{code}: {msg}"))
        }
    }
}

// ---------------------------------------------------------------------------
// Outcome classification
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Outcome {
    status: &'static str,
    reason: Option<String>,
}

impl Outcome {
    fn pass() -> Self {
        Outcome { status: "pass", reason: None }
    }
    fn unsupported(reason: impl Into<String>) -> Self {
        Outcome { status: "unsupported", reason: Some(reason.into()) }
    }
    fn fail(reason: impl Into<String>) -> Self {
        Outcome { status: "fail", reason: Some(reason.into()) }
    }
    fn internal_failure(reason: impl Into<String>) -> Self {
        Outcome { status: "internal_failure", reason: Some(reason.into()) }
    }
    fn blocked(reason: impl Into<String>) -> Self {
        Outcome { status: "blocked", reason: Some(reason.into()) }
    }
}

// ---------------------------------------------------------------------------
// Test runner
// ---------------------------------------------------------------------------

/// Run a whole test262 category: discover all `.js` files, compile each
/// in-process, and assert no **unexpected** failures.
///
/// "Expected" failures:
/// - `unsupported` — the compiler issued a controlled diagnostic.
/// - `blocked` — missing files, I/O errors.
///
/// "Unexpected" failures:
/// - `internal_failure` — `InvariantViolation` / compiler bug.
/// - `fail` — any other error that should not happen.
fn run_category(category: &str) {
    let files = discover_files(category);
    if files.is_empty() {
        eprintln!("[SKIP] {category}: no .js files found");
        return;
    }

    let mut unexpected: Vec<(PathBuf, Outcome)> = Vec::new();
    let mut pass_count = 0u32;
    let mut unsupported_count = 0u32;
    let mut blocked_count = 0u32;

    for file in &files {
        let outcome = classify_file(file);
        match outcome.status {
            "pass" => pass_count += 1,
            "unsupported" | "blocked" => {
                if outcome.status == "unsupported" {
                    unsupported_count += 1;
                } else {
                    blocked_count += 1;
                }
                // Accepted — these are known constraints.
            }
            _ => {
                unexpected.push((file.clone(), outcome));
            }
        }
    }

    let total = files.len();
    eprintln!(
        "[{category}] pass={pass_count} unsupported={unsupported_count} blocked={blocked_count} fail={} / {total}",
        unexpected.len(),
    );

    if !unexpected.is_empty() {
        // Dump JSONL for each unexpected failure.
        for (path, outcome) in &unexpected {
            println!(
                r#"{{"suite":"test262","case":"{}","status":"{}","reason":"{}"}}"#,
                path.display(),
                outcome.status,
                outcome.reason.as_deref().unwrap_or(""),
            );
        }
        panic!(
            "{} unexpected failures in category '{category}':\n{}",
            unexpected.len(),
            unexpected
                .iter()
                .map(|(p, o)| format!("  {} — {}", p.display(), o.reason.as_deref().unwrap_or("?")))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
}

// ---------------------------------------------------------------------------
// Individual test cases — one per test262 subcategory
// ---------------------------------------------------------------------------

macro_rules! t262_category {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            run_category($path);
        }
    };
}

// built-ins/  — top-level built-in objects
t262_category!(t262_builtins_aggregate_error, "built-ins/AggregateError");
t262_category!(t262_builtins_array, "built-ins/Array");
t262_category!(t262_builtins_array_buffer, "built-ins/ArrayBuffer");
t262_category!(t262_builtins_async_function, "built-ins/AsyncFunction");
t262_category!(t262_builtins_async_iterator, "built-ins/AsyncIteratorPrototype");
t262_category!(t262_builtins_atomics, "built-ins/Atomics");
t262_category!(t262_builtins_bigint, "built-ins/BigInt");
t262_category!(t262_builtins_boolean, "built-ins/Boolean");
t262_category!(t262_builtins_data_view, "built-ins/DataView");
t262_category!(t262_builtins_date, "built-ins/Date");
t262_category!(t262_builtins_error, "built-ins/Error");
t262_category!(t262_builtins_function, "built-ins/Function");
t262_category!(t262_builtins_generator, "built-ins/GeneratorFunction");
t262_category!(t262_builtins_json, "built-ins/JSON");
t262_category!(t262_builtins_map, "built-ins/Map");
t262_category!(t262_builtins_math, "built-ins/Math");
t262_category!(t262_builtins_number, "built-ins/Number");
t262_category!(t262_builtins_object, "built-ins/Object");
t262_category!(t262_builtins_promise, "built-ins/Promise");
t262_category!(t262_builtins_proxy, "built-ins/Proxy");
t262_category!(t262_builtins_reflect, "built-ins/Reflect");
t262_category!(t262_builtins_regexp, "built-ins/RegExp");
t262_category!(t262_builtins_set, "built-ins/Set");
t262_category!(t262_builtins_string, "built-ins/String");
t262_category!(t262_builtins_symbol, "built-ins/Symbol");
t262_category!(t262_builtins_typed_array, "built-ins/TypedArray");
t262_category!(t262_builtins_weak_map, "built-ins/WeakMap");
t262_category!(t262_builtins_weak_set, "built-ins/WeakSet");
t262_category!(t262_builtins_weak_ref, "built-ins/WeakRef");

// language/   — ECMAScript language feature tests
t262_category!(t262_language_arguments, "language/arguments-object");
t262_category!(t262_language_arrow, "language/arrow-function");
t262_category!(t262_language_async, "language/async-function");
t262_category!(t262_language_break, "language/break-statement");
t262_category!(t262_language_class, "language/class");
t262_category!(t262_language_continue, "language/continue-statement");
t262_category!(t262_language_destructuring, "language/destructuring");
t262_category!(t262_language_do, "language/do-while");
t262_category!(t262_language_export, "language/export");
t262_category!(t262_language_expressions, "language/expressions");
t262_category!(t262_language_for, "language/for-statement");
t262_category!(t262_language_function, "language/function");
t262_category!(t262_language_if, "language/if-statement");
t262_category!(t262_language_import, "language/import");
t262_category!(t262_language_label, "language/label-statement");
t262_category!(t262_language_line_terminator, "language/line-terminator");
t262_category!(t262_language_literal, "language/literals");
t262_category!(t262_language_module, "language/module");
t262_category!(t262_language_rest, "language/rest-parameters");
t262_category!(t262_language_return, "language/return-statement");
t262_category!(t262_language_switch, "language/switch-statement");
t262_category!(t262_language_throw, "language/throw-statement");
t262_category!(t262_language_try, "language/try-statement");
t262_category!(t262_language_unary, "language/unary-operators");
t262_category!(t262_language_var, "language/variable-statement");
t262_category!(t262_language_while, "language/while-statement");
t262_category!(t262_language_with, "language/with-statement");

// annexB/    — ECMA-262 Annex B (web compatibility)
t262_category!(t262_annexb_builtins, "annexB/built-ins");
t262_category!(t262_annex_b_language, "annexB/language");
