#!/usr/bin/env python3
"""Compare capability manifest "imports" with wasm module imports (wasm-tools print).

Usage:
  mise run check manifest -- [--fixture PATH.ts]
  mise run check manifest -- PATH.ts

Default fixture: fixtures/basics-hello/hello.ts

Fails if manifest import (module,name) pairs differ from wasm import section.
"""

import sys
import subprocess
import json
import tempfile
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

CORE_FIXTURES = [
    "fixtures/basics-hello/hello.ts",
    "fixtures/basics-types/types.ts",
    "fixtures/basics-equality/equality-operators.ts",
    "fixtures/basics-typeof/typeof-test.ts",
    "fixtures/basics-syntax/comma-expression.ts",
    "fixtures/core-semantics/number-stringify.ts",
]

# Standalone builtins-and-io fixtures that compile successfully.
# Each is checked for manifest import vs wasm import match.
BUILTINS_FIXTURES = [
    "fixtures/builtins-and-io/console-log.ts",
    "fixtures/builtins-and-io/math-floor.ts",
    "fixtures/builtins-and-io/math-random.ts",
    "fixtures/builtins-and-io/math-abs.ts",
    "fixtures/builtins-and-io/math-ceil.ts",
    "fixtures/builtins-and-io/math-max.ts",
    "fixtures/builtins-and-io/string-char-code-at.ts",
    "fixtures/builtins-and-io/string-at.ts",
    "fixtures/builtins-and-io/string-concat.ts",
    "fixtures/builtins-and-io/string-slice.ts",
    "fixtures/builtins-and-io/array-push.ts",
    "fixtures/builtins-and-io/array-slice.ts",
    "fixtures/builtins-and-io/array-concat.ts",
    "fixtures/builtins-and-io/array-every.ts",
    "fixtures/builtins-and-io/array-map.ts",
    "fixtures/builtins-and-io/array-reduce.ts",
    "fixtures/builtins-and-io/object-keys.ts",
    "fixtures/builtins-and-io/object-assign.ts",
    "fixtures/builtins-and-io/object-entries.ts",
    "fixtures/builtins-and-io/object-is.ts",
    "fixtures/builtins-and-io/json-stringify.ts",
    "fixtures/builtins-and-io/json-parse.ts",
    "fixtures/builtins-and-io/regexp-digit.ts",
    "fixtures/builtins-and-io/regexp-plus.ts",
    "fixtures/builtins-and-io/map-set.ts",
    "fixtures/builtins-and-io/set-size-clear.ts",
    "fixtures/builtins-and-io/error-message.ts",
    "fixtures/builtins-and-io/error-instanceof.ts",
    "fixtures/builtins-and-io/global-parseint.ts",
    "fixtures/builtins-and-io/global-isnan.ts",
    "fixtures/builtins-and-io/global-isfinite.ts",
    "fixtures/builtins-and-io/date-utc-getters.ts",
    "fixtures/builtins-and-io/date-epoch-get-time.ts",
    "fixtures/builtins-and-io/date-epoch-value-of.ts",
    "fixtures/builtins-and-io/value-of.ts",
]

# Fixtures expected to fail to build (unsupported diagnostics).
# These are checked for expected build failure only (no wasm/manifest comparison).
BUILD_FAIL_FIXTURES = [
    "fixtures/typescript-directives/triple-slash-reference-unsupported.ts",
    "fixtures/typescript-directives/reference-types-missing.ts",
    "fixtures/typescript-directives/reference-types-skip-lib-check.ts",
    "fixtures/typescript-directives/reference-types-ts-ignore.ts",
    "fixtures/typescript-directives/module-augmentation-unsupported.ts",
    "fixtures/typescript-directives/type-only-import-unsupported.ts",
]

def usage():
    print("Usage:")
    print("  mise run check manifest -- [--fixture PATH.ts]")
    print("  mise run check manifest -- PATH.ts")
    print("  mise run check manifest -- --all")
    print()
    print("A single path ending in .ts may be given without --fixture.")
    print("--all: check all CORE_FIXTURES.")
    print("Default (no args): check all CORE_FIXTURES.")
    print()
    print("Fails if manifest import (module,name) pairs differ from wasm import section.")

def check_fixture(fixture: str) -> int:
    """Check a single fixture. Returns 0 on success, 1 on failure."""
    return _check_single(fixture)

def _check_build_fail(fixture: str) -> int:
    """Check a fixture expected to fail to build.
    Returns 0 if build fails (expected), 1 if build succeeds (unexpected)."""
    fixture_path = REPO_ROOT / fixture
    if not fixture_path.exists():
        print(f"check_manifest_imports: fixture not found: {fixture}", file=sys.stderr)
        return 1

    print(f"check_manifest_imports: build (expect fail) {fixture}", file=sys.stderr)
    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        wasm_path = tmpd / "check.wasm"
        manifest_path = tmpd / "manifest.json"

        result = subprocess.run(
            ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", str(fixture_path),
             "-o", str(wasm_path), "--emit-manifest", str(manifest_path)],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            print(f"check_manifest_imports: expected build failure but build succeeded ({fixture})", file=sys.stderr)
            print(f"  stdout: {result.stdout[:200]}", file=sys.stderr)
            return 1

        print(f"check_manifest_imports: OK (expected failure) ({fixture})", file=sys.stderr)
        return 0

def _check_single(fixture: str) -> int:
    fixture_path = REPO_ROOT / fixture
    if not fixture_path.exists():
        print(f"check_manifest_imports: fixture not found: {fixture}", file=sys.stderr)
        return 1

    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        wasm_path = tmpd / "check.wasm"
        manifest_path = tmpd / "manifest.json"

        print(f"check_manifest_imports: build {fixture}", file=sys.stderr)
        result = subprocess.run(
            ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", str(fixture_path),
             "-o", str(wasm_path), "--emit-manifest", str(manifest_path)],
            cwd=REPO_ROOT
        )
        if result.returncode != 0:
            return result.returncode

        with open(manifest_path) as f:
            manifest = json.load(f)

        manifest_imports = set()
        if manifest.get("wasi", {}).get("stdout"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_write"))
        if manifest.get("wasi", {}).get("stdin"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_read"))
        if manifest.get("wasi", {}).get("stderr"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_write"))
        if manifest.get("wasi", {}).get("clock", {}).get("realtime"):
            manifest_imports.add(("wasi_snapshot_preview1", "clock_time_get"))
        if manifest.get("wasi", {}).get("random"):
            manifest_imports.add(("wasi_snapshot_preview1", "random_get"))
        manifest_imports.add(("wasi_snapshot_preview1", "proc_exit"))

        for imp in manifest.get("node_host", {}).get("imports", []):
            parts = imp.split(".")
            if len(parts) >= 2:
                manifest_imports.add((parts[0], parts[1]))

        result = subprocess.run(
            ["wasm-tools", "print", str(wasm_path)],
            capture_output=True,
            text=True,
            cwd=REPO_ROOT
        )
        if result.returncode != 0:
            print(f"check_manifest_imports: wasm-tools print failed", file=sys.stderr)
            return 1

        wasm_imports = set()
        for line in result.stdout.splitlines():
            import re
            match = re.search(r'\(import "([^"]*)" "([^"]*)"', line)
            if match:
                wasm_imports.add((match.group(1), match.group(2)))

        if manifest_imports != wasm_imports:
            print(f"check_manifest_imports: manifest imports != wasm imports ({fixture})", file=sys.stderr)
            print("--- manifest (module<tab>name) ---", file=sys.stderr)
            for module, name in sorted(manifest_imports):
                print(f"{module}\t{name}", file=sys.stderr)
            print("--- wasm ---", file=sys.stderr)
            for module, name in sorted(wasm_imports):
                print(f"{module}\t{name}", file=sys.stderr)
            return 1

        print(f"check_manifest_imports: OK ({fixture})", file=sys.stderr)
        return 0


def main():
    args = sys.argv[1:]

    fixtures = []  # empty means check all

    i = 0
    while i < len(args):
        if args[i] == "-h" or args[i] == "--help":
            usage()
            sys.exit(0)
        elif args[i] == "--all":
            fixtures = list(CORE_FIXTURES)
            i += 1
        elif args[i] == "--fixture":
            if i + 1 >= len(args):
                print("error: --fixture requires a path", file=sys.stderr)
                sys.exit(1)
            fixtures = [args[i + 1]]
            i += 2
        elif args[i].endswith(".ts") and not args[i].startswith("--"):
            fixtures = [args[i]]
            i += 1
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)

    # Check required commands
    for cmd in ["cargo", "wasm-tools"]:
        if not shutil.which(cmd):
            print(f"check_manifest_imports: missing required command: {cmd}", file=sys.stderr)
            sys.exit(1)

    exit_code = 0
    failures = 0
    total = 0

    # Run normal manifest checks
    if not fixtures:
        # Default run: check core + builtins + build-fail
        fixtures = list(CORE_FIXTURES) + list(BUILTINS_FIXTURES)
    elif fixtures == CORE_FIXTURES:
        # --all: also check builtins and build-fail
        fixtures = list(CORE_FIXTURES) + list(BUILTINS_FIXTURES)
    for fixture in fixtures:
        total += 1
        result = _check_single(fixture)
        if result != 0:
            exit_code = result
            failures += 1

    # Run build-fail checks (when running default set or --all)
    build_fail_fixtures = []
    if not fixtures or fixtures == (CORE_FIXTURES + BUILTINS_FIXTURES):
        build_fail_fixtures = list(BUILD_FAIL_FIXTURES)
    for fixture in build_fail_fixtures:
        total += 1
        result = _check_build_fail(fixture)
        if result != 0:
            exit_code = result
            failures += 1

    if exit_code != 0:
        print(f"check_manifest_imports: FAILED ({failures}/{total} failures)", file=sys.stderr)
        sys.exit(exit_code)

    print(f"check_manifest_imports: ALL OK ({total} fixtures)", file=sys.stderr)

if __name__ == "__main__":
    main()
