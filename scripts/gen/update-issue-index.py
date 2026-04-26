#!/usr/bin/env python3
"""Regenerate issues/index.md generated queue tables.

The old shell implementation repeatedly scanned every issue file for each
field. This version reads each issue once and preserves the generated markdown
contract used by issues/index.md.
"""

from __future__ import annotations

import argparse
import difflib
import sys
from pathlib import Path

# Import common issue parsing functions
sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from issue_common import (
    Issue,
    compute_blocked_ids,
    escape_cell,
    issue_field,
    issue_title,
    load_issues,
    render_blocked_table,
    render_done_table,
    render_ready_table,
    replace_generated_block,
    truncate,
    version_key,
)

REPO = Path(__file__).resolve().parents[2]
INDEX_PATH = REPO / "issues" / "index.md"


def log(message: str) -> None:
    print(message, file=sys.stderr)


def render_index(index_content: str, issues: list[Issue]) -> str:
    open_issues = [issue for issue in issues if issue.state == "open"]
    done_issues = [issue for issue in issues if issue.state == "done"]
    open_ids = {issue.name_id for issue in open_issues}
    blocked_ids = compute_blocked_ids(open_issues, open_ids)

    next_content = replace_generated_block(
        index_content,
        "<!-- generated:ready:start -->",
        "<!-- generated:ready:end -->",
        render_ready_table(open_issues, open_ids, blocked_ids),
    )
    next_content = replace_generated_block(
        next_content,
        "<!-- generated:blocked:start -->",
        "<!-- generated:blocked:end -->",
        render_blocked_table(open_issues, blocked_ids),
    )
    return replace_generated_block(
        next_content,
        "<!-- generated:done:start -->",
        "<!-- generated:done:end -->",
        render_done_table(done_issues),
    )


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="scripts/gen/update-issue-index.py",
        description="Regenerate the marked regions in issues/index.md from issues/open/*.md and issues/done/*.md.",
    )
    parser.add_argument("--check", action="store_true", help="compare against current state and fail if it would change")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    if not INDEX_PATH.is_file():
        log(f"missing {INDEX_PATH.relative_to(REPO)}")
        return 1

    index_content = INDEX_PATH.read_text(encoding="utf-8")
    next_content = render_index(index_content, load_issues(REPO))

    if args.check:
        if next_content != index_content:
            log("issues/index.md is stale; run scripts/manager update-issue-index")
            diff = difflib.unified_diff(
                index_content.splitlines(keepends=True),
                next_content.splitlines(keepends=True),
                fromfile="issues/index.md",
                tofile="issues/index.md.generated",
            )
            sys.stderr.writelines(diff)
            return 1
        log("issues/index.md OK (up to date)")
        return 0

    INDEX_PATH.write_text(next_content, encoding="utf-8")
    log("Updated issues/index.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
