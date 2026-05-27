#!/usr/bin/env python3
"""Unified differential test runner: Node.js vs ts2wasm/iwasm.

Reads fixture catalog from fixtures/catalog.yaml and runs each differential
fixture through the test runner, collecting pass/fail results as JSONL
records on stdout.

JSONL Schema (same as crates/cli/tests/differential_jsonl.rs):
  suite:    str   - fixture directory, e.g. "fixtures/basics-hello"
  case:     str   - fixture filename, e.g. "hello.ts"
  target:   str   - target runtime, e.g. "wasm32-wasi"
  status:   str   - one of: pass, fail, unsupported, blocked, skip-with-reason
  expected: str?  - Node.js stdout (present on fail)
  actual:   str?  - iwasm stdout (present on fail)
  reason:   str?  - human-readable explanation
  tracking: str?  - tracking ID (on unsupported/blocked)

Usage:
  python3 scripts/check/fixture-differential.py            # run all fixtures
  python3 scripts/check/fixture-differential.py --smoke    # quick subset
  python3 scripts/check/fixture-differential.py --limit 20 # first 20 fixtures
  python3 scripts/check/fixture-differential.py --help     # this message

Dependencies: python3, pyyaml, node, iwasm, ts2wasm binary
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"
TS2WASM_BIN = REPO_ROOT / "target" / "debug" / "ts2wasm"

# Known-passing smoke fixtures
SMOKE_FIXTURES = [
    ("test-infrastructure", "pass-fixture.ts"),
    ("basics-hello", "hello.ts"),
    ("primitives-control-flow", "number.ts"),
    ("primitives-control-flow", "string.ts"),
    ("core-semantics", "null-undefined.ts"),
    ("primitives-control-flow", "boolean-if.ts"),
]

# Known-unsupported smoke fixtures (expected to produce unsupported status)
SMOKE_UNSUPPORTED_FIXTURES = [
    ("test-infrastructure", "unsupported-fixture.ts"),
]

IWASM_TIMEOUT_SECONDS = 30
IWASM_LINK_WARNING_RE = re.compile(
    r"^\[[^\]\r\n]+]: warning: failed to link import function \([^)]+\)$"
)
LIVE_TIME_FIXTURES = {
    "fixtures/builtins-and-io/date-noarg-live-time-unsupported.ts",
    "fixtures/builtins-and-io/date-noarg-live-time.ts",
    "fixtures/builtins-and-io/date-now-live-time-unsupported.ts",
    "fixtures/builtins-and-io/date-now-live-time.ts",
}
NONDETERMINISTIC_RANDOM_FIXTURES = {
    "fixtures/builtins-and-io/math-random.ts",
}
NONDETERMINISTIC_BUFFER_FIXTURES = {
    "fixtures/node-apis/crypto-random-bytes.ts",
}
HOST_ENVIRONMENT_FIXTURES = {
    "fixtures/node-apis/process-env.ts",
}
WASI_ARGV_FIXTURES = {
    "fixtures/node-apis/process-argv.ts",
}
FIXTURE_FILESYSTEM_INPUTS = {
    "fixtures/node-apis/fs-read.ts": {
        "input.txt": "fixture-input",
    },
    "fixtures/node-apis/wasi-fs-read-write.ts": {
        "input.txt": "fixture-input",
    },
}
CONSOLE_TIMER_FIXTURES = {
    "fixtures/builtins-and-io/console-complete.ts",
    "fixtures/builtins-and-io/console-supplementary.ts",
}
NODE_TS_EXTENSIONLESS_IMPORT_DIRS = {
    "fixtures/module-system",
    "fixtures/stmt",
}
NODE_TS_MODULE_SPECIFIER_RE = re.compile(
    r"(?P<prefix>\bfrom\s+[\"']|\bimport\s+[\"']|\bimport\s*\(\s*[\"'])"
    r"(?P<specifier>\./[^\"']+)"
    r"(?P<suffix>[\"'])"
)
CATALOG_ASSERTIONS: dict[str, dict] = {}
NODE_BASELINE_ORACLES = {
    "fixtures/builtins-and-io/bun-stdin-text.ts": {
        "stdin": "hello",
        "script": 'const s = require("fs").readFileSync(0, "utf8"); console.log(s);',
    },
}
EXPECTED_REJECTION_FIXTURES = {
    "fixtures/builtins-and-io/json-parse-incomplete-object.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-control-string-array.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-control-string-object.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-control-string.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-literal.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-exponent.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-fraction.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-incomplete-minus.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/json-parse-trailing-invalid.ts": {
        "node": ("syntaxerror", "json"),
        "iwasm": ("syntaxerror", "json.parse"),
    },
    "fixtures/builtins-and-io/proxy-reflect-unsupported-diagnostic.ts": {
        "node": ("typeerror", "constructor"),
        "iwasm": (),
    },
    "fixtures/core-semantics/bigint-builtin-unknown-invalid-string-runtime-trap.ts": {
        "node": ("syntaxerror", "bigint"),
        "iwasm": ("unreachable",),
    },
    "fixtures/core-semantics/bigint-mixed-arithmetic-typeerror-trap.ts": {
        "node": ("typeerror", "cannot mix bigint"),
        "iwasm": ("typeerror", "cannot mix bigint"),
    },
    "fixtures/core-semantics/bigint-runtime-div-zero-trap.ts": {
        "node": ("rangeerror", "division by zero"),
        "iwasm": ("rangeerror", "division by zero"),
    },
    "fixtures/core-semantics/bigint-runtime-mixed-typeerror-trap.ts": {
        "node": ("typeerror", "cannot mix bigint"),
        "iwasm": ("typeerror", "cannot mix bigint"),
    },
    "fixtures/core-semantics/bigint-runtime-mixed-object-toprimitive-string-unsupported.ts": {
        "node": ("typeerror", "cannot convert object to primitive value"),
        "iwasm": ("typeerror", "cannot convert object to primitive value"),
    },
    "fixtures/core-semantics/bigint-runtime-rem-zero-trap.ts": {
        "node": ("rangeerror", "division by zero"),
        "iwasm": ("rangeerror", "division by zero"),
    },
    "fixtures/core-semantics/nested-namespace-abc.ts": {
        "node": ("err_invalid_typescript_syntax", "namespace declaration"),
        "compiler": ("unsupportedmodule", "nested namespace/module resolution"),
    },
    "fixtures/core-semantics/nested-namespace-unsupported.ts": {
        "node": ("referenceerror", "a is not defined"),
        "compiler": ("unsupportedmodule", "nested namespace/module resolution"),
    },
    "fixtures/negative/unsupported-eval.ts": {
        "node": ("referenceerror", "x is not defined"),
        "compiler": ("unresolvedname", "unresolved name", "`x`"),
    },
}


def normalize_iwasm_stdout(stdout: str) -> str:
    """Remove runtime diagnostics that iwasm writes to stdout, not program output."""
    lines = stdout.splitlines(keepends=True)
    if not lines:
        return stdout
    return "".join(
        line
        for line in lines
        if not IWASM_LINK_WARNING_RE.match(line.rstrip("\r\n"))
    )


def host_epoch_ms() -> int:
    return time.time_ns() // 1_000_000


def random_stdout_in_unit_interval(stdout: str) -> bool:
    try:
        value = float(stdout.strip())
    except ValueError:
        return False
    return 0.0 <= value < 1.0


def stdout_is_crypto_random_buffer(stdout: str) -> bool:
    return (
        re.fullmatch(r"<Buffer(?: [0-9a-f]{2})*>\r?\n?", stdout) is not None
    )


def stdout_is_nonnegative_integer(stdout: str) -> bool:
    text = stdout.strip()
    return text.isdigit()


def stdout_is_environment_value(stdout: str) -> bool:
    text = stdout.rstrip("\n")
    return text == "undefined" or bool(text)


def normalize_console_timer_stdout(stdout: str) -> str:
    return re.sub(r": \d+(?:\.\d+)?ms", ": <elapsed>ms", stdout)


def catalog_assertion_for(fixture_path: str) -> dict | None:
    assertion = CATALOG_ASSERTIONS.get(fixture_path)
    if not isinstance(assertion, dict):
        return None
    return assertion


def fixture_stdin_bytes(fixture_path: str) -> bytes | None:
    assertion = catalog_assertion_for(fixture_path)
    if assertion and "stdin" in assertion:
        return str(assertion["stdin"]).encode()
    baseline = NODE_BASELINE_ORACLES.get(fixture_path)
    if baseline and "stdin" in baseline:
        return str(baseline["stdin"]).encode()
    return None


def fixture_expected_stdout(fixture_path: str) -> str | None:
    assertion = catalog_assertion_for(fixture_path)
    if (
        assertion
        and assertion.get("exit_code", 0) == 0
        and isinstance(assertion.get("stdout"), str)
    ):
        return assertion["stdout"]
    return None


def expected_rejection_for(fixture_path: str) -> dict | None:
    rejection = EXPECTED_REJECTION_FIXTURES.get(fixture_path)
    if isinstance(rejection, dict):
        return rejection
    return None


def text_contains_all(text: str, needles: tuple[str, ...]) -> bool:
    lower = text.lower()
    return all(needle in lower for needle in needles)


def run_node_oracle(
    abs_fixture_path: Path,
    fixture_path: str,
    stdin_data: bytes | None,
    cwd: Path | None = None,
):
    baseline = NODE_BASELINE_ORACLES.get(fixture_path)
    if baseline:
        cmd = ["node", "-e", str(baseline["script"])]
    else:
        cmd = ["node", str(abs_fixture_path)]
    return subprocess.run(
        cmd,
        input=stdin_data,
        capture_output=True,
        text=False,
        timeout=30,
        cwd=cwd,
    )


def create_fixture_workdir(fixture_path: str) -> tempfile.TemporaryDirectory | None:
    files = FIXTURE_FILESYSTEM_INPUTS.get(fixture_path)
    if not files:
        return None
    workdir = tempfile.TemporaryDirectory(prefix="ts2wasm-fixture-fs-")
    root = Path(workdir.name)
    for relative_path, contents in files.items():
        path = root / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
    return workdir


def create_node_ts_module_workdir(
    abs_fixture_path: Path,
    fixture_path: str,
) -> tuple[tempfile.TemporaryDirectory, Path] | None:
    suite_path = str(Path(fixture_path).parent)
    if suite_path not in NODE_TS_EXTENSIONLESS_IMPORT_DIRS:
        return None
    source_dir = abs_fixture_path.parent
    workdir = tempfile.TemporaryDirectory(prefix="ts2wasm-node-ts-mod-")
    root = Path(workdir.name)
    for source in source_dir.glob("*.ts"):
        text = source.read_text(encoding="utf-8")
        text = rewrite_node_ts_module_specifiers(text, source_dir)
        (root / source.name).write_text(text, encoding="utf-8")
    return workdir, root / abs_fixture_path.name


def rewrite_node_ts_module_specifiers(source: str, source_dir: Path) -> str:
    def replace(match: re.Match) -> str:
        specifier = match.group("specifier")
        if Path(specifier).suffix:
            return match.group(0)
        candidate = source_dir / f"{specifier}.ts"
        if not candidate.exists():
            return match.group(0)
        return f"{match.group('prefix')}{specifier}.ts{match.group('suffix')}"

    return NODE_TS_MODULE_SPECIFIER_RE.sub(replace, source)


def usage() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Differential test runner: Node.js vs ts2wasm/iwasm",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--smoke",
        action="store_true",
        help="Run a quick smoke subset instead of all fixtures",
    )
    parser.add_argument(
        "--ts2wasm",
        type=str,
        default=str(TS2WASM_BIN),
        help="Path to ts2wasm binary (default: target/debug/ts2wasm)",
    )
    parser.add_argument(
        "--catalog",
        type=str,
        default=str(CATALOG_PATH),
        help="Path to fixture catalog YAML (default: fixtures/catalog.yaml)",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=0,
        help="Limit number of fixtures to run (0 = unlimited)",
    )
    parser.add_argument(
        "--sample",
        type=int,
        default=0,
        help="Alias for --limit: sample N fixtures (0 = unlimited)",
    )
    parser.add_argument(
        "--jsonl",
        action="store_true",
        default=False,
        help="No-op: output is already JSONL (present for compatibility)",
    )
    parser.add_argument(
        "--iwasm-timeout",
        type=int,
        default=IWASM_TIMEOUT_SECONDS,
        help="iwasm timeout in seconds (default: 30)",
    )
    return parser.parse_args()


def get_cargo_target_dir() -> Path:
    """Get the cargo target directory via cargo metadata."""
    try:
        result = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            timeout=30,
        )
        if result.returncode == 0:
            import json
            metadata = json.loads(result.stdout)
            return Path(metadata.get("target_directory", ""))
    except Exception:
        pass
    return REPO_ROOT / "target"


def find_ts2wasm_binary(custom_path: str | None = None) -> str:
    """Locate the ts2wasm binary via explicit path, target dir, cargo build, or PATH."""
    if custom_path:
        p = Path(custom_path)
        if p.exists():
            return str(p.resolve())
    # Try default target dir first
    if TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
    # Try detecting actual cargo target directory (handles worktrees with shared target dir)
    cargo_target = get_cargo_target_dir()
    if cargo_target != REPO_ROOT / "target":
        alt_bin = cargo_target / "debug" / "ts2wasm"
        if alt_bin.exists():
            return str(alt_bin.resolve())
    print("fixture-differential: building ts2wasm...", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "build", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    if result.returncode == 0 and TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
    # Check cargo target dir after build
    cargo_target = get_cargo_target_dir()
    alt_bin = cargo_target / "debug" / "ts2wasm"
    if alt_bin.exists():
        return str(alt_bin.resolve())
    which = subprocess.run(["which", "ts2wasm"], capture_output=True, text=True)
    if which.returncode == 0:
        return which.stdout.strip()
    raise RuntimeError(
        "ts2wasm binary not found; build with: cargo build -p ts2wasm-cli"
    )


def check_required_tools():
    """Check that node and iwasm are available."""
    for tool in ["node", "iwasm"]:
        if subprocess.run(["which", tool], capture_output=True).returncode != 0:
            raise RuntimeError(f"required tool not found: {tool}")


def load_catalog(catalog_path: str) -> list[tuple[str, str, str]]:
    """Load fixture catalog and return list of (dir_name, filename, fixture_path)."""
    import yaml

    path = Path(catalog_path)
    if not path.exists():
        raise FileNotFoundError(f"catalog not found: {catalog_path}")

    with open(path) as f:
        catalog = yaml.safe_load(f)

    if not isinstance(catalog, dict):
        raise ValueError("catalog must be a top-level mapping")

    fixtures = []
    directories = catalog.get("directories", {})
    if not isinstance(directories, dict):
        raise ValueError("catalog.directories must be a dict")

    for dir_name, dir_entry in directories.items():
        if not isinstance(dir_entry, dict):
            continue
        fixture_list = dir_entry.get("fixtures", [])
        if not isinstance(fixture_list, list):
            continue
        for fixture in fixture_list:
            if isinstance(fixture, str):
                fname = fixture
            elif isinstance(fixture, dict):
                fname = fixture.get("name", "")
            else:
                continue
            if not fname:
                continue
            fixture_path = f"fixtures/{dir_name}/{fname}"
            fixtures.append((dir_name, fname, fixture_path))

    fixtures.sort()
    return fixtures


def load_catalog_assertions(catalog_path: str) -> dict[str, dict]:
    import yaml

    path = Path(catalog_path)
    if not path.exists():
        return {}
    with open(path) as f:
        catalog = yaml.safe_load(f)
    assertions = {}
    feature_fixtures = (
        (catalog or {})
        .get("feature_matrix", {})
        .get("fixtures", {})
    )
    if not isinstance(feature_fixtures, dict):
        return assertions
    for dir_name, entry in feature_fixtures.items():
        if not isinstance(entry, dict):
            continue
        fixture_assertions = entry.get("assert", {})
        if not isinstance(fixture_assertions, dict):
            continue
        for filename, assertion in fixture_assertions.items():
            if isinstance(assertion, dict):
                assertions[f"fixtures/{dir_name}/{filename}"] = assertion
    return assertions


def run_fixture(
    ts2wasm_bin: str,
    dir_name: str,
    filename: str,
    fixture_path: str,
    iwasm_timeout: int,
) -> dict:
    """Run a single fixture through the differential test pipeline.

    Returns a JSONL record dict matching the TestRecord schema.
    """
    abs_fixture_path = REPO_ROOT / fixture_path
    suite = f"fixtures/{dir_name}"
    case = filename
    target = "wasm32-wasi"
    stdin_data = fixture_stdin_bytes(fixture_path)

    # Step 1: Run Node.js to get expected output
    node_stdout = None
    node_error = None
    node_workdir = create_fixture_workdir(fixture_path)
    node_module_workdir = create_node_ts_module_workdir(abs_fixture_path, fixture_path)
    try:
        node_fixture_path = abs_fixture_path
        node_cwd = Path(node_workdir.name) if node_workdir else None
        if node_module_workdir:
            node_workdir, node_fixture_path = node_module_workdir
            node_cwd = Path(node_workdir.name)
        node_result = run_node_oracle(
            node_fixture_path,
            fixture_path,
            stdin_data,
            node_cwd,
        )
        if node_result.returncode == 0:
            node_stdout = node_result.stdout.decode("utf-8", errors="replace")
        else:
            node_error = node_result.stderr.decode("utf-8", errors="replace")
            node_stdout = fixture_expected_stdout(fixture_path)
    except (subprocess.TimeoutExpired, FileNotFoundError) as e:
        node_error = str(e)
        node_stdout = fixture_expected_stdout(fixture_path)
    finally:
        if node_workdir:
            node_workdir.cleanup()

    # Step 2: Build with ts2wasm
    wasm_fd, wasm_path = tempfile.mkstemp(suffix=".wasm", prefix="ts2wasm-")
    os.close(wasm_fd)

    try:
        build_result = subprocess.run(
            [ts2wasm_bin, "build", str(abs_fixture_path), "-o", wasm_path],
            capture_output=True,
            text=True,
            timeout=60,
        )
    except (subprocess.TimeoutExpired, FileNotFoundError) as e:
        os.unlink(wasm_path)
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "blocked",
            "expected": None,
            "actual": None,
            "reason": f"Build execution failed: {e}",
            "tracking": "feature:ts2wasm-unavailable",
        }

    if not build_result.returncode == 0:
        os.unlink(wasm_path)
        stderr = build_result.stderr
        diag_code = extract_diag_code(stderr)
        feature_label = feature_label_from_diag(diag_code, stderr, fixture_path)

        if diag_code == "BackendIo":
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "blocked",
                "expected": None,
                "actual": None,
                "reason": "I/O or command execution failure",
                "tracking": "feature:backend-io",
            }
        elif diag_code == "InvariantViolation":
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "fail",
                "expected": None,
                "actual": None,
                "reason": "Internal compiler bug",
                "tracking": "feature:invariant-violation",
            }
        expected_rejection = expected_rejection_for(fixture_path)
        if expected_rejection and node_error:
            node_needles = expected_rejection["node"]
            compiler_needles = expected_rejection.get("compiler")
            if compiler_needles and text_contains_all(node_error, node_needles) and text_contains_all(
                stderr, compiler_needles
            ):
                return {
                    "suite": suite,
                    "case": case,
                    "target": target,
                    "status": "pass",
                    "expected": None,
                    "actual": None,
                    "reason": None,
                    "tracking": None,
                }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "unsupported",
            "expected": None,
            "actual": None,
            "reason": f"Unsupported syntax: {diag_code}/{feature_label}",
            "tracking": f"feature:{feature_label}",
        }

    # Step 3: Run with iwasm
    iwasm_workdir = create_fixture_workdir(fixture_path)
    try:
        iwasm_started_at_ms = host_epoch_ms()
        iwasm_cmd = ["iwasm", wasm_path]
        iwasm_cwd = None
        if iwasm_workdir:
            iwasm_cmd = ["iwasm", "--dir=.", wasm_path]
            iwasm_cwd = Path(iwasm_workdir.name)
        iwasm_result = subprocess.run(
            iwasm_cmd,
            input=stdin_data,
            capture_output=True,
            timeout=iwasm_timeout,
            cwd=iwasm_cwd,
        )
        iwasm_finished_at_ms = host_epoch_ms()
    except subprocess.TimeoutExpired:
        if iwasm_workdir:
            iwasm_workdir.cleanup()
        os.unlink(wasm_path)
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": None,
            "actual": None,
            "reason": "iwasm timed out",
            "tracking": "feature:iwasm-timeout",
        }
    except FileNotFoundError:
        if iwasm_workdir:
            iwasm_workdir.cleanup()
        os.unlink(wasm_path)
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "blocked",
            "expected": None,
            "actual": None,
            "reason": "Failed to execute iwasm",
            "tracking": "feature:iwasm-unavailable",
        }
    finally:
        if iwasm_workdir:
            iwasm_workdir.cleanup()

    os.unlink(wasm_path)

    if iwasm_result.returncode != 0:
        if node_stdout is None:
            expected_rejection = expected_rejection_for(fixture_path)
            if expected_rejection:
                iwasm_output = (
                    iwasm_result.stdout.decode("utf-8", errors="replace")
                    + iwasm_result.stderr.decode("utf-8", errors="replace")
                )
                node_needles = expected_rejection["node"]
                iwasm_needles = expected_rejection["iwasm"]
                if text_contains_all(node_error or "", node_needles) and text_contains_all(
                    iwasm_output, iwasm_needles
                ):
                    return {
                        "suite": suite,
                        "case": case,
                        "target": target,
                        "status": "pass",
                        "expected": None,
                        "actual": None,
                        "reason": None,
                        "tracking": None,
                    }
                return {
                    "suite": suite,
                    "case": case,
                    "target": target,
                    "status": "fail",
                    "expected": (
                        "Node and iwasm rejection diagnostics containing "
                        f"node={node_needles}, iwasm={iwasm_needles}"
                    ),
                    "actual": (
                        f"node={node_error or ''!r}, "
                        f"iwasm={iwasm_output!r}"
                    ),
                    "reason": "expected rejection diagnostic mismatch",
                    "tracking": "feature:rejection-mismatch",
                }
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "blocked",
                "expected": None,
                "actual": None,
                "reason": f"Node oracle failed after successful build: {node_error or 'unknown error'}",
                "tracking": "feature:node-oracle-fail",
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": None,
            "actual": None,
            "reason": "iwasm execution failed",
            "tracking": "feature:iwasm-fail",
        }

    iwasm_stdout = normalize_iwasm_stdout(
        iwasm_result.stdout.decode("utf-8", errors="replace")
    )

    if node_stdout is None:
        expected_rejection = expected_rejection_for(fixture_path)
        if expected_rejection:
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "fail",
                "expected": "iwasm rejection matching Node rejection",
                "actual": iwasm_stdout,
                "reason": "iwasm accepted a fixture that Node rejected",
                "tracking": "feature:rejection-mismatch",
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "blocked",
            "expected": None,
            "actual": None,
            "reason": f"Node oracle failed after successful build: {node_error or 'unknown error'}",
            "tracking": "feature:node-oracle-fail",
        }

    # Step 4: Compare outputs
    if fixture_path in LIVE_TIME_FIXTURES:
        try:
            observed_epoch_ms = int(iwasm_stdout.strip())
        except ValueError:
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "fail",
                "expected": f"epoch milliseconds in host window {iwasm_started_at_ms}..={iwasm_finished_at_ms}",
                "actual": iwasm_stdout,
                "reason": f"expected epoch milliseconds, got {iwasm_stdout!r}",
                "tracking": "feature:stdout-mismatch",
            }
        if iwasm_started_at_ms <= observed_epoch_ms <= iwasm_finished_at_ms:
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": f"{iwasm_started_at_ms}..={iwasm_finished_at_ms}",
            "actual": iwasm_stdout,
            "reason": (
                f"timestamp outside host execution window: "
                f"observed={observed_epoch_ms}, "
                f"window={iwasm_started_at_ms}..={iwasm_finished_at_ms}"
            ),
            "tracking": "feature:stdout-mismatch",
        }

    if fixture_path in NONDETERMINISTIC_RANDOM_FIXTURES:
        if random_stdout_in_unit_interval(node_stdout) and random_stdout_in_unit_interval(
            iwasm_stdout
        ):
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": "Math.random stdout in [0, 1)",
            "actual": iwasm_stdout,
            "reason": (
                f"expected Node and iwasm random stdout in [0, 1), "
                f"node={node_stdout!r}, iwasm={iwasm_stdout!r}"
            ),
            "tracking": "feature:stdout-mismatch",
        }

    if fixture_path in NONDETERMINISTIC_BUFFER_FIXTURES:
        if stdout_is_crypto_random_buffer(node_stdout) and stdout_is_crypto_random_buffer(
            iwasm_stdout
        ):
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": "crypto.randomBytes stdout as <Buffer ..>",
            "actual": iwasm_stdout,
            "reason": (
                f"expected Node and iwasm crypto.randomBytes Buffer stdout, "
                f"node={node_stdout!r}, iwasm={iwasm_stdout!r}"
            ),
            "tracking": "feature:stdout-mismatch",
        }

    if fixture_path in WASI_ARGV_FIXTURES:
        if stdout_is_nonnegative_integer(node_stdout) and stdout_is_nonnegative_integer(
            iwasm_stdout
        ):
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": "process.argv.length stdout as non-negative integer",
            "actual": iwasm_stdout,
            "reason": (
                f"expected Node and iwasm process.argv.length stdout as integer, "
                f"node={node_stdout!r}, iwasm={iwasm_stdout!r}"
            ),
            "tracking": "feature:stdout-mismatch",
        }

    if fixture_path in HOST_ENVIRONMENT_FIXTURES:
        if stdout_is_environment_value(node_stdout) and stdout_is_environment_value(
            iwasm_stdout
        ):
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": "process.env stdout value or undefined",
            "actual": iwasm_stdout,
            "reason": (
                f"expected Node and iwasm process.env stdout as environment value, "
                f"node={node_stdout!r}, iwasm={iwasm_stdout!r}"
            ),
            "tracking": "feature:stdout-mismatch",
        }

    if fixture_path in CONSOLE_TIMER_FIXTURES:
        if normalize_console_timer_stdout(iwasm_stdout) == normalize_console_timer_stdout(
            node_stdout
        ):
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "pass",
                "expected": None,
                "actual": None,
                "reason": None,
                "tracking": None,
            }
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": normalize_console_timer_stdout(node_stdout),
            "actual": normalize_console_timer_stdout(iwasm_stdout),
            "reason": "stdout mismatch after console timer normalization",
            "tracking": "feature:stdout-mismatch",
        }

    if iwasm_stdout == node_stdout:
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "pass",
            "expected": None,
            "actual": None,
            "reason": None,
            "tracking": None,
        }
    else:
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "fail",
            "expected": node_stdout,
            "actual": iwasm_stdout,
            "reason": f"stdout mismatch: node={node_stdout!r}, iwasm={iwasm_stdout!r}",
            "tracking": "feature:stdout-mismatch",
        }


def extract_diag_code(stderr: str) -> str:
    """Extract diagnostic code from compiler stderr, e.g. [UnsupportedSyntax]
    or [DuplicateLocal/ast-validator] -> DuplicateLocal."""
    start = stderr.find("[")
    if start >= 0:
        end = stderr.find("]", start)
        if end >= 0:
            inner = stderr[start + 1 : end]
            # Strip phase suffix: "DuplicateLocal/ast-validator" -> "DuplicateLocal"
            return inner.split("/")[0]
    return "Unknown"


def feature_label_from_diag(diag_code: str, stderr: str, fixture_path: str) -> str:
    """Map a diagnostic code to a feature label."""
    static_labels = {
        "BackendIo": "backend-io",
        "InvariantViolation": "invariant-violation",
        "UnresolvedName": "name-resolution",
        "UnresolvedFunction": "function-resolution",
        "DuplicateFunction": "duplicate-function",
        "DuplicateLocal": "duplicate-local",
        "DuplicateParameter": "duplicate-parameter",
        "NumberOutOfRange": "number-range",
        "ArityMismatch": "arity",
        "InvalidTopLevelReturn": "top-level-return",
        "UnsupportedBuiltin": "unsupported-builtin",
        "UnsupportedDate": "unsupported-date",
        "UnsupportedRegExp": "unsupported-regexp",
        "UnsupportedModule": "unsupported-module",
        "UnsupportedEval": "unsupported-eval",
        "UnsupportedTypeScriptSyntax": "unsupported-ts-syntax",
        "UnsupportedRuntimeSubset": "unsupported-runtime-subset",
        "UnsupportedSyntax": "unsupported-syntax",
        "SyntaxError": "syntax-error",
        "TypeScriptTypeCheck": "typescript-type-check",
        "UnsupportedTarget": "unsupported-target",
    }
    if diag_code in static_labels:
        return static_labels[diag_code]

    text = stderr.lower()
    path = fixture_path.lower()

    if "/built-ins/date/" in path:
        return "date"
    if "/built-ins/function/" in path:
        return "function"
    if "/class/" in path or "/class-" in path or "class " in text:
        return "class"
    if "/module/" in path or "/import/" in path or "/export/" in path or " import " in text or " export " in text:
        return "import-export"
    if "/regexp/" in path or "regexp" in text:
        return "regexp-literal"
    if "/built-ins/string/" in path or "string.prototype" in text:
        return "string-builtin"
    if "/async" in path or " async " in text or "await " in text:
        return "async"
    if "/destructuring/" in path or "destructur" in text:
        return "destructuring"
    if "/template/" in path or "template" in text:
        return "template-literal"
    if "/arrow" in path or "=>" in text or "arrow" in text:
        return "arrow-function"
    if "/spread/" in path or "spread" in text:
        return "spread"
    if "non-ascii" in text or "utf-8" in text or "utf8" in text:
        return "utf8-string"
    if "binary operator" in text or "unary operator" in text:
        return "operator"
    if "kind: function" in text or "nested function" in text:
        return "function"
    if "expression type not yet supported" in text:
        return "unsupported-expression"
    if "expected " in text or "unsupported character" in text:
        return "parser-syntax"
    return "unknown-unsupported"


def validate_record(record: dict) -> list[str]:
    """Validate a JSONL record. Returns list of error messages (empty = valid)."""
    errors = []
    for field in ["suite", "case", "target", "status"]:
        if field not in record:
            errors.append(f"missing field: {field}")
    status = record.get("status", "")
    valid_statuses = {"pass", "fail", "unsupported", "blocked", "skip-with-reason"}
    if status not in valid_statuses:
        errors.append(f"invalid status: {status}")
    if status in ("unsupported", "blocked", "skip-with-reason"):
        if not record.get("reason"):
            errors.append(f"missing reason for {status}")
        if not record.get("tracking"):
            errors.append(f"missing tracking for {status}")
    return errors


def print_record(record: dict):
    """Print a JSONL record to stdout."""
    print(json.dumps(record, ensure_ascii=False))


def main():
    global CATALOG_ASSERTIONS
    args = usage()

    # Check required tools
    missing_tools = []
    if not subprocess.run(["which", "node"], capture_output=True).returncode == 0:
        missing_tools.append("node")
    if not subprocess.run(["which", "iwasm"], capture_output=True).returncode == 0:
        missing_tools.append("iwasm")
    if missing_tools:
        print(
            f"fixture-differential: missing required tools: {', '.join(missing_tools)}",
            file=sys.stderr,
        )
        sys.exit(1)

    # Find ts2wasm binary
    try:
        ts2wasm_bin = find_ts2wasm_binary(args.ts2wasm)
    except RuntimeError as e:
        print(f"fixture-differential: {e}", file=sys.stderr)
        sys.exit(1)

    print(
        f"fixture-differential: using ts2wasm: {ts2wasm_bin}",
        file=sys.stderr,
    )

    if args.smoke:
        # Smoke mode: known-passing + known-unsupported fixtures
        fixtures = [(d, f, f"fixtures/{d}/{f}") for d, f in
                     SMOKE_FIXTURES + SMOKE_UNSUPPORTED_FIXTURES]
        CATALOG_ASSERTIONS = load_catalog_assertions(args.catalog)
        print(
            f"fixture-differential: smoke mode: {len(fixtures)} fixtures",
            file=sys.stderr,
        )
    else:
        # Full mode: load from catalog
        try:
            fixtures = load_catalog(args.catalog)
            CATALOG_ASSERTIONS = load_catalog_assertions(args.catalog)
        except (FileNotFoundError, ValueError, ImportError) as e:
            print(f"fixture-differential: catalog error: {e}", file=sys.stderr)
            sys.exit(1)

        limit = args.limit or args.sample
        if limit > 0:
            fixtures = fixtures[:limit]
        print(
            f"fixture-differential: loaded {len(fixtures)} fixtures from catalog",
            file=sys.stderr,
        )

    # Run fixtures
    counts = {
        "pass": 0, "fail": 0, "unsupported": 0,
        "blocked": 0, "skip-with-reason": 0,
    }
    total = len(fixtures)
    start_time = time.time()

    for i, (dir_name, filename, fixture_path) in enumerate(fixtures, 1):
        record = run_fixture(
            ts2wasm_bin,
            dir_name,
            filename,
            fixture_path,
            args.iwasm_timeout,
        )

        # Validate and print
        validation_errors = validate_record(record)
        if validation_errors:
            print(
                f"fixture-differential: validation errors for {fixture_path}: "
                f"{'; '.join(validation_errors)}",
                file=sys.stderr,
            )

        print_record(record)
        status = record.get("status", "fail")
        counts[status] = counts.get(status, 0) + 1

        if i % 50 == 0 or i == total:
            elapsed = time.time() - start_time
            rate = i / elapsed if elapsed > 0 else 0
            print(
                f"fixture-differential: progress: {i}/{total} "
                f"({rate:.1f} fixtures/s)",
                file=sys.stderr,
            )

    # Summary
    elapsed = time.time() - start_time
    pass_pct = (counts["pass"] * 100) // max(total, 1)
    print(
        f"fixture-differential: summary: "
        f"pass={counts['pass']}({pass_pct}%) "
        f"fail={counts['fail']} "
        f"unsupported={counts['unsupported']} "
        f"blocked={counts['blocked']} "
        f"total={total} "
        f"elapsed={elapsed:.1f}s",
        file=sys.stderr,
    )

    # Exit code logic:
    # - Any "fail" or "blocked" status means the gate failed
    # - "unsupported" is expected and does not fail
    has_errors = (counts.get("fail", 0) + counts.get("blocked", 0)) > 0
    if has_errors:
        print(
            "fixture-differential: FAILED: some fixtures have fail or blocked status",
            file=sys.stderr,
        )
        sys.exit(1)

    print("fixture-differential: PASSED", file=sys.stderr)


if __name__ == "__main__":
    main()
