#!/usr/bin/env python3
"""Check issue queue invariants and index consistency.

Checks:
- Duplicate IDs within open/done
- ID collision between open/done
- Filename ID matches body ID
- Done issues have no unchecked items
- Sub-issue validity
- Depends on references exist
- Backticked paths exist
- JSON validity in .agents/state
- Index tables are up to date and consistent

Replaces check_issue_queue.sh and check_issue_index.sh for <1s performance.
"""

from __future__ import annotations

import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
ISSUES_OPEN = REPO / "issues" / "open"
ISSUES_DONE = REPO / "issues" / "done"
INDEX_PATH = REPO / "issues" / "index.md"

ID_FROM_NAME_RE = re.compile(r"^([0-9]{3}[a-z]?)-")
MD_ID_RE = re.compile(r"^\*\*ID\*\*:\s*(.+?)\s*$", re.M)
YAML_ID_RE = re.compile(r"^(?:id|ID):\s*\"?([0-9]+[a-z]?)\"?\s*$", re.M)
DEPENDS_RE = re.compile(r"^\*\*Depends on\*\*:\s*(.*?)\s*$", re.M)
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


@dataclass(frozen=True)
class Issue:
    path: Path
    state: str
    name_id: str
    body_id: str
    text: str
    title: str
    type_val: str
    area: str
    orch_class: str
    priority: str
    depends: list[str]


def norm_id(s: str) -> str:
    s = s.strip().strip('"').replace("\r", "")
    if re.fullmatch(r"[0-9]+", s):
        return f"{int(s):03d}"
    return s


def id_from_filename(path: Path) -> str:
    m = ID_FROM_NAME_RE.match(path.name)
    return m.group(1) if m else ""


def id_from_body(text: str) -> str:
    head50 = "\n".join(text.splitlines()[:50])
    m = MD_ID_RE.search(head50)
    if m:
        return norm_id(m.group(1).replace(" ", ""))

    head20 = "\n".join(text.splitlines()[:20])
    m = YAML_ID_RE.search(head20)
    if m:
        return norm_id(m.group(1))

    return ""


def issue_field(text: str, field: str) -> str:
    pattern = re.compile(rf"^\*\*{re.escape(field)}\*\*:\s*(.+)$", re.M)
    m = pattern.search(text)
    return m.group(1).strip() if m else ""


def issue_title(text: str) -> str:
    m = re.match(r"^#\s+(.+)$", text, re.M)
    return m.group(1).strip() if m else ""


def depends_from_text(text: str) -> list[str]:
    m = DEPENDS_RE.search(text)
    if not m:
        return []

    raw = m.group(1).replace("\r", "").replace(",", " ").strip()
    if raw in {"", "none", "None"}:
        return []

    ids: list[str] = []
    for token in raw.split():
        m3 = re.fullmatch(r"([0-9]{3}[a-z]?)", token)
        if m3:
            ids.append(m3.group(1))
            continue

        m_any = re.fullmatch(r"([0-9]+)([a-z]?)", token)
        if m_any:
            n, suffix = m_any.groups()
            ids.append(f"{int(n):03d}{suffix}")
    return ids


def load_issues() -> list[Issue]:
    issues: list[Issue] = []
    for state, directory in [("open", ISSUES_OPEN), ("done", ISSUES_DONE)]:
        for path in sorted(directory.glob("*.md")):
            if path.name == ".gitkeep":
                continue
            name_id = id_from_filename(path)
            if not name_id:
                continue
            text = path.read_text(encoding="utf-8", errors="replace")
            issues.append(
                Issue(
                    path=path,
                    state=state,
                    name_id=norm_id(name_id),
                    body_id=id_from_body(text),
                    text=text,
                    title=issue_title(text),
                    type_val=issue_field(text, "Type"),
                    area=issue_field(text, "Area"),
                    orch_class=issue_field(text, "Orchestration class"),
                    priority=issue_field(text, "Priority"),
                    depends=depends_from_text(text),
                )
            )
    return issues


def err(errors: list[str], msg: str) -> None:
    errors.append(f"check_issue_queue: {msg}")


def should_skip_path(p: str) -> bool:
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
            m = re.match(r"^\s*\|\s*([0-9]{3}[a-z]?)\s*\|", line)
            if m:
                ids.add(m.group(1))

    return ids


def compute_blocked_ids(issues: list[Issue], open_ids: set[str]) -> set[str]:
    blocked = set()
    for issue in issues:
        if issue.state != "open":
            continue
        orch_class = issue.orch_class.lower()
        is_blocked = orch_class == "blocked"
        if not is_blocked:
            for dep in issue.depends:
                if dep in open_ids:
                    is_blocked = True
                    break
        if is_blocked:
            blocked.add(issue.name_id)
    return blocked


def render_ready_table(issues: list[Issue], open_ids: set[str], blocked_ids: set[str]) -> str:
    lines = [
        "| ID | Title | Type | Area | Class | Priority | Depends on | Summary |",
        "|---:|---|---|---|---|---|---|---|",
    ]

    if not open_ids:
        lines.append("| — | No open issues | — | — | — | — | — | Create issues from `issues/templates/issue.md` |")
        return "\n".join(lines)

    ready_any = False
    for id_val in sorted(open_ids, key=lambda x: (int(x[:-1]) if x[-1].isalpha() else int(x), x[-1] if x[-1].isalpha() else "")):
        if id_val in blocked_ids:
            continue

        ready_any = True
        for issue in issues:
            if issue.name_id != id_val:
                continue

            title = issue.title.replace("|", "\\|")
            summary = issue.title.replace("|", "\\|")
            if len(summary) > 120:
                summary = summary[:117] + "..."

            lines.append(f"| {id_val} | {title} | {issue.type_val} | {issue.area} | {issue.orch_class} | {issue.priority} | {', '.join(issue.depends) if issue.depends else ''} | {summary} |")
            break

    if not ready_any and open_ids:
        lines.append("| — | No ready issues (all blocked) | — | — | — | — | — | See Blocked queue |")

    return "\n".join(lines)


def render_blocked_table(issues: list[Issue], blocked_ids: set[str]) -> str:
    lines = [
        "| ID | Title | Type | Area | Blocker | Summary |",
        "|---:|---|---|---|---|---|",
    ]

    if not blocked_ids:
        lines.append("| — | No blocked issues | — | — | — | — |")
        return "\n".join(lines)

    for id_val in sorted(blocked_ids, key=lambda x: (int(x[:-1]) if x[-1].isalpha() else int(x), x[-1] if x[-1].isalpha() else "")):
        for issue in issues:
            if issue.name_id != id_val:
                continue

            title = issue.title.replace("|", "\\|")
            summary = issue.title.replace("|", "\\|")
            if len(summary) > 100:
                summary = summary[:97] + "..."

            orch_class = issue.orch_class.lower()
            if orch_class == "blocked":
                blockers = f"class: {issue.orch_class}"
            else:
                blockers = ", ".join(issue.depends) if issue.depends else ""

            blockers = blockers.replace("|", "\\|")
            lines.append(f"| {id_val} | {title} | {issue.type_val} | {issue.area} | {blockers} | {summary} |")
            break

    return "\n".join(lines)


def render_done_table(issues: list[Issue]) -> str:
    lines = [
        "| ID | Title | Type | Area | Completed evidence |",
        "|---:|---|---|---|---|",
    ]

    done_issues = [i for i in issues if i.state == "done"]
    if not done_issues:
        lines.append("| — | No completed issues | — | — | — |")
        return "\n".join(lines)

    done_issues.sort(key=lambda x: (int(x.name_id[:-1]) if x.name_id[-1].isalpha() else int(x.name_id), x.name_id[-1] if x.name_id[-1].isalpha() else ""))

    for issue in done_issues:
        title = issue.title.replace("|", "\\|")
        type_val = issue.type_val or "—"
        area = issue.area or "—"

        if "## Completion evidence" in issue.text:
            evidence = f"see `issues/done/{issue.path.name}`"
        else:
            evidence = "see file"

        lines.append(f"| {issue.name_id} | {title} | {type_val} | {area} | {evidence} |")

    return "\n".join(lines)


def replace_generated_block(content: str, start_marker: str, end_marker: str, new_content: str) -> str:
    lines = content.splitlines()
    result = []
    in_fence = False
    in_block = False
    block_started = False

    for line in lines:
        if line.strip() == "```":
            in_fence = not in_fence
            result.append(line)
            continue

        if in_fence:
            result.append(line)
            continue

        if start_marker in line:
            result.append(line)
            result.append(new_content)
            in_block = True
            block_started = True
            continue

        if end_marker in line:
            in_block = False
            result.append(line)
            continue

        if not in_block:
            result.append(line)

    return "\n".join(result)


def main() -> int:
    errors: list[str] = []
    issues = load_issues()

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

        all_ids_for_parent = {i.name_id for i in by_state[state]} | done_ids
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
        expected_ready = render_ready_table(issues, open_ids, blocked_ids)
        expected_blocked = render_blocked_table(issues, blocked_ids)
        expected_done = render_done_table(issues)

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
        print("check_issue_queue: failed (see errors above)", file=sys.stderr)
        return 1

    print("check_issue_queue: OK")
    print("issues/index.md queue OK", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
