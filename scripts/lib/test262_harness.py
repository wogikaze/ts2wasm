"""Shared test262 harness module for reference-coverage test262 operations."""

import functools
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from ts2wasm_binary import resolve_ts2wasm_binary

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

REPO_ROOT = Path(__file__).resolve().parents[2]
REFERENCE_ROOT = Path(os.environ.get("TS2WASM_REFERENCE_ROOT", REPO_ROOT / "reference")).resolve()
TEST262_ROOT = Path(os.environ.get("TS2WASM_TEST262_ROOT", REFERENCE_ROOT / "test262")).resolve()
HARNESS_DIR = TEST262_ROOT / "harness"
TS2WASM_BINARY = resolve_ts2wasm_binary()

CORE_HARNESS_FILES = ("sta.js", "assert.js")
UNSUPPORTED_FLAGS = ("IsHTMLDDA",)
SUPPORTED_FEATURES = (
    "class",
    "async-iteration",
    "generators",
    "async-functions",
    "IsHTMLDDA",
    # Commonly used features that don't block compilation
    "arrow-function",
    "Reflect",
    "Reflect.construct",
    "Reflect.set",
    "legacy-regexp",
    "cross-realm",
    "error-message",
    "Symbol.species",
    "Symbol.unscopables",
    "Symbol.replace",
    "Symbol.match",
    "Symbol.search",
    "Symbol.matchAll",
    "Symbol.split",
    "Symbol.iterator",
    "Symbol.toPrimitive",
    "Symbol.isConcatSpreadable",
    "string-trimming",
    "Array.prototype.values",
    "Array.prototype.at",
    "Array.prototype.flat",
    "stable-array-sort",
    "Symbol",
    "regexp-named-groups",
    "regexp-duplicate-named-groups",
    "regexp-lookbehind",
    "regexp-dotall",
    "String.prototype.matchAll",
    "String.prototype.replaceAll",
    "String.prototype.trimStart",
    "String.prototype.trimEnd",
    "TypedArray",
    "BigInt",
)
ASSERT_FAILURE_SENTINEL = "__TS2WASM_TEST262_ASSERT_FAIL__"

# Track unknown features already logged to stderr (deduplication).
_seen_unknown_test262_features = set()

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
  throw new Test262Error("$262.createRealm is not supported by this harness slice");
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

# JavaScript standard globals that the compiler may not resolve natively
WASM_GLOBALS = r"""
var NaN = 0/0;
var Infinity = 1/0;
"""

WASM_HARNESS_SHIM = r"""
var $262 = {};
$262.gc = function() {};
$262.evalScript = function() { console.log("__TS2WASM_TEST262_ASSERT_FAIL__"); };
$262.createRealm = function() { console.log("__TS2WASM_TEST262_ASSERT_FAIL__"); };
$262.detachArrayBuffer = function() { console.log("__TS2WASM_TEST262_ASSERT_FAIL__"); };
$262.IsHTMLDDA = undefined;
$262.agent = {};
$262.agent.start = function() { console.log("__TS2WASM_TEST262_ASSERT_FAIL__"); };

function $ERROR(message) {
  console.log("__TS2WASM_TEST262_ASSERT_FAIL__");
}

function $DONOTEVALUATE() {
  console.log("__TS2WASM_TEST262_ASSERT_FAIL__");
}

function assert(mustBeTrue, message) {
  if (mustBeTrue === true) {
    return;
  }
  console.log("__TS2WASM_TEST262_ASSERT_FAIL__");
}
"""


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def escape_json(s):
    """Escape string for JSON."""
    if s is None:
        return ""
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')


def repo_relative(path):
    """Return a stable repo-relative path string for evidence and filtering."""
    try:
        return path.resolve().relative_to(REPO_ROOT).as_posix()
    except ValueError:
        pass
    try:
        reference_relative = path.resolve().relative_to(REFERENCE_ROOT).as_posix()
        return f"reference/{reference_relative}"
    except ValueError:
        return path.as_posix()


# ---------------------------------------------------------------------------
# Metadata
# ---------------------------------------------------------------------------

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
        for feature in self.features:
            if feature not in SUPPORTED_FEATURES and feature not in _seen_unknown_test262_features:
                _seen_unknown_test262_features.add(feature)
                print(f"warn: unknown test262 feature `{feature}`", file=sys.stderr)
        return None

    @property
    def expects_negative(self):
        return self.negative_phase is not None

    @property
    def expects_parse_syntax_error(self):
        return self.negative_phase == "parse" and self.negative_type == "SyntaxError"


def _parse_yaml_list(value):
    value = value.split('#', 1)[0].strip()
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


# ---------------------------------------------------------------------------
# Harness file loading
# ---------------------------------------------------------------------------

@functools.lru_cache(maxsize=None)
def load_harness_file(name):
    path = HARNESS_DIR / name
    if not path.is_file():
        raise FileNotFoundError(f"missing test262 harness file: {path}")
    return path.read_text(encoding="utf-8")


# ---------------------------------------------------------------------------
# Source building
# ---------------------------------------------------------------------------

def build_test262_source(test_file, source_code, metadata, target="wasm"):
    """Create the source compiled by ts2wasm and executed by the Node oracle."""
    if metadata.raw:
        return source_code
    case_source = source_code

    if target == "wasm":
        chunks = [WASM_HOST_PRELUDE]
        chunks.append("\n/* standard globals shim */\n")
        chunks.append(WASM_GLOBALS)
        chunks.append("\n/* test262 harness shim: sta.js + assert.js */\n")
        chunks.append(WASM_HARNESS_SHIM)
    else:
        chunks = [TEST262_HOST_PRELUDE]
        for harness_name in CORE_HARNESS_FILES:
            chunks.append(f"\n/* test262 harness: {harness_name} */\n")
            chunks.append(load_harness_file(harness_name))

    # Load additional harness includes for both targets
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
    chunks.append(case_source)
    return "\n".join(chunks)


# ---------------------------------------------------------------------------
# Record creation
# ---------------------------------------------------------------------------

def create_test_record(suite, case_path, target, status, expected=None, actual=None, reason=None, tracking=None, source_code=None, error_line=None, stderr=None, duration_ms=None):
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
    if duration_ms is not None:
        record["duration_ms"] = duration_ms

    return json.dumps(record)


# ---------------------------------------------------------------------------
# Feature label
# ---------------------------------------------------------------------------

def feature_label(diag_code, stderr, test_file):
    """Generate feature label from diagnostic code."""
    feature_map = {
        "ExpectedNegativeSyntax": "negative-parse-syntaxerror",
        "UnsupportedSyntax": "feature-unsupported",
        "UnsupportedBuiltin": "builtin-api",
        "UnsupportedDate": "date",
        "UnsupportedRegExp": "regexp-literal",
        "UnsupportedModule": "import-export",
        "UnsupportedEval": "eval",
        "UnsupportedTypeScriptSyntax": "parser-syntax",
        "UnsupportedRuntimeSubset": "runtime-subset",
        "UnresolvedName": "feature-resolution",
        "UnresolvedFunction": "feature-resolution",
        "TypeError": "type-system",
        "RuntimeError": "runtime",
        "InvariantViolation": "compiler-invariant",
        "BackendIo": "io-backend",
        "CompilationError": "compilation",
    }
    return feature_map.get(diag_code, diag_code.lower())


# ---------------------------------------------------------------------------
# Negative classification
# ---------------------------------------------------------------------------

def classify_completed_negative(metadata):
    if metadata.expects_parse_syntax_error:
        return (
            "unsupported",
            "ExpectedNegativeSyntax",
            "negative-parse-syntaxerror",
            "expected negative parse/SyntaxError but compiler and runtime completed successfully",
        )
    return (
        "fail",
        "ExpectedNegativeFailure",
        "",
        f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} completed successfully",
    )


# ---------------------------------------------------------------------------
# Compile & run
# ---------------------------------------------------------------------------

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
        [str(TS2WASM_BINARY), "build", str(tmp_source), "-o", str(tmp_wasm)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )

    if result.returncode != 0:
        result_status = "unsupported"
        stderr_content = result.stderr
        result_stderr_full = stderr_content
        diag_match = re.search(r'(UnsupportedSyntax|UnsupportedBuiltin|UnsupportedDate|UnsupportedRegExp|UnsupportedModule|UnsupportedEval|UnsupportedTypeScriptSyntax|UnsupportedRuntimeSubset|UnresolvedName|UnresolvedFunction|TypeError|RuntimeError|InvariantViolation|BackendIo|CompilationError)', stderr_content)
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
            result_status, result_diag, result_feature, result_reason = classify_completed_negative(metadata)
    else:
        result_status = "pass" if metadata.expects_negative else "runtime_error"
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


# ---------------------------------------------------------------------------
# Node reference
# ---------------------------------------------------------------------------

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


# ---------------------------------------------------------------------------
# Process one test
# ---------------------------------------------------------------------------

def process_one_test(test_file, tmp_dir, verbose=False):
    """Process a single test file and return JSON record and status."""
    if verbose:
        print(f"Processing: {test_file}", file=sys.stderr)

    started_at = time.perf_counter()

    def elapsed_ms():
        return int(round((time.perf_counter() - started_at) * 1000))

    result_status, result_diag, result_feature, result_reason, result_actual, source_code, error_line, stderr_full = compile_and_run_test(test_file, tmp_dir)
    metadata = parse_test262_metadata(source_code)

    if result_status == "pass":
        if metadata.expects_negative:
            expected = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'}"
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "pass", expected, result_reason, source_code=source_code, stderr=stderr_full, duration_ms=elapsed_ms())
            return record, "pass"

        expected, node_ok = get_node_reference(test_file, tmp_dir)

        if node_ok and result_actual == expected:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "pass", expected, result_actual, source_code=source_code, duration_ms=elapsed_ms())
            return record, "pass"
        elif node_ok:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "mismatch", expected, result_actual, "output mismatch", source_code=source_code, stderr=stderr_full, duration_ms=elapsed_ms())
            return record, "mismatch"
        else:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "blocked", expected, result_actual, "node execution failed", source_code=source_code, duration_ms=elapsed_ms())
            return record, "blocked"

    elif result_status == "unsupported":
        tracking_key = f"feature:{result_feature}"
        reason = f"{result_diag}/{result_feature}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "unsupported", None, None, reason, tracking_key, source_code, error_line, stderr_full, duration_ms=elapsed_ms())
        return record, "unsupported"

    elif result_status == "blocked":
        reason = f"{result_diag}/{result_feature}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "blocked", None, None, reason, source_code=source_code, error_line=error_line, stderr=stderr_full, duration_ms=elapsed_ms())
        return record, "blocked"

    elif result_status == "fail":
        reason = f"{result_diag}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "fail", None, result_actual, reason, source_code=source_code, error_line=error_line, stderr=stderr_full, duration_ms=elapsed_ms())
        return record, "fail"

    return "", "fail"
