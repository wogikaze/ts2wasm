#!/usr/bin/env python3

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage:")
    print("  scripts/gate/coverage.sh <base-doc> <current-doc>")
    print()
    print("Checks:")
    print("  - executed count must not decrease per suite")
    print("  - build_pass count must not decrease per suite")
    print("  - semantic_pass count must not decrease per suite")
    print("  - fail count must not increase per suite")

def extract_col(file_path, suite, col):
    """Extract a column value from the coverage table for a given suite."""
    with open(file_path) as f:
        lines = f.readlines()
    
    in_table = False
    for line in lines:
        if "<!-- coverage-table:start -->" in line:
            in_table = True
            continue
        if "<!-- coverage-table:end -->" in line:
            in_table = False
            continue
        if in_table and line.startswith("|"):
            parts = [p.strip() for p in line.split("|")]
            if len(parts) > 2 and parts[1] == suite:
                # Column numbers are 1-indexed in the original script
                # col=4 -> parts[3], col=7 -> parts[6], etc.
                if col < len(parts):
                    return parts[col]
    return None

def main():
    if len(sys.argv) != 3:
        usage()
        sys.exit(1)
    
    base_doc = sys.argv[1]
    current_doc = sys.argv[2]
    
    current_path = REPO_ROOT / current_doc
    if not current_path.exists():
        print(f"missing current doc: {current_doc}", file=sys.stderr)
        sys.exit(1)
    
    base_path = REPO_ROOT / base_doc
    if not base_path.exists():
        print(f"base doc not found, skipping delta gate: {base_doc}", file=sys.stderr)
        sys.exit(0)
    
    status = 0
    suites = ["test262", "tsc", "tsgo"]
    
    for suite in suites:
        base_executed = extract_col(base_path, suite, 3)  # col 4 (0-indexed 3)
        base_build_pass = extract_col(base_path, suite, 6)  # col 7
        base_semantic_pass = extract_col(base_path, suite, 7)  # col 8
        base_fail = extract_col(base_path, suite, 8)  # col 9
        
        current_executed = extract_col(current_path, suite, 3)
        current_build_pass = extract_col(current_path, suite, 6)
        current_semantic_pass = extract_col(current_path, suite, 7)
        current_fail = extract_col(current_path, suite, 8)
        
        if not all([base_executed, base_build_pass, base_semantic_pass, base_fail,
                   current_executed, current_build_pass, current_semantic_pass, current_fail]):
            print(f"ERROR: incomplete coverage row for suite: {suite}", file=sys.stderr)
            status = 1
            continue
        
        try:
            base_executed = int(base_executed)
            base_build_pass = int(base_build_pass)
            base_semantic_pass = int(base_semantic_pass)
            base_fail = int(base_fail)
            current_executed = int(current_executed)
            current_build_pass = int(current_build_pass)
            current_semantic_pass = int(current_semantic_pass)
            current_fail = int(current_fail)
        except ValueError:
            print(f"ERROR: non-integer values for suite: {suite}", file=sys.stderr)
            status = 1
            continue
        
        if current_executed < base_executed:
            print(f"gate failure: executed decreased for {suite} ({base_executed} -> {current_executed})", file=sys.stderr)
            status = 1
        
        if current_build_pass < base_build_pass:
            print(f"gate failure: build_pass decreased for {suite} ({base_build_pass} -> {current_build_pass})", file=sys.stderr)
            status = 1
        
        if current_semantic_pass < base_semantic_pass:
            print(f"gate failure: semantic_pass decreased for {suite} ({base_semantic_pass} -> {current_semantic_pass})", file=sys.stderr)
            status = 1
        
        if current_fail > base_fail:
            print(f"gate failure: fail increased for {suite} ({base_fail} -> {current_fail})", file=sys.stderr)
            status = 1
    
    sys.exit(status)

if __name__ == "__main__":
    main()
