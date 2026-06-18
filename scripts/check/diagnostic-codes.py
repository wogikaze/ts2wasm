#!/usr/bin/env python3
"""Check bidirectional alignment between DiagCode enum and docs/reference/diagnostic-codes.md.

Extracts variant names from the Rust enum and code names from the reference doc
table, then reports any entries missing from either side.

Usage:
  python scripts/check/diagnostic-codes.py
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def extract_enum_codes() -> set[str]:
    path = REPO_ROOT / "crates" / "diagnostic" / "src" / "lib.rs"
    text = path.read_text()
    codes: set[str] = set()
    in_enum = False
    for line in text.splitlines():
        if "pub enum DiagCode" in line:
            in_enum = True
            continue
        if in_enum:
            m = re.match(r"^\s+(\w+),$", line)
            if m:
                codes.add(m.group(1))
            if "}" in line:
                break
    return codes


def extract_doc_codes() -> set[str]:
    path = REPO_ROOT / "docs" / "reference" / "diagnostic-codes.md"
    text = path.read_text()
    codes: set[str] = set()
    for line in text.splitlines():
        m = re.match(r"^\| *`(\w+)` *\|", line)
        if m:
            codes.add(m.group(1))
    return codes


def main() -> None:
    enum_codes = extract_enum_codes()
    doc_codes = extract_doc_codes()

    missing_from_doc = enum_codes - doc_codes
    orphan_in_doc = doc_codes - enum_codes

    has_errors = False

    if missing_from_doc:
        print(
            "check_diagnostic_codes: ERROR codes defined in enum "
            "but missing from docs/reference/diagnostic-codes.md:",
            file=sys.stderr,
        )
        for code in sorted(missing_from_doc):
            print(f"  {code}", file=sys.stderr)
        has_errors = True

    if orphan_in_doc:
        print(
            "check_diagnostic_codes: ERROR codes documented in "
            "docs/reference/diagnostic-codes.md but not in DiagCode enum:",
            file=sys.stderr,
        )
        for code in sorted(orphan_in_doc):
            print(f"  {code}", file=sys.stderr)
        has_errors = True

    if has_errors:
        sys.exit(1)

    print(
        f"check_diagnostic_codes: OK ({len(enum_codes)} codes, all documented)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
