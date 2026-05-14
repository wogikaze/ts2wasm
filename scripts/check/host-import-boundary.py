#!/usr/bin/env python3
"""Host import and target string boundary checker.

Ensures raw 'host.' import strings and 'wasm32-' target strings appear only in
audited, allowlisted files. All other code must use HostImport::spec() and
ExecutionTarget types from the runtime catalog / shared crate.

Usage:
  python scripts/check/host-import-boundary.py --check     # check source files
  python scripts/check/host-import-boundary.py --self-test # run self-tests
  python scripts/check/host-import-boundary.py --list-allowlist  # show allowlisted files

Exits 0 on pass, 1 on violation.
"""

import sys
import re
import argparse
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

HOST_STRING_ALLOWLIST = [
    # Runtime catalog: single source of truth for host imports
    "crates/runtime-catalog/",
    # Capability manifest
    "crates/backend-wasm/src/capability_manifest.rs",
    "crates/backend-wasm/src/capability.rs",
    # Test snapshots and test files
    "crates/backend-wasm/tests/",
    "crates/cli/tests/",
    "crates/compiler/tests/",
    "crates/runtime-catalog/tests/",
    "crates/shared/src/capability.rs",
    "crates/shared/src/test_status.rs",
    # Runtime ABI
    "crates/runtime-abi/",
    # Check scripts
    "scripts/check/",
    # Reference coverage / triage
    "scripts/run/",
    "scripts/lib/",
    # Shared definitions and ABI
    "crates/shared/src/abi.rs",
    # Artifacts (baseline data)
    "artifacts/",
    # Documentation
    "docs/",
    "site/docs/",
    # Fixtures (test source files)
    "fixtures/",
    # Third-party reference code
    "reference/",
    # Generated reports
    "reports/",
    # Web UI source
    "site/",
    # Design plans
    "plans/",
]

TARGET_STRING_ALLOWLIST = [
    # ExecutionTarget canonical definition
    "crates/shared/src/abi.rs",
    # Runtime catalog (link plan, manifest)
    "crates/runtime-catalog/",
    # Backend-wasm manifest and tests
    "crates/backend-wasm/src/capability_manifest.rs",
    "crates/backend-wasm/src/manifest.rs",
    "crates/backend-wasm/tests/",
    # Test files
    "crates/compiler/tests/",
    "crates/runtime-catalog/tests/",
    "crates/shared/src/capability.rs",
    "crates/shared/src/test_status.rs",
    "crates/cli/tests/",
    # Architecture checks and scripts
    "scripts/check/",
    "scripts/run/",
    "scripts/lib/",
    # Baseline and artifacts
    "artifacts/",
    # Documentation
    "docs/",
    "site/docs/",
    # Fixtures
    "fixtures/",
    # Third-party reference
    "reference/",
    # Reports
    "reports/",
    # Web UI
    "site/",
    # Design plans
    "plans/",
]

HOST_IMPORT_PATTERN = re.compile(r'(?<!\w)"host\.')
WASM32_TARGET_PATTERN = re.compile(r'"wasm32-')


def is_allowlisted(file_path: Path, allowlist) -> bool:
    """Check if a file path matches any entry in the allowlist."""
    rel = file_path.relative_to(REPO_ROOT).as_posix()
    for entry in allowlist:
        if rel == entry:
            return True
        if entry.endswith("/") and (rel.startswith(entry) or rel == entry.rstrip("/")):
            return True
    return False


def check_files(directory: str, pattern, allowlist) -> list:
    """Scan source files for raw pattern strings outside the allowlist."""
    violations = []
    extensions = {".rs", ".py", ".md", ".toml", ".json", ".yaml", ".yml", ".ts"}

    src_dir = REPO_ROOT / directory
    if not src_dir.exists():
        return violations

    for fpath in sorted(src_dir.rglob("*")):
        if not fpath.is_file():
            continue
        if fpath.suffix not in extensions:
            continue
        if any(part.startswith(".") for part in fpath.parts):
            continue
        if "__pycache__" in fpath.parts:
            continue
        if "node_modules" in fpath.parts:
            continue
        if "target" in fpath.parts:
            continue

        if is_allowlisted(fpath, allowlist):
            continue

        try:
            content = fpath.read_text(errors="replace")
        except (OSError, UnicodeDecodeError):
            continue

        for lineno, line in enumerate(content.splitlines(), 1):
            if pattern.search(line):
                rel = fpath.relative_to(REPO_ROOT).as_posix()
                violations.append(f"  {rel}:{lineno}: {line.strip()[:100]}")

    return violations


def self_test():
    """Run built-in self-tests."""
    errors = []

    # Test 1: Host pattern should detect known violations
    test_content = '"host.fs.readFileSync"'
    if not HOST_IMPORT_PATTERN.search(test_content):
        errors.append("host pattern: failed to detect 'host.' string")

    # Test 2: Target pattern should detect known violations
    test_content2 = '"wasm32-wasi"'
    if not WASM32_TARGET_PATTERN.search(test_content2):
        errors.append("target pattern: failed to detect 'wasm32-' string")

    # Test 3: Allowlist matching
    test_allowlist = [
        "crates/runtime-catalog/src/host_import.rs",
        "crates/backend-wasm/tests/",
    ]
    test_path = REPO_ROOT / "crates/runtime-catalog/src/host_import.rs"
    if not is_allowlisted(test_path, test_allowlist):
        errors.append("allowlist: did not match exact path")

    test_path2 = REPO_ROOT / "crates/backend-wasm/tests/host_import_capability.rs"
    if not is_allowlisted(test_path2, test_allowlist):
        errors.append("allowlist: did not match directory prefix")

    test_path3 = REPO_ROOT / "crates/ir/src/lowered/resolver.rs"
    if is_allowlisted(test_path3, test_allowlist):
        errors.append("allowlist: incorrectly matched non-allowlisted path")

    if errors:
        print("SELF-TEST FAILED:")
        for e in errors:
            print(f"  - {e}")
        sys.exit(1)

    print("SELF-TEST PASSED")
    sys.exit(0)


def main():
    parser = argparse.ArgumentParser(description="Host import boundary checker")
    parser.add_argument("--check", action="store_true", help="Check source files")
    parser.add_argument("--self-test", action="store_true", help="Run self-tests")
    parser.add_argument("--list-allowlist", action="store_true", help="Show allowlisted files")
    args = parser.parse_args()

    if args.self_test:
        self_test()
        return

    if args.list_allowlist:
        print("Host string allowlist:")
        for entry in sorted(HOST_STRING_ALLOWLIST):
            print(f"  {entry}")
        print()
        print("Target string allowlist:")
        for entry in sorted(TARGET_STRING_ALLOWLIST):
            print(f"  {entry}")
        return

    if args.check:
        all_ok = True

        violations = check_files(".", HOST_IMPORT_PATTERN, HOST_STRING_ALLOWLIST)
        if violations:
            print("ERROR: Raw 'host.' import strings found outside allowlisted files:")
            for v in violations:
                print(v)
            all_ok = False
        else:
            print("OK: No raw 'host.' import strings outside allowlist.")

        violations2 = check_files(".", WASM32_TARGET_PATTERN, TARGET_STRING_ALLOWLIST)
        if violations2:
            print("ERROR: Raw 'wasm32-' target strings found outside allowlisted files:")
            for v in violations2:
                print(v)
            all_ok = False
        else:
            print("OK: No raw 'wasm32-' target strings outside allowlist.")

        if not all_ok:
            sys.exit(1)
        sys.exit(0)

    parser.print_help()


if __name__ == "__main__":
    main()
