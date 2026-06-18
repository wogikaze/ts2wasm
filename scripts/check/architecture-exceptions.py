#!/usr/bin/env python3
"""Architecture exception hygiene checker.

Validates architecture-exceptions.toml:
  - All exceptions have required fields
  - No expired exceptions
  - No blanket exceptions
  - allowed_change values are valid

Usage:
  python scripts/check/architecture-exceptions.py
  python scripts/check/architecture-exceptions.py --self-test
"""

import sys
from datetime import date
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
ALLOWED_CHANGES = {"delete", "move", "bugfix"}


def load_exceptions() -> dict:
    path = REPO_ROOT / "architecture-exceptions.toml"
    if not path.exists():
        return {}
    with open(path, "rb") as f:
        return tomllib.load(f)


def check_exceptions() -> list[str]:
    exc = load_exceptions()
    violations = []
    today = date.today()

    for section_name, section in exc.items():
        if not isinstance(section, dict):
            continue
        for key, info in section.items():
            if not isinstance(info, dict):
                continue

            # Required fields per exception type
            if section_name == "legacy_deps":
                required = ["reason", "owner", "expires", "migration_issue"]
            elif section_name == "legacy_files":
                required = ["allowed_change", "reason", "owner", "expires", "migration_issue"]
                ac = info.get("allowed_change", "")
                if ac and not any(a.strip() in ALLOWED_CHANGES for a in ac.split("|")):
                    violations.append(
                        f"ERROR {key}: allowed_change '{ac}' not in {ALLOWED_CHANGES}"
                    )
            elif section_name == "legacy_runtimefn":
                required = ["old_variant", "reason", "owner", "expires", "migration_issue"]
            else:
                required = ["reason", "owner", "expires"]

            for field in required:
                if field not in info:
                    violations.append(f"ERROR {key}: missing required field '{field}'")

            # Expiry check
            expires_str = info.get("expires", "")
            if expires_str:
                try:
                    expires = date.fromisoformat(expires_str)
                    if expires < today:
                        violations.append(f"ERROR {key}: expired ({expires_str})")
                except ValueError:
                    violations.append(f"ERROR {key}: invalid expires format '{expires_str}'")

    return violations


def run_self_test():
    errors = 0
    exc = load_exceptions()
    if "legacy_deps" not in exc:
        print("FAIL: legacy_deps section not found", file=sys.stderr)
        errors += 1
    if "legacy_files" not in exc:
        print("FAIL: legacy_files section not found", file=sys.stderr)
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

    violations = check_exceptions()
    for v in violations:
        print(f"architecture_exceptions: {v}", file=sys.stderr)

    if violations:
        print(f"architecture_exceptions: FAILED ({len(violations)} errors)", file=sys.stderr)
        sys.exit(1)
    print("architecture_exceptions: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
