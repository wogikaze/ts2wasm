#!/usr/bin/env python3
"""
Generate allowed_files/forbidden_files for an issue based on area/type patterns.
"""

import json
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent
PATTERNS_FILE = PROJECT_ROOT / ".agents" / "state" / "issue_scope_patterns.json"


def merge_scope(base: dict, override: dict) -> dict:
    """Merge two scope dictionaries, with override taking precedence."""
    result = {
        "allowed_files": list(base.get("allowed_files", [])),
        "forbidden_files": list(base.get("forbidden_files", []))
    }
    
    # Merge allowed_files (union)
    for f in override.get("allowed_files", []):
        if f not in result["allowed_files"]:
            result["allowed_files"].append(f)
    
    # Merge forbidden_files (union)
    for f in override.get("forbidden_files", []):
        if f not in result["forbidden_files"]:
            result["forbidden_files"].append(f)
    
    return result


def generate_scope(area: str, type: str) -> dict:
    """Generate scope for an issue based on area and type."""
    with open(PATTERNS_FILE) as f:
        patterns = json.load(f)
    
    base = patterns.get("default", {"allowed_files": [], "forbidden_files": ["docs/"]})
    area_scope = patterns.get("by_area", {}).get(area, {})
    type_scope = patterns.get("by_type", {}).get(type, {})
    
    # Merge: base + area + type
    result = merge_scope(base, area_scope)
    result = merge_scope(result, type_scope)
    
    return result


def main():
    if len(sys.argv) < 3:
        print("Usage: gen-issue-scope.py <area> <type>", file=sys.stderr)
        print("Example: gen-issue-scope.py runtime/semantics bug", file=sys.stderr)
        sys.exit(1)
    
    area = sys.argv[1]
    type = sys.argv[2]
    
    scope = generate_scope(area, type)
    print(json.dumps(scope, indent=2))


if __name__ == "__main__":
    main()
