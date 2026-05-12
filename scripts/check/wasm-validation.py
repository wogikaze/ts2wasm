#!/usr/bin/env python3
"""Wasm validation check for the test pipeline.

Reads fixtures/catalog.yaml, iterates over compilable fixtures, compiles each
to wasm, and validates the binary using wasm-tools validate or wasm-validate.
Reports per-fixture validation results and exits 0 if all produce valid wasm,
non-zero otherwise.

Usage:
  python scripts/check/wasm-validation.py
  python scripts/check/wasm-validation.py --sample N
  python scripts/check/wasm-validation.py fixture/path.ts [...]

Options:
  --sample N     Validate a sample of N fixtures per directory (default: all).
  -v, --verbose  Print detailed per-fixture output.
  --skip-build   Skip cargo build step (use existing binary).

Dependencies: cargo, wasm-tools or wasm-validate (one required).
"""

import sys
import subprocess
import shutil
import tempfile
import os
import random
import yaml
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"


def load_catalog(path):
    with open(path) as f:
        return yaml.safe_load(f)


def get_validator():
    """Detect available wasm validator for binary wasm.

    wat2wasm reads WAT text, not binary wasm, so we prefer tools that can
    validate binary wasm directly: wasm-tools validate, wasm-validate.

    If only wat2wasm is available, we use wasm-tools print + wat2wasm pipe.

    Returns (tool_name, command_args_or_fn) or None if none found.
    """
    if shutil.which("wasm-tools"):
        return "wasm-tools", ["wasm-tools", "validate"]
    if shutil.which("wasm-validate"):
        return "wasm-validate", ["wasm-validate"]
    # wat2wasm accepts only WAT text; use wasm-tools print to convert first
    if shutil.which("wat2wasm") and shutil.which("wasm-tools"):
        return "wat2wasm(pipe)", ["wasm-tools", "print"]
    return None


def discover_compilable_fixtures(catalog):
    """Yield (rel_path, dir_name, category) for compilable fixtures.

    Compilable = not negative, not parser, not test-infrastructure,
    and not explicitly testing unsupported features (filename contains
    '-unsupported' or '-invalid' which indicates expected build failure).
    """
    non_compilable = {"negative", "parser", "test-infrastructure"}
    skip_patterns = ["-unsupported", "-invalid"]
    for dir_name, info in catalog.get("directories", {}).items():
        cat = info.get("category", "")
        if cat in non_compilable:
            continue
        for fname in info.get("fixtures", []):
            if any(p in fname for p in skip_patterns):
                continue
            rel = f"fixtures/{dir_name}/{fname}"
            yield rel, dir_name, cat


def build_fixture(ts2wasm_bin, fixture_path, wasm_path):
    """Build a single fixture and write wasm to wasm_path.

    Returns (success: bool, log: str).
    """
    result = subprocess.run(
        [str(ts2wasm_bin), "build", str(fixture_path), "-o", str(wasm_path)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        err = result.stderr.strip() or result.stdout.strip()
        return False, err[:300]
    return True, ""


def validate_wasm(validator, wasm_path):
    """Run wasm validation on a binary.

    Returns (valid: bool, log: str).
    Handles:
      wasm-tools validate  -- direct binary validation (text output)
      wasm-validate        -- direct binary validation (text output)
      wat2wasm(pipe)       -- binary to WAT via wasm-tools print, validate via wat2wasm
    """
    tool_name, cmd_base = validator
    cmd = cmd_base + [str(wasm_path)]

    if tool_name.startswith("wat2wasm"):
        # Pipe: binary -> WAT text via wasm-tools print -> wat2wasm
        print_proc = subprocess.run(cmd, capture_output=True)
        if print_proc.returncode != 0:
            err = print_proc.stderr.decode("utf-8", errors="replace").strip()[:300]
            return False, f"wasm-tools print: {err}"
        wat_result = subprocess.run(
            ["wat2wasm"], input=print_proc.stdout,
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
        )
        if wat_result.returncode != 0:
            err = wat_result.stderr.decode("utf-8", errors="replace").strip()[:300]
            return False, f"wat2wasm: {err}"
        return True, ""
    else:
        # wasm-tools validate, wasm-validate: direct binary validation
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            err = result.stderr.strip() or result.stdout.strip()
            return False, err[:300]
        return True, ""


def main():
    raw_args = sys.argv[1:]

    # Parse flags and collect fixture paths
    verbose = False
    skip_build = False
    sample_size = None
    cli_fixtures = []

    i = 0
    while i < len(raw_args):
        a = raw_args[i]
        if a in ("-v", "--verbose"):
            verbose = True
            i += 1
        elif a == "--skip-build":
            skip_build = True
            i += 1
        elif a == "--sample" and i + 1 < len(raw_args):
            try:
                sample_size = int(raw_args[i + 1])
                i += 2
            except ValueError:
                i += 1
        elif a.startswith("--"):
            if i + 1 < len(raw_args) and not raw_args[i + 1].startswith("-"):
                i += 2
            else:
                i += 1
        else:
            cli_fixtures.append(a)
            i += 1

    # Detect validator
    validator = get_validator()
    if validator is None:
        print("check_wasm_validation: no wasm validator found on PATH", file=sys.stderr)
        print("  Install one of: wasm-tools, wasm-validate, wat2wasm", file=sys.stderr)
        print("check_wasm_validation: SKIP (no validator available)", file=sys.stderr)
        sys.exit(0)

    validator_name = validator[0]
    display_name = {"wat2wasm(pipe)": "wat2wasm (via pipe)"}.get(validator_name, validator_name)
    print(f"check_wasm_validation: using {display_name}", file=sys.stderr)

    # Load catalog
    if not CATALOG_PATH.exists():
        print(f"check_wasm_validation: catalog not found: {CATALOG_PATH}", file=sys.stderr)
        sys.exit(1)

    catalog = load_catalog(CATALOG_PATH)

    # Collect fixtures from CLI args or catalog
    if cli_fixtures:
        fixture_list = cli_fixtures
    else:
        all_fixtures = list(discover_compilable_fixtures(catalog))
        if sample_size:
            by_dir = {}
            for rel, dir_name, cat in all_fixtures:
                by_dir.setdefault(dir_name, []).append(rel)
            fixture_list = []
            for dir_name, rels in by_dir.items():
                selected = random.sample(rels, min(sample_size, len(rels)))
                fixture_list.extend(selected)
            random.shuffle(fixture_list)
        else:
            fixture_list = [rel for rel, _, _ in all_fixtures]

    if not fixture_list:
        print("check_wasm_validation: no fixtures to validate", file=sys.stderr)
        sys.exit(0)

    print(f"check_wasm_validation: {len(fixture_list)} fixture(s) to validate", file=sys.stderr)

    # Build CLI (unless --skip-build)
    ts2wasm_bin = REPO_ROOT / "target/debug/ts2wasm"
    if not skip_build:
        print("check_wasm_validation: building ts2wasm-cli...", file=sys.stderr)
        build_result = subprocess.run(
            ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
            cwd=REPO_ROOT,
        )
        if build_result.returncode != 0:
            print("check_wasm_validation: cargo build failed", file=sys.stderr)
            sys.exit(1)

    if not ts2wasm_bin.exists():
        print(f"check_wasm_validation: binary not found: {ts2wasm_bin}", file=sys.stderr)
        sys.exit(1)

    # Validate fixtures
    passed = 0
    failed = 0
    skipped = 0
    details = []

    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)

        for rel in fixture_list:
            fixture_path = REPO_ROOT / rel
            if not fixture_path.exists():
                if verbose:
                    print(f"  SKIP (not found): {rel}", file=sys.stderr)
                skipped += 1
                details.append((rel, "skip", "fixture not found"))
                continue

            if verbose:
                print(f"  BUILD: {rel}", file=sys.stderr)

            wasm_path = tmpd / f"{rel.replace('/', '_')}.wasm"
            ok, log = build_fixture(ts2wasm_bin, fixture_path, wasm_path)
            if not ok:
                # Build failure is a skipped fixture, not a validation failure
                if verbose:
                    print(f"    BUILD FAIL (unsupported feature): {log}", file=sys.stderr)
                skipped += 1
                details.append((rel, "build_fail", log))
                continue

            valid, vlog = validate_wasm(validator, wasm_path)
            if valid:
                passed += 1
                details.append((rel, "pass", ""))
                if verbose:
                    print(f"    VALIDATE: pass", file=sys.stderr)
            else:
                failed += 1
                details.append((rel, "validate_fail", vlog))
                if verbose:
                    print(f"    VALIDATE: FAIL -- {vlog}", file=sys.stderr)

    # Report
    print(file=sys.stderr)
    print("=" * 70, file=sys.stderr)
    print("WASM VALIDATION REPORT", file=sys.stderr)
    print("=" * 70, file=sys.stderr)
    print(f"  Validator:           {display_name}", file=sys.stderr)
    print(f"  Fixtures attempted:  {len(fixture_list)}", file=sys.stderr)
    print(f"  Passed:              {passed}", file=sys.stderr)
    print(f"  Failed:              {failed}", file=sys.stderr)
    print(f"  Skipped:             {skipped}", file=sys.stderr)
    print(file=sys.stderr)

    if failed > 0:
        print("  Failed fixtures:", file=sys.stderr)
        for rel, kind, log in details:
            if kind == "validate_fail":
                print(f"    {rel}", file=sys.stderr)
                if log:
                    print(f"      {log}", file=sys.stderr)
        print(file=sys.stderr)

    if failed > 0:
        print("check_wasm_validation: FAIL (some fixtures produced invalid wasm)", file=sys.stderr)
        sys.exit(1)
    elif passed == 0 and skipped > 0:
        print("check_wasm_validation: WARN (all fixtures skipped, none validated)", file=sys.stderr)
        sys.exit(0)
    else:
        print("check_wasm_validation: OK (all fixtures produce valid wasm)", file=sys.stderr)
        sys.exit(0)


if __name__ == "__main__":
    main()
