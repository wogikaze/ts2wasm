#!/usr/bin/env python3
"""
Auto-generate issues from reference-coverage --detail output.

Usage:
  scripts/run/reference-coverage.sh test262 --limit 500 --detail | \
    python3 scripts/gen/issues-from-coverage.py --start-id 061
"""

import sys
import re
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple
import argparse


def parse_detail_output(lines: List[str]) -> Dict[str, List[Tuple[str, str, str]]]:
    """Parse --detail output and group by directory path.
    
    Returns: {directory: [(file_path, diag_code, feature_label), ...]}
    """
    in_details = False
    dir_groups = defaultdict(list)
    
    for line in lines:
        if line.startswith("# Per-file details"):
            in_details = True
            continue
        
        if not in_details:
            continue
        
        # Parse: file-path: diag-code: feature-label
        parts = line.strip().split(": ", 2)
        if len(parts) >= 3:
            file_path = parts[0]
            diag_code = parts[1]
            feature_label = parts[2]
            
            # Group by parent directory
            path_parts = file_path.split("/")
            if len(path_parts) >= 2:
                dir_key = "/".join(path_parts[:-1])
            else:
                dir_key = "/".join(path_parts)
            
            dir_groups[dir_key].append((file_path, diag_code, feature_label))
    
    return dir_groups


def dir_to_issue_title(dir_key: str, files: List[Tuple[str, str, str]]) -> str:
    """Convert directory key to issue title."""
    # Extract last directory component
    last_part = dir_key.split("/")[-1]
    
    # Convert kebab-case to title case
    title = last_part.replace("-", " ").replace("_", " ").title()
    
    # Add feature context from first file
    if files:
        feature_label = files[0][2]
        return f"Implement {title} ({feature_label})"
    
    return f"Implement {title}"


def generate_issue_content(
    issue_id: str,
    dir_key: str,
    files: List[Tuple[str, str, str]],
    suite: str = "test262"
) -> str:
    """Generate issue markdown content."""
    title = dir_to_issue_title(dir_key, files)
    count = len(files)
    
    # Get unique feature labels
    feature_labels = sorted(set(f[2] for f in files))
    feature_str = ", ".join(feature_labels)
    
    # Sample files (first 10)
    sample_files = files[:10]
    
    # Build file list
    file_list = "\n".join(f"- `{f[0]}`" for f in sample_files)
    if count > 10:
        file_list += f"\n- ... and {count - 10} more files"
    
    # Build validation command
    validation_cmd = f"scripts/run/reference-coverage.sh {suite} --limit {count * 2}"
    
    content = f"""---
id: {issue_id}
title: "{title}"
type: feature
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement support for {dir_key} to handle {count} failing test cases in reference tests.

## Problem

Reference test results show {count} cases fail in directory `{dir_key}` with diagnostics: {feature_str}. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

{dir_key} is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for {dir_key}
- [ ] Add fixtures for {dir_key} behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related directories (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] {dir_key} passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for {dir_key}
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
{validation_cmd}
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

{file_list}

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
"""
    return content


def main():
    parser = argparse.ArgumentParser(description="Generate issues from coverage detail output")
    parser.add_argument("--start-id", type=str, default="061", help="Starting issue ID (default: 061)")
    parser.add_argument("--suite", type=str, default="test262", help="Test suite name (default: test262)")
    parser.add_argument("--output-dir", type=str, default="issues/open", help="Output directory (default: issues/open)")
    args = parser.parse_args()
    
    # Read stdin
    lines = sys.stdin.readlines()
    
    # Parse detail output
    dir_groups = parse_detail_output(lines)
    
    # Generate issues
    output_dir = Path(args.output_dir)
    current_id = int(args.start_id)
    
    for dir_key, files in sorted(dir_groups.items()):
        if len(files) < 3:
            print(f"Skipping {dir_key} (only {len(files)} files)", file=sys.stderr)
            continue
        
        issue_id = f"{current_id:03d}"
        content = generate_issue_content(issue_id, dir_key, files, args.suite)
        
        # Sanitize directory key for filename
        safe_dir = dir_key.replace("/", "-").replace("_", "-")
        output_file = output_dir / f"{issue_id}-implement-{safe_dir}.md"
        output_file.write_text(content)
        
        print(f"Created {output_file}", file=sys.stderr)
        current_id += 1


if __name__ == "__main__":
    main()
