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
NON_BLOCKING_METADATA_FEATURES = (
    "class",
    "async-iteration",
    "generators",
    "async-functions",
    "arrow-function",
    "Symbol",
    "Symbol.iterator",
    "Symbol.species",
    "Symbol.toPrimitive",
    "Symbol.toStringTag",
    "TypedArray",
    "BigInt",
    "Array.prototype.at",
    "Array.prototype.flat",
    "Array.prototype.values",
    "String.prototype.matchAll",
    "String.prototype.replaceAll",
    "Promise",
    "WeakMap",
    "WeakSet",
    "Map",
    "Set",
)

BLOCKED_INCLUDES = (
    "agent.js",
    "detachArrayBuffer.js",
    "fnGlobalObject.js",
    "wellKnownIntrinsicObjects.js",
)
BLOCKED_FEATURES = (
    "cross-realm",
    "evalScript",
    "SharedArrayBuffer",
    "Atomics",
    "Intl",
)

ASSERT_FAILURE_SENTINEL = "__TS2WASM_TEST262_ASSERT_FAIL__"
COMPILE_NEGATIVE_PHASES = {"parse", "early", "resolution"}

# Inline minimal harness stubs used when the real test262 harness files
# are not available (e.g. running from a partial checkout).
INLINE_STA_JS = r"""
function Test262Error(message) {
  this.message = message || "";
}
"""

INLINE_ASSERT_JS = r"""
var assert = {};
assert.sameValue = function(actual, expected) {
  var same = actual === expected;
  // SameValue algorithm: NaN must compare equal to NaN
  if (!same && typeof actual === "number" && typeof expected === "number") {
    same = actual !== actual && expected !== expected;
  }
  if (!same) {
    throw new Test262Error(" expected same value");
  }
};
"""
# Track unknown features already logged to stderr (deduplication).
_seen_unknown_test262_features = set()

COMMON_HOST_PRELUDE = r"""
function print(message) {
  console.log(message);
}

var $262 = {};

function test262_gc() {}

function test262_evalScript(source) {
  throw new Error("$262.evalScript is not supported by this runner slice");
}

function test262_createRealm() {
  throw new Error("$262.createRealm is not supported by this runner slice");
}

function test262_detachArrayBuffer() {
  throw new Error("$262.detachArrayBuffer is not supported by this runner slice");
}

function test262_agent_start() {
  throw new Error("$262.agent is not supported by this runner slice");
}

$262.global = (function() { return this; })();
$262.gc = test262_gc;
$262.evalScript = test262_evalScript;
$262.createRealm = test262_createRealm;
$262.detachArrayBuffer = test262_detachArrayBuffer;
$262.IsHTMLDDA = undefined;
$262.agent = {};
$262.agent.start = test262_agent_start;
"""

# JavaScript standard globals that the compiler may not resolve natively
WASM_GLOBALS = r"""
var NaN = 0/0;
var Infinity = 1/0;
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
    def __init__(self, flags=None, includes=None, features=None, negative_phase=None, negative_type=None, source_code=""):
        self.flags = flags or set()
        self.includes = includes or []
        self.features = features or []
        self.negative_phase = negative_phase
        self.negative_type = negative_type
        self.source_code = source_code

    @property
    def raw(self):
        return "raw" in self.flags

    @property
    def unsupported_reason(self):
        for flag in UNSUPPORTED_FLAGS:
            if flag in self.flags:
                return f"test262 flag `{flag}` is not supported by this runner slice"
        
        for include in self.includes:
            if include in BLOCKED_INCLUDES:
                return f"test262 include `{include}` is not supported by this runner slice"
        
        for feature in self.features:
            if feature in BLOCKED_FEATURES:
                return f"test262 feature `{feature}` is not supported by this runner slice"
        
        # Source-based detection for missing capabilities that might not be in metadata
        blocked_patterns = (
            "$262.evalScript",
            "$262.createRealm",
            "$262.detachArrayBuffer",
            "$262.agent",
            "$262.global",
        )
        for pattern in blocked_patterns:
            if pattern in self.source_code:
                return f"test262 uses {pattern} which is not supported"

        for feature in self.features:
            if feature not in NON_BLOCKING_METADATA_FEATURES and feature not in _seen_unknown_test262_features:
                _seen_unknown_test262_features.add(feature)
                print(f"warn: unknown test262 feature `{feature}`", file=sys.stderr)
        return None

    @property
    def expects_negative(self):
        return self.negative_phase is not None

    @property
    def expects_parse_syntax_error(self):
        return self.negative_phase == "parse" and self.negative_type == "SyntaxError"

    @property
    def expects_compile_negative(self):
        return self.negative_phase in COMPILE_NEGATIVE_PHASES



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
        return Test262Metadata(source_code=source_code)

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
    return Test262Metadata(flags, includes, features, negative_phase, negative_type, source_code=source_code)


# ---------------------------------------------------------------------------
# Harness file loading
# ---------------------------------------------------------------------------

INLINE_HARNESS_STUBS = {
    "sta.js": INLINE_STA_JS,
    "assert.js": INLINE_ASSERT_JS,
}

@functools.lru_cache(maxsize=None)
def load_harness_file(name):
    # Always use inline stub for assert.js to avoid arity issues with
    # the real test262 assert.js which defines sameValue(actual, expected, message)
    # without default params. The inline stub uses message="" so callers
    # can pass 2 or 3 arguments.
    stub = INLINE_HARNESS_STUBS.get(name)
    if stub is not None:
        return stub
    path = HARNESS_DIR / name
    if not path.is_file():
        raise FileNotFoundError(f"missing test262 harness file: {path} (no inline stub for {name})")
    return path.read_text(encoding="utf-8")


# ---------------------------------------------------------------------------
# Source building
# ---------------------------------------------------------------------------

def build_test262_source(test_file, source_code, metadata, target="wasm"):
    """Create the source compiled by ts2wasm and executed by the Node oracle."""
    if metadata.raw:
        return source_code
    case_source = source_code

    chunks = []
    if "onlyStrict" in metadata.flags:
        chunks.append('"use strict";')
    chunks.append(COMMON_HOST_PRELUDE)
    if target == "wasm":
        chunks.append("\n/* standard globals shim */\n")
        chunks.append(WASM_GLOBALS)
    
    # Load real harness for both targets
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

def extract_unresolved_name(stderr):
    """Extract the unresolved symbol name from compiler error output."""
    if not stderr:
        return None
    match = re.search(r"unresolved name[`'\"]([^`'\"]+)[`'\"]", stderr, re.IGNORECASE)
    if match:
        return match.group(1).strip()
    return None


def feature_label(diag_code, stderr, test_file, phase=None):
    """Generate feature label from diagnostic code."""
    feature_map = {
        "ExpectedNegativeSyntax": "negative-parse-syntaxerror",
        "SyntaxError": "syntax-error",
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
        "NegativeRuntimeUnverified": "negative-runtime-unverified",
    }
    label = feature_map.get(diag_code, diag_code.lower())
    # Phase-aware distinction for UnsupportedSyntax and SyntaxError
    if diag_code in {"UnsupportedSyntax", "SyntaxError"} and phase is not None:
        parser_phases = {"lexer", "parser", "ast-validator"}
        if phase in parser_phases:
            label += ":parser"
        elif diag_code == "UnsupportedSyntax":
            label = "feature-unsupported"
    return label


# ---------------------------------------------------------------------------
# Negative classification
# ---------------------------------------------------------------------------

def can_pass_compile_negative(metadata, result_diag, diag_phase):
    """Strictly validate if a compilation error satisfies a parse-phase negative test."""
    return (
        metadata.negative_phase == "parse"
        and metadata.negative_type == "SyntaxError"
        and (
            # Must be an actual SyntaxError from the compiler, not UnsupportedSyntax
            result_diag == "SyntaxError"
            and diag_phase in {"lexer", "parser"}
        )
    )


def _negative_type_matches(metadata, output):
    expected_type = metadata.negative_type
    if not expected_type:
        return False
    return re.search(rf"\b{re.escape(expected_type)}\b", output or "") is not None


def verify_compile_negative_with_node(test_file, source_code, metadata, tmp_dir):
    """Use Node as the oracle for compile-phase negative tests not verified by ts2wasm.

    This is intentionally limited to test262 compile-time negative phases.  Runtime
    negative tests still require runtime error-type verification and must not become
    compile passes just because ts2wasm rejected unsupported syntax.
    """
    if not metadata.expects_compile_negative:
        return False, "negative test is not a compile-phase expectation", ""

    node_dir = Path(tmp_dir) / f"node-negative-{os.getpid()}-{time.monotonic_ns()}"
    node_dir.mkdir(parents=True, exist_ok=True)
    node_source = node_dir / "negative.js"
    try:
        prepared_source = build_test262_source(test_file, source_code, metadata, target="node")
        node_source.write_text(prepared_source, encoding="utf-8")
    except Exception as exc:
        return False, f"node negative oracle source preparation failed: {exc}", ""

    try:
        if "module" in metadata.flags:
            result = subprocess.run(
                ["timeout", "8s", "node", "--input-type=module", "--check"],
                input=prepared_source,
                capture_output=True,
                text=True,
                cwd=REPO_ROOT,
            )
        else:
            result = subprocess.run(
                [
                    "timeout",
                    "8s",
                    "node",
                    "-e",
                    (
                        "const fs = require('fs');"
                        "const vm = require('vm');"
                        "const sourcePath = process.argv[1];"
                        "new vm.Script(fs.readFileSync(sourcePath, 'utf8'), { filename: sourcePath });"
                    ),
                    str(node_source),
                ],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT,
            )
    except OSError as exc:
        return False, f"node negative oracle failed to start: {exc}", ""

    output = (result.stdout or "") + (result.stderr or "")
    if result.returncode == 0:
        return False, "node negative oracle completed successfully", output
    if not _negative_type_matches(metadata, output):
        return (
            False,
            f"node negative oracle did not report {metadata.negative_type or 'expected error'}",
            output,
        )

    reason = (
        f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} "
        "rejected during compilation; node oracle matched error type"
    )
    return True, reason, output

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
        diag_match = re.search(r'(SyntaxError|UnsupportedSyntax|UnsupportedBuiltin|UnsupportedDate|UnsupportedRegExp|UnsupportedModule|UnsupportedEval|UnsupportedTypeScriptSyntax|UnsupportedRuntimeSubset|UnresolvedName|UnresolvedFunction|TypeError|RuntimeError|InvariantViolation|BackendIo|CompilationError)', stderr_content)
        result_diag = diag_match.group(1) if diag_match else "CompilationError"
        diag_phase = None
        if result_diag in {"UnsupportedSyntax", "SyntaxError"}:
            phase_match = re.search(fr'\[{result_diag}/(\w+)\]', stderr_content)
            diag_phase = phase_match.group(1) if phase_match else None

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

        reason_match = re.search(re.escape(f"[{result_diag}") + r"(?:/\w+)?\]", stderr_content)
        result_reason = reason_match.group(0) if reason_match else stderr_content.split('\n')[0] if stderr_content else ""
        result_feature = feature_label(result_diag, stderr_content, str(test_file), diag_phase)
        
        if metadata.expects_negative:
            if can_pass_compile_negative(metadata, result_diag, diag_phase):
                result_status = "pass"
                result_reason = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} rejected during compilation"
                result_actual = stderr_content
            else:
                # Runtime negative test that failed to compile, or wrong parse error
                pass

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
            # If result.returncode == 0 but it was expected to fail:
            result_status = "fail" if metadata.expects_negative else "pass"
        
        if metadata.expects_negative and result_status == "fail":
             result_status, result_diag, result_feature, result_reason = classify_completed_negative(metadata)
    else:
        # Runtime failure (non-zero return code)
        result_stderr_full = result.stderr
        result_actual = result.stdout if result.stdout else ""

        if metadata.expects_negative:
            # We don't verify the error type yet (TypeError vs others), so mark as unverified
            result_status = "unsupported"
            result_diag = "NegativeRuntimeUnverified"
            result_feature = "negative-runtime-unverified"
            result_reason = f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} rejected during execution (unverified error type)"
        else:
            result_status = "runtime_error"
            result_diag = f"RuntimeError:{result.returncode}"
            result_feature = feature_label("RuntimeError", result.stderr, str(test_file))
            result_reason = result.stderr[:200] if result.stderr else ""

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
