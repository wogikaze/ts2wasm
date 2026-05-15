#!/usr/bin/env python3
"""HIR/MIR default switch gate checker.

Validates that docs/current-state.md contains an explicit "no-go" or "go" line
for HIR/MIR default mode. If "go", requires pass evidence for the semantic
canary suite, fncsem tests, hir_ tests, and reference coverage sample.

Usage:
  python scripts/check/hir-mir-default-gate.py --self-test
  python scripts/check/hir-mir-default-gate.py --check [path/to/current-state.md]
"""

import os
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_STATE_PATH = REPO_ROOT / "docs" / "current-state.md"

# Patterns to look for
GO_PATTERN = "HIR/MIR default: go"
NOGO_PATTERN = "HIR/MIR default: no-go"

# Evidence required when "go"
REQUIRED_EVIDENCE_WHEN_GO = [
    "semantic canary suite",
    "fncsem tests",
    "hir_ tests",
    "reference coverage sample",
]


def scan_current_state(state_path: Path) -> list[str]:
    """Scan current-state.md for HIR/MIR default status. Returns violations."""
    violations = []

    if not state_path.exists():
        return [f"current-state.md not found: {state_path}"]

    try:
        with open(state_path) as f:
            content = f.read()
    except Exception as e:
        return [f"error reading {state_path}: {e}"]

    lines = content.split("\n")
    found_status = False

    for i, line in enumerate(lines, 1):
        stripped = line.strip().lower()
        if GO_PATTERN.lower() in stripped:
            found_status = True
            # When "go", check for evidence
            evidence_lines = lines[i:]  # Check lines after the status
            evidence_text = "\n".join(evidence_lines).lower()
            for evidence in REQUIRED_EVIDENCE_WHEN_GO:
                if evidence not in evidence_text:
                    # Also check the full content for evidence mentions
                    if evidence not in content.lower():
                        violations.append(
                            f"HIR/MIR default is 'go' but missing evidence for: {evidence}"
                        )
        elif NOGO_PATTERN.lower() in stripped:
            found_status = True

    if not found_status:
        violations.append(
            f"no explicit HIR/MIR default status found (expected '{GO_PATTERN}' or '{NOGO_PATTERN}')"
        )

    return violations


def check() -> list[str]:
    """Run the HIR/MIR default gate check."""
    return scan_current_state(DEFAULT_STATE_PATH)


def self_test() -> bool:
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test 1: no-go status passes
    content1 = (
        "# Current State\n\n"
        "## HIR/MIR\n\n"
        "HIR/MIR default: no-go\n\n"
        "The HIR/MIR pipeline is disabled by default.\n"
    )
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write(content1)
        tmp_path = Path(f.name)

    try:
        violations = scan_current_state(tmp_path)
        assert len(violations) == 0, f"expected 0 violations for no-go, got {len(violations)}"
        passed += 1
    except Exception as e:
        print(f"FAIL: no-go status: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 2: go status without evidence fails
    content2 = (
        "# Current State\n\n"
        "## HIR/MIR\n\n"
        "HIR/MIR default: go\n\n"
    )
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write(content2)
        tmp_path = Path(f.name)

    try:
        violations = scan_current_state(tmp_path)
        # Should have violations for missing evidence
        assert len(violations) > 0, "expected violations for go without evidence"
        passed += 1
    except Exception as e:
        print(f"FAIL: go without evidence: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 3: missing status line fails
    content3 = (
        "# Current State\n\n"
        "This file has no HIR/MIR default line.\n"
    )
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write(content3)
        tmp_path = Path(f.name)

    try:
        violations = scan_current_state(tmp_path)
        assert len(violations) >= 1, "expected violations for missing status"
        passed += 1
    except Exception as e:
        print(f"FAIL: missing status: {e}", file=sys.stderr)
        failed += 1
    finally:
        tmp_path.unlink(missing_ok=True)

    # Test 4: file not found
    tmp_nonexistent = Path("/tmp/nonexistent-file-12345.md")
    try:
        violations = scan_current_state(tmp_nonexistent)
        assert len(violations) >= 1, "expected violations for missing file"
        passed += 1
    except Exception as e:
        print(f"FAIL: missing file: {e}", file=sys.stderr)
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
        # Optional: accept path to current-state.md
        state_path = DEFAULT_STATE_PATH
        for arg in args:
            if arg != "--check" and arg.endswith(".md"):
                state_path = Path(arg)
                break
        violations = scan_current_state(state_path)
        if violations:
            for v in violations:
                print(f"hir-mir-default-gate: ERROR: {v}", file=sys.stderr)
            sys.exit(1)
        print("hir-mir-default-gate: OK")
        sys.exit(0)

    print(f"unknown option: {args[0]}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
