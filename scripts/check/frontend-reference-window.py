#!/usr/bin/env python3
"""Frontend reference window checker.

Runs reference-coverage using --paths-file for the frontend reference window
and validates outcomes against the allowed set.

Allowed outcomes:
  - semantic_match
  - build_only
  - verified_negative_compile
  - unsupported (only if frontend-tracked)

Not allowed:
  - internal_failure
  - unsupported without frontend tracking

Usage:
  python scripts/check/frontend-reference-window.py --self-test
  python scripts/check/frontend-reference-window.py --check
  python scripts/check/frontend-reference-window.py --dry-run
"""

import os
import sys
import json
import subprocess
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
WINDOW_PATH = REPO_ROOT / "scripts" / "data" / "frontend-reference-window.txt"

ALLOWED_OUTCOMES = {
    "semantic_match",
    "build_only",
    "verified_negative_compile",
    "unsupported",
}

# Frontend-tracked unsupported features (checked by feature prefix)
FRONTEND_FEATURE_PREFIXES = [
    "TypeScript",
    "frontend.",
    "parser.",
    "lexer.",
]

# Internal failure outcomes
INTERNAL_FAILURE = "internal_failure"


def parse_window_file(window_path: Path) -> list[str]:
    """Parse the reference window file, returning path lines."""
    if not window_path.exists():
        return []

    paths = []
    with open(window_path) as f:
        for line in f:
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            paths.append(stripped)
    return paths


def is_frontend_tracked(feature_str: str) -> bool:
    """Check if a feature string indicates frontend tracking."""
    if not feature_str:
        return False
    feature_lower = feature_str.lower()
    for prefix in FRONTEND_FEATURE_PREFIXES:
        if feature_lower.startswith(prefix.lower()):
            return True
    return False


def check_outcome(
    case_path: str,
    status: str,
    outcome_kind: str,
    diag_code: str,
    feature: str,
    tracking: str,
) -> list[str]:
    """Check if a single outcome is allowed. Returns violations."""
    violations = []

    if outcome_kind == INTERNAL_FAILURE:
        violations.append(
            f"{case_path}: internal_failure is not allowed"
        )
        return violations

    if outcome_kind not in ALLOWED_OUTCOMES:
        violations.append(
            f"{case_path}: outcome '{outcome_kind}' is not in allowed set {ALLOWED_OUTCOMES}"
        )
        return violations

    if outcome_kind == "unsupported" and not is_frontend_tracked(feature):
        violations.append(
            f"{case_path}: unsupported outcome without frontend tracking (feature='{feature}')"
        )

    return violations


def check_jsonl(jsonl_path: Path) -> list[str]:
    """Check a JSONL file with reference coverage results."""
    violations = []
    if not jsonl_path.exists():
        return [f"JSONL file not found: {jsonl_path}"]

    with open(jsonl_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError as e:
                violations.append(f"invalid JSONL line: {e}")
                continue

            case_path = record.get("case", record.get("path", "unknown"))
            status = record.get("status", "")
            outcome_kind = record.get("outcome_kind", record.get("result", ""))
            diag_code = record.get("diag_code", "")
            feature = record.get("feature", "")
            tracking = record.get("tracking", "")

            violations.extend(
                check_outcome(case_path, status, outcome_kind, diag_code, feature, tracking)
            )

    return violations


def dry_run() -> list[str]:
    """Parse the window file and list paths without running coverage."""
    paths = parse_window_file(WINDOW_PATH)
    if not paths:
        return [f"window file empty or not found: {WINDOW_PATH}"]
    lines = [f"frontend-reference-window: {len(paths)} paths loaded"]
    for p in paths:
        lines.append(f"  {p}")
    return lines


def self_test() -> bool:
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test 1: parse window file
    paths = parse_window_file(WINDOW_PATH)
    if len(paths) == 50:
        passed += 1
    else:
        print(f"FAIL: expected 50 paths, got {len(paths)}", file=sys.stderr)
        failed += 1

    # Test 2: semantic_match passes
    violations = check_outcome("test.js", "pass", "semantic_match", "", "", "")
    if len(violations) == 0:
        passed += 1
    else:
        print(f"FAIL: semantic_match should be allowed: {violations}", file=sys.stderr)
        failed += 1

    # Test 3: internal_failure rejected
    violations = check_outcome("test.js", "fail", "internal_failure", "E001", "", "")
    if len(violations) == 1 and "internal_failure" in violations[0]:
        passed += 1
    else:
        print(f"FAIL: internal_failure should be rejected: {violations}", file=sys.stderr)
        failed += 1

    # Test 4: unsupported without frontend tracking rejected
    violations = check_outcome("test.js", "fail", "unsupported", "E002", "runtime.limit", "")
    if len(violations) >= 1:
        passed += 1
    else:
        print(f"FAIL: unsupported without frontend tracking should be rejected", file=sys.stderr)
        failed += 1

    # Test 5: unsupported with frontend tracking passes
    violations = check_outcome("test.js", "fail", "unsupported", "E003", "frontend.limit", "")
    if len(violations) == 0:
        passed += 1
    else:
        print(f"FAIL: unsupported with frontend tracking should be allowed: {violations}", file=sys.stderr)
        failed += 1

    # Test 6: check_outcome with unknown outcome
    violations = check_outcome("test.js", "fail", "unknown_outcome", "", "", "")
    if len(violations) >= 1:
        passed += 1
    else:
        print(f"FAIL: unknown outcome should be rejected", file=sys.stderr)
        failed += 1

    # Test 7: is_frontend_tracked detection
    if is_frontend_tracked("TypeScript.interface"):
        passed += 1
    else:
        print("FAIL: TypeScript prefix should be frontend-tracked", file=sys.stderr)
        failed += 1

    if is_frontend_tracked("runtime.anything"):
        print("FAIL: runtime prefix should not be frontend-tracked", file=sys.stderr)
        failed += 1
    else:
        passed += 1

    if is_frontend_tracked("frontend.boundary"):
        passed += 1
    else:
        print("FAIL: frontend prefix should be frontend-tracked", file=sys.stderr)
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

    if "--dry-run" in args:
        lines = dry_run()
        for line in lines:
            print(line, file=sys.stderr)
        if any("error" in l.lower() or "not found" in l.lower() for l in lines):
            sys.exit(1)
        sys.exit(0)

    if "--check" in args:
        # First validate the window file
        paths = parse_window_file(WINDOW_PATH)
        if not paths:
            print("frontend-reference-window: ERROR: reference window file is empty or missing. Prerequisite: REQ-REF-001 (reference corpus lockfile)", file=sys.stderr)
            sys.exit(1)

        # Check for reference corpus
        ref_root = REPO_ROOT / "reference"
        if not ref_root.exists():
            print("frontend-reference-window: ERROR: reference corpus not found. Prerequisite: REQ-REF-001", file=sys.stderr)
            sys.exit(1)

        violations = []

        # Check if user passed a JSONL file directly
        jsonl_path = None
        for arg in args:
            if arg.endswith(".jsonl"):
                jsonl_path = Path(arg)
                break

        if jsonl_path:
            violations = check_jsonl(jsonl_path)
        else:
            # Run reference-coverage with the paths file
            print("frontend-reference-window: running reference-coverage with --paths-file", file=sys.stderr)
            result = subprocess.run(
                ["python3", "scripts/manager.py", "reference-coverage", "test262",
                 "--jsonl", "--paths-file", str(WINDOW_PATH),
                 "--jobs", "4", "--no-dashboard-data"],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
            )
            if result.returncode != 0:
                violations.append(f"reference-coverage failed:\n{result.stderr[:500]}")
            else:
                # Check output
                violations.append(
                    "frontend-reference-window: coverage run completed. "
                    "Re-run with --check <output.jsonl> to validate outcomes."
                )

        if violations:
            for v in violations:
                print(f"frontend-reference-window: ERROR: {v}", file=sys.stderr)
            sys.exit(1)
        print("frontend-reference-window: OK")
        sys.exit(0)

    print(f"unknown option: {args[0]}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
