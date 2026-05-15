#!/usr/bin/env python3
"""Server/legacy coverage replay comparison.

Compares records from two JSONL files (server mode vs legacy subprocess mode)
by case path for status/outcome_kind/diag_code/feature/tracking fields.
Timing-only fields are ignored in comparison.

Usage:
  python scripts/check/coverage-replay-compare.py --server <server.jsonl> --legacy <legacy.jsonl>
  python scripts/check/coverage-replay-compare.py --self-test
"""

import os
import sys
import json
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

# Fields compared for equality
CLASSIFICATION_FIELDS = [
    "status",
    "outcome_kind",
    "result",
    "diag_code",
    "feature",
    "tracking",
]

# Fields ignored in comparison (timing, etc.)
IGNORED_FIELDS = [
    "duration_ms",
    "build_duration_ms",
    "semantic_duration_ms",
    "timestamp",
]

# Self-test data
SELF_TEST_SERVER_RECORDS = [
    {
        "case": "test262/test/language/comments/multi-line.js",
        "status": "pass",
        "outcome_kind": "semantic_match",
        "diag_code": "",
        "feature": "",
        "tracking": "",
    },
    {
        "case": "test262/test/language/identifiers/start-unicode-escape.js",
        "status": "fail",
        "outcome_kind": "unsupported",
        "diag_code": "E001",
        "feature": "frontend.parser",
        "tracking": "",
    },
]

SELF_TEST_LEGACY_RECORDS = [
    {
        "case": "test262/test/language/comments/multi-line.js",
        "status": "pass",
        "outcome_kind": "semantic_match",
        "diag_code": "",
        "feature": "",
        "tracking": "",
    },
    {
        "case": "test262/test/language/identifiers/start-unicode-escape.js",
        "status": "fail",
        "outcome_kind": "unsupported",
        "diag_code": "E001",
        "feature": "frontend.parser",
        "tracking": "",
    },
]

SELF_TEST_MISMATCH_LEGACY = [
    {
        "case": "test262/test/language/comments/multi-line.js",
        "status": "pass",
        "outcome_kind": "semantic_match",
        "diag_code": "",
        "feature": "",
        "tracking": "",
    },
    {
        "case": "test262/test/language/identifiers/start-unicode-escape.js",
        "status": "fail",
        "outcome_kind": "build_only",
        "diag_code": "",
        "feature": "",
        "tracking": "",
    },
]


def load_jsonl(jsonl_path: str) -> list[dict]:
    """Load JSONL file and return list of records."""
    path = Path(jsonl_path)
    if not path.exists():
        print(f"ERROR: file not found: {jsonl_path}", file=sys.stderr)
        sys.exit(1)

    records = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"ERROR: invalid JSONL in {jsonl_path}: {e}", file=sys.stderr)
                sys.exit(1)
            records.append(record)
    return records


def normalize_record(record: dict) -> dict:
    """Normalize a record for comparison: keep only classification fields."""
    normalized = {}
    for field in CLASSIFICATION_FIELDS:
        if field in record:
            normalized[field] = record[field]
        elif field == "outcome_kind" and "result" in record:
            # Some files use "result" instead of "outcome_kind"
            normalized["outcome_kind"] = record["result"]
        elif field == "result" and "outcome_kind" in record:
            # Some files use "outcome_kind" instead of "result"
            normalized["result"] = record["outcome_kind"]
        else:
            normalized[field] = ""
    return normalized


def compare_records(
    server_records: list[dict],
    legacy_records: list[dict],
) -> list[str]:
    """Compare server and legacy records by case path. Returns list of diff messages."""
    diffs = []

    # Index by case path
    server_by_path: dict[str, dict] = {}
    legacy_by_path: dict[str, dict] = {}

    for record in server_records:
        case_path = record.get("case", record.get("path", "unknown"))
        server_by_path[case_path] = normalize_record(record)

    for record in legacy_records:
        case_path = record.get("case", record.get("path", "unknown"))
        legacy_by_path[case_path] = normalize_record(record)

    # Check server records
    for case_path, server_norm in sorted(server_by_path.items()):
        if case_path not in legacy_by_path:
            diffs.append(f"MISSING_IN_LEGACY: {case_path}")
            continue

        legacy_norm = legacy_by_path[case_path]
        for field in CLASSIFICATION_FIELDS:
            s_val = server_norm.get(field, "")
            l_val = legacy_norm.get(field, "")
            if s_val != l_val:
                diffs.append(
                    f"DIFF: {case_path}: {field}: server='{s_val}' vs legacy='{l_val}'"
                )

    # Check legacy records not in server
    for case_path in sorted(legacy_by_path):
        if case_path not in server_by_path:
            diffs.append(f"MISSING_IN_SERVER: {case_path}")

    return diffs


def self_test() -> bool:
    """Run self-tests for the checker logic."""
    passed = 0
    failed = 0

    # Test 1: matching records produce no diffs
    diffs = compare_records(SELF_TEST_SERVER_RECORDS, SELF_TEST_LEGACY_RECORDS)
    if len(diffs) == 0:
        passed += 1
    else:
        print(f"FAIL: matching records should produce no diffs: {diffs}", file=sys.stderr)
        failed += 1

    # Test 2: mismatching records produce diffs
    diffs = compare_records(SELF_TEST_SERVER_RECORDS, SELF_TEST_MISMATCH_LEGACY)
    if len(diffs) >= 1 and any("DIFF:" in d and "identifiers" in d for d in diffs):
        passed += 1
    else:
        print(f"FAIL: mismatching records should produce diffs: {diffs}", file=sys.stderr)
        failed += 1

    # Test 3: normalize_record extracts correct fields
    record = {
        "case": "test.js",
        "status": "pass",
        "outcome_kind": "semantic_match",
        "diag_code": "",
        "feature": "",
        "tracking": "",
        "duration_ms": 123,
        "build_duration_ms": 45,
    }
    norm = normalize_record(record)
    if "duration_ms" not in norm and norm.get("status") == "pass":
        passed += 1
    else:
        print(f"FAIL: normalize_record should exclude timing fields: {norm}", file=sys.stderr)
        failed += 1

    # Test 4: loading JSONL files
    with tempfile.NamedTemporaryFile(mode="w", suffix=".jsonl", delete=False) as f:
        f.write(json.dumps({"case": "test.js", "status": "pass", "outcome_kind": "semantic_match"}) + "\n")
        tmp_path = Path(f.name)

    try:
        records = load_jsonl(str(tmp_path))
        assert len(records) == 1
        assert records[0]["case"] == "test.js"
        passed += 1
    except Exception as e:
        print(f"FAIL: load_jsonl: {e}", file=sys.stderr)
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

    server_path = None
    legacy_path = None

    i = 0
    while i < len(args):
        if args[i] == "--server":
            if i + 1 >= len(args):
                print("ERROR: --server requires a file path", file=sys.stderr)
                sys.exit(1)
            server_path = args[i + 1]
            i += 2
        elif args[i] == "--legacy":
            if i + 1 >= len(args):
                print("ERROR: --legacy requires a file path", file=sys.stderr)
                sys.exit(1)
            legacy_path = args[i + 1]
            i += 2
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            sys.exit(1)

    if not server_path or not legacy_path:
        print("ERROR: --server and --legacy are required", file=sys.stderr)
        sys.exit(1)

    server_records = load_jsonl(server_path)
    legacy_records = load_jsonl(legacy_path)

    diffs = compare_records(server_records, legacy_records)

    if diffs:
        for diff in diffs:
            print(f"coverage-replay-compare: {diff}", file=sys.stderr)
        print(f"coverage-replay-compare: FAILED ({len(diffs)} differences)", file=sys.stderr)
        sys.exit(1)

    total = len(server_records)
    print(f"coverage-replay-compare: OK ({total} records match)", file=sys.stderr)
    sys.exit(0)


if __name__ == "__main__":
    main()
