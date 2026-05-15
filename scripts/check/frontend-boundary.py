#!/usr/bin/env python3
"""Frontend ownership boundary checker.

Validates that crates/frontend does not import runtime/backend symbols,
and that crates/ir/src/semantic.rs and HIR files do not contain WASM/WAT
instructions or raw import strings.

Usage:
  python scripts/check/frontend-boundary.py --self-test
  python scripts/check/frontend-boundary.py --check
"""

import os
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

FRONTEND_FORBIDDEN_PATTERNS = [
    "ts2wasm_runtime_abi",
    "ts2wasm_runtime_catalog",
    "RuntimeFn",
    "HostImport",
    "CapabilityManifest",
    "wasm32-",
    "host.",
]

IR_SEMANTIC_FORBIDDEN_PATTERNS = [
    "i32.load",
    "i32.store",
    "i32.add",
    "i32.sub",
    "i32.mul",
    "i32.div",
    "i32.const",
    "i64.load",
    "i64.store",
    "wasi_snapshot_preview1",
    '"host.',
    "'host.",
]

FRONTEND_DIR = REPO_ROOT / "crates" / "frontend" / "src"
IR_SEMANTIC_PATH = REPO_ROOT / "crates" / "ir" / "src" / "semantic.rs"


def scan_file_for_patterns(filepath: Path, patterns: list[str], label: str) -> list[str]:
    """Scan a file for forbidden patterns. Returns list of violations."""
    violations = []
    if not filepath.exists():
        return [f"{label}: file not found: {filepath}"]
    try:
        with open(filepath) as f:
            lines = f.readlines()
    except Exception as e:
        return [f"{label}: error reading {filepath}: {e}"]

    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("//") or stripped.startswith("#"):
            continue
        for pattern in patterns:
            if pattern in stripped:
                violations.append(
                    f"{label}:{i}: forbidden pattern '{pattern}' found"
                )
    return violations


def scan_frontend_src() -> list[str]:
    """Scan crates/frontend/src/*.rs for forbidden patterns."""
    violations = []
    if not FRONTEND_DIR.exists():
        return [f"frontend src dir not found: {FRONTEND_DIR}"]

    for rs_file in sorted(FRONTEND_DIR.rglob("*.rs")):
        rel_path = rs_file.relative_to(REPO_ROOT)
        violations.extend(
            scan_file_for_patterns(rs_file, FRONTEND_FORBIDDEN_PATTERNS, str(rel_path))
        )
    return violations


def scan_ir_semantic() -> list[str]:
    """Scan crates/ir/src/semantic.rs and HIR files for WASM/WAT patterns."""
    violations = []

    # Check semantic.rs
    if IR_SEMANTIC_PATH.exists():
        rel_path = IR_SEMANTIC_PATH.relative_to(REPO_ROOT)
        violations.extend(
            scan_file_for_patterns(IR_SEMANTIC_PATH, IR_SEMANTIC_FORBIDDEN_PATTERNS, str(rel_path))
        )

    # Check HIR files
    ir_hir_dir = REPO_ROOT / "crates" / "ir" / "src" / "lowered"
    if ir_hir_dir.exists():
        for rs_file in sorted(ir_hir_dir.rglob("*.rs")):
            rel_path = rs_file.relative_to(REPO_ROOT)
            violations.extend(
                scan_file_for_patterns(rs_file, IR_SEMANTIC_FORBIDDEN_PATTERNS, str(rel_path))
            )

    return violations


def check_all() -> list[str]:
    """Run all boundary checks. Returns list of violations."""
    violations = []
    violations.extend(scan_frontend_src())
    violations.extend(scan_ir_semantic())
    return violations


def self_test() -> bool:
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test 1: create a temp file with a forbidden pattern, verify detection
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("fn test() {\n    let x = ts2wasm_runtime_abi::foo();\n}\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file_for_patterns(tmp_path, FRONTEND_FORBIDDEN_PATTERNS, "test.rs")
        assert len(violations) == 1, f"expected 1 violation, got {len(violations)}"
        assert "ts2wasm_runtime_abi" in violations[0]
        passed += 1
    except Exception as e:
        print(f"FAIL: test forbidden pattern detection: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 2: create a temp file with no forbidden patterns
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("fn test() {\n    let x = 42;\n}\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file_for_patterns(tmp_path, FRONTEND_FORBIDDEN_PATTERNS, "test.rs")
        assert len(violations) == 0, f"expected 0 violations, got {len(violations)}"
        passed += 1
    except Exception as e:
        print(f"FAIL: test clean file: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 3: comment lines are ignored
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("// this comment mentions ts2wasm_runtime_abi but should be ignored\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file_for_patterns(tmp_path, FRONTEND_FORBIDDEN_PATTERNS, "test.rs")
        assert len(violations) == 0, f"expected 0 violations for comment, got {len(violations)}"
        passed += 1
    except Exception as e:
        print(f"FAIL: test comment skipping: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    print(f"self-test: {passed} passed, {failed} failed")
    return failed == 0


def main():
    args = sys.argv[1:]

    if not args or "--help" in args or "-h" in args:
        print(__doc__)
        sys.exit(0)

    if "--self-test" in args:
        if self_test():
            sys.exit(0)
        sys.exit(1)

    if "--check" in args:
        violations = check_all()
        if violations:
            for v in violations:
                print(f"frontend-boundary: ERROR: {v}", file=sys.stderr)
            sys.exit(1)
        print("frontend-boundary: OK")
        sys.exit(0)

    print(f"unknown option: {args[0]}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
