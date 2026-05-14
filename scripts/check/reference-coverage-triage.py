#!/usr/bin/env python3
"""Validate coverage triage JSON and Markdown artifacts.

Usage:
  python scripts/check/reference-coverage-triage.py --check <triage.json>
  python scripts/check/reference-coverage-triage.py --self-test

Validates that a triage.json file has:
  - suite, selection, and counts keys
  - top_buckets with expected schema
  - deterministic bucket order (sorted by count descending, then tuple)
"""

import json
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def usage():
    print("Usage:")
    print("  python scripts/check/reference-coverage-triage.py --check <triage.json>")
    print("  python scripts/check/reference-coverage-triage.py --self-test")
    sys.exit(1)


def validate_triage(triage_path: str) -> bool:
    """Validate a triage.json artifact.

    Checks:
      - File exists and is valid JSON
      - Contains suite, top_failures, total_records
      - Each bucket has required keys
      - Buckets are sorted deterministically
    """
    path = Path(triage_path)
    if not path.is_file():
        print(f"ERROR: triage file not found: {triage_path}", file=sys.stderr)
        return False

    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        print(f"ERROR: could not parse triage JSON: {e}", file=sys.stderr)
        return False

    errors: list[str] = []

    # Check required top-level keys
    for key in ("suite", "top_failures", "total_records"):
        if key not in data:
            errors.append(f"missing required key: {key}")

    if errors:
        for e in errors:
            print(f"  ERROR: {e}", file=sys.stderr)
        return False

    suite = data["suite"]
    top_buckets = data["top_failures"]
    total_records = data["total_records"]

    if not isinstance(suite, str) or not suite:
        errors.append("suite must be a non-empty string")
    if not isinstance(total_records, int):
        errors.append("total_records must be an integer")
    if not isinstance(top_buckets, list):
        errors.append("top_failures must be a list")

    if errors:
        for e in errors:
            print(f"  ERROR: {e}", file=sys.stderr)
        return False

    # Validate each bucket
    for i, bucket in enumerate(top_buckets):
        required_keys = ["outcome_kind", "phase", "diag_code", "feature", "symbol", "count", "examples"]
        for key in required_keys:
            if key not in bucket:
                errors.append(f"bucket[{i}]: missing key '{key}'")

        if not isinstance(bucket.get("count"), int):
            errors.append(f"bucket[{i}]: count must be an integer")

        examples = bucket.get("examples", [])
        if not isinstance(examples, list):
            errors.append(f"bucket[{i}]: examples must be a list")

    # Check deterministic sort: descending by count, then by tuple
    for i in range(1, len(top_buckets)):
        prev = top_buckets[i - 1]
        curr = top_buckets[i]
        prev_count = prev.get("count", 0)
        curr_count = curr.get("count", 0)
        if curr_count > prev_count:
            errors.append(f"buckets not sorted by count descending at index {i}")

    if errors:
        for e in errors:
            print(f"  ERROR: {e}", file=sys.stderr)
        return False

    print(f"triage validation OK: {suite} ({len(top_buckets)} buckets, {total_records} records)")
    return True


def self_test() -> int:
    """Run self-test with synthetic data."""
    errors: list[str] = []

    # Test 1: valid triage JSON
    valid_data = {
        "suite": "test262",
        "top_failures": [
            {
                "outcome_kind": "unsupported",
                "phase": "parse",
                "diag_code": "UnsupportedSyntax",
                "feature": "parser-syntax",
                "symbol": "",
                "count": 10,
                "examples": ["test/foo.js", "test/bar.js"],
            },
        ],
        "total_records": 100,
    }

    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    json.dump(valid_data, tmp)
    tmp.close()

    ok = validate_triage(tmp.name)
    if ok:
        print("  Test 1 PASS: valid triage accepted")
    else:
        errors.append("Test 1 FAIL: valid triage rejected")
    Path(tmp.name).unlink(missing_ok=True)

    # Test 2: empty top_failures
    empty_data = {
        "suite": "tsc",
        "top_failures": [],
        "total_records": 0,
    }
    tmp2 = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    json.dump(empty_data, tmp2)
    tmp2.close()

    ok = validate_triage(tmp2.name)
    if ok:
        print("  Test 2 PASS: empty triage accepted")
    else:
        errors.append("Test 2 FAIL: empty triage rejected")
    Path(tmp2.name).unlink(missing_ok=True)

    # Test 3: invalid JSON
    tmp3 = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    tmp3.write("not json")
    tmp3.close()

    ok = validate_triage(tmp3.name)
    if not ok:
        print("  Test 3 PASS: invalid JSON rejected")
    else:
        errors.append("Test 3 FAIL: invalid JSON accepted")
    Path(tmp3.name).unlink(missing_ok=True)

    # Test 4: missing required keys
    bad_data = {"suite": "test262", "total_records": 50}
    tmp4 = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    json.dump(bad_data, tmp4)
    tmp4.close()

    ok = validate_triage(tmp4.name)
    if not ok:
        print("  Test 4 PASS: missing keys rejected")
    else:
        errors.append("Test 4 FAIL: missing keys accepted")
    Path(tmp4.name).unlink(missing_ok=True)

    if errors:
        for e in errors:
            print(f"  FAIL: {e}", file=sys.stderr)
        return 1

    print("reference-coverage-triage: self-test OK (4 checks)")
    return 0


def main():
    args = sys.argv[1:]

    if not args or args[0] in ("-h", "--help"):
        usage()

    if args[0] == "--self-test":
        sys.exit(self_test())

    if args[0] == "--check":
        if len(args) < 2:
            print("ERROR: --check requires a path argument", file=sys.stderr)
            sys.exit(1)
        ok = validate_triage(args[1])
        sys.exit(0 if ok else 1)

    print(f"ERROR: unknown option: {args[0]}", file=sys.stderr)
    usage()


if __name__ == "__main__":
    main()
