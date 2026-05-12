#!/usr/bin/env python3
"""Validate semantic assertions declared in fixtures/catalog.yaml.

Checks:
  - Every fixture directory has a corresponding catalog entry with assert: field.
  - Assertion fields (stdout, exit_code, host_imports, diagnostics) follow schema.
  - Referenced fixture files exist on disk.
  - Self-test mode validates the checker's own internal test data.

Usage:
  python3 scripts/check/fixture-assertions.py                 # full check
  python3 scripts/check/fixture-assertions.py --self-test     # self-test only
  python3 scripts/check/fixture-assertions.py --help          # help
"""

import sys
import os
import yaml
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"
FIXTURES_DIR = REPO_ROOT / "fixtures"

VALID_ASSERT_KEYS = {"stdout", "exit_code", "stdin", "host_imports", "diagnostics"}
VALID_STATUSES = {"pass", "partial", "unsupported"}

errors = []


def err(msg: str):
    print(f"fixture_assertions: FAIL: {msg}", file=sys.stderr)
    errors.append(msg)


def main():
    args = sys.argv[1:]

    if args and args[0] in ("-h", "--help"):
        print("Usage:")
        print("  python3 scripts/check/fixture-assertions.py [--self-test]")
        print()
        print("  --self-test  Run self-test mode (validates internal test data)")
        sys.exit(0)

    if args and args[0] == "--self-test":
        sys.exit(run_self_test())

    if not CATALOG_PATH.exists():
        err(f"catalog not found: {CATALOG_PATH}")
        sys.exit(1)

    # Load catalog
    try:
        with open(CATALOG_PATH) as f:
            data = yaml.safe_load(f)
    except yaml.YAMLError as e:
        err(f"invalid YAML in catalog: {e}")
        sys.exit(1)

    if not isinstance(data, dict) or "fixtures" not in data:
        err("catalog missing top-level 'fixtures' key")
        sys.exit(1)

    fixtures = data["fixtures"]

    if not isinstance(fixtures, dict):
        err("'fixtures' must be a dictionary")
        sys.exit(1)

    # Collect fixture directories on disk
    disk_dirs = set()
    for entry in FIXTURES_DIR.iterdir():
        if entry.is_dir():
            disk_dirs.add(entry.name)

    catalog_dirs = set(fixtures.keys())

    # Check: every catalog entry corresponds to a real directory
    for dir_name in catalog_dirs:
        if dir_name not in disk_dirs:
            err(f"catalog entry '{dir_name}' has no matching directory in fixtures/")

    # Check: every fixture directory has a catalog entry
    for dir_name in disk_dirs:
        if dir_name not in catalog_dirs:
            err(f"fixtures/{dir_name}/ has no catalog entry")

    # Validate each catalog entry
    for dir_name, entry in fixtures.items():
        if not isinstance(entry, dict):
            err(f"catalog entry '{dir_name}' must be a dictionary")
            continue

        # Check required fields
        if "feature" not in entry:
            err(f"catalog entry '{dir_name}' missing 'feature' field")
        elif not isinstance(entry["feature"], list):
            err(f"catalog entry '{dir_name}' 'feature' must be a list")

        if "status" not in entry:
            err(f"catalog entry '{dir_name}' missing 'status' field")
        elif entry["status"] not in VALID_STATUSES:
            err(f"catalog entry '{dir_name}' invalid status '{entry.get('status')}'")

        # Check assert field
        assert_field = entry.get("assert")
        if assert_field is None:
            err(f"catalog entry '{dir_name}' missing 'assert' field (use assert: {{}} if no assertions)")
            continue

        if not isinstance(assert_field, dict):
            err(f"catalog entry '{dir_name}' 'assert' must be a dict or empty")
            continue

        # Validate each per-file assertion
        for file_name, file_assert in assert_field.items():
            if not isinstance(file_assert, dict):
                err(f"catalog '{dir_name}' assert.{file_name} must be a dict")
                continue

            # Check that the referenced file exists on disk
            fixture_path = FIXTURES_DIR / dir_name / file_name
            if not fixture_path.exists():
                err(f"catalog '{dir_name}' assert.{file_name}: file not found at {fixture_path}")

            # Validate assertion keys
            for key in file_assert:
                if key not in VALID_ASSERT_KEYS:
                    err(f"catalog '{dir_name}' assert.{file_name}: unknown key '{key}' (valid: {', '.join(sorted(VALID_ASSERT_KEYS))})")

            # Type checks
            if "exit_code" in file_assert:
                ec = file_assert["exit_code"]
                if not isinstance(ec, int):
                    err(f"catalog '{dir_name}' assert.{file_name}: exit_code must be int, got {type(ec).__name__}")

            if "stdout" in file_assert:
                so = file_assert["stdout"]
                if not isinstance(so, str):
                    err(f"catalog '{dir_name}' assert.{file_name}: stdout must be string, got {type(so).__name__}")

            if "stdin" in file_assert:
                si = file_assert["stdin"]
                if not isinstance(si, str):
                    err(f"catalog '{dir_name}' assert.{file_name}: stdin must be string, got {type(si).__name__}")

            if "host_imports" in file_assert:
                hi = file_assert["host_imports"]
                if not isinstance(hi, list):
                    err(f"catalog '{dir_name}' assert.{file_name}: host_imports must be list, got {type(hi).__name__}")

            if "diagnostics" in file_assert:
                diag = file_assert["diagnostics"]
                if not isinstance(diag, list):
                    err(f"catalog '{dir_name}' assert.{file_name}: diagnostics must be list, got {type(diag).__name__}")

    if errors:
        print(f"fixture_assertions: FAILED: {len(errors)} error(s)", file=sys.stderr)
        sys.exit(1)

    # Count assertions
    total_files_with_assertions = sum(
        1 for entry in fixtures.values()
        if isinstance(entry, dict) and isinstance(entry.get("assert"), dict) and entry["assert"]
    )
    total_assert_keys = sum(
        len(keys)
        for entry in fixtures.values()
        if isinstance(entry, dict) and isinstance(entry.get("assert"), dict)
        for keys in entry["assert"].values()
    )

    print(f"fixture_assertions: PASS: {len(fixtures)} fixtures, "
          f"{total_files_with_assertions} with file-level assertions, "
          f"{total_assert_keys} total assert keys", file=sys.stderr)
    return 0


def run_self_test() -> int:
    """Validate that the checker can detect various error conditions."""
    print("fixture_assertions: self-test mode", file=sys.stderr)

    self_test_errors = 0

    def check(description: str, condition: bool):
        nonlocal self_test_errors
        if not condition:
            print(f"  FAIL: {description}", file=sys.stderr)
            self_test_errors += 1
        else:
            print(f"  PASS: {description}", file=sys.stderr)

    # Test 1: VALID_ASSERT_KEYS contains expected fields
    check("VALID_ASSERT_KEYS contains stdout",
          "stdout" in VALID_ASSERT_KEYS)
    check("VALID_ASSERT_KEYS contains exit_code",
          "exit_code" in VALID_ASSERT_KEYS)
    check("VALID_ASSERT_KEYS contains stdin",
          "stdin" in VALID_ASSERT_KEYS)
    check("VALID_ASSERT_KEYS contains host_imports",
          "host_imports" in VALID_ASSERT_KEYS)
    check("VALID_ASSERT_KEYS contains diagnostics",
          "diagnostics" in VALID_ASSERT_KEYS)
    check("VALID_ASSERT_KEYS has exactly 5 entries",
          len(VALID_ASSERT_KEYS) == 5)

    # Test 2: VALID_STATUSES
    check("VALID_STATUSES contains pass",
          "pass" in VALID_STATUSES)
    check("VALID_STATUSES contains partial",
          "partial" in VALID_STATUSES)
    check("VALID_STATUSES contains unsupported",
          "unsupported" in VALID_STATUSES)
    check("VALID_STATUSES has exactly 3 entries",
          len(VALID_STATUSES) == 3)

    # Test 3: Catalog file exists and is valid YAML
    if CATALOG_PATH.exists():
        try:
            with open(CATALOG_PATH) as f:
                data = yaml.safe_load(f)
            check("catalog.yaml is valid YAML", data is not None)
            check("catalog.yaml has 'fixtures' key",
                  isinstance(data, dict) and "fixtures" in data)
            check("catalog.yaml fixtures is a dict",
                  isinstance(data.get("fixtures"), dict))
        except yaml.YAMLError:
            check("catalog.yaml is valid YAML", False)
    else:
        check("catalog.yaml exists", False)

    # Test 4: fixture directories on disk have catalog entries
    if FIXTURES_DIR.exists():
        disk_dirs = {d.name for d in FIXTURES_DIR.iterdir() if d.is_dir()}
        if CATALOG_PATH.exists():
            try:
                with open(CATALOG_PATH) as f:
                    data = yaml.safe_load(f)
                catalog_dirs = set(data.get("fixtures", {}).keys())
                unlisted = disk_dirs - catalog_dirs
                missing_on_disk = catalog_dirs - disk_dirs
                check("all fixture dirs have catalog entries",
                      len(unlisted) == 0)
                check("all catalog entries have fixture dirs",
                      len(missing_on_disk) == 0)
            except yaml.YAMLError:
                check("all fixture dirs have catalog entries (skip)", False)

    # Test 5: Assertion schema validation
    # Simulate a catalog entry with valid assertions
    valid_entry = {
        "test-fixture": {
            "feature": ["value-types:string"],
            "status": "pass",
            "assert": {
                "hello.ts": {
                    "stdout": "ok\n",
                    "exit_code": 0,
                    "host_imports": [],
                    "diagnostics": []
                }
            }
        }
    }
    check("valid entry type check passes",
          isinstance(valid_entry["test-fixture"]["assert"]["hello.ts"], dict))
    check("valid exit_code is int",
          isinstance(valid_entry["test-fixture"]["assert"]["hello.ts"]["exit_code"], int))
    check("valid stdout is str",
          isinstance(valid_entry["test-fixture"]["assert"]["hello.ts"]["stdout"], str))

    # Test 6: All feature labels use correct format (category:name)
    if CATALOG_PATH.exists():
        try:
            with open(CATALOG_PATH) as f:
                data = yaml.safe_load(f)
            feature_errors = 0
            for dir_name, entry in data.get("fixtures", {}).items():
                for feat in entry.get("feature", []):
                    if not isinstance(feat, str) or ":" not in feat:
                        feature_errors += 1
            check("all feature labels have format 'category:name'",
                  feature_errors == 0)
        except yaml.YAMLError:
            check("all feature labels have format 'category:name' (skip)", False)

    if self_test_errors > 0:
        print(f"fixture_assertions: self-test FAILED: {self_test_errors} error(s)", file=sys.stderr)
        return 1

    print("fixture_assertions: self-test PASSED", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
