#!/usr/bin/env python3
"""Reference suite coverage measurement

Usage:
  python scripts/manager.py reference-coverage <suite> [--limit N] [--json] [--detail]
      [--paths-file PATH] [--path-filter TEXT] [--dashboard-data]
      [--jsonl] [--jobs N] [--sample N] [--category PATTERN] [--no-server] [--no-semantic]

Suites:
  test262   -> reference/test262/test/**/*.js
  tsc       -> reference/typescript/tests/cases/compiler/**/*.ts
  tsgo      -> reference/typescript-go/testdata/tests/**

Notes:
  - This script classifies compile outcomes using ts2wasm diagnostics.
  - build_pass: build succeeded
  - semantic_pass: build succeeded and iwasm stdout exactly matches Node.js stdout
  - unsupported: source/compiler diagnostics except internal/backend failures
  - blocked: stderr contains [BackendIo] or command timeout
  - fail: internal compiler failures such as [InvariantViolation]
  - --json: output results as JSON instead of key=value pairs
  - --detail: output per-file details (file-path: diag-code: feature-label)
  - --paths-file: run a deterministic subset listed as repo-relative or suite-relative paths
  - --path-filter: run only files whose repo-relative path contains TEXT (repeatable)
  - --dashboard-data: refresh dashboard data after writing this suite coverage result
  - --jsonl: output results as JSONL (test262 only, uses server batch builds unless --no-server)
  - --no-semantic: skip Node/iwasm semantic comparison (compile-only, faster for local full runs)
  - --jobs N: number of parallel jobs (default: CPU count)
  - --sample N: max files per category (test262 only, uses category-based sampling)
   - --category PATTERN: regex filter for test categories (test262 only, used with --sample)
   - --no-server: use legacy subprocess mode (default: server mode with batch parallel build)
   - TS2WASM_REFERENCE_ROOT may point at an external reference/ directory for
    validation from isolated git worktrees.
   - TS2WASM_SERVER_EMIT_WASM=0 disables server-side wasm emission for semantic runs.
   - TS2WASM_SERVER_MAX_WORKERS caps server workers (default: min(CPU, 32)).
   - TS2WASM_REFERENCE_COVERAGE_BATCH controls server batch size (default: 1000).
   - TS2WASM_NOTIFY_NEW_PASSES=0 skips baseline/new-pass notification work.
"""

import sys
import subprocess
import json
import tempfile
import re
import shutil
import os
import threading
import itertools
import time
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed, wait, FIRST_COMPLETED
from datetime import datetime

# Path for auto-issue generation subprocess
GEN_ISSUES_SCRIPT = Path(__file__).parent.parent / "gen" / "coverage-to-issues.py"
UPDATE_ISSUE_INDEX_SCRIPT = Path(__file__).parent.parent / "gen" / "update-issue-index.py"

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
# Lazy import: test262_harness is only needed for test262 suite;
# tsc/tsgo suites use raw source directly.
test262_runner = None

def _ensure_test262_runner():
    global test262_runner
    if test262_runner is None:
        import test262_harness as _t262
        test262_runner = _t262
    return test262_runner

try:
    sys.path.insert(0, str(Path(__file__).parent.parent / "report"))
    from new_passes_notify import notify_new_passes
except ImportError:
    notify_new_passes = None

from ts2wasm_binary import resolve_ts2wasm_binary
from path_env import resolve_env_path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
TS2WASM_BINARY = None
REFERENCE_ROOT = Path(os.environ.get("TS2WASM_REFERENCE_ROOT", REPO_ROOT / "reference")).resolve()
COVERAGE_RESULTS_DIR = REPO_ROOT / "artifacts" / "coverage" / "results"
SERVER_BATCH_RESPONSE_TIMEOUT_SECS = 35


def ts2wasm_binary():
    """Resolve the compiler binary only after reference prerequisites pass."""
    global TS2WASM_BINARY
    if TS2WASM_BINARY is None:
        TS2WASM_BINARY = resolve_ts2wasm_binary()
    return TS2WASM_BINARY

SUITE_METADATA = {
    "test262": {
        "name": "test262",
        "repo_path": REFERENCE_ROOT / "test262",
        "path": REFERENCE_ROOT / "test262" / "test",
        "pattern": "reference/test262/test/**/*.js",
        "clone_cmd": "git clone https://github.com/tc39/test262.git reference/test262",
        "clone_hint": "git clone https://github.com/tc39/test262.git reference/test262",
    },
    "tsc": {
        "name": "TypeScript compiler cases",
        "repo_path": REFERENCE_ROOT / "typescript",
        "path": REFERENCE_ROOT / "typescript" / "tests" / "cases" / "compiler",
        "pattern": "reference/typescript/tests/cases/compiler/**/*.ts",
        "clone_cmd": "git clone --depth 1 https://github.com/microsoft/TypeScript.git reference/typescript",
        "clone_hint": "git clone --depth 1 https://github.com/microsoft/TypeScript.git reference/typescript",
    },
    "tsgo": {
        "name": "typescript-go testdata",
        "repo_path": REFERENCE_ROOT / "typescript-go",
        "path": REFERENCE_ROOT / "typescript-go" / "testdata" / "tests",
        "pattern": "reference/typescript-go/testdata/tests/**/*",
        "clone_cmd": "git clone --depth 1 https://github.com/microsoft/typescript-go.git reference/typescript-go",
        "clone_hint": "git clone --depth 1 https://github.com/microsoft/typescript-go.git reference/typescript-go",
    },
}

def suite_metadata_for_root(suite_key, reference_root):
    if suite_key == "test262":
        return {
            "name": "test262",
            "repo_path": reference_root / "test262",
            "path": reference_root / "test262" / "test",
            "pattern": "reference/test262/test/**/*.js",
            "clone_cmd": "git clone https://github.com/tc39/test262.git reference/test262",
            "clone_hint": "git clone https://github.com/tc39/test262.git reference/test262",
        }
    if suite_key == "tsc":
        return {
            "name": "TypeScript compiler cases",
            "repo_path": reference_root / "typescript",
            "path": reference_root / "typescript" / "tests" / "cases" / "compiler",
            "pattern": "reference/typescript/tests/cases/compiler/**/*.ts",
            "clone_cmd": "git clone --depth 1 https://github.com/microsoft/TypeScript.git reference/typescript",
            "clone_hint": "git clone --depth 1 https://github.com/microsoft/TypeScript.git reference/typescript",
        }
    if suite_key == "tsgo":
        return {
            "name": "typescript-go testdata",
            "repo_path": reference_root / "typescript-go",
            "path": reference_root / "typescript-go" / "testdata" / "tests",
            "pattern": "reference/typescript-go/testdata/tests/**/*",
            "clone_cmd": "git clone --depth 1 https://github.com/microsoft/typescript-go.git reference/typescript-go",
            "clone_hint": "git clone --depth 1 https://github.com/microsoft/typescript-go.git reference/typescript-go",
        }
    return None

def reference_root_from_absolute_filters(suite_key, path_filters):
    suite_dirs = {
        "test262": "test262",
        "tsc": "typescript",
        "tsgo": "typescript-go",
    }
    suite_dir = suite_dirs.get(suite_key)
    if suite_dir is None:
        return None

    for path_filter in path_filters:
        if not path_filter.startswith("/"):
            continue
        candidate = Path(path_filter).resolve()
        if not candidate.exists():
            continue

        parts = candidate.parts
        for index, part in enumerate(parts):
            if part == suite_dir:
                return Path(*parts[:index])
    return None

def missing_reference_hint(suite_key, config):
    """Emit clear instructions for restoring missing reference sources."""
    repo_path = config["repo_path"]
    pattern = config["pattern"]
    print(f"reference coverage failed: required {suite_key} source is missing", file=sys.stderr)
    print("Expected path:", repo_path, file=sys.stderr)
    print(f"Expected files matching: {pattern}", file=sys.stderr)
    print("Please initialize reference sources first, for example:", file=sys.stderr)
    print(f"  {config['clone_hint']}", file=sys.stderr)
    print("Or resume with an existing shallow checkout:", file=sys.stderr)
    print(f"  git -C {repo_path} fetch --depth 1 && git -C {repo_path} pull --ff-only", file=sys.stderr)
    print("After checkout/pull, rerun this command.", file=sys.stderr)

def resolve_suite_paths(suite, path_filters=None):
    """Resolve suite metadata and files, and validate repository presence."""
    if suite not in SUITE_METADATA:
        return None, None

    path_filters = path_filters or []
    config = SUITE_METADATA[suite]
    if not config["repo_path"].exists() or not config["path"].exists():
        inferred_root = reference_root_from_absolute_filters(suite, path_filters)
        inferred_config = suite_metadata_for_root(suite, inferred_root) if inferred_root else None
        if (
            inferred_config is not None
            and inferred_config["repo_path"].exists()
            and inferred_config["path"].exists()
        ):
            config = inferred_config
        else:
            missing_reference_hint(suite, config)
            return None, None

    if suite == "test262":
        files = sorted(config["path"].glob("**/*.js"))
    elif suite == "tsc":
        files = sorted(config["path"].glob("**/*.ts"))
    else:
        files = sorted(config["path"].rglob("*"))
        files = [f for f in files if f.is_file()]

    if len(files) == 0:
        missing_reference_hint(suite, config)
        return None, None

    return config, files

def check_prerequisites():
    """Validate all reference coverage prerequisites before the main loop.

    Checks:
    - Reference test suite directories exist and contain test files
    - iwasm binary is reachable on PATH
    - Node.js binary is reachable on PATH

    Prints a clear diagnostic for each missing prerequisite.
    Returns True if all prerequisites are satisfied, False otherwise.
    """
    all_ok = True

    # Check reference suite directories
    for suite_key, config in SUITE_METADATA.items():
        repo_path = config["repo_path"]
        suite_path = config["path"]
        if not repo_path.exists():
            print(
                f"ERROR: {suite_key} reference repository not found at {repo_path}",
                file=sys.stderr,
            )
            print(f"  {config['clone_hint']}", file=sys.stderr)
            all_ok = False
        elif not suite_path.exists():
            print(
                f"ERROR: {suite_key} test directory not found at {suite_path}",
                file=sys.stderr,
            )
            print(f"  Expected under {repo_path}", file=sys.stderr)
            all_ok = False
        else:
            # Verify at least one test file exists
            if suite_key == "test262":
                sample = list(suite_path.glob("**/*.js"))
            elif suite_key == "tsc":
                sample = list(suite_path.glob("**/*.ts"))
            else:
                sample = [f for f in suite_path.rglob("*") if f.is_file()]
            if not sample:
                print(
                    f"ERROR: {suite_key} test directory {suite_path} contains no test files",
                    file=sys.stderr,
                )
                all_ok = False
            else:
                print(f"  OK: {suite_key} ({len(sample)} files)")

    # Check iwasm binary
    iwasm_path = shutil.which("iwasm")
    if iwasm_path:
        print(f"  OK: iwasm ({iwasm_path})")
    else:
        print("ERROR: iwasm not found on PATH", file=sys.stderr)
        print("  Install iwasm (wamr) or add it to your PATH", file=sys.stderr)
        all_ok = False

    # Check Node.js binary
    node_path = shutil.which("node")
    if node_path:
        print(f"  OK: node ({node_path})")
    else:
        print("ERROR: node not found on PATH", file=sys.stderr)
        print("  Install Node.js or add it to your PATH", file=sys.stderr)
        all_ok = False

    return all_ok


def usage():
    print("Usage:")
    print("  python scripts/manager.py reference-coverage <suite> [--limit N] [--json] [--detail]")
    print("      [--paths-file PATH] [--path-filter TEXT] [--dashboard-data] [--no-dashboard-data]")
    print("      [--jsonl] [--jobs N] [--sample N] [--category PATTERN] [--no-server] [--no-semantic]")
    print("      [--check-prerequisites]")
    print()
    print("Suites:")
    print("  test262   -> reference/test262/test/**/*.js")
    print("  tsc       -> reference/typescript/tests/cases/compiler/**/*.ts")
    print("  tsgo      -> reference/typescript-go/testdata/tests/**")
    print()
    print("Flags:")
    print("  --jsonl      Output results as JSONL (test262 only, server batch mode by default)")
    print("  --jobs N     Number of parallel jobs (default: CPU count)")
    print("  --no-semantic disable semantic check (skip Node/iwasm execution after build)")
    print("  --sample N   Max files per category (test262 only, uses category-based sampling)")
    print("  --category PATTERN  Regex filter for test categories (test262 only, used with --sample)")
    print("  --check-prerequisites  Validate prerequisites and exit (no coverage run)")

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
        pass

    # Worktree-compatible fallback: use unresolved absolute path.
    # path.resolve() follows symlinks (e.g. reference/test262 -> parent repo)
    # which breaks .relative_to(REFERENCE_ROOT) in worktrees.
    # os.path.abspath does NOT follow symlinks, keeping the worktree-relative path.
    try:
        abspath = Path(os.path.abspath(path))
        reference_relative = abspath.relative_to(REFERENCE_ROOT).as_posix()
        return f"reference/{reference_relative}"
    except ValueError:
        return path.as_posix()

def _parse_yaml_scalar(value):
    return value.strip().strip("'\"")

def parse_test262_negative_metadata(source_code):
    """Parse the test262 negative metadata subset used by coverage classification."""
    match = re.search(r'/\*---(.*?)---\*/', source_code, re.DOTALL)
    if not match:
        return None, None

    in_negative = False
    negative_phase = None
    negative_type = None

    for raw_line in match.group(1).splitlines():
        stripped = raw_line.strip()
        if not stripped or stripped.startswith("#"):
            continue

        if not raw_line.startswith((" ", "\t")):
            in_negative = False

        if ":" not in stripped:
            continue

        key, value = stripped.split(":", 1)
        key = key.strip()
        value = _parse_yaml_scalar(value)

        if key == "negative":
            in_negative = True
        elif in_negative and key == "phase":
            negative_phase = value
        elif in_negative and key == "type":
            negative_type = value

    return negative_phase, negative_type

def is_expected_negative_parse_syntax_error(suite, file_path):
    if suite != "test262":
        return False

    try:
        source_code = file_path.read_text(encoding="utf-8")
    except OSError:
        return False

    negative_phase, negative_type = parse_test262_negative_metadata(source_code)
    return negative_phase == "parse" and negative_type == "SyntaxError"

def parse_paths_file(paths_file, suite_config, all_files):
    """Resolve a deterministic subset file list against the current suite."""
    list_path = Path(paths_file)
    if not list_path.is_absolute():
        list_path = REPO_ROOT / list_path

    if not list_path.is_file():
        print(f"--paths-file not found: {list_path}", file=sys.stderr)
        sys.exit(1)

    all_files_by_repo_path = {repo_relative(path): path for path in all_files}
    all_files_by_suite_path = {}
    for path in all_files:
        try:
            suite_relative = path.resolve().relative_to(suite_config["path"]).as_posix()
        except ValueError:
            continue
        all_files_by_suite_path[suite_relative] = path

    selected = []
    seen = set()
    with open(list_path, "r", encoding="utf-8") as handle:
        for line_number, raw_line in enumerate(handle, start=1):
            entry = raw_line.strip()
            if not entry or entry.startswith("#"):
                continue

            path = all_files_by_repo_path.get(entry)
            if path is None:
                path = all_files_by_suite_path.get(entry)
            if path is None:
                print(
                    f"{list_path}:{line_number}: path is not in selected suite: {entry}",
                    file=sys.stderr,
                )
                sys.exit(1)

            key = path.resolve()
            if key in seen:
                continue
            seen.add(key)
            selected.append(path)

    if not selected:
        print(f"--paths-file selected no files: {list_path}", file=sys.stderr)
        sys.exit(1)

    return selected

def apply_path_filters(files, path_filters):
    """Apply deterministic repo-relative substring filters."""
    if not path_filters:
        return files

    def matches_filter(path, path_filter):
        stable_path = repo_relative(path)
        if path_filter in stable_path:
            return True
        if path_filter.startswith("/"):
            try:
                return Path(path_filter).resolve() == path.resolve()
            except OSError:
                return False
        return False

    selected = [
        path for path in files
        if any(matches_filter(path, path_filter) for path_filter in path_filters)
    ]
    if not selected:
        filters = ", ".join(path_filters)
        print(f"--path-filter selected no files: {filters}", file=sys.stderr)
        sys.exit(1)

    return selected

def evidence_command(suite, limit, paths_file, path_filters):
    """Build a reproducible command string for reports and coverage artifacts."""
    parts = ["mise", "run", "reference-coverage", "--", suite]
    if limit is not None:
        parts.extend(["--limit", str(limit)])
    if paths_file:
        parts.extend(["--paths-file", str(paths_file)])
    for path_filter in path_filters:
        parts.extend(["--path-filter", path_filter])
    return " ".join(parts)

def refresh_web_ui_data():
    """Regenerate web UI data without changing this command's stdout contract."""
    command = [sys.executable, str(REPO_ROOT / "scripts/gen/web-ui-data.py")]
    out_dir = os.environ.get("TS2WASM_WEB_UI_DATA_DIR")
    if not out_dir:
        docs_repo = resolve_env_path(os.environ.get("TS2WASM_DOCS_REPO_PATH"), REPO_ROOT)
        if docs_repo:
            out_dir = str(Path(docs_repo) / "coverage" / "web-ui" / "public" / "data")
    if out_dir:
        command.extend(["--out-dir", out_dir])
    for jsonl_file in sorted((REPO_ROOT / "artifacts/coverage/results").glob("*-results.jsonl")):
        command.extend(["--test-jsonl", str(jsonl_file)])
    result = subprocess.run(
        command,
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

def write_coverage_result(summary):
    COVERAGE_RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    output_path = COVERAGE_RESULTS_DIR / f"{summary['suite']}.json"
    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=2, sort_keys=False)
        handle.write("\n")

def test262_harness_dir_for(file_path):
    parts = file_path.resolve().parts
    for index, part in enumerate(parts):
        if part == "test262":
            return Path(*parts[: index + 1]) / "harness"
    t262 = _ensure_test262_runner()
    return t262.HARNESS_DIR

def prepare_build_inputs(suite, file_path, tmp_dir):
    """Return source paths for wasm and Node execution."""
    if suite != "test262":
        return file_path, file_path

    t262 = _ensure_test262_runner()
    source_code = file_path.read_text(encoding="utf-8")
    metadata = t262.parse_test262_metadata(source_code)
    t262.HARNESS_DIR = test262_harness_dir_for(file_path)
    wasm_source = tmp_dir / "test262-wasm-input.js"
    node_source = tmp_dir / "test262-node-input.js"
    wasm_source.write_text(
        t262.build_test262_source(file_path, source_code, metadata, target="wasm"),
        encoding="utf-8",
    )
    node_source.write_text(
        t262.build_test262_source(file_path, source_code, metadata, target="node"),
        encoding="utf-8",
    )
    return wasm_source, node_source

def feature_label(diag_code, err_file, file_path):
    """Generate feature label from diagnostic code and error output."""
    # Based on scripts/lib/feature-labels.sh
    
    # First check diagnostic codes
    if diag_code == "BackendIo":
        return "backend-io"
    elif diag_code == "InvariantViolation":
        return "invariant-violation"
    elif diag_code == "UnresolvedName":
        return "name-resolution"
    elif diag_code == "UnresolvedFunction":
        return "function-resolution"
    elif diag_code == "DuplicateFunction":
        return "duplicate-function"
    elif diag_code == "DuplicateLocal":
        return "duplicate-local"
    elif diag_code == "DuplicateParameter":
        return "duplicate-parameter"
    elif diag_code == "NumberOutOfRange":
        return "number-range"
    elif diag_code == "ArityMismatch":
        return "arity"
    elif diag_code == "InvalidTopLevelReturn":
        return "top-level-return"
    elif diag_code == "UnsupportedBuiltin":
        return "builtin-api"
    elif diag_code == "UnsupportedDate":
        return "date"
    elif diag_code == "UnsupportedRegExp":
        return "regexp-literal"
    elif diag_code == "UnsupportedModule":
        return "import-export"
    elif diag_code == "UnsupportedEval":
        return "eval"
    elif diag_code == "UnsupportedTypeScriptSyntax":
        return "parser-syntax"
    elif diag_code == "UnsupportedRuntimeSubset":
        return "runtime-subset"
    
    # Check file path for feature detection
    path_lc = file_path.lower() if file_path else ""
    
    if "/built-ins/date/" in path_lc or "/built-ins/date." in path_lc:
        return "date"
    elif "/built-ins/array/" in path_lc:
        return "array-builtin"
    elif "/built-ins/function/" in path_lc or "/built-ins/function." in path_lc:
        return "function"
    elif "/built-ins/object/" in path_lc:
        return "object-builtin"
    elif "/regexp/" in path_lc or "/regular-expressions/" in path_lc or "/built-ins/regexp/" in path_lc:
        return "regexp-literal"
    elif "/built-ins/string/" in path_lc:
        return "string-builtin"
    elif "/built-ins/escape/" in path_lc or "/built-ins/unescape/" in path_lc:
        return "legacy-global-builtin"
    elif "/built-ins/" in path_lc:
        return "builtin-api"
    elif "/annexb/language/comments/" in path_lc:
        return "html-comment"
    elif "/annexb/language/eval-code/" in path_lc:
        return "eval"
    elif (
        "/annexb/language/expressions/logical-assignment/" in path_lc
        and "/emulates-undefined-" in path_lc
    ):
        return "annexb-ishtmldda"
    elif "/annexb/language/expressions/logical-assignment/" in path_lc:
        return "logical-assignment"
    elif "/annexb/language/expressions/template-literal/legacy-octal-escape-sequence-" in path_lc:
        return "legacy-octal-escape"
    elif (
        "/annexb/language/expressions/" in path_lc
        and path_lc.endswith("/emulates-undefined.js")
    ) or path_lc.endswith("/annexb/language/statements/if/emulated-undefined.js"):
        return "annexb-ishtmldda"
    elif "/for-await-of/" in path_lc:
        return "async-iteration"
    elif "/class/" in path_lc or "/class-" in path_lc or "/classes/" in path_lc:
        return "class"
    elif "/module/" in path_lc or "/import/" in path_lc or "/export/" in path_lc:
        return "import-export"
    elif "/async-" in path_lc or "/async/" in path_lc or "/generators/" in path_lc:
        return "async"
    elif "/destructuring/" in path_lc:
        return "destructuring"
    elif "/template/" in path_lc:
        return "template-literal"
    elif "/arrow-function/" in path_lc or "/arrow/" in path_lc:
        return "arrow-function"
    elif "/spread/" in path_lc:
        return "spread"
    elif ".tsx" in path_lc or "jsx" in path_lc:
        return "jsx"
    elif "declarationemit" in path_lc or "declarationmap" in path_lc or "declare" in path_lc:
        return "declaration-emit"
    elif "accessor" in path_lc:
        return "class-accessor"
    elif "parameterproperty" in path_lc:
        return "parameter-property"
    elif "anonymousclass" in path_lc or "anonclass" in path_lc or "unnamedclass" in path_lc or "classfields" in path_lc or "classfield" in path_lc:
        return "class"
    elif "alias" in path_lc:
        return "type-alias"
    elif "ambient" in path_lc:
        return "ambient-declaration"
    elif "amd" in path_lc or "systemmodule" in path_lc:
        return "module-system-amd"
    elif "package" in path_lc or "nodemodules" in path_lc or "paths" in path_lc or "resolution" in path_lc:
        return "module-resolution"
    elif "exportassignment" in path_lc or "import" in path_lc or "export" in path_lc or "module" in path_lc:
        return "import-export"
    elif "enum" in path_lc:
        return "enum"
    elif "decorator" in path_lc:
        return "decorator"
    elif "assertion" in path_lc or "satisfies" in path_lc or "asconst" in path_lc:
        return "type-assertion"
    elif "bindingpattern" in path_lc or "destructur" in path_lc:
        return "destructuring"
    elif "conditional" in path_lc or "keyof" in path_lc or "infer" in path_lc or "generic" in path_lc or "typepredicate" in path_lc:
        return "type-system"
    elif "scope" in path_lc:
        return "scope-analysis"
    elif "arguments" in path_lc or "args" in path_lc:
        return "arguments-object"
    elif "objectliteral" in path_lc or "object" in path_lc:
        return "object-literal"
    elif "jsdoc" in path_lc:
        return "jsdoc"
    
    # Check error text for feature detection
    text = err_file.lower() if err_file else ""
    
    if "issue-227" in text:
        return "typescript-directive"
    elif "only `export class`" in text:
        return "import-export"
    elif "unsupported expression" in text and "kind: import" in text:
        return "import-export"
    elif "class " in text:
        return "class"
    elif " import " in text or " export " in text or "require(" in text or 'require("' in text:
        return "import-export"
    elif "regexp" in text or "regular expression" in text:
        return "regexp-literal"
    elif "type annotation" in text or "typescript" in text or "interface " in text or " enum " in text:
        return "type-annotation"
    elif "reference types" in text or "type directive" in text:
        return "type-directive-resolution"
    elif "destructur" in text:
        return "destructuring"
    elif " async " in text or "await " in text or "generator" in text:
        return "async"
    elif "=>" in text or "arrow" in text:
        return "arrow-function"
    elif "template" in text:
        return "template-literal"
    elif "spread" in text:
        return "spread"
    elif "rest parameter" in text or "rest " in text:
        return "rest-parameter"
    elif "default parameter" in text or "default " in text:
        return "default-parameter"
    elif "switch" in text:
        return "switch"
    elif "while" in text or "do-while" in text or " for " in text:
        return "loop"
    elif "break" in text or "continue" in text:
        return "break-continue"
    elif "dynamic propert" in text or "computed propert" in text or "property access" in text or "property key" in text:
        return "property-access"
    elif "string literal key" in text or "object literal" in text:
        return "object-literal"
    elif "non-ascii" in text or "utf-8" in text or "utf8" in text:
        return "utf8-string"
    elif "==" in text:
        return "equality-operator"
    elif "binary operator" in text or "unary operator" in text:
        return "operator"
    elif "try statement" in text or "catch" in text or "finally" in text:
        return "try-catch"
    elif "new classname" in text or "new " in text:
        return "new-expression"
    elif "super" in text:
        return "super"
    elif "method" in text:
        return "method-call"
    elif "constructor" in text:
        return "class"
    elif "unsupported character" in text or "unterminated" in text or "expected " in text or "invalid number literal" in text:
        return "parser-syntax"
    elif "only identifier calls" in text:
        return "call-expression"
    elif "nested function" in text or "kind: function" in text:
        return "function"
    elif "expression type not yet supported" in text:
        return "unsupported-expression"
    elif "console." in text or "process." in text or "readfilesync" in text:
        return "builtin-api"
    
    return "unknown-unsupported"

def main():
    # Handle --check-prerequisites before suite parsing (works standalone or with a suite)
    if "--check-prerequisites" in sys.argv:
        ok = check_prerequisites()
        sys.exit(0 if ok else 1)

    if len(sys.argv) < 2:
        usage()
        sys.exit(1)
    
    suite = sys.argv[1]
    args = sys.argv[2:]
    
    limit = None
    json_output = False
    detail_output = False
    paths_file = None
    path_filters = []
    web_ui = True
    jsonl_output = False
    jobs = None
    semantic_check = True
    sample = None
    category_pattern = None
    server_mode = True
    auto_issues = False
    suite_detail_rows = []
    suite_detail_counter = [0]
    
    i = 0
    while i < len(args):
        if args[i] == "--limit":
            if i + 1 >= len(args):
                print("--limit requires a non-negative integer", file=sys.stderr)
                sys.exit(1)
            try:
                limit = int(args[i + 1])
            except ValueError:
                print("--limit requires a non-negative integer", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] == "--json":
            json_output = True
            i += 1
        elif args[i] == "--detail":
            detail_output = True
            i += 1
        elif args[i] == "--paths-file":
            if i + 1 >= len(args):
                print("--paths-file requires a path", file=sys.stderr)
                sys.exit(1)
            paths_file = args[i + 1]
            i += 2
        elif args[i] == "--path-filter":
            if i + 1 >= len(args):
                print("--path-filter requires a non-empty string", file=sys.stderr)
                sys.exit(1)
            if args[i + 1] == "":
                print("--path-filter requires a non-empty string", file=sys.stderr)
                sys.exit(1)
            path_filters.append(args[i + 1])
            i += 2
        elif args[i] == "--dashboard-data":
            web_ui = True
            i += 1
        elif args[i] == "--no-dashboard-data":
            web_ui = False
            i += 1
        elif args[i] == "--jsonl":
            jsonl_output = True
            i += 1
        elif args[i] == "--no-semantic":
            semantic_check = False
            i += 1
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
        elif args[i] == "--sample":
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
        elif args[i] == "--no-server":
            server_mode = False
            i += 1
        elif args[i] == "--auto-issues":
            auto_issues = True
            i += 1
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    if suite not in SUITE_METADATA:
        print(f"unknown suite: {suite}", file=sys.stderr)
        usage()
        sys.exit(1)

    if jsonl_output and suite != "test262":
        print("ERROR: --jsonl is only supported for suite=test262", file=sys.stderr)
        sys.exit(1)

    suite_config, files = resolve_suite_paths(suite, path_filters)
    if files is None:
        sys.exit(1)
    
    denominator = len(files)
    evidence = evidence_command(suite, limit, paths_file, path_filters)
    
    if sample is not None and sample < 1:
        if jsonl_output:
            print(f"Sample mode: 0 files selected", file=sys.stderr)
            if suite == "test262":
                print(f"=== {suite} Summary ===", file=sys.stderr)
                print("Pass: 0", file=sys.stderr)
                print("Fail: 0", file=sys.stderr)
                print("Unsupported: 0", file=sys.stderr)
                print("Blocked: 0", file=sys.stderr)
                print("Total: 0", file=sys.stderr)
            sys.exit(0)

    if limit == 0:
        summary = {
            "suite": suite,
            "suite_name": suite,
            "denominator": 0,
            "executed": 0,
            "build_coverage_percent": "0.00",
            "semantic_coverage_percent": "0.00",
            "build_pass": 0,
            "semantic_pass": 0,
            "mismatch": 0,
            "runtime_error": 0,
            "fail": 0,
            "unsupported": 0,
            "blocked": 0,
            "skip_with_reason": 0,
            "unsupported_diagcodes": {},
            "unsupported_features": {},
            "status": "in-progress",
            "selection": {
                "paths_file": paths_file,
                "path_filters": path_filters,
            },
            "evidence": evidence,
        }
        if json_output:
            print(json.dumps(summary, indent=2))
        else:
            print(f"suite={suite}")
            print("denominator=0")
            print("executed=0")
            print("coverage_percent=0.00")
            print("semantic_coverage_percent=0.00")
            print("build_pass=0")
            print("semantic_pass=0")
            print("mismatch=0")
            print("runtime_error=0")
            print("fail=0")
            print("unsupported=0")
            print("blocked=0")
            print("skip_with_reason=0")
            print("unsupported_diagcodes=")
            print("unsupported_features=")
            print("semantic_enabled=0")
        sys.exit(0)

    if paths_file:
        files = parse_paths_file(paths_file, suite_config, files)

    files = apply_path_filters(files, path_filters)
    
    if limit:
        files = files[:limit]

    if sample and suite == "test262":
        category_seen = {}
        sampled = []
        for f in files:
            cat_match = re.search(r'test/([^/]+)/', str(f))
            cat = cat_match.group(1) if cat_match else "default"
            if category_pattern and not re.search(category_pattern, cat):
                continue
            seen = category_seen.get(cat, 0)
            if seen >= sample:
                continue
            category_seen[cat] = seen + 1
            sampled.append(f)
        files = sampled
        print(f"Sample mode: {len(files)} files selected (max {sample} per category)", file=sys.stderr)

    # JSONL output mode (test262 only) uses the full differential harness.
    # Keep it before the aggregate coverage path so `mise run test262` does
    # not first compile the same selected files through server mode.
    if jsonl_output and suite == "test262":
        if not files:
            print(f"No files selected for {suite}", file=sys.stderr)
            sys.exit(0)
        if jobs is None:
            jobs = os.cpu_count() or 4
        if jobs < 1:
            jobs = 1

        t262 = _ensure_test262_runner()
        # The harness loader is cached by file name; set the root once for the
        # selected corpus before building sources in parallel.
        t262.HARNESS_DIR = test262_harness_dir_for(files[0])

        results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
        results_dir.mkdir(parents=True, exist_ok=True)
        jsonl_file = results_dir / f"{suite}-results.jsonl"

        passed = 0
        failed = 0
        unsupported = 0
        blocked = 0
        total_duration_ms = 0
        completed = 0
        total = len(files)
        last_progress = 0

        jsonl_started_at = time.perf_counter()
        include_jsonl_source = os.environ.get("TS2WASM_JSONL_SOURCE", "0") not in ("0", "false", "False", "no", "NO")
        node_oracle_policy = os.environ.get("TS2WASM_TEST262_NODE_ORACLE", "auto").strip().lower()
        try:
            metadata_prefix_bytes = int(os.environ.get("TS2WASM_TEST262_METADATA_PREFIX_BYTES", "8192") or "8192")
        except ValueError:
            metadata_prefix_bytes = 8192
        try:
            semantic_jobs = int(os.environ.get("TS2WASM_REFERENCE_COVERAGE_SEMANTIC_JOBS", "0") or "0")
        except ValueError:
            semantic_jobs = 0
        if semantic_jobs < 1:
            semantic_jobs = min(max(jobs, (os.cpu_count() or jobs) * 2), 32)
        try:
            prepare_jobs = int(os.environ.get("TS2WASM_REFERENCE_COVERAGE_PREPARE_JOBS", "0") or "0")
        except ValueError:
            prepare_jobs = 0
        if prepare_jobs < 1:
            prepare_jobs = min(max(jobs * 4, jobs), 64)

        metadata_cache_enabled = os.environ.get("TS2WASM_TEST262_METADATA_CACHE", "1") not in (
            "0", "false", "False", "no", "NO"
        )
        metadata_cache_file = REPO_ROOT / "artifacts" / "coverage" / "cache" / "test262-metadata-v2.json"
        metadata_cache_signature = {
            "version": 2,
            "unsupported_flags": list(t262.UNSUPPORTED_FLAGS),
            "supported_features": list(t262.SUPPORTED_FEATURES),
        }
        metadata_cache_entries = {}
        metadata_cache_dirty = [False]
        metadata_cache_lock = threading.Lock()

        def cache_key(file_path):
            return str(file_path.resolve())

        def load_metadata_cache():
            if not metadata_cache_enabled or not metadata_cache_file.is_file():
                return
            try:
                data = json.loads(metadata_cache_file.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                return
            if data.get("signature") != metadata_cache_signature:
                return
            entries = data.get("entries")
            if isinstance(entries, dict):
                metadata_cache_entries.update(entries)

        def metadata_from_cache(entry):
            return t262.Test262Metadata(
                flags=set(entry.get("flags") or []),
                includes=list(entry.get("includes") or []),
                features=list(entry.get("features") or []),
                negative_phase=entry.get("negative_phase"),
                negative_type=entry.get("negative_type"),
            )

        def cache_entry_valid(entry, stat_result):
            return (
                entry
                and entry.get("mtime_ns") == getattr(stat_result, "st_mtime_ns", None)
                and entry.get("size") == getattr(stat_result, "st_size", None)
            )

        def update_metadata_cache(file_path, stat_result, metadata, unsupported_reason):
            if not metadata_cache_enabled or stat_result is None or metadata is None:
                return
            entry = {
                "mtime_ns": getattr(stat_result, "st_mtime_ns", None),
                "size": getattr(stat_result, "st_size", None),
                "flags": sorted(metadata.flags),
                "includes": list(metadata.includes),
                "features": list(metadata.features),
                "negative_phase": metadata.negative_phase,
                "negative_type": metadata.negative_type,
                "unsupported_reason": unsupported_reason,
            }
            with metadata_cache_lock:
                metadata_cache_entries[cache_key(file_path)] = entry
                metadata_cache_dirty[0] = True

        def seed_metadata_cache_from_previous_jsonl():
            # If a previous run already produced metadata-unsupported records,
            # reuse those path classifications without opening tens of thousands
            # of test files on the next run.  The stat guard below keeps the seed
            # safe when the corpus changes.
            if not metadata_cache_enabled or not jsonl_file.is_file():
                return
            seeded = 0
            prefix = "UnsupportedTest262Metadata/test262-metadata: "
            try:
                with jsonl_file.open("r", encoding="utf-8") as handle:
                    for line in handle:
                        try:
                            record = json.loads(line)
                        except json.JSONDecodeError:
                            continue
                        if record.get("status") != "unsupported":
                            continue
                        reason = record.get("reason") or ""
                        if not reason.startswith(prefix):
                            continue
                        case_path = record.get("case")
                        if not case_path:
                            continue
                        file_path = Path(case_path)
                        if not file_path.is_absolute():
                            file_path = REPO_ROOT / file_path
                        try:
                            stat_result = file_path.stat()
                        except OSError:
                            continue
                        entry = {
                            "mtime_ns": getattr(stat_result, "st_mtime_ns", None),
                            "size": getattr(stat_result, "st_size", None),
                            "flags": [],
                            "includes": [],
                            "features": [],
                            "negative_phase": None,
                            "negative_type": None,
                            "unsupported_reason": reason[len(prefix):],
                        }
                        metadata_cache_entries[cache_key(file_path)] = entry
                        seeded += 1
            except OSError:
                return
            if seeded:
                metadata_cache_dirty[0] = True

        def save_metadata_cache():
            if not metadata_cache_enabled or not metadata_cache_dirty[0]:
                return
            metadata_cache_file.parent.mkdir(parents=True, exist_ok=True)
            with metadata_cache_lock:
                payload = {
                    "signature": metadata_cache_signature,
                    "entries": metadata_cache_entries,
                }
            tmp_cache_file = metadata_cache_file.with_suffix(".tmp")
            tmp_cache_file.write_text(json.dumps(payload, separators=(",", ":")), encoding="utf-8")
            tmp_cache_file.replace(metadata_cache_file)

        load_metadata_cache()
        seed_metadata_cache_from_previous_jsonl()

        def record_source(item):
            return item.get("source_code", "") if include_jsonl_source else None

        def elapsed_ms(started_at):
            return int(round((time.perf_counter() - started_at) * 1000))

        def extract_error_line(text, source_code):
            line_match = re.search(r"(?:at line |:)(\d+)(?::|$)", text or "")
            if line_match:
                return int(line_match.group(1))
            pos_match = re.search(r"at (\d+)\.\.(\d+)", text or "")
            if pos_match and source_code:
                try:
                    byte_pos = int(pos_match.group(1))
                    return source_code[:byte_pos].count("\n") + 1
                except ValueError:
                    return None
            return None

        def consume_record(jsonl_out, record, status):
            nonlocal passed, failed, unsupported, blocked, total_duration_ms
            nonlocal completed, last_progress
            if record:
                jsonl_out.write(record + "\n")
                try:
                    total_duration_ms += int(json.loads(record).get("duration_ms", 0) or 0)
                except json.JSONDecodeError:
                    pass
            if status == "pass":
                passed += 1
            elif status in ("fail", "mismatch", "runtime_error"):
                failed += 1
            elif status == "unsupported":
                unsupported += 1
            elif status == "blocked":
                blocked += 1
            completed += 1
            progress = int((completed / total) * 100)
            if progress >= last_progress + 5:
                print(f"Progress: {progress}% ({completed}/{total})", file=sys.stderr)
                last_progress = progress

        def make_unsupported_record(item, diag_code, feature, reason, started_at=None, stderr=None):
            tracking_key = f"feature:{feature}" if feature else None
            duration = elapsed_ms(started_at or item["started_at"])
            record = t262.create_test_record(
                "test262",
                str(item["file_path"]),
                "wasm-iwasm",
                "unsupported",
                None,
                None,
                f"{diag_code}/{feature}: {reason}" if feature else f"{diag_code}: {reason}",
                tracking_key,
                record_source(item),
                item.get("error_line"),
                stderr,
                duration,
            )
            return record, "unsupported"

        def make_blocked_record(item, diag_code, feature, reason, started_at=None, stderr=None):
            duration = elapsed_ms(started_at or item["started_at"])
            record = t262.create_test_record(
                "test262",
                str(item["file_path"]),
                "wasm-iwasm",
                "blocked",
                None,
                None,
                f"{diag_code}/{feature}: {reason}" if feature else f"{diag_code}: {reason}",
                source_code=record_source(item),
                error_line=item.get("error_line"),
                stderr=stderr,
                duration_ms=duration,
            )
            return record, "blocked"

        def make_fail_record(item, diag_code, reason, actual=None, started_at=None, stderr=None):
            duration = elapsed_ms(started_at or item["started_at"])
            record = t262.create_test_record(
                "test262",
                str(item["file_path"]),
                "wasm-iwasm",
                "fail",
                None,
                actual,
                f"{diag_code}: {reason}" if reason else diag_code,
                source_code=record_source(item),
                error_line=item.get("error_line"),
                stderr=stderr,
                duration_ms=duration,
            )
            return record, "fail"

        def make_negative_pass_record(item, phase, typ, reason, actual=None, started_at=None, stderr=None):
            duration = elapsed_ms(started_at or item["started_at"])
            expected = f"negative {phase}/{typ or 'error'}"
            record = t262.create_test_record(
                "test262",
                str(item["file_path"]),
                "wasm-iwasm",
                "pass",
                expected,
                actual or reason,
                source_code=record_source(item),
                stderr=stderr,
                duration_ms=duration,
            )
            return record, "pass"

        def read_metadata_prefix(file_path):
            # The test262 frontmatter is normally at the top of the file.  Reading
            # only the prefix lets us reject metadata-unsupported cases without
            # pulling tens of thousands of full sources into Python/JSONL.
            with file_path.open("rb") as handle:
                raw = handle.read(max(metadata_prefix_bytes, 1024) + 1)
            prefix = raw.decode("utf-8", errors="replace")
            start = prefix.find("/*---")
            if start >= 0:
                end = prefix.find("---*/", start + 5)
                if end >= 0:
                    header = prefix[:end + 5]
                    return header, t262.parse_test262_metadata(header), True
                return prefix, None, False
            # If there is no frontmatter in the prefix, treat metadata as empty.
            # Test262 metadata is front-loaded; supported cases still read the
            # full source below before compilation.
            return prefix, t262.parse_test262_metadata(prefix), len(raw) <= metadata_prefix_bytes

        def prepare_jsonl_item(pair):
            index, file_path = pair
            started_at = time.perf_counter()
            item = {
                "type": "build_item",
                "id": index,
                "file_path": file_path,
                "source_code": "",
                "metadata": None,
                "build_source": "",
                "started_at": started_at,
                "error_line": None,
            }

            try:
                stat_result = file_path.stat()
            except OSError as exc:
                return {
                    "type": "early_record",
                    "index": index,
                    "record_status": make_blocked_record(
                        item, "HarnessError", "test262-harness", f"failed to stat source: {exc}", started_at
                    ),
                }

            cached_metadata_entry = metadata_cache_entries.get(cache_key(file_path)) if metadata_cache_enabled else None
            if cache_entry_valid(cached_metadata_entry, stat_result):
                metadata = metadata_from_cache(cached_metadata_entry)
                item["metadata"] = metadata
                unsupported_reason = cached_metadata_entry.get("unsupported_reason") or metadata.unsupported_reason
                if unsupported_reason:
                    if include_jsonl_source:
                        try:
                            item["source_code"] = file_path.read_text(encoding="utf-8")
                        except (OSError, UnicodeDecodeError):
                            item["source_code"] = ""
                    return {
                        "type": "early_record",
                        "index": index,
                        "record_status": make_unsupported_record(
                            item,
                            "UnsupportedTest262Metadata",
                            "test262-metadata",
                            unsupported_reason,
                            started_at,
                        ),
                    }
                try:
                    source_code = file_path.read_text(encoding="utf-8")
                except (OSError, UnicodeDecodeError) as exc:
                    return {
                        "type": "early_record",
                        "index": index,
                        "record_status": make_blocked_record(
                            item, "HarnessError", "test262-harness", f"failed to read source: {exc}", started_at
                        ),
                    }
                item["source_code"] = source_code
                try:
                    item["build_source"] = t262.build_test262_source(
                        file_path, source_code, metadata, target="wasm"
                    )
                except Exception as exc:
                    return {
                        "type": "early_record",
                        "index": index,
                        "record_status": make_blocked_record(
                            item, "HarnessError", "test262-harness", str(exc), started_at
                        ),
                    }
                return {"type": "build_item", "index": index, "item": item}

            try:
                prefix_source, metadata, metadata_complete = read_metadata_prefix(file_path)
            except (OSError, UnicodeDecodeError) as exc:
                return {
                    "type": "early_record",
                    "index": index,
                    "record_status": make_blocked_record(
                        item, "HarnessError", "test262-harness", f"failed to read source: {exc}", started_at
                    ),
                }

            if metadata is not None:
                item["metadata"] = metadata
                unsupported_reason = metadata.unsupported_reason
                if unsupported_reason:
                    update_metadata_cache(file_path, stat_result, metadata, unsupported_reason)
                    if include_jsonl_source:
                        try:
                            item["source_code"] = file_path.read_text(encoding="utf-8")
                        except (OSError, UnicodeDecodeError):
                            item["source_code"] = prefix_source
                    return {
                        "type": "early_record",
                        "index": index,
                        "record_status": make_unsupported_record(
                            item,
                            "UnsupportedTest262Metadata",
                            "test262-metadata",
                            unsupported_reason,
                            started_at,
                        ),
                    }

            try:
                source_code = file_path.read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError) as exc:
                return {
                    "type": "early_record",
                    "index": index,
                    "record_status": make_blocked_record(
                        item, "HarnessError", "test262-harness", f"failed to read source: {exc}", started_at
                    ),
                }

            item["source_code"] = source_code
            if metadata is None or not metadata_complete:
                metadata = t262.parse_test262_metadata(source_code)
            item["metadata"] = metadata
            unsupported_reason = metadata.unsupported_reason
            update_metadata_cache(file_path, stat_result, metadata, unsupported_reason)
            if unsupported_reason:
                return {
                    "type": "early_record",
                    "index": index,
                    "record_status": make_unsupported_record(
                        item,
                        "UnsupportedTest262Metadata",
                        "test262-metadata",
                        unsupported_reason,
                        started_at,
                    ),
                }

            try:
                item["build_source"] = t262.build_test262_source(
                    file_path, source_code, metadata, target="wasm"
                )
            except Exception as exc:
                return {
                    "type": "early_record",
                    "index": index,
                    "record_status": make_blocked_record(
                        item, "HarnessError", "test262-harness", str(exc), started_at
                    ),
                }
            return {"type": "build_item", "index": index, "item": item}

        def get_node_reference_for_item(item, thread_tmp):
            node_source = t262.build_test262_source(
                item["file_path"], item["source_code"], item["metadata"], target="node"
            )
            tmp_source = thread_tmp / "node-source.js"
            tmp_source.write_text(node_source, encoding="utf-8")
            result = subprocess.run(
                ["timeout", "5s", "node", str(tmp_source)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT,
            )
            node_ok = result.returncode == 0
            if item["metadata"].expects_negative:
                node_ok = result.returncode != 0
            return result.stdout + result.stderr, node_ok

        def should_run_node_oracle(item, actual):
            if node_oracle_policy in ("always", "1", "true", "yes"):
                return True
            if node_oracle_policy in ("never", "0", "false", "no"):
                return False
            if actual:
                return True
            source = item.get("source_code", "")
            # Most passing test262 cases are silent.  For those, iwasm success
            # with empty stdout is enough for coverage accounting and avoids
            # thousands of short-lived Node oracle processes.  Keep Node for
            # tests that visibly request output so mismatches are still caught.
            return bool(re.search(r"\b(?:print|console\.log)\s*\(", source))

        def make_fast_oracle_pass_record(item, actual):
            duration = elapsed_ms(item["started_at"])
            expected = actual if node_oracle_policy in ("never", "0", "false", "no") else ""
            reason = "node oracle skipped for silent positive test"
            record = t262.create_test_record(
                "test262",
                str(item["file_path"]),
                "wasm-iwasm",
                "pass",
                expected,
                actual,
                reason,
                source_code=record_source(item),
                duration_ms=duration,
            )
            return record, "pass"

        def classify_server_error(item, build_resp):
            metadata = item["metadata"]
            diag_code = build_resp.get("code") or "CompilationError"
            message = build_resp.get("message") or diag_code
            stderr = f"[{diag_code}] {message}"
            item["error_line"] = extract_error_line(message, item.get("source_code", ""))

            if metadata.expects_negative:
                reason = (
                    f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} "
                    "rejected during compilation"
                )
                return make_negative_pass_record(
                    item,
                    metadata.negative_phase,
                    metadata.negative_type,
                    reason,
                    actual=reason,
                    stderr=stderr,
                )

            feature = t262.feature_label(diag_code, message, str(item["file_path"]))
            if diag_code == "BackendIo":
                return make_blocked_record(item, diag_code, feature, message, stderr=stderr)
            if diag_code == "InvariantViolation":
                return make_fail_record(item, diag_code, message, stderr=stderr)
            return make_unsupported_record(item, diag_code, feature, message, stderr=stderr)

        def classify_completed_negative_for_jsonl(item):
            metadata = item["metadata"]
            status, diag_code, feature, reason = t262.classify_completed_negative(metadata)
            if status == "unsupported":
                return make_unsupported_record(item, diag_code, feature, reason)
            if status == "fail":
                return make_fail_record(item, diag_code, reason)
            return make_negative_pass_record(
                item, metadata.negative_phase, metadata.negative_type, reason, actual=reason
            )

        def run_wasm_oracle_for_item(item, wasm_path):
            metadata = item["metadata"]
            thread_tmp = Path(tempfile.mkdtemp(dir=tmp_dir))
            try:
                if not semantic_check:
                    duration = elapsed_ms(item["started_at"])
                    record = t262.create_test_record(
                        "test262",
                        str(item["file_path"]),
                        "wasm-build",
                        "pass",
                        None,
                        None,
                        "build_pass",
                        source_code=record_source(item),
                        duration_ms=duration,
                    )
                    return record, "pass"

                wasm_result = subprocess.run(
                    ["timeout", "5s", "iwasm", str(wasm_path)],
                    capture_output=True,
                    text=True,
                    cwd=REPO_ROOT,
                )

                if wasm_result.returncode == 0:
                    actual = wasm_result.stdout
                    if t262.ASSERT_FAILURE_SENTINEL in actual:
                        return make_fail_record(
                            item,
                            "Test262AssertionFailure",
                            "test262 assertion failed",
                            actual=actual,
                            stderr=wasm_result.stderr,
                        )
                    if metadata.expects_negative:
                        return classify_completed_negative_for_jsonl(item)

                    if not should_run_node_oracle(item, actual):
                        return make_fast_oracle_pass_record(item, actual)

                    expected, node_ok = get_node_reference_for_item(item, thread_tmp)
                    if node_ok and actual == expected:
                        duration = elapsed_ms(item["started_at"])
                        record = t262.create_test_record(
                            "test262",
                            str(item["file_path"]),
                            "wasm-iwasm",
                            "pass",
                            expected,
                            actual,
                            source_code=record_source(item),
                            duration_ms=duration,
                        )
                        return record, "pass"
                    if node_ok:
                        duration = elapsed_ms(item["started_at"])
                        record = t262.create_test_record(
                            "test262",
                            str(item["file_path"]),
                            "wasm-iwasm",
                            "mismatch",
                            expected,
                            actual,
                            "output mismatch",
                            source_code=record_source(item),
                            stderr=wasm_result.stderr,
                            duration_ms=duration,
                        )
                        return record, "mismatch"
                    duration = elapsed_ms(item["started_at"])
                    record = t262.create_test_record(
                        "test262",
                        str(item["file_path"]),
                        "wasm-iwasm",
                        "blocked",
                        expected,
                        actual,
                        "node execution failed",
                        source_code=record_source(item),
                        duration_ms=duration,
                    )
                    return record, "blocked"

                if metadata.expects_negative:
                    reason = (
                        f"negative {metadata.negative_phase}/{metadata.negative_type or 'error'} "
                        "rejected during execution"
                    )
                    return make_negative_pass_record(
                        item,
                        metadata.negative_phase,
                        metadata.negative_type,
                        reason,
                        actual=reason,
                        stderr=wasm_result.stderr,
                    )

                item["error_line"] = extract_error_line(wasm_result.stderr, item.get("source_code", ""))
                return make_fail_record(
                    item,
                    f"RuntimeError:{wasm_result.returncode}",
                    wasm_result.stderr[:200] if wasm_result.stderr else "runtime execution failed",
                    actual=wasm_result.stdout,
                    stderr=wasm_result.stderr,
                )
            finally:
                shutil.rmtree(thread_tmp, ignore_errors=True)

        def start_jsonl_server():
            return subprocess.Popen(
                [str(ts2wasm_binary()), "server"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=sys.stderr,
                cwd=REPO_ROOT,
            )

        def stop_jsonl_server(proc):
            if proc is None:
                return
            try:
                proc.stdin.write(json.dumps({"id": -1, "source": ""}).encode("utf-8") + b"\n")
                proc.stdin.flush()
            except (BrokenPipeError, OSError):
                pass
            try:
                proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait()

        def read_server_line_with_timeout(proc):
            read_err = [None]
            read_result = [None]

            def do_read():
                try:
                    read_result[0] = proc.stdout.readline()
                except Exception as exc:
                    read_err[0] = exc

            reader = threading.Thread(target=do_read, daemon=True)
            reader.start()
            reader.join(timeout=SERVER_BATCH_RESPONSE_TIMEOUT_SECS)
            if reader.is_alive():
                return None, TimeoutError("server batch response timed out")
            if read_err[0] is not None:
                return None, read_err[0]
            if not read_result[0]:
                return None, EOFError("server disconnected")
            return read_result[0], None

        def run_legacy_jsonl_batch(jsonl_out, batch_items):
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                futures = {
                    executor.submit(t262.process_one_test, item["file_path"], tmp_dir, False): item
                    for item in batch_items
                }
                for future in as_completed(futures):
                    record, status = future.result()
                    consume_record(jsonl_out, record, status)

        with tempfile.TemporaryDirectory() as tmp_dir_name:
            tmp_dir = Path(tmp_dir_name)
            with open(jsonl_file, "w", encoding="utf-8") as jsonl_out:
                if not server_mode:
                    run_legacy_jsonl_batch(
                        jsonl_out,
                        [{"file_path": file_path} for file_path in files],
                    )
                else:
                    prepared = [None] * len(files)
                    with ThreadPoolExecutor(max_workers=prepare_jobs) as executor:
                        futures = {
                            executor.submit(prepare_jsonl_item, pair): pair[0]
                            for pair in enumerate(files)
                        }
                        for future in as_completed(futures):
                            result = future.result()
                            prepared[result["index"]] = result

                    build_items = []
                    for result in prepared:
                        if result["type"] == "early_record":
                            record, status = result["record_status"]
                            consume_record(jsonl_out, record, status)
                        else:
                            build_items.append(result["item"])

                    server_proc = None
                    semantic_executor = None
                    pending_semantic = set()
                    max_pending_semantic = max(semantic_jobs * 4, 1)

                    def drain_semantic(block=False):
                        while pending_semantic:
                            if block:
                                done, _ = wait(pending_semantic, return_when=FIRST_COMPLETED)
                            else:
                                done, _ = wait(pending_semantic, timeout=0, return_when=FIRST_COMPLETED)
                                if not done:
                                    return
                            for future in done:
                                pending_semantic.remove(future)
                                record, status = future.result()
                                consume_record(jsonl_out, record, status)
                            if not block:
                                return

                    try:
                        if build_items:
                            server_proc = start_jsonl_server()
                            if semantic_check:
                                semantic_executor = ThreadPoolExecutor(max_workers=semantic_jobs)

                        try:
                            batch_size = int(os.environ.get("TS2WASM_REFERENCE_COVERAGE_BATCH", "1000") or "1000")
                        except ValueError:
                            batch_size = 1000
                        batch_size = max(1, batch_size)
                        emit_mode = "wasm" if semantic_check else "check"
                        if build_items:
                            print(
                                f"Starting ts2wasm server (jsonl {emit_mode} mode, "
                                f"{jobs} compile workers, {semantic_jobs} semantic workers, "
                                f"{prepare_jobs} prepare workers)...",
                                file=sys.stderr,
                            )
                        for start in range(0, len(build_items), batch_size):
                            batch = build_items[start:start + batch_size]
                            for item in batch:
                                item["started_at"] = time.perf_counter()

                            if server_proc is None:
                                run_legacy_jsonl_batch(jsonl_out, batch)
                                continue

                            request = {
                                "id": -1,
                                "emit": emit_mode,
                                "jobs": jobs,
                                "items": [
                                    {"id": item["id"], "source": item["build_source"]}
                                    for item in batch
                                ],
                            }
                            try:
                                server_proc.stdin.write(json.dumps(request).encode("utf-8") + b"\n")
                                server_proc.stdin.flush()
                            except (BrokenPipeError, OSError):
                                try:
                                    server_proc.kill()
                                except OSError:
                                    pass
                                server_proc = None
                                run_legacy_jsonl_batch(jsonl_out, batch)
                                continue

                            resp_line, err = read_server_line_with_timeout(server_proc)
                            if err is not None:
                                try:
                                    server_proc.kill()
                                except OSError:
                                    pass
                                server_proc = None
                                run_legacy_jsonl_batch(jsonl_out, batch)
                                continue

                            try:
                                build_results = json.loads(resp_line.decode("utf-8"))
                            except json.JSONDecodeError:
                                try:
                                    server_proc.kill()
                                except OSError:
                                    pass
                                server_proc = None
                                run_legacy_jsonl_batch(jsonl_out, batch)
                                continue

                            results_by_id = {result["id"]: result for result in build_results}

                            def finish_item(item):
                                build_resp = results_by_id.get(item["id"])
                                if build_resp is None:
                                    return make_blocked_record(
                                        item,
                                        "ServerProtocolError",
                                        "test262-harness",
                                        "server response missing item id",
                                    )
                                if build_resp.get("status") != "ok":
                                    return classify_server_error(item, build_resp)
                                if not semantic_check:
                                    duration = elapsed_ms(item["started_at"])
                                    record = t262.create_test_record(
                                        "test262",
                                        str(item["file_path"]),
                                        "wasm-build",
                                        "pass",
                                        None,
                                        None,
                                        "build_pass",
                                        source_code=record_source(item),
                                        duration_ms=duration,
                                    )
                                    return record, "pass"
                                wasm_path = build_resp.get("wasm_path")
                                if not wasm_path:
                                    # Older or failed server-side emit path. Re-run the item through
                                    # the legacy harness to preserve JSONL semantics instead of
                                    # misclassifying a build-only success as a runtime pass.
                                    return t262.process_one_test(item["file_path"], tmp_dir, False)
                                return run_wasm_oracle_for_item(item, wasm_path)

                            for item in batch:
                                if semantic_executor is None:
                                    record, status = finish_item(item)
                                    consume_record(jsonl_out, record, status)
                                else:
                                    pending_semantic.add(semantic_executor.submit(finish_item, item))
                                    if len(pending_semantic) >= max_pending_semantic:
                                        drain_semantic(block=True)
                            drain_semantic(block=False)

                        drain_semantic(block=True)
                    finally:
                        if semantic_executor is not None:
                            semantic_executor.shutdown(wait=True)
                        stop_jsonl_server(server_proc)

        save_metadata_cache()

        print(f"\n=== {suite} Summary ===", file=sys.stderr)
        print(f"Pass: {passed}", file=sys.stderr)
        print(f"Fail: {failed}", file=sys.stderr)
        print(f"Unsupported: {unsupported}", file=sys.stderr)
        print(f"Blocked: {blocked}", file=sys.stderr)
        wall_duration_ms = int(round((time.perf_counter() - jsonl_started_at) * 1000))
        print(f"Total: {passed + failed + unsupported + blocked}", file=sys.stderr)
        print(f"Duration: {wall_duration_ms}ms", file=sys.stderr)
        if os.environ.get("TS2WASM_REFERENCE_COVERAGE_SHOW_CASE_DURATION_SUM") == "1":
            print(f"CaseDurationSum: {total_duration_ms}ms", file=sys.stderr)

        summary = {
            "suite": suite,
            "passed": passed,
            "failed": failed,
            "unsupported": unsupported,
            "blocked": blocked,
            "total": passed + failed + unsupported + blocked,
            "duration_ms": wall_duration_ms,
            "wall_duration_ms": wall_duration_ms,
            "case_duration_sum_ms": total_duration_ms,
            "timestamp": datetime.now().isoformat(),
            "jsonl_file": str(jsonl_file),
            "server_mode": bool(server_mode),
            "semantic_enabled": bool(semantic_check),
        }
        summary_file = results_dir / f"{suite}-summary.json"
        summary_file.write_text(json.dumps(summary, indent=2), encoding="utf-8")

        legacy = dict(summary)
        legacy.pop("jsonl_file", None)
        legacy_file = results_dir / f"{suite}.json"
        legacy_file.write_text(json.dumps(legacy, indent=2), encoding="utf-8")

        if (
            semantic_check
            and notify_new_passes is not None
            and os.environ.get("TS2WASM_NOTIFY_NEW_PASSES", "1") != "0"
        ):
            try:
                notify_new_passes(jsonl_file, suite=suite)
            except Exception as e:
                print(f"WARNING: notification failed: {e}", file=sys.stderr)

        if web_ui:
            refresh_web_ui_data()

        return

    coverage_started_at = time.perf_counter()

    executed = 0
    fail_count = 0
    unsupported_count = 0
    blocked_count = 0
    skip_count = 0
    
    unsupported_diag_counts = {}
    unsupported_feature_counts = {}
    
    # Semantic checks require test262 harness (only available for test262 suite)
    if suite != "test262":
        semantic_check = False
    semantic_enabled = bool(semantic_check and shutil.which("node") and shutil.which("iwasm"))
    server_emit_wasm = semantic_enabled and os.environ.get("TS2WASM_SERVER_EMIT_WASM", "1") != "0"
    
    if jobs is None:
        jobs = os.cpu_count() or 4
    
    # Server-mode setup
    use_server = server_mode
    server_proc = None
    
    def _start_server():
        """Start (or restart) the ts2wasm batch server process."""
        proc = subprocess.Popen(
            [str(ts2wasm_binary()), "server"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=sys.stderr,
            cwd=REPO_ROOT,
        )
        return proc
    
    if server_mode:
        print(f"Starting ts2wasm server (batch mode, {jobs} workers)...", file=sys.stderr)
        server_proc = _start_server()

    def _suite_detail_status(metrics):
        if metrics["build_pass"]:
            if metrics["semantic_pass"]:
                return "pass"
            if metrics["mismatch"]:
                return "mismatch"
            if metrics["runtime_error"]:
                return "runtime_error"
            if metrics["blocked"]:
                return "blocked"
            return "pass"
        if metrics["blocked"]:
            return "blocked"
        if metrics["fail"]:
            return "fail"
        if metrics["unsupported"]:
            return "unsupported"
        return "unsupported"

    def _append_suite_detail(metrics):
        status = _suite_detail_status(metrics)
        reason = None
        detail_line = metrics.get("detail_line") or ""
        if detail_line:
            parts = detail_line.split(": ", 1)
            if len(parts) == 2:
                reason = parts[1]
        if reason is None:
            diag_code = metrics.get("diag_code")
            feature = metrics.get("feature_label")
            if diag_code and feature:
                reason = f"{diag_code}/{feature}"
            elif diag_code:
                reason = diag_code
        if reason is None and status != "pass":
            reason = status
        suite_detail_rows.append({
            "id": str(suite_detail_counter[0]),
            "suite": suite,
            "case": metrics.get("file_path"),
            "name": metrics.get("case_name"),
            "target": "wasm-iwasm",
            "status": status,
            "count": 1,
            "reason": reason,
        })
        suite_detail_counter[0] += 1
    
    def _run_semantic_check(file_path, source_code, metadata, thread_tmp, out_wasm, result_metrics):
        """Run node and iwasm for a build-pass file, updating result_metrics."""
        t262 = _ensure_test262_runner()
        if metadata is not None and metadata.expects_negative:
            wasm_result = subprocess.run(
                ["timeout", "8s", "iwasm", str(out_wasm)],
                capture_output=True,
                cwd=REPO_ROOT,
            )
            if (
                wasm_result.returncode != 0
                or t262.ASSERT_FAILURE_SENTINEL.encode("utf-8") in wasm_result.stdout
            ):
                result_metrics["semantic_pass"] = True
            else:
                result_metrics["mismatch"] = True
            return

        node_source = t262.build_test262_source(
            file_path, source_code, metadata, target="node"
        )
        node_input = thread_tmp / "node.js"
        node_input.write_text(node_source, encoding="utf-8")
        
        node_result = subprocess.run(
            ["timeout", "8s", "node", str(node_input)],
            capture_output=True,
            cwd=REPO_ROOT,
        )
        wasm_result = subprocess.run(
            ["timeout", "8s", "iwasm", str(out_wasm)],
            capture_output=True,
            cwd=REPO_ROOT,
        )
        
        if node_result.returncode != 0:
            result_metrics["blocked"] = True
        elif wasm_result.returncode != 0:
            result_metrics["runtime_error"] = True
        elif node_result.stdout == wasm_result.stdout:
            result_metrics["semantic_pass"] = True
        else:
            result_metrics["mismatch"] = True
    
    def _classify_build_response(build_resp, item, semantic_enabled, tmp_dir):
        """Classify a batch build response into result_metrics. Returns result_metrics."""
        rm = item["result_metrics"]
        detail_path = item["detail_path"]
        
        if build_resp["status"] == "ok":
            rm["build_pass"] = True
            
            if semantic_enabled:
                _complete_semantic_for_build_item(item, rm, tmp_dir)
            
            if detail_output and not rm.get("detail_line"):
                rm["detail_line"] = f"{detail_path}: build_pass"
        else:
            diag_code = build_resp.get("code", "Unknown")
            rm["diag_code"] = diag_code

            metadata = item.get("metadata")
            if metadata is not None and metadata.expects_negative:
                rm["build_pass"] = True
                if semantic_enabled:
                    rm["semantic_pass"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: build_pass"
                return rm
            
            if diag_code == "BackendIo":
                rm["blocked"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: blocked"
            elif diag_code == "InvariantViolation":
                rm["fail"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: fail: InvariantViolation"
            else:
                rm["unsupported"] = True
                rm["feature_label"] = feature_label(diag_code, None, str(item["file_path"]))
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: {diag_code}: {rm['feature_label']}"
        
        return rm

    def _complete_semantic_for_build_item(item, result_metrics, tmp_dir):
        """Run Node/iwasm comparison for one server build-pass item.

        When server-side wasm emission is enabled, reuse the wasm_path returned
        by the batch server.  The old path rebuilt every passing item with a
        separate `ts2wasm build` subprocess, which doubled the expensive work.
        """
        thread_tmp = Path(tempfile.mkdtemp(dir=tmp_dir))
        try:
            server_wasm_path = item.get("wasm_path")
            if server_wasm_path:
                _run_semantic_check(
                    item["file_path"], item["source_code"], item["metadata"],
                    thread_tmp, Path(server_wasm_path), result_metrics
                )
                return result_metrics

            build_input = thread_tmp / "in.js"
            build_input.write_text(item["build_source"], encoding="utf-8")
            out_wasm = thread_tmp / "out.wasm"

            build_result = subprocess.run(
                ["timeout", "8s", str(ts2wasm_binary()), "build", str(build_input), "-o", str(out_wasm)],
                capture_output=True,
                cwd=REPO_ROOT
            )
            if build_result.returncode == 0:
                _run_semantic_check(
                    item["file_path"], item["source_code"], item["metadata"],
                    thread_tmp, out_wasm, result_metrics
                )
            return result_metrics
        finally:
            shutil.rmtree(thread_tmp, ignore_errors=True)

    def _accumulate_case_result(result, item):
        """Update counters/detail output from a normalized result dict."""
        nonlocal executed, build_pass_count, semantic_pass_count, mismatch_count
        nonlocal runtime_error_count, blocked_count, fail_count
        nonlocal unsupported_count, unsupported_diag_counts, unsupported_feature_counts

        executed += 1
        if detail_output:
            result.setdefault("file_path", str(item["file_path"]))
            result.setdefault("detail_path", item["detail_path"])
            result.setdefault("case_name", item["file_path"].name)
            _append_suite_detail(result)

        if result["unsupported"] and result["diag_code"] == "ExpectedNegativeSyntax":
            unsupported_count += 1
            unsupported_diag_counts["ExpectedNegativeSyntax"] = unsupported_diag_counts.get("ExpectedNegativeSyntax", 0) + 1
            unsupported_feature_counts["negative-parse-syntaxerror"] = unsupported_feature_counts.get("negative-parse-syntaxerror", 0) + 1
            if result["detail_line"]:
                file_details.append(result["detail_line"])
            return
        if result["build_pass"]:
            build_pass_count += 1
            if result["semantic_pass"]:
                semantic_pass_count += 1
            elif result["mismatch"]:
                mismatch_count += 1
            elif result["runtime_error"]:
                runtime_error_count += 1
            elif result["blocked"]:
                blocked_count += 1
            if result["detail_line"]:
                file_details.append(result["detail_line"])
            return
        if result["blocked"]:
            blocked_count += 1
            if result["detail_line"]:
                file_details.append(result["detail_line"])
            return
        if result["fail"]:
            fail_count += 1
            if result["detail_line"]:
                file_details.append(result["detail_line"])
            return
        if result["unsupported"]:
            unsupported_count += 1
            diag_code = result["diag_code"]
            feat = result["feature_label"]
            unsupported_diag_counts[diag_code] = unsupported_diag_counts.get(diag_code, 0) + 1
            unsupported_feature_counts[feat] = unsupported_feature_counts.get(feat, 0) + 1
            if result["detail_line"]:
                file_details.append(result["detail_line"])
            return

    def _run_build_item_in_subprocess(item, semantic_enabled, tmp_dir):
        """Fallback path: run a pre-processed build item through standalone compiler."""
        rm = item["result_metrics"]
        detail_path = item["detail_path"]
        thread_tmp = Path(tempfile.mkdtemp(dir=tmp_dir))
        try:
            build_input = thread_tmp / "in.js"
            build_input.write_text(item["build_source"], encoding="utf-8")
            out_wasm = thread_tmp / "out.wasm"

            build_result = subprocess.run(
                ["timeout", "8s", str(ts2wasm_binary()), "build", str(build_input), "-o", str(out_wasm)],
                capture_output=True,
                cwd=REPO_ROOT,
            )

            if build_result.returncode == 0:
                rm["build_pass"] = True
                if semantic_enabled:
                    _run_semantic_check(
                        item["file_path"], item["source_code"], item["metadata"],
                        thread_tmp, out_wasm, rm
                    )
                if detail_output and not rm.get("detail_line"):
                    rm["detail_line"] = f"{detail_path}: build_pass"
                return rm

            if build_result.returncode == 124:
                rm["blocked"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: blocked"
                return rm

            err_content = build_result.stderr.decode("utf-8", errors="ignore")
            diag_match = re.search(r"\[([A-Za-z0-9_]+)\]", err_content)
            diag_code = diag_match.group(1) if diag_match else "Unknown"
            rm["diag_code"] = diag_code
            if diag_code == "BackendIo":
                rm["blocked"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: blocked"
            elif diag_code == "InvariantViolation":
                rm["fail"] = True
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: fail: InvariantViolation"
            else:
                rm["unsupported"] = True
                rm["feature_label"] = feature_label(diag_code, err_content, str(item["file_path"]))
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: {diag_code}: {rm['feature_label']}"
            return rm
        finally:
            shutil.rmtree(thread_tmp, ignore_errors=True)
    
    def _process_one_file(file_path):
        """Process a single file for coverage measurement. Thread-safe.
        
        In server mode, returns a pre-processed item dict for batch building.
        In subprocess mode, runs the full build and returns result_metrics.
        """
        if not file_path.is_file():
            return None
        
        detail_path = repo_relative(file_path)
        result_metrics = {
            "file_path": str(file_path),
            "detail_path": detail_path,
            "case_name": file_path.name,
            "build_pass": False,
            "semantic_pass": False,
            "mismatch": False,
            "runtime_error": False,
            "blocked": False,
            "fail": False,
            "unsupported": False,
            "diag_code": None,
            "feature_label": None,
            "detail_line": None,
        }
        
        # Read source once
        try:
            source_code = file_path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            return None
        
        # For non-test262 suites (tsc/tsgo), skip test262-specific processing
        # and use the raw source directly.
        is_test262 = (suite == "test262")
        
        if is_test262:
            t262 = _ensure_test262_runner()
            metadata = t262.parse_test262_metadata(source_code)
            unsupported_reason = metadata.unsupported_reason
            if unsupported_reason:
                result_metrics["unsupported"] = True
                result_metrics["diag_code"] = "UnsupportedTest262Metadata"
                result_metrics["feature_label"] = "test262-metadata"
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: UnsupportedTest262Metadata: test262-metadata"
                return result_metrics
            t262.HARNESS_DIR = test262_harness_dir_for(file_path)
            build_source = t262.build_test262_source(
                file_path, source_code, metadata, target="wasm"
            )
        else:
            metadata = None
            build_source = source_code
        
        if use_server:
            # Server mode: return pre-processed item (batch build later)
            return {
                "type": "build_item",
                "id": id_counter[0],
                "file_path": file_path,
                "detail_path": detail_path,
                "source_code": source_code,
                "metadata": metadata,
                "build_source": build_source,
                "result_metrics": result_metrics,
            }
        
        # === Legacy subprocess mode ===
        thread_tmp = Path(tempfile.mkdtemp(dir=tmp_dir))
        try:
            build_input = thread_tmp / "in.js"
            build_input.write_text(build_source, encoding="utf-8")
            
            out_wasm = thread_tmp / "out.wasm"
            
            build_result = subprocess.run(
                ["timeout", "8s", str(ts2wasm_binary()), "build", str(build_input), "-o", str(out_wasm)],
                capture_output=True,
                cwd=REPO_ROOT
            )
            
            if build_result.returncode == 0:
                result_metrics["build_pass"] = True
                
                if semantic_enabled:
                    _run_semantic_check(file_path, source_code, metadata, thread_tmp, out_wasm, result_metrics)
                
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: build_pass"
                return result_metrics
            
            if build_result.returncode == 124:  # timeout
                result_metrics["blocked"] = True
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: blocked"
                return result_metrics
            
            # Extract diagnostic code
            err_content = build_result.stderr.decode('utf-8', errors='ignore')
            diag_match = re.search(r'\[([A-Za-z0-9_]+)\]', err_content)
            diag_code = diag_match.group(1) if diag_match else "Unknown"
            result_metrics["diag_code"] = diag_code

            if is_test262 and metadata.expects_negative:
                result_metrics["build_pass"] = True
                if semantic_enabled:
                    result_metrics["semantic_pass"] = True
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: build_pass"
                return result_metrics
            
            if diag_code == "BackendIo":
                result_metrics["blocked"] = True
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: blocked"
            elif diag_code == "InvariantViolation":
                result_metrics["fail"] = True
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: fail: InvariantViolation"
            else:
                result_metrics["unsupported"] = True
                feat = feature_label(diag_code, err_content, str(file_path))
                result_metrics["feature_label"] = feat
                if detail_output:
                    result_metrics["detail_line"] = f"{detail_path}: {diag_code}: {feat}"
            return result_metrics
        finally:
            shutil.rmtree(thread_tmp, ignore_errors=True)
    
    build_pass_count = 0
    semantic_pass_count = 0
    mismatch_count = 0
    runtime_error_count = 0
    
    file_details = []
    
    # Thread-safe counter for server items (list for mutation in closure)
    id_counter = [0]
    
    def _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir):
        """Process a batch of build items via parallel subprocess calls."""
        t0 = time.perf_counter()
        def _run_one(item):
            return _run_build_item_in_subprocess(item, semantic_enabled, tmp_dir)
        with ThreadPoolExecutor(max_workers=jobs) as pool:
            results = list(pool.map(_run_one, batch))
        for item, result in zip(batch, results):
            _accumulate_case_result(result, item)
        print(f"  parallel subprocess: {len(batch)} items in {time.perf_counter()-t0:.2f}s", file=sys.stderr)

    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_dir = Path(tmp_dir)
        
        if use_server:
            # === Server mode: parallel pre-process + batch build ===
            # Phase 1: Pre-process all files in parallel
            build_items = []
            early_results = []
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                futures = {executor.submit(_process_one_file, f): f for f in files}
                for future in as_completed(futures):
                    result = future.result()
                    if result is None:
                        continue
                    if isinstance(result, dict) and result.get("type") == "build_item":
                        result["id"] = id_counter[0]
                        id_counter[0] += 1
                        build_items.append(result)
                    else:
                        early_results.append(result)
            
            # Phase 2: Batch build via server (with auto-restart on crash)
            batch_size = 200
            for i in range(0, len(build_items), batch_size):
                batch = build_items[i:i+batch_size]

                # If server died in a previous batch, try to restart it
                if server_proc is None:
                    try:
                        server_proc = _start_server()
                        print(f"Restarted ts2wasm server for batch {i//batch_size + 1}", file=sys.stderr)
                    except OSError:
                        pass  # Fall through to subprocess fallback

                if server_proc is None:
                    _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir)
                    continue

                request = {
                    "id": -1,
                    "emit": "wasm" if server_emit_wasm else "check",
                    "items": [{"id": item["id"], "source": item["build_source"]} for item in batch]
                }
                request["jobs"] = jobs
                req = json.dumps(request)
                try:
                    server_proc.stdin.write(req.encode("utf-8") + b"\n")
                    server_proc.stdin.flush()
                except (BrokenPipeError, OSError):
                    # Server process died (e.g. stack overflow); parallel subprocess fallback.
                    if server_proc is not None:
                        try:
                            server_proc.kill()
                        except OSError:
                            pass
                        server_proc = None
                    _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir)
                    continue
                # Read response with timeout to avoid blocking if server crashes
                # Use poll to check server liveness, then read with short timeout
                resp_line = None
                _read_err = [None]
                _read_result = [None]
                def _do_read():
                    try:
                        _read_result[0] = server_proc.stdout.readline()
                    except Exception as e:
                        _read_err[0] = e
                _reader = threading.Thread(target=_do_read, daemon=True)
                _reader.start()
                _reader.join(timeout=SERVER_BATCH_RESPONSE_TIMEOUT_SECS)
                if _reader.is_alive():
                    # Server likely crashed; kill and fall back
                    if server_proc is not None:
                        try:
                            server_proc.kill()
                        except OSError:
                            pass
                        server_proc = None
                    _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir)
                    continue
                if _read_err[0] is not None or _read_result[0] is None or not _read_result[0]:
                    # Server disconnected; parallel subprocess fallback.
                    if server_proc is not None:
                        try:
                            server_proc.kill()
                        except OSError:
                            pass
                        server_proc = None
                    _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir)
                    continue
                resp_line = _read_result[0]
                try:
                    build_results = json.loads(resp_line.decode("utf-8"))
                except json.JSONDecodeError:
                    if server_proc is not None:
                        try:
                            server_proc.kill()
                        except OSError:
                            pass
                        server_proc = None
                    _parallel_subprocess_batch(batch, semantic_enabled, tmp_dir)
                    continue
                results_by_id = {r["id"]: r for r in build_results}
                
                classified_results = []
                for item in batch:
                    build_response = results_by_id[item["id"]]
                    if build_response.get("wasm_path"):
                        item["wasm_path"] = build_response["wasm_path"]
                    result = _classify_build_response(
                        build_response, item, False, tmp_dir
                    )
                    classified_results.append((item, result))

                if semantic_enabled:
                    def _complete_pair(pair):
                        item, result = pair
                        if result["build_pass"]:
                            result = _complete_semantic_for_build_item(item, result, tmp_dir)
                        return item, result

                    with ThreadPoolExecutor(max_workers=jobs) as pool:
                        classified_results = list(pool.map(_complete_pair, classified_results))

                for item, result in classified_results:
                    _accumulate_case_result(result, item)
            
            # Phase 3: Process early results (negative-parse-syntaxerror etc.)
            for result in early_results:
                _accumulate_case_result(result, {
                    "file_path": Path(result["file_path"]),
                    "detail_path": result["detail_path"],
                })
        else:
            # === Legacy subprocess mode ===
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                futures = {executor.submit(_process_one_file, f): f for f in files}
                for future in as_completed(futures):
                    result = future.result()
                    if result is None:
                        continue
                    executed += 1
                    if detail_output:
                        _append_suite_detail(result)
                    
                    if result["unsupported"] and result["diag_code"] == "ExpectedNegativeSyntax":
                        unsupported_count += 1
                        unsupported_diag_counts["ExpectedNegativeSyntax"] = unsupported_diag_counts.get("ExpectedNegativeSyntax", 0) + 1
                        unsupported_feature_counts["negative-parse-syntaxerror"] = unsupported_feature_counts.get("negative-parse-syntaxerror", 0) + 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                        continue
                    
                    if result["build_pass"]:
                        build_pass_count += 1
                        if result["semantic_pass"]:
                            semantic_pass_count += 1
                        elif result["mismatch"]:
                            mismatch_count += 1
                        elif result["runtime_error"]:
                            runtime_error_count += 1
                        elif result["blocked"]:
                            blocked_count += 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                        continue
                    
                    if result["blocked"]:
                        blocked_count += 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                        continue
                    
                    if result["fail"]:
                        fail_count += 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                        continue
                    
                    if result["unsupported"]:
                        unsupported_count += 1
                        diag_code = result["diag_code"]
                        feat = result["feature_label"]
                        unsupported_diag_counts[diag_code] = unsupported_diag_counts.get(diag_code, 0) + 1
                        unsupported_feature_counts[feat] = unsupported_feature_counts.get(feat, 0) + 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                        continue
    
        # Server cleanup
        if server_mode and server_proc:
            try:
                server_proc.stdin.write(json.dumps({"id": -1, "source": ""}).encode("utf-8") + b"\n")
                server_proc.stdin.flush()
            except (BrokenPipeError, OSError):
                pass  # Server already dead; nothing to clean up
            try:
                server_proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                server_proc.kill()
                server_proc.wait()
    
    # Build unsupported diagcodes string
    unsupported_diagcodes = ",".join(
        f"{code}:{count}" for code, count in 
        sorted(unsupported_diag_counts.items(), key=lambda x: (-x[1], x[0]))
    )
    
    # Build unsupported features string
    unsupported_features = ",".join(
        f"{feature}:{count}" for feature, count in 
        sorted(unsupported_feature_counts.items(), key=lambda x: (-x[1], x[0]))
    )
    
    coverage_percent = "0.00"
    if denominator > 0:
        coverage_percent = f"{(build_pass_count / denominator) * 100:.2f}"
    
    semantic_coverage_percent = "0.00"
    if denominator > 0:
        semantic_coverage_percent = f"{(semantic_pass_count / denominator) * 100:.2f}"

    summary = {
        "suite": suite,
        "suite_name": suite,
        "denominator": denominator,
        "executed": executed,
        "build_coverage_percent": coverage_percent,
        "semantic_coverage_percent": semantic_coverage_percent,
        "build_pass": build_pass_count,
        "semantic_pass": semantic_pass_count,
        "mismatch": mismatch_count,
        "runtime_error": runtime_error_count,
        "fail": fail_count,
        "unsupported": unsupported_count,
        "blocked": blocked_count,
        "skip_with_reason": skip_count,
        "duration_ms": int(round((time.perf_counter() - coverage_started_at) * 1000)),
        "unsupported_diagcodes": unsupported_diag_counts,
        "unsupported_features": unsupported_feature_counts,
        "status": "in-progress",
        "selection": {
            "paths_file": paths_file,
            "path_filters": path_filters,
        },
        "evidence": evidence,
    }

    if web_ui:
        write_coverage_result(summary)
        refresh_web_ui_data()
    if detail_output and suite_detail_rows:
        results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
        results_dir.mkdir(parents=True, exist_ok=True)
        results_path = results_dir / f"{suite}-results.jsonl"
        with results_path.open("w", encoding="utf-8") as handle:
            for record in suite_detail_rows:
                handle.write(json.dumps(record, sort_keys=True))
                handle.write("\n")

    if json_output:
        print(json.dumps(summary, indent=2))
    else:
        print(f"suite={suite}")
        print(f"denominator={denominator}")
        print(f"executed={executed}")
        print(f"coverage_percent={coverage_percent}")
        print(f"semantic_coverage_percent={semantic_coverage_percent}")
        print(f"build_pass={build_pass_count}")
        print(f"semantic_pass={semantic_pass_count}")
        print(f"mismatch={mismatch_count}")
        print(f"runtime_error={runtime_error_count}")
        print(f"fail={fail_count}")
        print(f"unsupported={unsupported_count}")
        print(f"blocked={blocked_count}")
        print(f"skip_with_reason={skip_count}")
        print(f"unsupported_diagcodes={unsupported_diagcodes}")
        print(f"unsupported_features={unsupported_features}")
        print(f"semantic_enabled={1 if semantic_enabled else 0}")
        
        if detail_output:
            print("\n# Per-file details")
            for detail in file_details:
                print(detail)

    # Auto-issue generation: pipe detail lines to gen-issues-from-coverage
    if auto_issues and detail_output:
        print("# Auto-generating issues from coverage details...", file=sys.stderr)
        detail_text = "\n".join(file_details) + "\n"
        try:
            gen_result = subprocess.run(
                [sys.executable, str(GEN_ISSUES_SCRIPT), "--suite", suite],
                input=detail_text,
                capture_output=True,
                text=True,
                cwd=REPO_ROOT,
            )
            if gen_result.stdout:
                print(gen_result.stdout, end="", file=sys.stderr)
            if gen_result.returncode != 0:
                print(f"# Issue generation stderr: {gen_result.stderr}", file=sys.stderr)
            else:
                # Update issue index after generation
                subprocess.run(
                    [sys.executable, str(UPDATE_ISSUE_INDEX_SCRIPT)],
                    capture_output=True,
                    cwd=REPO_ROOT,
                )
                print("# Issue index updated", file=sys.stderr)
        except Exception as e:
            print(f"# Auto-issue generation failed: {e}", file=sys.stderr)

if __name__ == "__main__":
    main()
