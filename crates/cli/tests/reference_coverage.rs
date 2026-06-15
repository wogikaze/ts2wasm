//! Native Rust test262 / tsc reference coverage runner.
//!
//! Runs test262 conformance tests in-process (no Python, no IPC) and reports
//! results as cargo-nextest test cases.  Each test262 subcategory (e.g.
//! `built-ins/Array`, `built-ins/JSON`) is a single `#[test]` function.
//!
//! ## Architecture
//!
//! ```text
//! test262 .js file
//!   → process_test262_includes()  (Rust, harness stub injection)
//!   → compile_source_text()       (Rust in-process, no IPC)
//!   → [wasm binary]
//!       → iwasm subprocess (WAMR execution)
//!       → Node.js persistent oracle (vm.createContext, ~0 startup)
//!   → Compare stdout → semantic_pass or semantic_mismatch
//! ```
//!
//! ## Usage
//!
//! ```sh
//! cargo nextest run -p ts2wasm-cli --test reference_coverage -- t262_builtins_json
//! ```

use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::OnceLock;

use ts2wasm_compiler::server::compile_source_text;
use ts2wasm_compiler::test262_preprocessor::process_test262_includes;
use ts2wasm_shared::test_helpers::unique_temp_dir;

// ---------------------------------------------------------------------------
// Persistent Node.js oracle — evaluates JS in a fresh VM context per file,
// avoiding ~35ms startup overhead of spawning `node` per invocation.
// ---------------------------------------------------------------------------

/// Inline Node.js script for the persistent oracle server.
/// Reads JSON lines from stdin, each `{id, source}`, evaluates `source` in a
/// fresh `vm.createContext` with standard ECMAScript globals, and returns a
/// JSON line `{id, stdout, stderr, exit_code}`.  Each evaluation is wrapped
/// in a top-level try-catch so one crashing test never kills the server.
const NODE_ORACLE_SCRIPT: &str = r##"
const rl=require('readline'),vm=require('vm');
const I=rl.createInterface({input:process.stdin,terminal:false});
I.on('line',l=>{
  let id=0,source='',chunks=[];
  try{
    const p=JSON.parse(l);id=p.id;source=p.source;
    const ctx=vm.createContext({
      console:{log:(...a)=>chunks.push(a.join(' ')+'\n')},
      setTimeout,clearTimeout,setInterval,clearInterval,
      Math,Date,JSON,Array,Object,String,Number,Boolean,
      RegExp,Map,Set,WeakMap,WeakSet,Promise,Symbol,
      Error,TypeError,RangeError,SyntaxError,ReferenceError,
      URIError,EvalError,
      Int8Array,Uint8Array,Uint8ClampedArray,Int16Array,Uint16Array,
      Int32Array,Uint32Array,Float32Array,Float64Array,
      BigInt64Array,BigUint64Array,DataView,ArrayBuffer,
      SharedArrayBuffer,Atomics,BigInt,
      decodeURI,decodeURIComponent,encodeURI,encodeURIComponent,
      isFinite,isNaN,parseFloat,parseInt,
      Infinity,NaN,undefined,
    });
    Object.assign(ctx,{globalThis:ctx});
    vm.runInNewContext(source,ctx,{timeout:5000});
  }catch(e){
    // Error goes to stderr field.
    process.stdout.write(JSON.stringify({id,stdout:chunks.join(''),stderr:e.message,exit_code:1})+'\n');
    return;
  }
  process.stdout.write(JSON.stringify({id,stdout:chunks.join(''),stderr:'',exit_code:0})+'\n');
});
"##;

/// A persistent Node.js subprocess that evaluates JS source text in isolated
/// VM contexts.  Created once per test category; destroyed on Drop.
///
/// Stores the child handle and accesses piped stdin/stdout through `unsafe`
/// (the pipes outlive the child handle conceptually and are managed manually).
struct NodeOracle {
    child: Child,
    /// The child's stdin, wrapped for buffered writes.  `None` after the
    /// writer handle has been moved out.
    stdin: Option<BufWriter<std::process::ChildStdin>>,
    next_id: u64,
}

impl NodeOracle {
    fn start() -> Result<Self, String> {
        let mut child = Command::new("node")
            .arg("-e")
            .arg(NODE_ORACLE_SCRIPT)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("node spawn failed: {e}"))?;

        let stdin = BufWriter::new(child.stdin.take().ok_or("no stdin")?);
        Ok(NodeOracle {
            child,
            stdin: Some(stdin),
            next_id: 0,
        })
    }

    /// Evaluate `source` in an isolated VM context.  Returns `(stdout, stderr)`.
    /// Automatically restarts the Node.js process if it crashes.
    fn evaluate(&mut self, source: &str) -> Result<(String, String), String> {
        let mut last_error = String::new();
        for attempt in 0..2 {
            match self.evaluate_once(source) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = e;
                    if attempt == 0 {
                        // Process likely crashed — restart.
                        self.restart().map_err(|r| format!("restart failed: {r}"))?;
                    }
                }
            }
        }
        Err(format!("node oracle crashed twice: {last_error}"))
    }

    fn evaluate_once(&mut self, source: &str) -> Result<(String, String), String> {
        let id = self.next_id;
        self.next_id += 1;

        // Send request.
        let json = serde_json::json!({"id": id, "source": source});
        let line = serde_json::to_string(&json).map_err(|e| format!("serialize: {e}"))?;
        let stdin = self.stdin.as_mut().ok_or("stdin consumed")?;
        writeln!(stdin, "{line}").map_err(|e| format!("write: {e}"))?;
        stdin.flush().map_err(|e| format!("flush: {e}"))?;

        // Read response (one JSON line from stdout).
        let stdout = self.child.stdout.as_mut().ok_or("no stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("read: {e}"))?;

        let resp: serde_json::Value =
            serde_json::from_str(&line).map_err(|e| format!("parse: {e} — raw: {line:?}"))?;

        Ok((
            resp["stdout"].as_str().unwrap_or("").to_owned(),
            resp["stderr"].as_str().unwrap_or("").to_owned(),
        ))
    }

    fn restart(&mut self) -> Result<(), String> {
        // Kill old process.
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.stdin = None;

        // Start new.
        let mut child = Command::new("node")
            .arg("-e")
            .arg(NODE_ORACLE_SCRIPT)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("node spawn failed: {e}"))?;

        self.stdin = Some(BufWriter::new(child.stdin.take().ok_or("no stdin")?));
        self.child = child;
        self.next_id = 0;
        Ok(())
    }
}

impl Drop for NodeOracle {
    fn drop(&mut self) {
        // Signal shutdown by closing stdin.
        drop(self.stdin.take());
        let _ = self.child.wait();
    }
}

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

/// Run a single test262 file through the in-process compiler, optionally run
/// the wasm through iwasm and the source through the Node.js oracle for
/// semantic comparison, and classify the outcome.
fn classify_file(abs_path: &Path, node: Option<&mut NodeOracle>) -> Outcome {
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
    let wasm_path = match compile_source_text(&processed, &placeholder, Path::new(&tmpdir)) {
        Ok(path) => path,
        Err(d) => {
            return classify_diagnostic(d);
        }
    };

    // 4. (Optional) Semantic comparison via iwasm + Node.js oracle.
    if let Some(node) = node {
        return match semantic_check(&wasm_path, &processed, node) {
            Ok(true) => Outcome::pass(),
            Ok(false) => Outcome::semantic_mismatch("iwasm output != Node.js output"),
            Err(e) => Outcome::fail(format!("semantic check error: {e}")),
        };
    }

    // Compile-only mode — wasm binary produced.
    let _wasm_bytes = fs::read(&wasm_path).unwrap_or_default();
    Outcome::pass()
}

/// Classify a compiler diagnostic into an outcome.
fn classify_diagnostic(d: ts2wasm_compiler::Diagnostic) -> Outcome {
    let code = format!("{:?}", d.code);
    let msg = d.message;

    if code == "InvariantViolation" {
        return Outcome::internal_failure(format!("InvariantViolation: {msg}"));
    }

    if code.starts_with("Unsupported")
        || code.starts_with("Unresolved")
        || code == "SyntaxError"
        || code == "ArityMismatch"
        || code == "BackendIo"
    {
        return Outcome::unsupported(format!("{code}: {msg}"));
    }

    Outcome::fail(format!("{code}: {msg}"))
}

/// Run iwasm on the compiled wasm and Node.js on the source, and return
/// `true` if outputs match (semantic pass).
fn semantic_check(wasm_path: &Path, source: &str, node: &mut NodeOracle) -> Result<bool, String> {
    // Run via iwasm.
    let iwasm_output = run_iwasm(wasm_path)?;

    // Run via Node.js oracle.
    let (node_stdout, node_stderr) = node.evaluate(source).map_err(|e| format!("node: {e}"))?;
    let node_output = if node_stderr.is_empty() {
        node_stdout
    } else {
        // Test262 assertions may produce stderr output; include it.
        node_stdout + &node_stderr
    };

    Ok(iwasm_output.trim() == node_output.trim())
}

/// Run `iwasm` on a wasm file and return its combined stdout+stderr.
fn run_iwasm(wasm_path: &Path) -> Result<String, String> {
    let output = Command::new("iwasm")
        .arg(wasm_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("iwasm spawn: {e}"))?
        .wait_with_output()
        .map_err(|e| format!("iwasm wait: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    Ok(if stderr.is_empty() {
        stdout
    } else {
        stdout + &stderr
    })
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
        Outcome {
            status: "pass",
            reason: None,
        }
    }
    fn unsupported(reason: impl Into<String>) -> Self {
        Outcome {
            status: "unsupported",
            reason: Some(reason.into()),
        }
    }
    fn fail(reason: impl Into<String>) -> Self {
        Outcome {
            status: "fail",
            reason: Some(reason.into()),
        }
    }
    fn internal_failure(reason: impl Into<String>) -> Self {
        Outcome {
            status: "internal_failure",
            reason: Some(reason.into()),
        }
    }
    fn semantic_mismatch(reason: impl Into<String>) -> Self {
        Outcome {
            status: "semantic_mismatch",
            reason: Some(reason.into()),
        }
    }
    fn blocked(reason: impl Into<String>) -> Self {
        Outcome {
            status: "blocked",
            reason: Some(reason.into()),
        }
    }
}

// ---------------------------------------------------------------------------
// Test runner
// ---------------------------------------------------------------------------

/// Run a whole test262 category: discover all `.js` files, compile each
/// in-process, run semantic checks (iwasm + Node.js oracle), and assert no
/// **unexpected** failures.
///
/// "Expected" outcomes:
/// - `pass` — compiled + wasm output matches Node.js output.
/// - `unsupported` — the compiler issued a controlled diagnostic.
/// - `blocked` — missing files, I/O errors.
///
/// "Unexpected" outcomes:
/// - `internal_failure` — `InvariantViolation` / compiler bug.
/// - `semantic_mismatch` — wasm output differs from Node.js output.
/// - `fail` — any other error.
fn run_category(category: &str) {
    let files = discover_files(category);
    if files.is_empty() {
        eprintln!("[SKIP] {category}: no .js files found");
        return;
    }

    // Start a Node.js oracle for the duration of this category.
    let mut node = match NodeOracle::start() {
        Ok(n) => Some(n),
        Err(e) => {
            eprintln!("[WARN] Node.js oracle unavailable, skipping semantic checks: {e}");
            None
        }
    };

    let mut unexpected: Vec<(PathBuf, Outcome)> = Vec::new();
    let mut pass_count = 0u32;
    let mut unsupported_count = 0u32;
    let mut blocked_count = 0u32;
    let mut mismatch_count = 0u32;

    for file in &files {
        let outcome = classify_file(file, node.as_mut());
        match outcome.status {
            "pass" => pass_count += 1,
            "semantic_mismatch" => {
                mismatch_count += 1;
                unexpected.push((file.clone(), outcome));
            }
            "unsupported" | "blocked" => {
                if outcome.status == "unsupported" {
                    unsupported_count += 1;
                } else {
                    blocked_count += 1;
                }
            }
            _ => {
                unexpected.push((file.clone(), outcome));
            }
        }
    }

    let total = files.len();
    let semantic_label = if node.is_some() {
        " semantic"
    } else {
        " (compile-only)"
    };
    eprintln!(
        "[{category}]{semantic_label} pass={pass_count} mismatch={mismatch_count} unsupported={unsupported_count} blocked={blocked_count} / {total}"
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
t262_category!(
    t262_builtins_async_iterator,
    "built-ins/AsyncIteratorPrototype"
);
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
// language/expressions/ — split into sub-categories to avoid 240s timeout
t262_category!(t262_language_expressions_addition, "language/expressions/addition");
t262_category!(t262_language_expressions_array, "language/expressions/array");
t262_category!(t262_language_expressions_arrow_function, "language/expressions/arrow-function");
t262_category!(t262_language_expressions_assignment, "language/expressions/assignment");
t262_category!(t262_language_expressions_assignmenttargettype, "language/expressions/assignmenttargettype");
t262_category!(t262_language_expressions_async_arrow_function, "language/expressions/async-arrow-function");
t262_category!(t262_language_expressions_async_function, "language/expressions/async-function");
t262_category!(t262_language_expressions_async_generator, "language/expressions/async-generator");
t262_category!(t262_language_expressions_await, "language/expressions/await");
t262_category!(t262_language_expressions_bitwise_and, "language/expressions/bitwise-and");
t262_category!(t262_language_expressions_bitwise_not, "language/expressions/bitwise-not");
t262_category!(t262_language_expressions_bitwise_or, "language/expressions/bitwise-or");
t262_category!(t262_language_expressions_bitwise_xor, "language/expressions/bitwise-xor");
t262_category!(t262_language_expressions_call, "language/expressions/call");
t262_category!(t262_language_expressions_class, "language/expressions/class");
t262_category!(t262_language_expressions_coalesce, "language/expressions/coalesce");
t262_category!(t262_language_expressions_comma, "language/expressions/comma");
t262_category!(t262_language_expressions_compound_assignment, "language/expressions/compound-assignment");
t262_category!(t262_language_expressions_concatenation, "language/expressions/concatenation");
t262_category!(t262_language_expressions_conditional, "language/expressions/conditional");
t262_category!(t262_language_expressions_delete, "language/expressions/delete");
t262_category!(t262_language_expressions_division, "language/expressions/division");
t262_category!(t262_language_expressions_does_not_equals, "language/expressions/does-not-equals");
t262_category!(t262_language_expressions_dynamic_import, "language/expressions/dynamic-import");
t262_category!(t262_language_expressions_equals, "language/expressions/equals");
t262_category!(t262_language_expressions_exponentiation, "language/expressions/exponentiation");
t262_category!(t262_language_expressions_function, "language/expressions/function");
t262_category!(t262_language_expressions_generators, "language/expressions/generators");
t262_category!(t262_language_expressions_greater_than, "language/expressions/greater-than");
t262_category!(t262_language_expressions_greater_than_or_equal, "language/expressions/greater-than-or-equal");
t262_category!(t262_language_expressions_grouping, "language/expressions/grouping");
t262_category!(t262_language_expressions_import_meta, "language/expressions/import.meta");
t262_category!(t262_language_expressions_in, "language/expressions/in");
t262_category!(t262_language_expressions_instanceof, "language/expressions/instanceof");
t262_category!(t262_language_expressions_left_shift, "language/expressions/left-shift");
t262_category!(t262_language_expressions_less_than, "language/expressions/less-than");
t262_category!(t262_language_expressions_less_than_or_equal, "language/expressions/less-than-or-equal");
t262_category!(t262_language_expressions_logical_and, "language/expressions/logical-and");
t262_category!(t262_language_expressions_logical_assignment, "language/expressions/logical-assignment");
t262_category!(t262_language_expressions_logical_not, "language/expressions/logical-not");
t262_category!(t262_language_expressions_logical_or, "language/expressions/logical-or");
t262_category!(t262_language_expressions_member_expression, "language/expressions/member-expression");
t262_category!(t262_language_expressions_modulus, "language/expressions/modulus");
t262_category!(t262_language_expressions_multiplication, "language/expressions/multiplication");
t262_category!(t262_language_expressions_new, "language/expressions/new");
t262_category!(t262_language_expressions_new_target, "language/expressions/new.target");
t262_category!(t262_language_expressions_object, "language/expressions/object");
t262_category!(t262_language_expressions_optional_chaining, "language/expressions/optional-chaining");
t262_category!(t262_language_expressions_postfix_decrement, "language/expressions/postfix-decrement");
t262_category!(t262_language_expressions_postfix_increment, "language/expressions/postfix-increment");
t262_category!(t262_language_expressions_prefix_decrement, "language/expressions/prefix-decrement");
t262_category!(t262_language_expressions_prefix_increment, "language/expressions/prefix-increment");
t262_category!(t262_language_expressions_property_accessors, "language/expressions/property-accessors");
t262_category!(t262_language_expressions_relational, "language/expressions/relational");
t262_category!(t262_language_expressions_right_shift, "language/expressions/right-shift");
t262_category!(t262_language_expressions_strict_does_not_equals, "language/expressions/strict-does-not-equals");
t262_category!(t262_language_expressions_strict_equals, "language/expressions/strict-equals");
t262_category!(t262_language_expressions_subtraction, "language/expressions/subtraction");
t262_category!(t262_language_expressions_super, "language/expressions/super");
t262_category!(t262_language_expressions_tagged_template, "language/expressions/tagged-template");
t262_category!(t262_language_expressions_template_literal, "language/expressions/template-literal");
t262_category!(t262_language_expressions_this, "language/expressions/this");
t262_category!(t262_language_expressions_typeof, "language/expressions/typeof");
t262_category!(t262_language_expressions_unary_minus, "language/expressions/unary-minus");
t262_category!(t262_language_expressions_unary_plus, "language/expressions/unary-plus");
t262_category!(t262_language_expressions_unsigned_right_shift, "language/expressions/unsigned-right-shift");
t262_category!(t262_language_expressions_void, "language/expressions/void");
t262_category!(t262_language_expressions_yield, "language/expressions/yield");
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
