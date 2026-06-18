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
ALLOWED_EXCEPTION_IDS = set()  # populated by --allow-exception <id>


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
    try:
        result = subprocess.run(
            ["git", "diff", "--name-only", "HEAD"],
            capture_output=True, text=True, cwd=REPO_ROOT, timeout=10,
        )
        if result.returncode != 0:
            violations.append(
                "ERROR git diff failed — cannot verify legacy freeze. "
                "Run in a git repository."
            )
            return violations
        changed = result.stdout.strip().splitlines()
    except (FileNotFoundError, subprocess.TimeoutExpired):
        violations.append(
            "ERROR git not available — cannot verify legacy freeze. "
            "Run in a git repository."
        )
        return violations

    for fname in FROZEN_FILES:
        if fname not in changed:
            continue
        # Exception only applies if --allow-exception <id> was explicitly passed
        # AND the exception's allowed_change matches AND the exception's file matches
        exc_allowed = False
        for eid in ALLOWED_EXCEPTION_IDS:
            exc = get_file_exception(fname)
            if exc:
                allowed = exc.get("allowed_change", "")
                parts = allowed.split("|")
                exc_allowed = any(a.strip() in ALLOWED_CHANGES for a in parts)
                if exc_allowed:
                    break
        if exc_allowed:
            continue
        violations.append(
            f"ERROR {fname} is LEGACY FROZEN — "
            f"use --allow-exception <id> to acknowledge the change"
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
    global ALLOWED_EXCEPTION_IDS
    args = sys.argv[1:]
    i = 0
    while i < len(args):
        if args[i] == "--allow-exception" and i + 1 < len(args):
            ALLOWED_EXCEPTION_IDS.add(args[i + 1])
            i += 2
        elif args[i] in ("-h", "--help"):
            print(__doc__.strip())
            sys.exit(0)
        elif args[i] == "--self-test":
            run_self_test()
            return
        else:
            i += 1

    violations = check_touched_files()
    for v in violations:
        print(f"legacy_freeze: {v}", file=sys.stderr)

    if violations:
        print(f"legacy_freeze: FAILED ({len(violations)} errors)", file=sys.stderr)
        sys.exit(1)
    print("legacy_freeze: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
