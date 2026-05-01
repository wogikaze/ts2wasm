#!/usr/bin/env python3
"""Reference suite coverage measurement

Usage:
  python scripts/manager.py reference-coverage <suite> [--limit N] [--json] [--detail]
      [--paths-file PATH] [--path-filter TEXT] [--web-ui]
      [--jsonl] [--jobs N] [--sample N] [--category PATTERN]

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
  - --web-ui: refresh web-ui/public/data after writing this suite coverage result
  - --jsonl: output results as JSONL (test262 only, enables full harness with parallel exec)
  - --jobs N: number of parallel jobs (default: CPU count)
  - --sample N: max files per category (test262 only, uses category-based sampling)
  - --category PATTERN: regex filter for test categories (test262 only, used with --sample)
  - TS2WASM_REFERENCE_ROOT may point at an external reference/ directory for
    validation from isolated git worktrees.
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
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
import test262_harness as test262_runner

try:
    sys.path.insert(0, str(Path(__file__).parent.parent / "report"))
    from new_passes_notify import notify_new_passes
except ImportError:
    notify_new_passes = None

from ts2wasm_binary import resolve_ts2wasm_binary

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
TS2WASM_BINARY = resolve_ts2wasm_binary()
REFERENCE_ROOT = Path(os.environ.get("TS2WASM_REFERENCE_ROOT", REPO_ROOT / "reference")).resolve()
COVERAGE_RESULTS_DIR = REPO_ROOT / "artifacts" / "coverage" / "results"

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

def usage():
    print("Usage:")
    print("  python scripts/manager.py reference-coverage <suite> [--limit N] [--json] [--detail]")
    print("      [--paths-file PATH] [--path-filter TEXT] [--web-ui] [--no-web-ui]")
    print("      [--jsonl] [--jobs N] [--sample N] [--category PATTERN]")
    print()
    print("Suites:")
    print("  test262   -> reference/test262/test/**/*.js")
    print("  tsc       -> reference/typescript/tests/cases/compiler/**/*.ts")
    print("  tsgo      -> reference/typescript-go/testdata/tests/**")
    print()
    print("Flags:")
    print("  --jsonl      Output results as JSONL (test262 only, enables full harness with parallel exec)")
    print("  --jobs N     Number of parallel jobs (default: CPU count)")
    print("  --sample N   Max files per category (test262 only, uses category-based sampling)")
    print("  --category PATTERN  Regex filter for test categories (test262 only, used with --sample)")

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
    return test262_runner.HARNESS_DIR

def prepare_build_inputs(suite, file_path, tmp_dir):
    """Return source paths for wasm and Node execution."""
    if suite != "test262":
        return file_path, file_path

    source_code = file_path.read_text(encoding="utf-8")
    metadata = test262_runner.parse_test262_metadata(source_code)
    test262_runner.HARNESS_DIR = test262_harness_dir_for(file_path)
    wasm_source = tmp_dir / "test262-wasm-input.js"
    node_source = tmp_dir / "test262-node-input.js"
    wasm_source.write_text(
        test262_runner.build_test262_source(file_path, source_code, metadata, target="wasm"),
        encoding="utf-8",
    )
    node_source.write_text(
        test262_runner.build_test262_source(file_path, source_code, metadata, target="node"),
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
    sample = None
    category_pattern = None
    server_mode = False
    
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
        elif args[i] == "--web-ui":
            web_ui = True
            i += 1
        elif args[i] == "--no-web-ui":
            web_ui = False
            i += 1
        elif args[i] == "--jsonl":
            jsonl_output = True
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
        elif args[i] == "--server":
            server_mode = True
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

    executed = 0
    fail_count = 0
    unsupported_count = 0
    blocked_count = 0
    skip_count = 0
    
    unsupported_diag_counts = {}
    unsupported_feature_counts = {}
    
    semantic_enabled = bool(shutil.which("node") and shutil.which("iwasm"))
    
    if jobs is None:
        jobs = os.cpu_count() or 4
    
    # Server-mode setup
    use_server = server_mode
    server_proc = None
    
    if server_mode:
        print(f"Starting ts2wasm server (1 process, batch mode)...", file=sys.stderr)
        server_proc = subprocess.Popen(
            [str(TS2WASM_BINARY), "server"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=sys.stderr,
            cwd=REPO_ROOT,
        )
    
    def _run_semantic_check(file_path, source_code, metadata, thread_tmp, out_wasm, result_metrics):
        """Run node and iwasm for a build-pass file, updating result_metrics."""
        node_source = test262_runner.build_test262_source(
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
                thread_tmp = Path(tempfile.mkdtemp(dir=tmp_dir))
                try:
                    build_input = thread_tmp / "in.js"
                    build_input.write_text(item["build_source"], encoding="utf-8")
                    out_wasm = thread_tmp / "out.wasm"
                    
                    build_result = subprocess.run(
                        ["timeout", "8s", str(TS2WASM_BINARY), "build", str(build_input), "-o", str(out_wasm)],
                        capture_output=True,
                        cwd=REPO_ROOT
                    )
                    if build_result.returncode == 0:
                        _run_semantic_check(
                            item["file_path"], item["source_code"], item["metadata"],
                            thread_tmp, out_wasm, rm
                        )
                finally:
                    shutil.rmtree(thread_tmp, ignore_errors=True)
            
            if detail_output and not rm.get("detail_line"):
                rm["detail_line"] = f"{detail_path}: build_pass"
        else:
            diag_code = build_resp.get("code", "Unknown")
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
                rm["feature_label"] = feature_label(diag_code, None, str(item["file_path"]))
                if detail_output:
                    rm["detail_line"] = f"{detail_path}: {diag_code}: {rm['feature_label']}"
        
        return rm
    
    def _process_one_file(file_path):
        """Process a single file for coverage measurement. Thread-safe.
        
        In server mode, returns a pre-processed item dict for batch building.
        In subprocess mode, runs the full build and returns result_metrics.
        """
        if not file_path.is_file():
            return None
        
        detail_path = repo_relative(file_path)
        result_metrics = {
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
        
        # Check for expected negative parse syntax error
        negative_phase, negative_type = parse_test262_negative_metadata(source_code)
        if negative_phase == "parse" and negative_type == "SyntaxError":
            result_metrics["unsupported"] = True
            result_metrics["diag_code"] = "ExpectedNegativeSyntax"
            result_metrics["feature_label"] = "negative-parse-syntaxerror"
            if detail_output:
                result_metrics["detail_line"] = f"{detail_path}: ExpectedNegativeSyntax: negative-parse-syntaxerror"
            return result_metrics
        
        metadata = test262_runner.parse_test262_metadata(source_code)
        test262_runner.HARNESS_DIR = test262_harness_dir_for(file_path)
        build_source = test262_runner.build_test262_source(
            file_path, source_code, metadata, target="wasm"
        )
        
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
                ["timeout", "8s", str(TS2WASM_BINARY), "build", str(build_input), "-o", str(out_wasm)],
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
    
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_dir = Path(tmp_dir)
        
        if use_server:
            # === Server mode: parallel pre-process + batch build ===
            # Phase 1: Pre-process all files in parallel
            build_items = []
            early_results = []
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                for result in executor.map(_process_one_file, files):
                    if result is None:
                        continue
                    if isinstance(result, dict) and result.get("type") == "build_item":
                        result["id"] = id_counter[0]
                        id_counter[0] += 1
                        build_items.append(result)
                    else:
                        early_results.append(result)
            
            # Phase 2: Batch build via server
            batch_size = 500
            for i in range(0, len(build_items), batch_size):
                batch = build_items[i:i+batch_size]
                req = json.dumps({
                    "id": -1,
                    "items": [{"id": item["id"], "source": item["build_source"]} for item in batch]
                })
                server_proc.stdin.write(req.encode("utf-8") + b"\n")
                server_proc.stdin.flush()
                resp_line = server_proc.stdout.readline()
                if not resp_line:
                    # Server disconnected; mark remaining as blocked
                    for item in batch:
                        item["result_metrics"]["blocked"] = True
                    break
                build_results = json.loads(resp_line.decode("utf-8"))
                results_by_id = {r["id"]: r for r in build_results}
                
                for item in batch:
                    result = _classify_build_response(
                        results_by_id[item["id"]], item, semantic_enabled, tmp_dir
                    )
                    executed += 1
                    if result["unsupported"] and result["diag_code"] == "ExpectedNegativeSyntax":
                        unsupported_count += 1
                        unsupported_diag_counts["ExpectedNegativeSyntax"] = unsupported_diag_counts.get("ExpectedNegativeSyntax", 0) + 1
                        unsupported_feature_counts["negative-parse-syntaxerror"] = unsupported_feature_counts.get("negative-parse-syntaxerror", 0) + 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                    elif result["build_pass"]:
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
                    elif result["blocked"]:
                        blocked_count += 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                    elif result["fail"]:
                        fail_count += 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
                    elif result["unsupported"]:
                        unsupported_count += 1
                        diag_code = result["diag_code"]
                        feat = result["feature_label"]
                        unsupported_diag_counts[diag_code] = unsupported_diag_counts.get(diag_code, 0) + 1
                        unsupported_feature_counts[feat] = unsupported_feature_counts.get(feat, 0) + 1
                        if result["detail_line"]:
                            file_details.append(result["detail_line"])
            
            # Phase 3: Process early results (negative-parse-syntaxerror etc.)
            for result in early_results:
                executed += 1
                if result["unsupported"] and result["diag_code"] == "ExpectedNegativeSyntax":
                    unsupported_count += 1
                    unsupported_diag_counts["ExpectedNegativeSyntax"] = unsupported_diag_counts.get("ExpectedNegativeSyntax", 0) + 1
                    unsupported_feature_counts["negative-parse-syntaxerror"] = unsupported_feature_counts.get("negative-parse-syntaxerror", 0) + 1
                    if result["detail_line"]:
                        file_details.append(result["detail_line"])
        else:
            # === Legacy subprocess mode ===
            with ThreadPoolExecutor(max_workers=jobs) as executor:
                futures = {executor.submit(_process_one_file, f): f for f in files}
                for future in as_completed(futures):
                    result = future.result()
                    if result is None:
                        continue
                    executed += 1
                    
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
            server_proc.stdin.write(json.dumps({"id": -1, "source": ""}).encode("utf-8") + b"\n")
            server_proc.stdin.flush()
            try:
                server_proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                server_proc.kill()
                server_proc.wait()
    
    # JSONL output mode (test262 only, uses full harness with parallel execution)
    if jsonl_output and suite == "test262":
        if not files:
            print(f"No files selected for {suite}", file=sys.stderr)
            sys.exit(0)
        if jobs is None:
            jobs = os.cpu_count() or 4
        if jobs < 1:
            jobs = 1

        results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
        results_dir.mkdir(parents=True, exist_ok=True)
        jsonl_file = results_dir / f"{suite}-results.jsonl"

        passed = 0
        failed = 0
        unsupported = 0
        blocked = 0

        with tempfile.TemporaryDirectory() as tmp_dir:
            tmp_dir = Path(tmp_dir)
            with open(jsonl_file, 'w', encoding='utf-8') as jsonl_out:
                with ThreadPoolExecutor(max_workers=jobs) as executor:
                    futures = {executor.submit(test262_runner.process_one_test, f, tmp_dir, False): f for f in files}
                    completed = 0
                    total = len(files)
                    last_progress = 0

                    for future in as_completed(futures):
                        record, status = future.result()
                        if record:
                            jsonl_out.write(record + "\n")
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

        print(f"\n=== {suite} Summary ===", file=sys.stderr)
        print(f"Pass: {passed}", file=sys.stderr)
        print(f"Fail: {failed}", file=sys.stderr)
        print(f"Unsupported: {unsupported}", file=sys.stderr)
        print(f"Blocked: {blocked}", file=sys.stderr)
        print(f"Total: {passed + failed + unsupported + blocked}", file=sys.stderr)

        # Save summary files
        summary = {
            "suite": suite,
            "passed": passed,
            "failed": failed,
            "unsupported": unsupported,
            "blocked": blocked,
            "total": passed + failed + unsupported + blocked,
            "timestamp": datetime.now().isoformat(),
            "jsonl_file": str(jsonl_file),
        }
        summary_file = results_dir / f"{suite}-summary.json"
        summary_file.write_text(json.dumps(summary, indent=2), encoding="utf-8")

        # Legacy format
        legacy = dict(summary)
        legacy.pop("jsonl_file", None)
        legacy_file = results_dir / f"{suite}.json"
        legacy_file.write_text(json.dumps(legacy, indent=2), encoding="utf-8")

        # Notify new passes
        if notify_new_passes is not None:
            try:
                notify_new_passes(jsonl_file, suite=suite)
            except Exception as e:
                print(f"WARNING: notification failed: {e}", file=sys.stderr)

        # Web UI refresh
        if web_ui:
            refresh_web_ui_data()

        # Don't execute the sequential code path below
        return

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

if __name__ == "__main__":
    main()
