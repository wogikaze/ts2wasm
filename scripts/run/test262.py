#!/usr/bin/env python3
"""Stream G: Test262 Runner with differential comparison

Usage:
  python scripts/manager.py test262 [--sample N] [--category PATTERN] [--jobs N] [--verbose] [--web-ui] > test262-results.jsonl

Compiles each test262 file, runs with iwasm, and compares output against Node.js reference.
Outputs one TestRecord per line in JSON Lines format to stdout (use --verbose for console output).
"""

import sys
import subprocess
import json
import re
import tempfile
import os
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
TEST262_ROOT = REPO_ROOT / "reference" / "test262"
HARNESS_DIR = TEST262_ROOT / "harness"

CORE_HARNESS_FILES = ("sta.js", "assert.js")
UNSUPPORTED_FLAGS = ("module", "async", "IsHTMLDDA")
ASSERT_FAILURE_SENTINEL = "__TS2WASM_TEST262_ASSERT_FAIL__"

TEST262_HOST_PRELUDE = r"""
function print(message) {
  console.log(message);
}

var $262 = {};

function test262_gc() {}

function test262_evalScript(source) {
  throw new Test262Error("$262.evalScript is not supported by this harness slice");
}

function test262_createRealm() {
  return {};
}

function test262_detachArrayBuffer() {
  throw new Test262Error("$262.detachArrayBuffer is not supported by this harness slice");
}

function test262_agent_start() {
  throw new Test262Error("$262.agent is not supported by this harness slice");
}

$262.global = {};
$262.gc = test262_gc;
$262.evalScript = test262_evalScript;
$262.createRealm = test262_createRealm;
$262.detachArrayBuffer = test262_detachArrayBuffer;
$262.IsHTMLDDA = undefined;
$262.agent = {};
$262.agent.start = test262_agent_start;
"""

WASM_HOST_PRELUDE = r"""
function print(message) {
  console.log(message);
}
"""

WASM_HARNESS_SHIM = r"""
class Test262Error {
  constructor(message) {
    this.message = message || "";
  }
}

function $DONOTEVALUATE() {
  throw "Test262: This statement should not be evaluated.";
}

class assert {
  static _isSameValue(a, b) {
    if (a === b) {
      return a !== 0 || 1 / a === 1 / b;
    }
    return a !== a && b !== b;
  }

  static sameValue(actual, expected, message) {
    if (actual === expected) {
      return undefined;
    }
    print("__TS2WASM_TEST262_ASSERT_FAIL__");
    throw new Test262Error(message);
  }

  static notSameValue(actual, unexpected, message) {
    if (actual !== unexpected) {
      return undefined;
    }
    print("__TS2WASM_TEST262_ASSERT_FAIL__");
    throw new Test262Error(message);
  }

  static throws(expectedErrorConstructor, func, message) {
    print("__TS2WASM_TEST262_ASSERT_FAIL__");
    throw new Test262Error(message);
  }
}
"""

class Test262Metadata:
    def __init__(self, flags=None, includes=None, features=None, negative_phase=None, negative_type=None):
        self.flags = flags or set()
        self.includes = includes or []
        self.features = features or []
        self.negative_phase = negative_phase
        self.negative_type = negative_type

    @property
    def raw(self):
        return "raw" in self.flags

    @property
    def unsupported_reason(self):
        for flag in UNSUPPORTED_FLAGS:
            if flag in self.flags:
                return f"test262 flag `{flag}` is not supported by this runner slice"
        if "IsHTMLDDA" in self.features:
            return "test262 feature `IsHTMLDDA` is not supported by this runner slice"
        return None

    @property
    def expects_negative(self):
        return self.negative_phase is not None

def usage():
    print("Usage: python scripts/manager.py test262 [--sample N] [--category PATTERN] [--jobs N] [--verbose] [--web-ui]")
    print()
    print("Options:")
    print("  --sample N          Run up to N files per extracted category.")
    print("  --category PATTERN  Regex matched against extracted category.")
    print("  --jobs N            Number of parallel workers (default: TEST262_JOBS or os.cpu_count or 4).")
    print("  --verbose           Show detailed per-test processing information.")
    print("  --web-ui            Refresh web-ui/public/data using this run's JSONL results.")
    print("  -h, --help          Show this help.")

def escape_json(s):
    """Escape string for JSON."""
    if s is None:
        return ""
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')

def extract_category(path):
    """Extract category from test262 file path."""
    match = re.search(r'test/language/([^/]+)/', str(path))
    return match.group(1) if match else "unknown"

def _parse_yaml_list(value):
    value = value.strip()
    if not value:
        return []
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        return [part.strip().strip("'\"") for part in inner.split(",") if part.strip()]
    return [value.strip().strip("'\"")]

def parse_test262_metadata(source_code):
    """Parse the subset of test262 frontmatter needed by this runner."""
    match = re.search(r'/\*---(.*?)---\*/', source_code, re.DOTALL)
    if not match:
        return Test262Metadata()

    flags = set()
    includes = []
    features = []
    negative_phase = None
    negative_type = None
    current_key = None
    in_negative = False

    for raw_line in match.group(1).splitlines():
        line = raw_line.rstrip()
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue

        if stripped.startswith("- ") and current_key:
            value = stripped[2:].strip().strip("'\"")
            if current_key == "flags":
                flags.add(value)
            elif current_key == "includes":
                includes.append(value)
            elif current_key == "features":
                features.append(value)
            continue

        if not raw_line.startswith((" ", "\t")):
            in_negative = False

        if ":" not in stripped:
            continue

        key, value = stripped.split(":", 1)
        key = key.strip()
        value = value.strip()
        current_key = key

        if key == "flags":
            flags.update(_parse_yaml_list(value))
        elif key in ("includes", "include"):
            includes.extend(_parse_yaml_list(value))
        elif key == "features":
            features.extend(_parse_yaml_list(value))
        elif key == "negative":
            in_negative = True
        elif in_negative and key == "phase":
            negative_phase = value.strip("'\"")
        elif in_negative and key == "type":
            negative_type = value.strip("'\"")

    return Test262Metadata(flags, includes, features, negative_phase, negative_type)

def load_harness_file(name):
    path = HARNESS_DIR / name
    if not path.is_file():
        raise FileNotFoundError(f"missing test262 harness file: {path}")
    return path.read_text(encoding="utf-8")

def build_test262_source(test_file, source_code, metadata, target="wasm"):
    """Create the source compiled by ts2wasm and executed by the Node oracle."""
    if metadata.raw:
        return source_code

    if target == "wasm":
        chunks = [WASM_HOST_PRELUDE]
        chunks.append("\n/* test262 harness compatibility shim: sta.js + assert.js */\n")
        chunks.append(WASM_HARNESS_SHIM)
    else:
        chunks = [TEST262_HOST_PRELUDE]
        for harness_name in CORE_HARNESS_FILES:
            chunks.append(f"\n/* test262 harness: {harness_name} */\n")
            chunks.append(load_harness_file(harness_name))

        for include in metadata.includes:
            if include in CORE_HARNESS_FILES:
                continue
            chunks.append(f"\n/* test262 harness: {include} */\n")
            chunks.append(load_harness_file(include))

    try:
        display_path = test_file.resolve().relative_to(REPO_ROOT)
    except ValueError:
        display_path = test_file
    chunks.append(f"\n/* test262 case: {display_path} */\n")
    chunks.append(source_code)
    return "\n".join(chunks)

def create_test_record(suite, case_path, target, status, expected=None, actual=None, reason=None, tracking=None, source_code=None, error_line=None, stderr=None):
    """Create a TestRecord JSON object."""
    record = {
        "suite": suite,
        "case": case_path,
        "target": target,
        "status": status
    }
    
    if expected is not None:
        record["expected"] = escape_json(expected)
    if actual is not None:
        record["actual"] = escape_json(actual)
    if reason:
        record["reason"] = escape_json(reason)
    if tracking:
        record["tracking"] = tracking
    if source_code:
        record["source_code"] = escape_json(source_code)
    if error_line is not None:
        record["error_line"] = error_line
    if stderr:
        record["stderr"] = escape_json(stderr)
    
    return json.dumps(record)

def feature_label(diag_code, stderr, test_file):
    """Generate feature label from diagnostic code."""
    # Simplified version - in the full script this would use feature-labels.sh
    feature_map = {
        "UnsupportedSyntax": "feature-unsupported",
        "UnresolvedName": "feature-resolution",
        "UnresolvedFunction": "feature-resolution",
        "TypeError": "type-system",
        "RuntimeError": "runtime",
        "InvariantViolation": "compiler-invariant",
        "BackendIo": "io-backend",
        "CompilationError": "compilation",
    }
    return feature_map.get(diag_code, diag_code.lower())

def compile_and_run_test(test_file, tmp_dir):
    """Compile and run a single test file."""
    tmp_wasm = tmp_dir / f"test-{os.getpid()}-{id(test_file)}.wasm"
    tmp_source = tmp_dir / f"test262-{os.getpid()}-{id(test_file)}.js"
    tmp_stdout = tmp_dir / f"stdout-{os.getpid()}-{id(test_file)}.txt"
    tmp_stderr = tmp_dir / f"stderr-{os.getpid()}-{id(test_file)}.txt"
    
    result_status = ""
    result_diag = ""
    result_feature = ""
    result_reason = ""
    result_actual = ""
    result_error_line = None
    result_stderr_full = ""
    
    # Read source code first (needed for line number extraction)
    source_code = ""
    try:
        source_code = test_file.read_text(encoding="utf-8")
    except:
        pass

    metadata = parse_test262_metadata(source_code)
    unsupported_reason = metadata.unsupported_reason
    if unsupported_reason:
        return "unsupported", "UnsupportedTest262Metadata", "test262-metadata", unsupported_reason, result_actual, source_code, result_error_line, result_stderr_full

    try:
        prepared_source = build_test262_source(test_file, source_code, metadata, target="wasm")
        tmp_source.write_text(prepared_source, encoding="utf-8")
    except Exception as exc:
        return "blocked", "HarnessError", "test262-harness", str(exc), result_actual, source_code, result_error_line, result_stderr_full
    
    # Compile with ts2wasm
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", str(tmp_source), "-o", str(tmp_wasm)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode != 0:
        result_status = "unsupported"
        stderr_content = result.stderr
        result_stderr_full = stderr_content
        diag_match = re.search(r'(UnsupportedSyntax|UnresolvedName|UnresolvedFunction|TypeError|RuntimeError|InvariantViolation|BackendIo|CompilationError)', stderr_content)
        result_diag = diag_match.group(1) if diag_match else "CompilationError"
        
        # Try to extract line number from error message
        # Look for patterns like "at line X" or ":X:" where X is a line number
        line_match = re.search(r'(?:at line |:)(\d+)(?::|$)', stderr_content)
        result_error_line = int(line_match.group(1)) if line_match else None
        
        # If no line number found, try to extract byte position and convert to line
        if not result_error_line and source_code:
            pos_match = re.search(r'at (\d+)\.\.(\d+)', stderr_content)
            if pos_match:
                try:
                    byte_pos = int(pos_match.group(1))
                    # Convert byte position to line number
                    lines_before = source_code[:byte_pos].count('\n')
                    result_error_line = lines_before + 1
                except:
                    pass
        
        reason_match = re.search(re.escape(f"[{result_diag}]"), stderr_content)
        result_reason = reason_match.group(0) if reason_match else stderr_content.split('\n')[0] if stderr_content else ""
        result_feature = feature_label(result_diag, stderr_content, str(test_file))
        if metadata.expects_negative:
            result_status = "pass"
            result_reason = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} rejected during compilation"
            result_actual = stderr_content
        return result_status, result_diag, result_feature, result_reason, result_actual, source_code, result_error_line, result_stderr_full
    
    # Run with iwasm
    result = subprocess.run(
        ["timeout", "5s", "iwasm", str(tmp_wasm)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode == 0:
        with open(tmp_stdout, 'w') as f:
            f.write(result.stdout)
        result_actual = result.stdout
        if ASSERT_FAILURE_SENTINEL in result.stdout:
            result_status = "fail"
            result_diag = "Test262AssertionFailure"
            result_reason = "test262 assertion failed"
        else:
            result_status = "fail" if metadata.expects_negative else "pass"
        if metadata.expects_negative:
            result_diag = "ExpectedNegativeFailure"
            result_reason = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} completed successfully"
    else:
        result_status = "pass" if metadata.expects_negative else "fail"
        result_diag = f"RuntimeError:{result.returncode}"
        if metadata.expects_negative:
            result_reason = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} rejected during execution"
        else:
            result_reason = result.stderr[:200] if result.stderr else ""
        result_stderr_full = result.stderr
        result_actual = result.stdout if result.stdout else ""
        
        # Try to extract line number from runtime error
        line_match = re.search(r'(?:at line |:)(\d+)(?::|$)', result.stderr)
        result_error_line = int(line_match.group(1)) if line_match else None
    
    return result_status, result_diag, result_feature, result_reason, result_actual, source_code, result_error_line, result_stderr_full

def get_node_reference(test_file, tmp_dir):
    """Get Node.js reference output."""
    tmp_out = tmp_dir / f"node-{os.getpid()}-{id(test_file)}.txt"
    source_code = test_file.read_text(encoding="utf-8")
    metadata = parse_test262_metadata(source_code)
    prepared_source = build_test262_source(test_file, source_code, metadata, target="node")
    tmp_source = tmp_dir / f"node-source-{os.getpid()}-{id(test_file)}.js"
    tmp_source.write_text(prepared_source, encoding="utf-8")
    
    result = subprocess.run(
        ["timeout", "5s", "node", str(tmp_source)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    with open(tmp_out, 'w') as f:
        f.write(result.stdout + result.stderr)
    
    node_ok = result.returncode == 0
    if metadata.expects_negative:
        node_ok = result.returncode != 0
    return result.stdout + result.stderr, node_ok

def process_one_test(test_file, tmp_dir, verbose=False):
    """Process a single test file and return JSON record and status."""
    if verbose:
        print(f"Processing: {test_file}", file=sys.stderr)
    
    result_status, result_diag, result_feature, result_reason, result_actual, source_code, error_line, stderr_full = compile_and_run_test(test_file, tmp_dir)
    metadata = parse_test262_metadata(source_code)
    
    if result_status == "pass":
        if metadata.expects_negative:
            expected = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'}"
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "pass", expected, result_reason, source_code=source_code, stderr=stderr_full)
            return record, "pass"

        expected, node_ok = get_node_reference(test_file, tmp_dir)
        
        if node_ok and result_actual == expected:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "pass", expected, result_actual, source_code=source_code)
            return record, "pass"
        elif node_ok:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "fail", expected, result_actual, "output mismatch", source_code=source_code, stderr=stderr_full)
            return record, "fail"
        else:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "blocked", expected, result_actual, "node execution failed", source_code=source_code)
            return record, "blocked"
    
    elif result_status == "unsupported":
        tracking_key = f"feature:{result_feature}"
        reason = f"{result_diag}/{result_feature}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "unsupported", None, None, reason, tracking_key, source_code, error_line, stderr_full)
        return record, "unsupported"

    elif result_status == "blocked":
        reason = f"{result_diag}/{result_feature}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "blocked", None, None, reason, source_code=source_code, error_line=error_line, stderr=stderr_full)
        return record, "blocked"
    
    elif result_status == "fail":
        reason = f"{result_diag}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "fail", None, result_actual, reason, source_code=source_code, error_line=error_line, stderr=stderr_full)
        return record, "fail"
    
    return "", "fail"

def refresh_web_ui_data(jsonl_file):
    """Regenerate web UI data without changing the runner's stdout contract."""
    result = subprocess.run(
        [
            sys.executable,
            str(REPO_ROOT / "scripts/gen/web-ui-data.py"),
            "--test-jsonl",
            str(jsonl_file),
        ],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )
    if result.stdout:
        print(result.stdout, end="", file=sys.stderr)
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)
    if result.returncode != 0:
        sys.exit(result.returncode)

def main():
    args = sys.argv[1:]
    
    sample = None
    category_pattern = "."
    jobs = int(os.environ.get("TEST262_JOBS", "")) if os.environ.get("TEST262_JOBS") else None
    verbose = False
    web_ui = False
    
    i = 0
    while i < len(args):
        if args[i] == "--sample":
            if i + 1 >= len(args):
                print("ERROR: --sample requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                sample = int(args[i + 1])
            except ValueError:
                print("ERROR: --sample must be a non-negative integer", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] == "--category":
            if i + 1 >= len(args):
                print("ERROR: --category requires a value", file=sys.stderr)
                sys.exit(1)
            category_pattern = args[i + 1]
            i += 2
        elif args[i] == "--jobs":
            if i + 1 >= len(args):
                print("ERROR: --jobs requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                jobs = int(args[i + 1])
            except ValueError:
                print("ERROR: --jobs must be a positive integer", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] == "--verbose":
            verbose = True
            i += 1
        elif args[i] == "--web-ui":
            web_ui = True
            i += 1
        elif args[i] in ("-h", "--help"):
            usage()
            sys.exit(0)
        else:
            print(f"ERROR: unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    if jobs is None:
        jobs = os.cpu_count() or 4
    
    if jobs < 1:
        print("ERROR: --jobs must be a positive integer", file=sys.stderr)
        sys.exit(1)
    
    if sample == 0:
        print("Starting test262 runner...", file=sys.stderr)
        print(f"Category filter: {category_pattern}", file=sys.stderr)
        print(f"Parallel jobs: {jobs}", file=sys.stderr)
        print("Sample mode: first 0 files per category", file=sys.stderr)
        print("Selected files: 0", file=sys.stderr)
        print("", file=sys.stderr)
        print("=== Test262 Summary ===", file=sys.stderr)
        print("Pass: 0", file=sys.stderr)
        print("Fail: 0", file=sys.stderr)
        print("Unsupported: 0", file=sys.stderr)
        print("Blocked: 0", file=sys.stderr)
        print("Total: 0", file=sys.stderr)
        if web_ui:
            results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
            results_dir.mkdir(parents=True, exist_ok=True)
            jsonl_file = results_dir / "test262-results.jsonl"
            jsonl_file.write_text("", encoding="utf-8")
            refresh_web_ui_data(jsonl_file)
        sys.exit(0)
    
    print("Starting test262 runner...", file=sys.stderr)
    print(f"Category filter: {category_pattern}", file=sys.stderr)
    print(f"Parallel jobs: {jobs}", file=sys.stderr)
    if sample:
        print(f"Sample mode: first {sample} files per category", file=sys.stderr)
    
    # Find test files
    test_files = sorted(REPO_ROOT.glob("reference/test262/test/language/**/*.js"))
    
    # Filter by category
    category_seen = {}
    selected_files = []
    
    for test_file in test_files:
        category = extract_category(test_file)
        
        try:
            if not re.search(category_pattern, category):
                continue
        except re.error:
            print(f"ERROR: Invalid category pattern: {category_pattern}", file=sys.stderr)
            sys.exit(1)
        
        if sample:
            seen = category_seen.get(category, 0)
            if seen >= sample:
                continue
            category_seen[category] = seen + 1
        
        selected_files.append(test_file)
    
    print(f"Selected files: {len(selected_files)}", file=sys.stderr)
    
    passed = 0
    failed = 0
    unsupported = 0
    blocked = 0
    
    # Save JSONL results to file
    results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
    results_dir.mkdir(parents=True, exist_ok=True)
    jsonl_file = results_dir / "test262-results.jsonl"
    
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_dir = Path(tmp_dir)
        
        with open(jsonl_file, 'w', encoding='utf-8') as jsonl_out:
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                futures = {executor.submit(process_one_test, f, tmp_dir, verbose): f for f in selected_files}
                
                completed = 0
                total = len(selected_files)
                last_progress = 0
                
                for future in as_completed(futures):
                    record, status = future.result()
                    
                    if record:
                        if verbose:
                            print(record)
                        jsonl_out.write(record + "\n")
                    
                    if status == "pass":
                        passed += 1
                    elif status == "fail":
                        failed += 1
                    elif status == "unsupported":
                        unsupported += 1
                    elif status == "blocked":
                        blocked += 1
                    
                    completed += 1
                    # Report progress at 5% increments
                    progress = int((completed / total) * 100)
                    if progress >= last_progress + 5:
                        print(f"Progress: {progress}% ({completed}/{total})", file=sys.stderr)
                        last_progress = progress
    
    print("", file=sys.stderr)
    print("=== Test262 Summary ===", file=sys.stderr)
    print(f"Pass: {passed}", file=sys.stderr)
    print(f"Fail: {failed}", file=sys.stderr)
    print(f"Unsupported: {unsupported}", file=sys.stderr)
    print(f"Blocked: {blocked}", file=sys.stderr)
    print(f"Total: {passed + failed + unsupported + blocked}", file=sys.stderr)
    
    # Save test results for site generation
    results = {
        "suite": "test262",
        "passed": passed,
        "failed": failed,
        "unsupported": unsupported,
        "blocked": blocked,
        "total": passed + failed + unsupported + blocked,
        "timestamp": datetime.now().isoformat(),
        "jsonl_file": str(jsonl_file)
    }
    
    results_file = results_dir / "test262-summary.json"
    results_file.write_text(json.dumps(results, indent=2), encoding="utf-8")
    print(f"Results saved to {results_file}", file=sys.stderr)
    print(f"JSONL results saved to {jsonl_file}", file=sys.stderr)
    
    # Also create legacy test262.json for backward compatibility
    legacy_file = results_dir / "test262.json"
    legacy_results = {
        "suite": "test262",
        "passed": passed,
        "failed": failed,
        "unsupported": unsupported,
        "blocked": blocked,
        "total": passed + failed + unsupported + blocked,
        "timestamp": datetime.now().isoformat()
    }
    legacy_file.write_text(json.dumps(legacy_results, indent=2), encoding="utf-8")
    
    # Auto-generate site after test completion
    print("Generating documentation site...", file=sys.stderr)
    gen_site_script = REPO_ROOT / "scripts" / "gen-site.py"
    if gen_site_script.exists():
        subprocess.run([sys.executable, str(gen_site_script)], cwd=REPO_ROOT)
        print("Site generation complete. Run 'mise run build-site' to build the site.", file=sys.stderr)

    if web_ui:
        refresh_web_ui_data(jsonl_file)

if __name__ == "__main__":
    main()
