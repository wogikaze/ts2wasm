#!/usr/bin/env python3
"""Reference suite coverage measurement

Usage:
  python scripts/manager.py reference-coverage <suite> [--limit N] [--json] [--detail]
      [--paths-file PATH] [--path-filter TEXT] [--web-ui]

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
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
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
    print("      [--paths-file PATH] [--path-filter TEXT] [--web-ui]")
    print()
    print("Suites:")
    print("  test262   -> reference/test262/test/**/*.js")
    print("  tsc       -> reference/typescript/tests/cases/compiler/**/*.ts")
    print("  tsgo      -> reference/typescript-go/testdata/tests/**")

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
    web_ui = False
    
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
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    if suite not in SUITE_METADATA:
        print(f"unknown suite: {suite}", file=sys.stderr)
        usage()
        sys.exit(1)

    suite_config, files = resolve_suite_paths(suite, path_filters)
    if files is None:
        sys.exit(1)
    
    denominator = len(files)
    evidence = evidence_command(suite, limit, paths_file, path_filters)
    
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
        if web_ui:
            write_coverage_result(summary)
            refresh_web_ui_data()
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
    
    executed = 0
    fail_count = 0
    unsupported_count = 0
    blocked_count = 0
    skip_count = 0
    
    unsupported_diag_counts = {}
    unsupported_feature_counts = {}
    
    semantic_enabled = bool(shutil.which("node") and shutil.which("iwasm"))
    
    build_pass_count = 0
    semantic_pass_count = 0
    
    file_details = []
    
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_dir = Path(tmp_dir)
        
        for file_path in files:
            if not file_path.is_file():
                continue
            executed += 1
            detail_path = repo_relative(file_path)

            if is_expected_negative_parse_syntax_error(suite, file_path):
                unsupported_count += 1
                unsupported_diag_counts["ExpectedNegativeSyntax"] = unsupported_diag_counts.get("ExpectedNegativeSyntax", 0) + 1
                unsupported_feature_counts["negative-parse-syntaxerror"] = unsupported_feature_counts.get("negative-parse-syntaxerror", 0) + 1
                if detail_output:
                    file_details.append(f"{detail_path}: ExpectedNegativeSyntax: negative-parse-syntaxerror")
                continue
            
            out_wasm = tmp_dir / "out.wasm"
            err_file = tmp_dir / "err.txt"
            
            # Build with ts2wasm
            result = subprocess.run(
                ["timeout", "8s", "cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", 
                 str(file_path), "-o", str(out_wasm)],
                capture_output=True,
                cwd=REPO_ROOT
            )
            
            if result.returncode == 0:
                build_pass_count += 1
                
                if semantic_enabled:
                    node_out = tmp_dir / "node.out"
                    wasm_out = tmp_dir / "wasm.out"
                    
                    node_result = subprocess.run(
                        ["timeout", "8s", "node", str(file_path)],
                        capture_output=True,
                        cwd=REPO_ROOT
                    )
                    wasm_result = subprocess.run(
                        ["timeout", "8s", "iwasm", str(out_wasm)],
                        capture_output=True,
                        cwd=REPO_ROOT
                    )
                    
                    if (node_result.returncode == 0 and wasm_result.returncode == 0 and
                        node_result.stdout == wasm_result.stdout):
                        semantic_pass_count += 1
                
                if detail_output:
                    file_details.append(f"{detail_path}: build_pass")
                continue
            
            if result.returncode == 124:  # timeout
                blocked_count += 1
                if detail_output:
                    file_details.append(f"{detail_path}: blocked")
                continue
            
            # Extract diagnostic code
            err_content = result.stderr.decode('utf-8', errors='ignore')
            diag_match = re.search(r'\[([A-Za-z0-9_]+)\]', err_content)
            diag_code = diag_match.group(1) if diag_match else "Unknown"
            
            if diag_code == "BackendIo":
                blocked_count += 1
                if detail_output:
                    file_details.append(f"{detail_path}: blocked")
            elif diag_code == "InvariantViolation":
                fail_count += 1
                if detail_output:
                    file_details.append(f"{detail_path}: fail: InvariantViolation")
            else:
                unsupported_count += 1
                feature_label_result = feature_label(diag_code, err_content, str(file_path))
                unsupported_diag_counts[diag_code] = unsupported_diag_counts.get(diag_code, 0) + 1
                unsupported_feature_counts[feature_label_result] = unsupported_feature_counts.get(feature_label_result, 0) + 1
                if detail_output:
                    file_details.append(f"{detail_path}: {diag_code}: {feature_label_result}")
    
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
