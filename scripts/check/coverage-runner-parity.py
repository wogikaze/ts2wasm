#!/usr/bin/env python3
"""Runner Parity Checker: compare server and legacy subprocess outcome classification.

Usage:
  python scripts/check/coverage-runner-parity.py --self-test
  python scripts/check/coverage-runner-parity.py --suite test262 --paths-file <paths-file>

The parity checker runs the same paths through both server-mode and legacy subprocess
mode, then compares the ``outcome``, ``status``, ``diagnostic_code``, ``feature``,
and ``tracking`` fields. Timing and log fields are ignored.

Exit 0 if no mismatches found, 1 if any field-level diff is detected.
"""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
COVERAGE_RESULTS_DIR = REPO_ROOT / "artifacts" / "coverage" / "results"


COMPARE_FIELDS = {"suite", "case", "status", "outcome", "diagnostic_code", "feature", "tracking"}
IGNORE_FIELDS = {"duration_ms", "stderr", "source_code", "expected", "actual", "reason"}
DEFAULT_PARITY_PATHS = REPO_ROOT / "scripts" / "data" / "coverage-parity-test262.txt"


def usage():
    print("Usage:")
    print("  python scripts/check/coverage-runner-parity.py --self-test")
    print("  python scripts/check/coverage-runner-parity.py --suite test262 --paths-file PATH")
    sys.exit(1)


def run_coverage(suite: str, paths_file: Path, server_mode: bool) -> list[dict]:
    """Run reference-coverage and return parsed JSONL records."""
    tmp_jsonl = Path(tempfile.mktemp(suffix=".jsonl"))
    try:
        cmd = [
            sys.executable,
            str(REPO_ROOT / "scripts" / "run" / "reference-coverage.py"),
            suite,
            "--paths-file", str(paths_file),
            "--jsonl",
            "--no-semantic",
            "--no-dashboard-data",
            "--jobs", "1",
        ]
        if not server_mode:
            cmd.append("--no-server")

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            cwd=REPO_ROOT,
            timeout=300,
        )

        if result.returncode != 0:
            print(f"ERROR: coverage run failed (server={server_mode}): {result.stderr[:200]}",
                  file=sys.stderr)
            return []

        # Find the JSONL output file
        jsonl_path = COVERAGE_RESULTS_DIR / f"{suite}-results.jsonl"
        if not jsonl_path.is_file():
            print(f"ERROR: JSONL results not found at {jsonl_path}", file=sys.stderr)
            return []

        records = []
        with open(jsonl_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    records.append(json.loads(line))
                except json.JSONDecodeError:
                    continue
        return records
    finally:
        tmp_jsonl.unlink(missing_ok=True)


def compare_records(server_records: list[dict], legacy_records: list[dict]) -> list[dict]:
    """Compare server and legacy records field by field.

    Returns a list of mismatch dicts with case and field-level diffs.
    """
    # Index by case path
    server_by_case = {r.get("case", ""): r for r in server_records}
    legacy_by_case = {r.get("case", ""): r for r in legacy_records}

    mismatches: list[dict] = []

    all_cases = sorted(set(list(server_by_case.keys()) + list(legacy_by_case.keys())))

    for case in all_cases:
        s = server_by_case.get(case, {})
        l = legacy_by_case.get(case, {})

        if not s and not l:
            continue

        diffs = {}
        for field in COMPARE_FIELDS:
            sv = s.get(field)
            lv = l.get(field)
            if sv != lv:
                diffs[field] = {"server": sv, "legacy": lv}

        if diffs:
            mismatches.append({
                "case": case,
                "diffs": diffs,
            })

    return mismatches


def run_parity_check(suite: str, paths_file: Path) -> int:
    """Compare server and legacy mode outcomes."""
    corpus_check = subprocess.run(
        [sys.executable, str(REPO_ROOT / "scripts/run/reference-corpus.py"), "verify", "--allow-missing-corpora"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if corpus_check.returncode != 0:
        print("ERROR: reference corpus is missing or inconsistent. Run reference-corpus verify first.",
              file=sys.stderr)
        return 1

    print(f"Running server mode...", file=sys.stderr)
    server_records = run_coverage(suite, paths_file, server_mode=True)

    print(f"Running legacy mode...", file=sys.stderr)
    legacy_records = run_coverage(suite, paths_file, server_mode=False)

    if not server_records and not legacy_records:
        print("ERROR: both server and legacy modes produced no records", file=sys.stderr)
        return 1

    mismatches = compare_records(server_records, legacy_records)

    if not mismatches:
        print(f"Parity check PASS: {len(server_records)} server records, "
              f"{len(legacy_records)} legacy records, 0 mismatches")
        return 0

    print(f"Parity check FAIL: {len(mismatches)} mismatches found", file=sys.stderr)
    for i, m in enumerate(mismatches[:10]):
        print(f"  Mismatch {i + 1}: {m['case']}", file=sys.stderr)
        for field, diff in m["diffs"].items():
            print(f"    {field}: server={diff['server']} vs legacy={diff['legacy']}",
                  file=sys.stderr)

    if len(mismatches) > 10:
        print(f"  ... and {len(mismatches) - 10} more mismatches", file=sys.stderr)

    return 1


def self_test() -> int:
    """Run self-test with synthetic JSONL mismatch data."""
    errors: list[str] = []

    # Test 1: identical records
    server = [
        {"case": "test/foo.js", "suite": "test262", "status": "pass", "outcome": "semantic_pass"},
    ]
    legacy = [
        {"case": "test/foo.js", "suite": "test262", "status": "pass", "outcome": "semantic_pass"},
    ]
    mismatches = compare_records(server, legacy)
    if len(mismatches) == 0:
        print("  Test 1 PASS: identical records produce no mismatches")
    else:
        errors.append(f"Test 1 FAIL: expected 0 mismatches, got {len(mismatches)}")

    # Test 2: different outcomes
    server2 = [
        {"case": "test/bar.js", "suite": "test262", "status": "pass", "outcome": "semantic_pass"},
    ]
    legacy2 = [
        {"case": "test/bar.js", "suite": "test262", "status": "unsupported", "outcome": "unsupported"},
    ]
    mismatches2 = compare_records(server2, legacy2)
    if len(mismatches2) == 1 and "outcome" in mismatches2[0]["diffs"]:
        print("  Test 2 PASS: different outcomes detected")
    else:
        errors.append(f"Test 2 FAIL: expected 1 mismatch, got {len(mismatches2)}")

    # Test 3: field-level diff detail
    s3 = {"case": "test/baz.js", "suite": "test262", "status": "pass", "outcome": "semantic_pass", "diagnostic_code": ""}
    l3 = {"case": "test/baz.js", "suite": "test262", "status": "unsupported", "outcome": "unsupported", "diagnostic_code": "UnsupportedSyntax"}
    mismatches3 = compare_records([s3], [l3])
    if len(mismatches3) == 1 and len(mismatches3[0]["diffs"]) >= 2:
        print("  Test 3 PASS: field-level diffs detected")
    else:
        errors.append(f"Test 3 FAIL: expected field-level diffs, got {mismatches3}")

    if errors:
        for e in errors:
            print(f"  FAIL: {e}", file=sys.stderr)
        return 1

    print("coverage-runner-parity: self-test OK (3 checks)")
    return 0


def main():
    args = sys.argv[1:]

    if not args or args[0] in ("-h", "--help"):
        usage()

    if args[0] == "--self-test":
        sys.exit(self_test())

    suite = None
    paths_file = None

    i = 0
    while i < len(args):
        if args[i] == "--suite":
            i += 1
            if i >= len(args):
                print("ERROR: --suite requires a value", file=sys.stderr)
                sys.exit(1)
            suite = args[i]
        elif args[i] == "--paths-file":
            i += 1
            if i >= len(args):
                print("ERROR: --paths-file requires a value", file=sys.stderr)
                sys.exit(1)
            paths_file = Path(args[i])
        else:
            print(f"ERROR: unknown option: {args[i]}", file=sys.stderr)
            sys.exit(1)
        i += 1

    if not suite:
        print("ERROR: --suite is required", file=sys.stderr)
        sys.exit(1)

    if not paths_file:
        paths_file = DEFAULT_PARITY_PATHS
        if not paths_file.is_file():
            print(f"ERROR: default parity paths file not found: {paths_file}", file=sys.stderr)
            sys.exit(1)

    sys.exit(run_parity_check(suite, paths_file))


if __name__ == "__main__":
    main()
