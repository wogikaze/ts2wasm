#!/usr/bin/env python3
"""Regenerate issues/index.md generated queue tables.

The old shell implementation repeatedly scanned every issue file for each
field. This version reads each issue once and preserves the generated markdown
contract used by issues/index.md.
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from dataclasses import dataclass
from pathlib import Path


REPO = Path(__file__).resolve().parents[2]
INDEX_PATH = REPO / "issues" / "index.md"
OPEN_DIR = REPO / "issues" / "open"
DONE_DIR = REPO / "issues" / "done"


FIELD_TO_YAML = {
    "ID": "id",
    "Title": "title",
    "Status": "status",
    "Type": "type",
    "Area": "area",
    "Priority": "priority",
    "Depends on": "depends_on",
    "Orchestration class": "class",
    "Orchestration upstream": "upstream",
}


@dataclass(frozen=True)
class Issue:
    path: Path
    state: str
    issue_id: str
    text: str


def log(message: str) -> None:
    print(message, file=sys.stderr)


def version_key(issue_id: str) -> tuple[int, str]:
    match = re.fullmatch(r"([0-9]+)([a-z]?)", issue_id)
    if not match:
        return (10**9, issue_id)
    number, suffix = match.groups()
    return (int(number), suffix)


def sorted_issue_paths(directory: Path) -> list[Path]:
    return sorted(directory.glob("*.md"), key=lambda p: p.as_posix())


def yaml_value(text: str, field: str, *, first_lines: int = 20) -> str:
    pattern = re.compile(rf"^[ \t]*{re.escape(field)}:[ \t]*(.*)$", re.I)
    for line in text.splitlines()[:first_lines]:
        match = pattern.match(line)
        if not match:
            continue
        value = match.group(1).replace('"', "")
        value = value.replace("[", "").replace("]", "")
        return value.rstrip()
    return ""


def markdown_field(text: str, field: str) -> str:
    match = re.search(rf"^\*\*{re.escape(field)}\*\*:[ \t]*(.*)$", text, re.M)
    return match.group(1).rstrip() if match else ""


def issue_field(issue: Issue, field: str) -> str:
    value = yaml_value(issue.text, FIELD_TO_YAML.get(field, field))
    if value:
        return value
    return markdown_field(issue.text, field)


def issue_title(issue: Issue) -> str:
    title = yaml_value(issue.text, "title")
    if title:
        return title
    match = re.search(r"^# (.*)$", issue.text, re.M)
    return match.group(1).rstrip() if match else ""


def issue_problem_summary(issue: Issue) -> str:
    match = re.search(r"^Problem:[ \t]*(.*)$", issue.text, re.M)
    if match:
        return match.group(1).rstrip()
    return issue_title(issue)


def issue_id_from_text(text: str) -> str:
    match = re.search(r"^[ \t]*id:[ \t]*(.*)$", text, re.M)
    if match:
        return match.group(1).replace('"', "").rstrip()

    match = re.search(r"^\*\*ID\*\*:[ \t]*(.*)$", text, re.M)
    if match:
        return match.group(1).rstrip()

    return ""


def parse_depends_ids(raw: str) -> list[str]:
    raw = raw.strip()
    if raw.startswith("[") and raw.endswith("]"):
        raw = raw[1:-1]
    raw = raw.replace(",", " ").replace("]", " ")
    raw = " ".join(raw.split())
    if raw in {"", "none", "[]"}:
        return []
    return raw.split()


def issue_depends_ids(issue: Issue) -> list[str]:
    raw = yaml_value(issue.text, "depends_on")
    if not raw:
        raw = markdown_field(issue.text, "Depends on")
    return parse_depends_ids(raw)


def truncate(value: str, length: int) -> str:
    if len(value) <= length:
        return value
    return value[: length - 3] + "..."


def escape_cell(value: str) -> str:
    return value.replace("|", "\\|")


def load_issues() -> list[Issue]:
    issues: list[Issue] = []
    for state, directory in (("open", OPEN_DIR), ("done", DONE_DIR)):
        for path in sorted_issue_paths(directory):
            text = path.read_text(encoding="utf-8", errors="replace")
            issue_id = issue_id_from_text(text)
            if not issue_id:
                match = re.match(r"^([0-9]+[a-z]?)-", path.stem)
                issue_id = match.group(1) if match else ""
            if not issue_id:
                continue
            issues.append(Issue(path=path, state=state, issue_id=issue_id, text=text))
    return issues


def compute_blocked_ids(open_issues: list[Issue], open_ids: set[str]) -> set[str]:
    blocked_ids: set[str] = set()
    for issue in open_issues:
        issue_class = " ".join(issue_field(issue, "Orchestration class").lower().split())
        blocked = issue_class == "blocked"
        if not blocked:
            blocked = any(dep in open_ids for dep in issue_depends_ids(issue))
        if blocked:
            blocked_ids.add(issue.issue_id)
    return blocked_ids


def render_ready_table(open_issues: list[Issue], open_ids: set[str], blocked_ids: set[str]) -> str:
    by_id = {issue.issue_id: issue for issue in open_issues}
    lines = [
        "| ID | Title | Type | Area | Class | Priority | Depends on | Summary |",
        "|---:|---|---|---|---|---|---|---|",
    ]

    if not open_ids:
        lines.append("| — | No open issues | — | — | — | — | — | Create issues from `issues/templates/issue.md` |")
        return "\n".join(lines)

    ready_any = False
    for issue_id in sorted(open_ids, key=version_key):
        if issue_id in blocked_ids:
            continue
        issue = by_id.get(issue_id)
        if issue is None:
            continue
        ready_any = True
        summary = truncate(escape_cell(issue_problem_summary(issue)), 120)
        lines.append(
            f"| {issue_id} | {escape_cell(issue_title(issue))} | "
            f"{issue_field(issue, 'Type')} | {issue_field(issue, 'Area')} | "
            f"{issue_field(issue, 'Orchestration class')} | {issue_field(issue, 'Priority')} | "
            f"{issue_field(issue, 'Depends on')} | {summary} |"
        )

    if not ready_any:
        lines.append("| — | No ready issues (all blocked) | — | — | — | — | — | See Blocked queue |")
    return "\n".join(lines)


def render_blocked_table(open_issues: list[Issue], blocked_ids: set[str]) -> str:
    by_id = {issue.issue_id: issue for issue in open_issues}
    lines = [
        "| ID | Title | Type | Area | Blocker | Summary |",
        "|---:|---|---|---|---|---|",
    ]

    if not blocked_ids:
        lines.append("| — | No blocked issues | — | — | — | — |")
        return "\n".join(lines)

    for issue_id in sorted(blocked_ids, key=version_key):
        issue = by_id.get(issue_id)
        if issue is None:
            continue
        blockers = issue_field(issue, "Depends on")
        issue_class = " ".join(issue_field(issue, "Orchestration class").lower().split())
        if issue_class == "blocked":
            blockers = f"class: {issue_field(issue, 'Orchestration class')}"
        summary = truncate(escape_cell(issue_problem_summary(issue)), 100)
        lines.append(
            f"| {issue_id} | {escape_cell(issue_title(issue))} | "
            f"{issue_field(issue, 'Type')} | {issue_field(issue, 'Area')} | "
            f"{escape_cell(blockers)} | {summary} |"
        )
    return "\n".join(lines)


def render_done_table(done_issues: list[Issue]) -> str:
    lines = [
        "| ID | Title | Type | Area | Completed evidence |",
        "|---:|---|---|---|---|",
    ]

    if not done_issues:
        lines.append("| — | No completed issues | — | — | — |")
        return "\n".join(lines)

    for issue in sorted(done_issues, key=lambda i: version_key(i.issue_id)):
        title = issue_title(issue)
        if not markdown_field(issue.text, "ID"):
            title = yaml_value(issue.text, "title", first_lines=len(issue.text.splitlines()))
        if not title:
            title = issue_title(issue)

        issue_type = issue_field(issue, "Type")
        if not issue_type:
            issue_type = yaml_value(issue.text, "type", first_lines=len(issue.text.splitlines()))
            issue_type = issue_type.split("|", 1)[0].rstrip()

        area = issue_field(issue, "Area")
        if not area:
            area = yaml_value(issue.text, "area", first_lines=len(issue.text.splitlines()))
            area = area.split("|", 1)[0].rstrip()

        evidence = "see file"
        if re.search(r"^## Completion evidence$", issue.text, re.M):
            evidence = f"see `issues/done/{issue.path.name}`"

        lines.append(
            f"| {issue.issue_id} | {escape_cell(title)} | {issue_type or '—'} | "
            f"{area or '—'} | {evidence} |"
        )
    return "\n".join(lines)


def replace_generated_block(content: str, start_marker: str, end_marker: str, new_content: str) -> str:
    result: list[str] = []
    in_fence = False
    in_block = False

    for line in content.splitlines():
        if line == "```":
            in_fence = not in_fence
            result.append(line)
            continue
        if in_fence:
            result.append(line)
            continue
        if line == start_marker:
            result.append(line)
            result.extend(new_content.splitlines())
            in_block = True
            continue
        if line == end_marker:
            in_block = False
            result.append(line)
            continue
        if not in_block:
            result.append(line)

    return "\n".join(result) + "\n"


def render_index(index_content: str, issues: list[Issue]) -> str:
    open_issues = [issue for issue in issues if issue.state == "open"]
    done_issues = [issue for issue in issues if issue.state == "done"]
    open_ids = {issue.issue_id for issue in open_issues}
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
    next_content = render_index(index_content, load_issues())

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
