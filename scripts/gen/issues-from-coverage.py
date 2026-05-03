#!/usr/bin/env python3
"""
Auto-generate issues from reference-coverage --detail output.

Usage:
  mise run reference-coverage -- test262 --limit 500 --detail | \
    mise run gen-issues-from-coverage -- --start-id 061 --suite test262
"""

import sys
import re
import subprocess
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple
import argparse
from datetime import date

REPO_ROOT = Path(__file__).resolve().parents[2]


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
    
    return split_mixed_feature_groups(groups)


def split_mixed_feature_groups(
    groups: Dict[str, List[Tuple[str, str, str]]]
) -> Dict[str, List[Tuple[str, str, str]]]:
    """Split generated buckets that mix unrelated feature labels."""
    split_groups = defaultdict(list)
    for group_key, files in groups.items():
        feature_labels = sorted(set(feature for _, _, feature in files))
        if len(feature_labels) <= 1:
            split_groups[group_key].extend(files)
            continue

        for file_path, diag_code, feature_label in files:
            split_groups[f"{group_key}-{feature_label}"].append((file_path, diag_code, feature_label))
    return split_groups


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


def area_for_feature_labels(feature_labels: List[str], diag_codes: List[str] | None = None) -> str:
    diag_codes = diag_codes or []
    if diag_codes and set(diag_codes) == {"UnsupportedSyntax"}:
        return "frontend/syntax"

    if len(set(feature_labels)) > 1:
        return "reference/triage"

    feature = feature_labels[0] if feature_labels else "unknown-unsupported"
    frontend_syntax = {
        "ambient-declaration",
        "class",
        "class-accessor",
        "declaration-emit",
        "decorator",
        "destructuring",
        "enum",
        "import-export",
        "jsx",
        "module-system-amd",
        "parser-syntax",
        "parameter-property",
        "type-alias",
        "type-annotation",
        "type-assertion",
        "type-directive-resolution",
        "type-system",
    }
    frontend_resolver = {
        "function-resolution",
        "module-resolution",
        "name-resolution",
        "scope-analysis",
    }
    frontend_semantics = {
        "arguments-object",
        "async",
        "async-iteration",
        "call-expression",
        "function",
        "html-comment",
        "logical-assignment",
        "object-literal",
        "switch",
        "unsupported-expression",
    }
    runtime_builtins = {
        "array-builtin",
        "builtin-api",
        "date",
        "function-object",
        "legacy-global-builtin",
        "regexp-literal",
        "string-builtin",
    }

    if feature in frontend_syntax:
        return "frontend/syntax"
    if feature in frontend_resolver:
        return "frontend/resolver"
    if feature in frontend_semantics:
        return "frontend/semantics"
    if feature in runtime_builtins:
        return "runtime/builtins"
    return "reference/triage"


def affected_paths_for_area(area: str) -> tuple[List[str], str]:
    if area.startswith("frontend/"):
        return (
            [
                "crates/frontend/src/",
                "crates/cli/src/",
                "fixtures/",
                "scripts/run/reference-triage.py",
            ],
            "unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned",
        )
    if area.startswith("runtime/"):
        return (
            [
                "crates/backend-wasm/src/",
                "crates/runtime-abi/src/",
                "crates/cli/src/",
                "fixtures/",
                "scripts/run/reference-triage.py",
            ],
            "parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering",
        )
    return (
        [
            "issues/open/",
            "scripts/run/reference-triage.py",
            "fixtures/",
        ],
        "implementation code until the triage report assigns a concrete frontend/runtime/backend owner",
    )


def rel_issue_path(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def load_existing_issue_texts() -> List[Tuple[str, str]]:
    issues = []
    for state in ["open", "done"]:
        issue_dir = REPO_ROOT / "issues" / state
        if not issue_dir.exists():
            continue
        for path in sorted(issue_dir.glob("*.md")):
            issues.append((rel_issue_path(path), path.read_text(encoding="utf-8", errors="replace")))
    return issues


def duplicate_candidates(
    group_key: str,
    title: str,
    files: List[Tuple[str, str, str]],
    existing_issues: List[Tuple[str, str]],
) -> List[str]:
    labels = {feature for _, _, feature in files}
    paths = {file_path for file_path, _, _ in files[:20]}
    title_terms = {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", title)}
    candidates = []
    for issue_path, text in existing_issues:
        score = 0
        reasons = []
        if any(file_path in text for file_path in paths):
            score += 4
            reasons.append("same reference path")
        if any(label in text for label in labels):
            score += 2
            reasons.append("same feature label")
        if group_key in text:
            score += 2
            reasons.append("same group key")
        issue_title_match = re.search(r'^title:\s*"?(.+?)"?\s*$', text, re.M)
        issue_title = issue_title_match.group(1).strip().strip('"') if issue_title_match else issue_path
        overlap = title_terms & {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", issue_title)}
        if overlap:
            score += min(3, len(overlap))
            reasons.append("title overlap")
        if score >= 4:
            candidates.append(f"- `{issue_path}` - {issue_title} ({', '.join(reasons)})")
    return candidates[:10]


def is_high_confidence_duplicate(
    group_key: str,
    title: str,
    files: List[Tuple[str, str, str]],
    existing_issues: List[Tuple[str, str]],
) -> bool:
    """Return True if a high-confidence duplicate already exists (skip creation)."""
    labels = {feature for _, _, feature in files}
    title_terms = {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", title)}
    paths = {file_path for file_path, _, _ in files}

    for issue_path, text in existing_issues:
        score = 0
        if any(label in text for label in labels):
            score += 3
        if group_key in text:
            score += 3
        issue_title_match = re.search(r'^title:\s*"?(.+?)"?\s*$', text, re.M)
        issue_title = issue_title_match.group(1).strip().strip('"') if issue_title_match else ""
        overlap = title_terms & {term.lower() for term in re.findall(r"[A-Za-z0-9_+-]{4,}", issue_title)}
        if overlap:
            score += min(4, len(overlap))
        if any(file_path in text for file_path in paths):
            score += 3
        if score >= 5:
            return True
    return False


def run_reference_triage(suite: str, file_path: str, max_dump_chars: int) -> str:
    result = subprocess.run(
        [
            "python3",
            str(REPO_ROOT / "scripts" / "run" / "reference-triage.py"),
            "--max-dump-chars",
            str(max_dump_chars),
            suite,
            file_path,
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        timeout=60,
    )
    if result.returncode != 0:
        return "\n".join([
            "### Smart triage unavailable",
            "",
            "```text",
            (result.stderr or result.stdout).strip()[:4000],
            "```",
        ])
    return result.stdout.strip()


def generate_issue_content(
    issue_id: str,
    group_key: str,
    files: List[Tuple[str, str, str]],
    suite: str = "test262",
    existing_issues: List[Tuple[str, str]] | None = None,
    triage_limit: int = 1,
    triage_max_dump_chars: int = 3500,
) -> str:
    """Generate issue markdown content."""
    title = group_key_to_title(group_key, files, suite)
    count = len(files)
    today = date.today().isoformat()
    
    # Get unique feature labels
    feature_labels = sorted(set(f[2] for f in files))
    feature_str = ", ".join(feature_labels)
    diag_codes = sorted(set(f[1] for f in files))
    area = area_for_feature_labels(feature_labels, diag_codes)
    expected_paths, do_not_touch = affected_paths_for_area(area)
    
    # Sample files (first 10)
    sample_files = files[:10]
    
    # Build file list
    file_list = "\n".join(f"- `{f[0]}`" for f in sample_files)
    if count > 10:
        file_list += f"\n- ... and {count - 10} more files"
    
    # Build validation command
    validation_cmd = f"mise run reference-coverage -- {suite} --limit {count * 2}"
    exact_triage_cmd = f"mise run reference-triage -- {suite} {sample_files[0][0]}" if sample_files else ""
    exact_coverage_cmd = f"mise run reference-coverage -- {suite} --path-filter {sample_files[0][0]} --detail" if sample_files else validation_cmd
    
    # Adjust description based on suite
    if suite in ["test262", "tsgo"]:
        scope_desc = f"{group_key} feature"
        problem_desc = f"Reference test results show {count} cases fail with {group_key} diagnostic"
    else:
        scope_desc = f"{group_key}"
        problem_desc = f"Reference test results show {count} cases fail in directory `{group_key}` with diagnostics: {feature_str}"

    existing_issues = existing_issues or []
    duplicates = duplicate_candidates(group_key, title, files, existing_issues)
    duplicate_text = "\n".join(duplicates) if duplicates else "- none found by path/title/feature scan"
    triage_reports = [
        run_reference_triage(suite, file_path, triage_max_dump_chars)
        for file_path, _, _ in sample_files[:triage_limit]
    ]
    triage_text = "\n\n".join(triage_reports) if triage_reports else "Not generated. Rerun with `--triage-limit 1` or higher."
    
    content = f"""---
id: {issue_id}
title: "{title}"
type: spike
area: {area}
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: {today}
updated: {today}
---

## Summary

Triage {scope_desc} across {count} failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

{problem_desc}. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: {scope_desc} has {count} reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
{exact_triage_cmd}
```

Coverage window:

```sh
{exact_coverage_cmd}
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

{chr(10).join(f"- `{path}`" for path in expected_paths)}

Do not touch:

- {do_not_touch}

## Acceptance criteria

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
{validation_cmd}
{exact_coverage_cmd}
{exact_triage_cmd}
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

## Duplicate detection

{duplicate_text}

## Smart triage

{triage_text}

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
    """Find the smallest available issue ID by scanning existing issues (open + done)."""
    used: set[int] = set()
    for issue_root in [output_dir, output_dir.parent / "done"]:
        if not issue_root.exists():
            continue
        for file_path in issue_root.glob("*.md"):
            match = re.match(r'^(\d+)-', file_path.name)
            if match:
                used.add(int(match.group(1)))
    candidate = 1
    while candidate in used:
        candidate += 1
    return candidate


def main():
    parser = argparse.ArgumentParser(description="Generate issues from coverage detail output")
    parser.add_argument("--start-id", type=int, default=None, help="Starting issue ID (auto-detect if not specified)")
    parser.add_argument("--suite", type=str, default="test262", help="Test suite name (default: test262)")
    parser.add_argument("--output-dir", type=str, default="issues/open", help="Output directory (default: issues/open)")
    parser.add_argument("--triage-limit", type=int, default=1, help="Number of representative files to smart-triage per generated issue")
    parser.add_argument("--triage-max-dump-chars", type=int, default=3500, help="Maximum characters per compiler dump in smart triage")
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
    existing_issues = load_existing_issue_texts()
    
    for group_key, files in sorted(groups.items()):
        # Lower threshold for tsc to get finer granularity
        min_files = 1 if args.suite == "tsc" else 3
        if len(files) < min_files:
            print(f"Skipping {group_key} (only {len(files)} files)", file=sys.stderr)
            continue
        
        issue_id = f"{current_id:03d}"
        title = group_key_to_title(group_key, files, args.suite)

        # Check for high-confidence duplicates: skip if existing issue has
        # same title structure (same title terms + same feature labels)
        if is_high_confidence_duplicate(group_key, title, files, existing_issues):
            print(f"Skipping {group_key} (high-confidence duplicate already exists)", file=sys.stderr)
            continue

        content = generate_issue_content(
            issue_id,
            group_key,
            files,
            args.suite,
            existing_issues=existing_issues,
            triage_limit=args.triage_limit,
            triage_max_dump_chars=args.triage_max_dump_chars,
        )
        
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
