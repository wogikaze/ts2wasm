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
        "--iwasm-timeout",
        type=int,
        default=IWASM_TIMEOUT_SECONDS,
        help="iwasm timeout in seconds (default: 30)",
    )
    return parser.parse_args()


def find_ts2wasm_binary(custom_path: str | None = None) -> str:
    """Locate the ts2wasm binary via explicit path, target dir, cargo build, or PATH."""
    if custom_path:
        p = Path(custom_path)
        if p.exists():
            return str(p.resolve())
    if TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
    print("fixture-differential: building ts2wasm...", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "build", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    if result.returncode == 0 and TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
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

    # Step 1: Run Node.js to get expected output
    try:
        node_result = subprocess.run(
            ["node", str(abs_fixture_path)],
            capture_output=True,
            text=True,
            timeout=30,
        )
        if node_result.returncode != 0:
            return {
                "suite": suite,
                "case": case,
                "target": target,
                "status": "blocked",
                "expected": None,
                "actual": None,
                "reason": "Node oracle failed",
                "tracking": "feature:node-oracle-fail",
            }
        node_stdout = node_result.stdout
    except (subprocess.TimeoutExpired, FileNotFoundError) as e:
        return {
            "suite": suite,
            "case": case,
            "target": target,
            "status": "blocked",
            "expected": None,
            "actual": None,
            "reason": f"Node oracle unavailable: {e}",
            "tracking": "feature:node-oracle-fail",
        }

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
        else:
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
    try:
        iwasm_result = subprocess.run(
            ["iwasm", wasm_path],
            capture_output=True,
            text=True,
            timeout=iwasm_timeout,
        )
    except subprocess.TimeoutExpired:
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

    os.unlink(wasm_path)

    if iwasm_result.returncode != 0:
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

    iwasm_stdout = iwasm_result.stdout

    # Step 4: Compare outputs
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
    """Extract diagnostic code from compiler stderr, e.g. [UnsupportedSyntax]."""
    start = stderr.find("[")
    if start >= 0:
        end = stderr.find("]", start)
        if end >= 0:
            return stderr[start + 1 : end]
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
        print(
            f"fixture-differential: smoke mode: {len(fixtures)} fixtures",
            file=sys.stderr,
        )
    else:
        # Full mode: load from catalog
        try:
            fixtures = load_catalog(args.catalog)
        except (FileNotFoundError, ValueError, ImportError) as e:
            print(f"fixture-differential: catalog error: {e}", file=sys.stderr)
            sys.exit(1)

        limit = args.limit
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
