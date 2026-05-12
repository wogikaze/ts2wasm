#!/usr/bin/env python3
"""Validate fixtures/catalog.yaml against the filesystem and schema rules.

The fixture catalog is a YAML manifest of every .ts fixture file under
fixtures/.  Each entry must record the fixture's category, current test
status, and a short expected-behavior description.

Schema:
  version: int (required, must be 1)
  categories: dict[str, str]  (category → description)
  directories: dict[str, dir-entry]
    dir-entry:
      category: str (key in categories)
      status: str (one of: pass, fail, unsupported, blocked, skip, unknown)
      expected: str (one-line summary)
      fixtures:
        - name: str (filename)
          status: str (optional, overrides dir-level default)
          expected: str (optional, overrides dir-level default)

Rules:
  1. Every .ts file under fixtures/ must appear in exactly one directory entry.
  2. Every file in catalog.yaml must exist on disk.
  3. Catalog directories must match actual top-level fixture directories.
  4. All status values must be from the defined set.
  5. All category values must be keys in categories.

Usage:
  python3 scripts/check/fixture-catalog.py          # validate
  python3 scripts/check/fixture-catalog.py --help   # this message
"""

import sys
import yaml
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"
VALID_STATUSES = {"pass", "fail", "unsupported", "blocked", "skip", "unknown"}


def usage():
    print("Usage: python3 scripts/check/fixture-catalog.py")
    print()
    print("Validates fixtures/catalog.yaml against the filesystem.")
    print("Exit code 0 = OK, 1 = errors.")


def collect_fs_fixtures() -> dict[str, set[str]]:
    """Return {dir_name: {filename, ...}} from the filesystem."""
    fixtures_dir = REPO_ROOT / "fixtures"
    result: dict[str, set[str]] = {}
    for entry in sorted(fixtures_dir.iterdir()):
        if not entry.is_dir():
            continue
        names: set[str] = set()
        for f in entry.iterdir():
            if f.suffix == ".ts":
                names.add(f.name)
        if names:
            result[entry.name] = names
    return result


def validate_catalog(
    catalog: dict,
    fs_fixtures: dict[str, set[str]],
) -> int:
    errors = 0
    err = lambda msg: print(f"fixture-catalog: {msg}", file=sys.stderr)

    # -- version check --
    version = catalog.get("version")
    if version != 1:
        err(f"version must be 1, got {version!r}")
        errors += 1

    # -- categories check --
    categories = catalog.get("categories", {})
    if not isinstance(categories, dict):
        err("categories must be a dict")
        errors += 1
        categories = {}

    # -- directories check --
    dirs = catalog.get("directories", {})
    if not isinstance(dirs, dict):
        err("directories must be a dict")
        errors += 1
        dirs = {}

    # Check each catalog directory against filesystem
    for dir_name, dir_entry in dirs.items():
        if not isinstance(dir_entry, dict):
            err(f"directory {dir_name!r}: entry must be a dict")
            errors += 1
            continue

        cat = dir_entry.get("category", "")
        if cat not in categories:
            err(f"directory {dir_name!r}: unknown category {cat!r}")
            errors += 1

        status = dir_entry.get("status", "unknown")
        if status not in VALID_STATUSES:
            err(f"directory {dir_name!r}: invalid status {status!r}")
            errors += 1

        # Check each fixture in catalog
        fixtures = dir_entry.get("fixtures", [])
        if not isinstance(fixtures, list):
            err(f"directory {dir_name!r}: fixtures must be a list")
            errors += 1
            continue

        catalog_names = set()
        for fixture in fixtures:
            if isinstance(fixture, str):
                fname = fixture
                fstatus = status
                fexpected = dir_entry.get("expected", "")
            elif isinstance(fixture, dict):
                fname = fixture.get("name", "")
                fstatus = fixture.get("status", status)
                fexpected = fixture.get("expected", dir_entry.get("expected", ""))
            else:
                err(f"directory {dir_name!r}: invalid fixture entry type {type(fixture).__name__}")
                errors += 1
                continue

            if not fname:
                err(f"directory {dir_name!r}: fixture entry missing name")
                errors += 1
                continue

            catalog_names.add(fname)

            if fstatus not in VALID_STATUSES:
                err(f"directory {dir_name!r}/{fname}: invalid status {fstatus!r}")
                errors += 1

            if not fexpected:
                err(f"directory {dir_name!r}/{fname}: missing expected behavior description")
                errors += 1

        # Check for missing/extra files
        fs_names = fs_fixtures.get(dir_name, set())
        missing_from_catalog = fs_names - catalog_names
        extra_in_catalog = catalog_names - fs_names

        for fname in sorted(missing_from_catalog):
            err(f"directory {dir_name!r}/: fixture {fname!r} exists on disk but is not in catalog")
            errors += 1

        for fname in sorted(extra_in_catalog):
            err(f"directory {dir_name!r}/: fixture {fname!r} is in catalog but does not exist on disk")
            errors += 1

    # Check for directories on disk that are missing from catalog
    catalog_dirs = set(dirs.keys())
    fs_dirs = set(fs_fixtures.keys())
    for dir_name in sorted(fs_dirs - catalog_dirs):
        err(f"directory {dir_name!r}/: exists on disk but is not in catalog")
        errors += 1

    return errors


def main():
    args = sys.argv[1:]

    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)

    if not CATALOG_PATH.exists():
        print(f"fixture-catalog: missing {CATALOG_PATH}", file=sys.stderr)
        sys.exit(1)

    with open(CATALOG_PATH) as f:
        try:
            catalog = yaml.safe_load(f)
        except yaml.YAMLError as e:
            print(f"fixture-catalog: YAML parse error: {e}", file=sys.stderr)
            sys.exit(1)

    if not isinstance(catalog, dict):
        print("fixture-catalog: catalog must be a top-level mapping", file=sys.stderr)
        sys.exit(1)

    fs_fixtures = collect_fs_fixtures()
    errors = validate_catalog(catalog, fs_fixtures)

    if errors:
        print(f"fixture-catalog: FAILED ({errors} error(s))", file=sys.stderr)
        sys.exit(1)

    print("fixture-catalog: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
