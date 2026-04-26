#!/usr/bin/env python3
"""Common issue parsing and rendering functions.

Shared between check-issue-health.py and update-issue-index.py.
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path


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


ID_FROM_NAME_RE = re.compile(r"^([0-9]{3}[a-z]?)-")
MD_ID_RE = re.compile(r"^\*\*ID\*\*:\s*(.+?)\s*$", re.M)
YAML_ID_RE = re.compile(r"^(?:id|ID):\s*\"?([0-9]+[a-z]?)\"?\s*$", re.M)
DEPENDS_RE = re.compile(r"^\*\*Depends on\*\*:\s*(.*?)\s*$", re.M)
YAML_DEPENDS_RE = re.compile(r"^(?:depends_on|Depends on):\s*\[(.*?)\]\s*$", re.M)


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
    yaml_key_map = {
        "Type": "type",
        "Area": "area",
        "Orchestration class": "class",
        "Priority": "priority",
    }
    yaml_key = yaml_key_map.get(field, field.lower())
    yaml_pattern = re.compile(rf"^{re.escape(yaml_key)}:\s*\"?(.+?)\"?\s*$", re.M)
    m = yaml_pattern.search(text)
    if m:
        return m.group(1).strip().strip('"')

    md_pattern = re.compile(rf"^\*\*{re.escape(field)}\*\*:\s*(.+)$", re.M)
    m = md_pattern.search(text)
    return m.group(1).strip() if m else ""


def issue_title(text: str) -> str:
    lines = text.splitlines()
    frontmatter_lines = []
    in_frontmatter = False
    for line in lines[:30]:
        if line.strip() == "---":
            if in_frontmatter:
                break
            in_frontmatter = True
            continue
        if in_frontmatter:
            frontmatter_lines.append(line)
    
    frontmatter_text = "\n".join(frontmatter_lines)
    
    m = re.search(r'^[ \t]*title:[ \t]*"?(.+?)"?\s*$', frontmatter_text, re.M)
    if m:
        return m.group(1).strip().strip('"')
    m = re.match(r"^#\s+(.+)$", text, re.M)
    return m.group(1).strip() if m else ""


def depends_from_text(text: str) -> list[str]:
    m = YAML_DEPENDS_RE.search(text)
    if m:
        raw = m.group(1).replace("\r", "").replace(",", " ").strip()
        if raw in {"", "none", "None", "[]"}:
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


def load_issues(repo_root: Path) -> list[Issue]:
    issues: list[Issue] = []
    issues_open = repo_root / "issues" / "open"
    issues_done = repo_root / "issues" / "done"
    
    for state, directory in [("open", issues_open), ("done", issues_done)]:
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


def version_key(issue_id: str) -> tuple[int, str]:
    match = re.fullmatch(r"([0-9]+)([a-z]?)", issue_id)
    if not match:
        return (10**9, issue_id)
    number, suffix = match.groups()
    return (int(number), suffix)


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


def truncate(value: str, length: int) -> str:
    if len(value) <= length:
        return value
    return value[: length - 3] + "..."


def escape_cell(value: str) -> str:
    return value.replace("|", "\\|")


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
