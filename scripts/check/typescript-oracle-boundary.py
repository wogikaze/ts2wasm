#!/usr/bin/env python3
"""TypeScript Oracle boundary isolation checker.

Scans production build path files and fails if they reference the TypeScript
oracle. Oracle calls are only allowed in:
  - crates/frontend/src/typescript_oracle.rs
  - CLI check command path (crates/cli/src/check*.rs)
  - Tests
  - scripts/check and scripts/run triage files

Usage:
  python scripts/check/typescript-oracle-boundary.py --self-test
  python scripts/check/typescript-oracle-boundary.py --check
"""

import os
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

# Production build path directories
PRODUCTION_DIRS = [
    "crates/compiler/src/pipeline.rs",
    "crates/compiler/src/stages",
    "crates/backend-wasm/src",
    "crates/ir/src",
]

# Forbidden oracle call patterns
FORBIDDEN_PATTERNS = [
    "collect_typescript_diagnostics",
    "check_typescript_file",
    "typescript-oracle.js",
    'Command::new("node")',
]

# Allowed paths (oracle usage permitted)
ALLOWED_PATHS = [
    "crates/frontend/src/typescript_oracle.rs",
]

# Allowed path prefixes (any file under these dirs)
ALLOWED_PREFIXES = [
    "crates/cli/src/check",
    "tests/",
    "scripts/check/",
    "scripts/run/",
]


def is_allowed(filepath: Path) -> bool:
    """Check if a file path is in the allowed list."""
    try:
        rel = str(filepath.relative_to(REPO_ROOT))
    except ValueError:
        # File is not under REPO_ROOT (e.g., temp file) — not allowed
        return False

    # Check exact allowed paths
    for allowed in ALLOWED_PATHS:
        if rel == allowed:
            return True

    # Check allowed prefixes
    for prefix in ALLOWED_PREFIXES:
        if rel.startswith(prefix):
            return True

    return False


def scan_file(filepath: Path, patterns: list[str]) -> list[str]:
    """Scan a single file for forbidden patterns. Returns violations."""
    violations = []
    if not filepath.exists():
        return []

    try:
        with open(filepath) as f:
            lines = f.readlines()
    except Exception as e:
        return [f"error reading {filepath}: {e}"]

    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("//") or stripped.startswith("#") or stripped.startswith("/*"):
            continue
        for pattern in patterns:
            if pattern in stripped:
                try:
                    display = filepath.relative_to(REPO_ROOT)
                except ValueError:
                    display = filepath
                violations.append(
                    f"{display}:{i}: forbidden oracle pattern '{pattern}'"
                )

    return violations


def scan_production_path() -> list[str]:
    """Scan production build path files for oracle calls."""
    violations = []

    for dir_spec in PRODUCTION_DIRS:
        path = REPO_ROOT / dir_spec
        if path.is_file():
            # Single file
            if not is_allowed(path):
                violations.extend(scan_file(path, FORBIDDEN_PATTERNS))
        elif path.is_dir():
            # Directory - scan all .rs files
            for rs_file in sorted(path.rglob("*.rs")):
                if not is_allowed(rs_file):
                    violations.extend(scan_file(rs_file, FORBIDDEN_PATTERNS))

    return violations


def check() -> list[str]:
    """Run the oracle boundary check."""
    return scan_production_path()


def self_test() -> bool:
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test 1: detect forbidden pattern in production path
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("fn compile() {\n    collect_typescript_diagnostics();\n}\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file(tmp_path, FORBIDDEN_PATTERNS)
        assert len(violations) == 1, f"expected 1 violation, got {len(violations)}"
        assert "collect_typescript_diagnostics" in violations[0]
        passed += 1
    except Exception as e:
        print(f"FAIL: detect forbidden pattern: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 2: clean file produces no violations
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("fn compile() {\n    let x = 42;\n}\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file(tmp_path, FORBIDDEN_PATTERNS)
        assert len(violations) == 0, f"expected 0 violations, got {len(violations)}"
        passed += 1
    except Exception as e:
        print(f"FAIL: clean file: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 3: comment lines are ignored
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("// This calls collect_typescript_diagnostics() but is a comment\n")
        tmp_path = Path(f.name)

    try:
        violations = scan_file(tmp_path, FORBIDDEN_PATTERNS)
        assert len(violations) == 0, f"expected 0 violations for comment, got {len(violations)}"
        passed += 1
    except Exception as e:
        print(f"FAIL: comment skipping: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 4: allowed path is not flagged
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as f:
        f.write("fn oracle_check() {\n    collect_typescript_diagnostics();\n}\n")
        tmp_path = Path(f.name)

    try:
        # This tests scan_file directly (path-independent)
        violations = scan_file(tmp_path, FORBIDDEN_PATTERNS)
        assert len(violations) == 1, "scan_file should detect patterns regardless of path"
        passed += 1
    except Exception as e:
        print(f"FAIL: scan_file detection: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 5: is_allowed correctly identifies allowed paths
    allowed_test_path = REPO_ROOT / "crates" / "frontend" / "src" / "typescript_oracle.rs"
    test_result = is_allowed(allowed_test_path)
    if test_result:
        passed += 1
    else:
        print(f"FAIL: is_allowed should return True for {allowed_test_path}", file=sys.stderr)
        failed += 1

    # Test 6: is_allowed correctly rejects production paths
    prod_test_path = REPO_ROOT / "crates" / "compiler" / "src" / "pipeline.rs"
    test_result = is_allowed(prod_test_path)
    if not test_result:
        passed += 1
    else:
        print(f"FAIL: is_allowed should return False for {prod_test_path}", file=sys.stderr)
        failed += 1

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
        violations = scan_production_path()
        if violations:
            for v in violations:
                print(f"typescript-oracle-boundary: ERROR: {v}", file=sys.stderr)
            sys.exit(1)
        print("typescript-oracle-boundary: OK")
        sys.exit(0)

    print(f"unknown option: {args[0]}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
