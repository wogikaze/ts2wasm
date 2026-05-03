#!/usr/bin/env python3
"""Check issue health invariants and index consistency.

Checks:
- Duplicate IDs within open/done
- ID collision between open/done
- Filename ID matches body ID
- Sequential IDs (no gaps in numeric range)
- Done issues have no unchecked items
- Sub-issue validity
- Depends on references exist
- Backticked paths exist
- JSON validity in .agents/state
- Index tables are up to date and consistent

Replaces legacy issue queue/index checks for <1s performance.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

# Import common issue parsing functions
sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from issue_common import (
    Issue,
    compute_blocked_ids,
    id_from_body,
    id_from_filename,
    issue_field,
    issue_title,
    load_issues,
    norm_id,
    render_blocked_table,
    render_done_table,
    render_ready_table,
    render_summary_table,
    replace_generated_block,
)

REPO = Path(__file__).resolve().parents[2]
INDEX_PATH = REPO / "issues" / "index.md"

PATH_RE = re.compile(r"`((?:crates|docs|fixtures|scripts|reference|issues|reports|\.github|\.agents|artifacts)/[^` ]+)")

PATH_PREFIXES = (
    "crates/",
    "docs/",
    "fixtures/",
    "scripts/",
    "reference/",
    "issues/",
    "reports/",
    ".github/",
    ".agents/",
    "artifacts/",
)


def err(errors: list[str], msg: str) -> None:
    errors.append(f"check_issue_health: {msg}")


def should_skip_path(p: str) -> bool:
    if p.startswith("reference/"):
        return True
    if "..." in p or "|" in p:
        return True
    if len(p) < 4:
        return True
    if not p.startswith(PATH_PREFIXES):
        return True
    if p.endswith("*"):
        return True
    if "YYYY" in p or "xxxx" in p:
        return True
    # Skip paths in migration issues that don't exist yet
    if "(after migration)" in p or "(not yet created)" in p:
        return True
    # Skip generated artifact paths under gitignored directory
    if p.startswith("artifacts/coverage/results/"):
        return True
    return False


def check_json(errors: list[str]) -> None:
    for base in [REPO / ".agents" / "state", REPO / ".agents" / "state" / "examples"]:
        if not base.exists():
            continue
        for path in base.glob("*.json"):
            try:
                json.loads(path.read_text(encoding="utf-8"))
            except Exception as e:
                err(errors, f"{path.relative_to(REPO)}: invalid JSON ({e})")


def extract_table_ids(content: str, start_marker: str, end_marker: str) -> set[str]:
    """Extract IDs from a generated table section, skipping fenced code blocks."""
    lines = content.splitlines()
    in_fence = False
    in_table = False
    ids = set()

    for line in lines:
        if line.strip() == "```":
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        if start_marker in line:
            in_table = True
            continue
        if end_marker in line:
            in_table = False
            continue
        if in_table:
            m = re.match(r"^\s*\|\s*([0-9]+[a-z]?)\s*\|", line)
            if m:
                ids.add(m.group(1))

    return ids


def main() -> int:
    errors: list[str] = []
    issues = load_issues(REPO)

    by_state: dict[str, list[Issue]] = {"open": [], "done": []}
    by_id: dict[str, list[Issue]] = {}

    for issue in issues:
        by_state[issue.state].append(issue)
        by_id.setdefault(issue.name_id, []).append(issue)

    # Duplicate IDs within open/done
    for state in ["open", "done"]:
        seen: dict[str, Path] = {}
        for issue in by_state[state]:
            if issue.name_id in seen:
                err(errors, f"duplicate id prefix in issues/{state}: {issue.name_id}")
            seen[issue.name_id] = issue.path

    # Collision between open and done
    open_ids = {i.name_id for i in by_state["open"]}
    done_ids = {i.name_id for i in by_state["done"]}
    for issue_id in sorted(open_ids & done_ids):
        err(errors, f"id present in both issues/open/ and issues/done/: {issue_id}")

    # Duplicate titles within open/
    open_title_seen: dict[str, list[Issue]] = {}
    for issue in by_state["open"]:
        open_title_seen.setdefault(issue.title, []).append(issue)
    for title, matches in open_title_seen.items():
        if len(matches) > 1:
            rel_paths = ", ".join(m.path.relative_to(REPO).as_posix() for m in matches)
            err(errors, f"duplicate title in open/: \"{title}\" in {rel_paths}")

    # Sequential IDs (no gaps in numeric range) — only check the 5000+ range
    # to avoid conflicts with the legacy hex-suffix sub-issue scheme (<1000).
    all_ids = {norm_id(i) for i in open_ids | done_ids}
    modern_nums: set[int] = set()
    for i in all_ids:
        m = re.match(r"^(5\d{3,})", i)
        if m:
            modern_nums.add(int(m.group(1)))
    if len(modern_nums) > 1:
        sorted_nums = sorted(modern_nums)
        expected = set(range(sorted_nums[0], sorted_nums[-1] + 1))
        missing = sorted(expected - modern_nums)
        if missing:
            chunks: list[list[int]] = []
            for n in missing:
                if not chunks or n != chunks[-1][-1] + 1:
                    chunks.append([n])
                else:
                    chunks[-1].append(n)
            gap_desc = "; ".join(
                f"{c[0]}-{c[-1]}" if len(c) > 1 else str(c[0]) for c in chunks
            )
            err(errors, f"non-sequential IDs: gaps found in 5000+ range: {gap_desc}")

    # ID mismatch
    for issue in issues:
        rel = issue.path.relative_to(REPO)
        if not issue.body_id:
            err(errors, f"{rel}: missing **ID** or id: in header (expected id {issue.name_id} matching filename)")
        elif issue.body_id != issue.name_id:
            err(errors, f"{rel}: id mismatch: filename {issue.name_id} vs body {issue.body_id}")

    # Done unchecked items
    for issue in by_state["done"]:
        base = issue.path.name
        if "sample" in base or base.startswith("000-"):
            continue
        if "- [ ]" in issue.text:
            err(errors, f"{issue.path.relative_to(REPO)}: has unchecked list items - [ ] but file is in issues/done/")

    # Sub-issue validity
    for state in ["open", "done"]:
        parent_needed: set[str] = set()
        sub_seen: dict[tuple[str, str], Path] = {}
        for issue in by_state[state]:
            m = re.fullmatch(r"([0-9]{3})([a-z])", issue.name_id)
            if not m:
                continue
            parent, sub = m.groups()
            key = (parent, sub)
            if key in sub_seen:
                err(errors, f"{issue.path.relative_to(REPO)}: duplicate sub-issue id {issue.name_id} conflicts with {sub_seen[key].relative_to(REPO)}")
            sub_seen[key] = issue.path
            parent_needed.add(parent)

        all_ids_for_parent = open_ids | done_ids
        for parent in sorted(parent_needed):
            if parent not in all_ids_for_parent:
                err(errors, f"sub-issues exist for parent {parent} but parent issue not found in open/ or done/")

    # Depends on
    existing_ids = open_ids | done_ids
    for issue in by_state["open"]:
        for dep in issue.depends:
            if dep not in existing_ids:
                err(errors, f"{issue.path.relative_to(REPO)}: **Depends on** id {dep} has no matching issue")

    # Backticked paths
    for issue in issues:
        # Skip path checks for migration issues that reference paths that don't exist yet
        if "migrate" in issue.title.lower():
            continue

        lines = issue.text.splitlines()
        for i, line in enumerate(lines):
            for p in PATH_RE.findall(line):
                p = p.strip().rstrip("),")
                if should_skip_path(p):
                    continue
                if not (REPO / p).exists():
                    err(errors, f"{issue.path.relative_to(REPO)}: missing path: {p}")

    # JSON validity
    check_json(errors)

    # Index consistency
    if not INDEX_PATH.exists():
        err(errors, f"missing {INDEX_PATH}")
    else:
        index_content = INDEX_PATH.read_text(encoding="utf-8")

        # Check for stale placeholder
        if "No ready issues yet" in index_content:
            err(errors, "stale Ready queue text (No ready issues yet) in issues/index.md")

        # Compute expected tables
        blocked_ids = compute_blocked_ids(issues, open_ids)
        expected_summary = render_summary_table(issues)
        expected_ready = render_ready_table(issues, open_ids, blocked_ids)
        expected_blocked = render_blocked_table(issues, blocked_ids)
        expected_done = render_done_table(issues)

        expected_index = replace_generated_block(
            index_content,
            "<!-- generated:summary:start -->",
            "<!-- generated:summary:end -->",
            expected_summary,
        )
        expected_index = replace_generated_block(
            expected_index,
            "<!-- generated:ready:start -->",
            "<!-- generated:ready:end -->",
            expected_ready,
        )
        expected_index = replace_generated_block(
            expected_index,
            "<!-- generated:blocked:start -->",
            "<!-- generated:blocked:end -->",
            expected_blocked,
        )
        expected_index = replace_generated_block(
            expected_index,
            "<!-- generated:done:start -->",
            "<!-- generated:done:end -->",
            expected_done,
        )
        if expected_index != index_content:
            err(errors, "issues/index.md is stale; run scripts/manager update-issue-index")

        # Extract actual IDs from index
        actual_ready_ids = extract_table_ids(index_content, "<!-- generated:ready:start -->", "<!-- generated:ready:end -->")
        actual_blocked_ids = extract_table_ids(index_content, "<!-- generated:blocked:start -->", "<!-- generated:blocked:end -->")

        # Check all open IDs are in ready or blocked
        for id_val in open_ids:
            if id_val not in actual_ready_ids and id_val not in actual_blocked_ids:
                err(errors, f"open issue ID {id_val} is missing from Ready or Blocked tables in issues/index.md")

        # Check ready/block counts
        if len(actual_ready_ids) == 0 and len(actual_blocked_ids) == 0 and open_ids:
            err(errors, "issues are open but Ready and Blocked tables list no issue IDs")

    if errors:
        for msg in errors:
            print(msg, file=sys.stderr)
        print("check_issue_health: failed (see errors above)", file=sys.stderr)
        return 1

    print("check_issue_health: OK")
    print("issues/index.md queue OK", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
