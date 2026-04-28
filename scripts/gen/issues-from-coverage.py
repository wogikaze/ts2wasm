#!/usr/bin/env python3
"""
Auto-generate issues from reference-coverage --detail output.

Usage:
  scripts/manager reference-coverage test262 --limit 500 --detail | \
    python3 scripts/gen/issues-from-coverage.py --start-id 061
"""

import sys
import re
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple
import argparse


def parse_detail_output(lines: List[str], suite: str) -> Dict[str, List[Tuple[str, str, str]]]:
    """Parse --detail output and group by appropriate strategy.
    
    For test262/tsgo: group by feature label (Date, Function, RegExp, etc.)
    For tsc: group by directory path for finer granularity
    
    Returns: {group_key: [(file_path, diag_code, feature_label), ...]}
    """
    in_details = False
    groups = defaultdict(list)
    
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
            
            if suite in ["test262", "tsgo"]:
                # Group by feature label for test262/tsgo
                group_key = feature_label
            else:
                # Group by filename prefix for tsc (finer granularity)
                # Extract filename without extension
                filename = file_path.split("/")[-1]
                # Use first part of filename (before number or underscore) as group key
                match = re.match(r'^([A-Za-z]+)', filename)
                if match:
                    group_key = match.group(1)
                else:
                    # Fallback to parent directory
                    path_parts = file_path.split("/")
                    if len(path_parts) >= 2:
                        group_key = "/".join(path_parts[:-1])
                    else:
                        group_key = "/".join(path_parts)
            
            groups[group_key].append((file_path, diag_code, feature_label))
    
    return groups


def group_key_to_title(group_key: str, files: List[Tuple[str, str, str]], suite: str) -> str:
    """Convert group key to issue title."""
    if suite in ["test262", "tsgo"]:
        # Feature label - use existing title map
        title_map = {
            "unknown-unsupported": "Investigate and classify unknown-unsupported cases",
            "parser-syntax": "Implement parser syntax extensions",
            "name-resolution": "Implement name resolution",
            "function-resolution": "Implement function resolution",
            "regexp-literal": "Implement RegExp literal support",
            "date": "Implement Date object support",
            "function": "Implement function support",
            "property-access": "Implement property access support",
            "unsupported-expression": "Implement unsupported expression types",
            "equality-operator": "Implement equality operators",
            "type-annotation": "Implement TypeScript type annotations",
            "class": "Implement class syntax",
            "import-export": "Implement import/export module syntax",
            "async": "Implement async/await support",
            "destructuring": "Implement destructuring",
            "template-literal": "Implement template literals",
            "legacy-octal-escape": "Implement legacy octal escape handling",
            "logical-assignment": "Implement logical assignment operators",
            "arrow-function": "Implement arrow functions",
            "spread": "Implement spread operator",
            "rest-parameter": "Implement rest parameters",
            "default-parameter": "Implement default parameters",
            "switch": "Implement switch statement",
            "loop": "Implement loop constructs",
            "break-continue": "Implement break/continue",
            "object-literal": "Implement object literal enhancements",
            "utf8-string": "Implement UTF-8 string support",
            "operator": "Implement operator support",
            "try-catch": "Implement try-catch-finally",
            "new-expression": "Implement new expression",
            "super": "Implement super keyword",
            "method-call": "Implement method call support",
            "call-expression": "Implement call expression support",
            "builtin-api": "Implement built-in API support",
        }
        return title_map.get(group_key, f"Implement {group_key} support")
    else:
        # Directory key for tsc
        last_part = group_key.split("/")[-1]
        title = last_part.replace("-", " ").replace("_", " ").title()
        return f"Implement {title}"


def generate_issue_content(
    issue_id: str,
    group_key: str,
    files: List[Tuple[str, str, str]],
    suite: str = "test262"
) -> str:
    """Generate issue markdown content."""
    title = group_key_to_title(group_key, files, suite)
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
    validation_cmd = f"scripts/manager reference-coverage {suite} --limit {count * 2}"
    
    # Adjust description based on suite
    if suite in ["test262", "tsgo"]:
        scope_desc = f"{group_key} feature"
        problem_desc = f"Reference test results show {count} cases fail with {group_key} diagnostic"
    else:
        scope_desc = f"{group_key}"
        problem_desc = f"Reference test results show {count} cases fail in directory `{group_key}` with diagnostics: {feature_str}"
    
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

Implement {scope_desc} to handle {count} failing test cases in reference tests.

## Problem

{problem_desc}. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

{scope_desc} is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for {scope_desc}
- [ ] Add fixtures for {scope_desc} behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] {scope_desc} passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for {scope_desc}
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


def find_next_issue_id(output_dir: Path) -> int:
    """Find the next available issue ID by scanning existing issues."""
    max_id = 0
    for file_path in output_dir.glob("*.md"):
        match = re.match(r'^(\d+)-', file_path.name)
        if match:
            issue_id = int(match.group(1))
            if issue_id > max_id:
                max_id = issue_id
    return max_id + 1


def main():
    parser = argparse.ArgumentParser(description="Generate issues from coverage detail output")
    parser.add_argument("--start-id", type=int, default=None, help="Starting issue ID (auto-detect if not specified)")
    parser.add_argument("--suite", type=str, default="test262", help="Test suite name (default: test262)")
    parser.add_argument("--output-dir", type=str, default="issues/open", help="Output directory (default: issues/open)")
    args = parser.parse_args()
    
    # Auto-detect next issue ID if not specified
    output_dir = Path(args.output_dir)
    if args.start_id is None:
        args.start_id = find_next_issue_id(output_dir)
        print(f"Auto-detected starting issue ID: {args.start_id:03d}", file=sys.stderr)
    
    # Read stdin
    lines = sys.stdin.readlines()
    
    # Parse detail output
    groups = parse_detail_output(lines, args.suite)
    
    # Generate issues
    output_dir = Path(args.output_dir)
    current_id = int(args.start_id)
    
    for group_key, files in sorted(groups.items()):
        # Lower threshold for tsc to get finer granularity
        min_files = 1 if args.suite == "tsc" else 3
        if len(files) < min_files:
            print(f"Skipping {group_key} (only {len(files)} files)", file=sys.stderr)
            continue
        
        issue_id = f"{current_id:03d}"
        content = generate_issue_content(issue_id, group_key, files, args.suite)
        
        # Sanitize group key for filename
        if args.suite in ["test262", "tsgo"]:
            # Feature label - use directly
            safe_key = group_key.replace("-", "-")
        else:
            # Directory key - replace slashes
            safe_key = group_key.replace("/", "-").replace("_", "-")
        
        output_file = output_dir / f"{issue_id}-implement-{safe_key}.md"
        output_file.write_text(content)
        
        print(f"Created {output_file}", file=sys.stderr)
        current_id += 1


if __name__ == "__main__":
    main()
