#!/usr/bin/env python3
"""Mechanical fixture layout rules (taxonomy hygiene).

Rules:
  - Top-level entries under fixtures/ must be directories (no loose .ts at fixtures root).
  - Directory names: lowercase ASCII, digits, hyphen only (kebab-case prefix style).

Usage: python scripts/manager.py check-fixture-catalog
"""

import sys
import re
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage:")
    print("  python scripts/manager.py check-fixture-catalog")
    print()
    print("Validates fixtures/ directory layout conventions.")

def main():
    args = sys.argv[1:]
    
    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)
    
    fixtures_dir = REPO_ROOT / "fixtures"
    
    if not fixtures_dir.exists():
        print("check_fixture_catalog: missing fixtures/", file=sys.stderr)
        sys.exit(1)
    
    errors = 0
    err = lambda msg: print(f"check_fixture_catalog: {msg}", file=sys.stderr) or globals().update(errors=1)
    
    for entry in fixtures_dir.iterdir():
        base = entry.name
        
        if entry.is_file():
            print(f"check_fixture_catalog: fixtures/ must not contain loose files at top level: {base}", file=sys.stderr)
            errors += 1
        elif not entry.is_dir():
            print(f"check_fixture_catalog: fixtures/ top-level entry is not a directory: {base}", file=sys.stderr)
            errors += 1
            continue
        
        # Check kebab-case: [a-z0-9]+(-[a-z0-9]+)*
        if not re.match(r'^[a-z0-9]+(-[a-z0-9]+)*$', base):
            print(f"check_fixture_catalog: fixtures/ directory name must be kebab-case [a-z0-9-]+ only: {base}", file=sys.stderr)
            errors += 1
    
    if errors != 0:
        sys.exit(1)
    
    print("check_fixture_catalog: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
