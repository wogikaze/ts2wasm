#!/usr/bin/env python3
"""Legacy file freeze enforcement — touched-file deny.

Frozen files must not be modified except for delete, move, or bugfix
(explicitly allowed in architecture-exceptions.toml).

Usage:
  python scripts/check/legacy-freeze.py
  python scripts/check/legacy-freeze.py --self-test
"""

import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

FROZEN_FILES = [
    "crates/backend-wasm/src/native_lowered.rs",
    "crates/backend-wasm/src/runtime/core/typed.rs",
    "crates/backend-wasm/src/native_runtime_embed.rs",
    "crates/runtime-catalog/src/runtime_fn.rs",
]

ALLOWED_CHANGES = {"delete", "move", "bugfix"}


def load_exceptions() -> dict:
    path = REPO_ROOT / "architecture-exceptions.toml"
    if not path.exists():
        return {"legacy_files": {}}
    with open(path, "rb") as f:
        return tomllib.load(f)


def get_file_exception(rel_path: str) -> dict | None:
    exc = load_exceptions()
    for fpath, info in exc.get("legacy_files", {}).items():
        if fpath == rel_path:
            return dict(info)
    return None


def check_touched_files() -> list[str]:
    violations = []
    result = subprocess.run(
        ["git", "diff", "--name-only", "HEAD"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    changed = result.stdout.strip().splitlines()

    for fname in FROZEN_FILES:
        if fname not in changed:
            continue
        exc = get_file_exception(fname)
        if exc:
            allowed = exc.get("allowed_change", "")
            parts = allowed.split("|")
            if any(a.strip() in ALLOWED_CHANGES for a in parts):
                continue
        violations.append(
            f"ERROR {fname} is LEGACY FROZEN — "
            f"modification requires exception in architecture-exceptions.toml"
        )
    return violations


def run_self_test():
    errors = 0

    # Test: parse exception format
    exc = load_exceptions()
    legacy = exc.get("legacy_files", {})
    if "crates/backend-wasm/src/native_lowered.rs" not in legacy:
        print("FAIL: exception file not parsed correctly", file=sys.stderr)
        errors += 1

    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print("self-test: OK", file=sys.stderr)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--self-test" in args:
        run_self_test()
        return

    violations = check_touched_files()
    for v in violations:
        print(f"legacy_freeze: {v}", file=sys.stderr)

    if violations:
        print(f"legacy_freeze: FAILED ({len(violations)} errors)", file=sys.stderr)
        sys.exit(1)
    print("legacy_freeze: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
