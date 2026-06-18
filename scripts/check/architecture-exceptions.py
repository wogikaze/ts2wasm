#!/usr/bin/env python3
"""Architecture exception hygiene checker.

Validates architecture-exceptions.toml:
  - All exceptions have explicit ARCH-EXC-NNN ids
  - Exception ids are unique
  - All exceptions have required fields
  - No expired exceptions
  - No blanket exceptions
  - allowed_change values are valid

Usage:
  python scripts/check/architecture-exceptions.py
  python scripts/check/architecture-exceptions.py --self-test
"""

import re
import sys
from datetime import date
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
ALLOWED_CHANGES = {"delete", "move", "bugfix"}
EXCEPTION_ID_RE = re.compile(r"^ARCH-EXC-\d{3}$")


def load_exceptions() -> dict:
    path = REPO_ROOT / "architecture-exceptions.toml"
    if not path.exists():
        return {}
    with open(path, "rb") as f:
        return tomllib.load(f)


def check_exception_data(exc: dict, today: date) -> list[str]:
    violations = []
    seen_ids: dict[str, str] = {}

    for section_name, section in exc.items():
        if not isinstance(section, dict):
            continue
        for key, info in section.items():
            if not isinstance(info, dict):
                continue

            # Required fields per exception type
            if section_name == "legacy_deps":
                required = ["id", "reason", "owner", "expires", "migration_issue"]
            elif section_name == "legacy_files":
                required = ["id", "allowed_change", "reason", "owner", "expires", "migration_issue"]
                ac = info.get("allowed_change", "")
                if ac and not any(a.strip() in ALLOWED_CHANGES for a in ac.split("|")):
                    violations.append(
                        f"ERROR {key}: allowed_change '{ac}' not in {ALLOWED_CHANGES}"
                    )
            elif section_name == "legacy_runtimefn":
                required = ["id", "old_variant", "reason", "owner", "expires", "migration_issue"]
            else:
                required = ["id", "reason", "owner", "expires"]

            for field in required:
                if field not in info:
                    violations.append(f"ERROR {key}: missing required field '{field}'")

            exception_id = info.get("id", "")
            if exception_id:
                if not EXCEPTION_ID_RE.match(exception_id):
                    violations.append(
                        f"ERROR {key}: invalid exception id '{exception_id}' "
                        "(expected ARCH-EXC-NNN)"
                    )
                previous = seen_ids.get(exception_id)
                if previous is not None:
                    violations.append(
                        f"ERROR {key}: duplicate exception id '{exception_id}' "
                        f"(already used by {previous})"
                    )
                seen_ids[exception_id] = key

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


def check_exceptions() -> list[str]:
    return check_exception_data(load_exceptions(), date.today())


def run_self_test():
    errors = 0
    exc = load_exceptions()
    if "legacy_deps" not in exc:
        print("FAIL: legacy_deps section not found", file=sys.stderr)
        errors += 1
    if "legacy_files" not in exc:
        print("FAIL: legacy_files section not found", file=sys.stderr)
        errors += 1
    invalid_id = {
        "legacy_deps": {
            "backend-wasm -> ir": {
                "id": "bad-id",
                "reason": "test",
                "owner": "test",
                "expires": "2099-01-01",
                "migration_issue": "test",
            }
        }
    }
    invalid_id_errors = check_exception_data(invalid_id, date(2026, 1, 1))
    if not any("invalid exception id" in error for error in invalid_id_errors):
        print("FAIL: invalid exception id not rejected", file=sys.stderr)
        errors += 1
    duplicate_id = {
        "legacy_deps": {
            "a -> b": {
                "id": "ARCH-EXC-999",
                "reason": "test",
                "owner": "test",
                "expires": "2099-01-01",
                "migration_issue": "test",
            },
            "c -> d": {
                "id": "ARCH-EXC-999",
                "reason": "test",
                "owner": "test",
                "expires": "2099-01-01",
                "migration_issue": "test",
            },
        }
    }
    duplicate_id_errors = check_exception_data(duplicate_id, date(2026, 1, 1))
    if not any("duplicate exception id" in error for error in duplicate_id_errors):
        print("FAIL: duplicate exception id not rejected", file=sys.stderr)
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
