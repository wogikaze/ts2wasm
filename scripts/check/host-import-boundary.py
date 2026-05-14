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
    "crates/runtime-catalog/",
    "crates/backend-wasm/src/capability_manifest.rs",
    "crates/backend-wasm/src/capability.rs",
    "crates/backend-wasm/tests/",
    "crates/cli/tests/",
    "crates/compiler/tests/",
    "crates/runtime-catalog/tests/",
    "crates/shared/src/capability.rs",
    "crates/shared/src/test_status.rs",
    "crates/runtime-abi/",
    "scripts/check/",
    "scripts/run/",
    "scripts/lib/",
    "crates/shared/src/abi.rs",
    "artifacts/",
    "docs/",
    "site/docs/",
    "fixtures/",
    "reference/",
    "reports/",
    "site/",
    "plans/",
]

TARGET_STRING_ALLOWLIST = [
    "crates/shared/src/abi.rs",
    "crates/runtime-catalog/",
    "crates/backend-wasm/src/capability_manifest.rs",
    "crates/backend-wasm/src/manifest.rs",
    "crates/backend-wasm/tests/",
    "crates/compiler/tests/",
    "crates/runtime-catalog/tests/",
    "crates/shared/src/capability.rs",
    "crates/shared/src/test_status.rs",
    "crates/cli/tests/",
    "scripts/check/",
    "scripts/run/",
    "scripts/lib/",
    "artifacts/",
    "docs/",
    "site/docs/",
    "fixtures/",
    "reference/",
    "reports/",
    "site/",
    "plans/",
]

HOST_IMPORT_PATTERN = re.compile(r'(?<!\w)"host\.')
WASM32_TARGET_PATTERN = re.compile(r'"wasm32-')


def is_allowlisted(file_path: Path, allowlist) -> bool:
    rel = file_path.relative_to(REPO_ROOT).as_posix()
    for entry in allowlist:
        if rel == entry:
            return True
        if entry.endswith("/") and (rel.startswith(entry) or rel == entry.rstrip("/")):
            return True
    return False


def check_files(directory: str, pattern, allowlist) -> list:
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
        if "__pycache__" in fpath.parts or "node_modules" in fpath.parts or "target" in fpath.parts:
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
    errors = []
    if not HOST_IMPORT_PATTERN.search('"host.fs.readFileSync"'):
        errors.append("host pattern: failed to detect 'host.' string")
    if not WASM32_TARGET_PATTERN.search('"wasm32-wasi"'):
        errors.append("target pattern: failed to detect 'wasm32-' string")
    test_al = ["crates/runtime-catalog/src/host_import.rs", "crates/backend-wasm/tests/"]
    if not is_allowlisted(REPO_ROOT / "crates/runtime-catalog/src/host_import.rs", test_al):
        errors.append("allowlist: did not match exact path")
    if not is_allowlisted(REPO_ROOT / "crates/backend-wasm/tests/host_import_capability.rs", test_al):
        errors.append("allowlist: did not match directory prefix")
    if is_allowlisted(REPO_ROOT / "crates/ir/src/lowered/resolver.rs", test_al):
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
    elif args.list_allowlist:
        for name, al in [("Host string allowlist", HOST_STRING_ALLOWLIST),
                         ("Target string allowlist", TARGET_STRING_ALLOWLIST)]:
            print(f"{name}:")
            for e in sorted(al):
                print(f"  {e}")
    elif args.check:
        all_ok = True
        violations = check_files(".", HOST_IMPORT_PATTERN, HOST_STRING_ALLOWLIST)
        if violations:
            print("ERROR: Raw 'host.' import strings outside allowlist:")
            for v in violations:
                print(v)
            all_ok = False
        else:
            print("OK: No raw 'host.' import strings outside allowlist.")
        violations2 = check_files(".", WASM32_TARGET_PATTERN, TARGET_STRING_ALLOWLIST)
        if violations2:
            print("ERROR: Raw 'wasm32-' target strings outside allowlist:")
            for v in violations2:
                print(v)
            all_ok = False
        else:
            print("OK: No raw 'wasm32-' target strings outside allowlist.")
        if not all_ok:
            sys.exit(1)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
