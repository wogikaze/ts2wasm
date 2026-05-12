#!/usr/bin/env python3
"""Check that negative diagnostic fixtures produce the expected compiler errors.

Scans fixtures/negative/ for .ts files and runs the compiler on each,
verifying that the expected diagnostic code appears in the stderr output.

Usage:
  python3 scripts/check/compiler-diagnostics.py

The checker matches diagnostic codes from the format [CodeName/phase] or
[CodeName] in stderr output.  Fixture filenames use kebab-case, e.g.
unresolved-name.ts which maps to diagnostic code UnresolvedName.

Some internal diagnostic codes (InvariantViolation, BackendIo) are not
triggerable from source input and are excluded.

For TypeScriptTypeCheck, uses `ts2wasm check` instead of `ts2wasm build`.
"""

import sys
import subprocess
import re
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
FIXTURES_DIR = REPO_ROOT / "fixtures" / "negative"
CLI_BINARY = REPO_ROOT / "target" / "debug" / "ts2wasm"

# Mapping from kebab-case fixture name to PascalCase diagnostic display code.
# Only needed when the two differ (most simply match by title-casing the
# fixture name).
FIXTURE_DIAG_MAP: dict[str, str] = {
    # Exact match (kebab filename -> Pascal diagnostic code)
    "unresolved-name": "UnresolvedName",
    "unresolved-function": "UnresolvedName",  # Name resolver catches block-scoped fns
    "duplicate-function": "DuplicateFunction",
    "duplicate-local": "DuplicateLocal",
    "duplicate-parameter": "DuplicateLocal",  # DuplicateLocal is the actual compiler output
    "arity-mismatch": "ArityMismatch",
    "invalid-top-level-return": "InvalidTopLevelReturn",
    "syntax-error": "SyntaxError",
    "number-out-of-range": "UnsupportedSyntax",
    "unsupported-syntax": "UnsupportedSyntax",
    "unsupported-builtin": "UnsupportedBuiltin",
    "unsupported-date": "UnsupportedDate",
    "unsupported-regexp": "UnsupportedSyntax",  # maps via display_code()
    "unsupported-module": "UnsupportedModule",
    "unsupported-eval": "UnsupportedEval",
    "unsupported-typescript-syntax": "UnsupportedTypeScriptSyntax",
    "unsupported-runtime-subset": "UnsupportedSyntax",  # maps via display_code()
    "typescript-type-check": "TypeScriptTypeCheck",
}

# Fixtures corresponding to diagnostic codes that cannot be triggered from source input.
UNTRIGGERABLE_FIXTURES: set[str] = set()

# Diagnostic codes that can only be triggered via `ts2wasm check` (TypeScript oracle).
CHECK_SUBCOMMAND_CODES = {"TypeScriptTypeCheck"}

# Fixtures that need `check` subcommand instead of `build` (type info only, not compiler).
CHECK_SUBCOMMAND_FIXTURES = {"typescript-type-check"}

# Regex to extract diagnostic code from stderr lines like:
#   error: [CodeName/phase] msg
#   error: [CodeName] msg
# (leading/trailing spaces are allowed)
DIAG_CODE_RE = re.compile(r"error:\s*\[(\w+)(?:/[\w-]+)?\]")


def build_cli() -> str | None:
    """Build the CLI binary if not already built. Returns None on success."""
    if CLI_BINARY.exists():
        return None
    result = subprocess.run(
        ["cargo", "build", "-p", "ts2wasm-cli"],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        return f"cargo build failed:\n{result.stderr[:500]}"
    return None


def get_diag_code(stderr: str) -> str | None:
    """Extract the diagnostic code from stderr output.

    The diagnostic format is:  error: [CodeName/phase]  or  error: [CodeName]
    """
    for line in stderr.split("\n"):
        m = DIAG_CODE_RE.search(line)
        if m:
            return m.group(1)
    return None


def run_compiler(fixture_path: Path) -> tuple[str, int]:
    """Run the compiler on the fixture and return (stderr, exit_code)."""
    fixture_name = fixture_path.stem

    if fixture_name in CHECK_SUBCOMMAND_FIXTURES:
        cmd = [str(CLI_BINARY), "check", str(fixture_path)]
    else:
        cmd = [str(CLI_BINARY), "build", "-o", "/dev/null", str(fixture_path)]

    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
        timeout=30,
    )
    return result.stderr, result.returncode


def collect_fixtures() -> list[Path]:
    """Collect all .ts fixture files from fixtures/negative/."""
    if not FIXTURES_DIR.exists():
        return []
    return sorted(FIXTURES_DIR.glob("*.ts"))


def main() -> None:
    args = sys.argv[1:]
    if args and args[0] in ("-h", "--help"):
        print("Usage: python3 scripts/check/compiler-diagnostics.py")
        print()
        print("Runs the compiler on all fixtures in fixtures/negative/ and")
        print("verifies that each produces the expected diagnostic code.")
        print()
        print("Exit codes: 0 = all pass, 1 = failures found")
        sys.exit(0)

    # Build CLI if needed
    build_err = build_cli()
    if build_err:
        print(f"check_compiler_diagnostics: ERROR: {build_err}", file=sys.stderr)
        sys.exit(1)

    fixtures = collect_fixtures()
    if not fixtures:
        print(
            "check_compiler_diagnostics: WARN: no negative fixtures found",
            file=sys.stderr,
        )
        sys.exit(0)

    failures: list[str] = []
    passes = 0
    skipped = 0

    for fixture in fixtures:
        fixture_name = fixture.stem

        # Skip untriggerable fixtures
        if fixture_name in UNTRIGGERABLE_FIXTURES:
            print(
                f"check_compiler_diagnostics: SKIP {fixture_name}.ts "
                f"(cannot be triggered from source input)"
            )
            skipped += 1
            continue

        expected = FIXTURE_DIAG_MAP.get(fixture_name)
        if expected is None:
            print(
                f"check_compiler_diagnostics: SKIP {fixture_name}.ts "
                f"(no diagnostic mapping defined)"
            )
            skipped += 1
            continue

        stderr, exit_code = run_compiler(fixture)

        if exit_code == 0:
            failures.append(
                f"  {fixture_name}.ts: compiler exited 0 (expected [{expected}])"
            )
            continue

        actual = get_diag_code(stderr)

        if actual is None:
            failures.append(
                f"  {fixture_name}.ts: no diagnostic code found in stderr:\n"
                f"    {stderr[:300].strip()}"
            )
            continue

        if actual == expected:
            passes += 1
            print(f"check_compiler_diagnostics: PASS {fixture_name}.ts -> [{actual}]")
        else:
            failures.append(
                f"  {fixture_name}.ts: expected [{expected}], got [{actual}]"
            )

    print()

    if failures:
        print("check_compiler_diagnostics: FAILURES:", file=sys.stderr)
        for f in failures:
            print(f, file=sys.stderr)
        print(
            f"\ncheck_compiler_diagnostics: FAILED "
            f"({len(failures)} failures, {passes} passed, {skipped} skipped)",
            file=sys.stderr,
        )
        sys.exit(1)

    print(
        f"check_compiler_diagnostics: OK ({passes} passed, {skipped} skipped)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
