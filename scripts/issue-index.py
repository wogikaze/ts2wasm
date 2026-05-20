#!/usr/bin/env python3
"""Generate issue-views/index.json from repo-local issue files."""

import json
import os
import re
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ISSUES_DIR = Path("issues")
DONE_DIR = ISSUES_DIR / "done"
VIEWS_DIR = Path("issue-views")
HEADER_RE = re.compile(r"^([A-Za-z]+):\s*(.*)")

VIEWS_DIR.mkdir(exist_ok=True)
DONE_DIR.mkdir(exist_ok=True)


def parse_issue(path: Path) -> dict:
    content = path.read_text()
    parts = content.split("\n---\n", 1)
    header = {}
    for line in parts[0].strip().split("\n"):
        m = HEADER_RE.match(line.strip())
        if m:
            header[m.group(1)] = m.group(2).strip()
    header["_path"] = str(path)
    header["_body"] = parts[1] if len(parts) > 1 else ""
    return header


def issue_files():
    for root in (ISSUES_DIR, DONE_DIR):
        if not root.exists():
            continue
        for path in sorted(root.glob("*.md")):
            if path.name == "README.md":
                continue
            yield path


def test_requirement_commands(body: str) -> list[str]:
    match = re.search(r"## Test-Requirements\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if not match:
        return []
    commands = []
    for line in match.group(1).splitlines():
        stripped = line.strip()
        if not stripped.startswith("- Test command"):
            continue
        _, _, value = stripped.partition(":")
        cmd = value.strip().strip("`")
        if cmd:
            commands.append(cmd)
    return commands


def has_open_checkboxes(body: str) -> bool:
    return "[ ]" in body


def can_archive_done_issue(path: Path, body: str) -> tuple[bool, str]:
    if has_open_checkboxes(body):
        return False, "contains unchecked [ ] items"

    for cmd in test_requirement_commands(body):
        result = subprocess.run(
            cmd,
            shell=True,
            cwd=Path.cwd(),
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            stderr = result.stderr.strip().splitlines()
            detail = stderr[0] if stderr else f"exit {result.returncode}"
            return False, f"verification failed: `{cmd}` ({detail})"

    return True, ""


def archive_done_issues() -> list[str]:
    moved = []
    for path in sorted(ISSUES_DIR.glob("*.md")):
        if path.name == "README.md":
            continue
        issue = parse_issue(path)
        if issue.get("Status") not in {"done", "dropped"}:
            continue
        ok, reason = can_archive_done_issue(path, issue["_body"])
        if not ok:
            print(
                f"issue-index: keep {path.name} in issues/ — {reason}",
                file=sys.stderr,
            )
            continue
        target = DONE_DIR / path.name
        if target.exists():
            target.unlink()
        shutil.move(str(path), str(target))
        moved.append(path.name)
    return moved


def sort_key(header: dict) -> tuple:
    status = header.get("Status", "")
    doing = 0 if status == "doing" else 1 if status == "open" else 2 if status == "blocked" else 9
    pri = {"P0": 0, "P1": 1, "P2": 2, "P3": 3, "P4": 4}.get(header.get("Priority", "P9"), 5)
    return (doing, pri, header.get("Created", ""))


def main() -> int:
    moved = archive_done_issues()

    issues = []
    for path in issue_files():
        issue = parse_issue(path)
        if "Id" in issue:
            issues.append(issue)

    id_to_issue = {issue["Id"]: issue for issue in issues}
    dep_map = {}
    for issue in issues:
        iid = issue["Id"]
        dep_map.setdefault(iid, {"blocks": [], "blocked_by": []})
        for dep in issue.get("DependsOn", "").split():
            dep = dep.strip()
            if dep:
                dep_map.setdefault(iid, {"blocks": [], "blocked_by": []})
                dep_map[iid]["blocked_by"].append(dep)
                dep_map.setdefault(dep, {"blocks": [], "blocked_by": []})
                dep_map[dep]["blocks"].append(iid)

    active_statuses = {"open", "doing", "blocked"}
    active_issues = [issue for issue in issues if issue.get("Status") in active_statuses]
    done_issues = [issue for issue in issues if issue.get("Status") in ("done", "dropped")]

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S+00:00")

    index_issues = []
    for issue in sorted(active_issues, key=sort_key):
        iid = issue["Id"]
        deps = issue.get("DependsOn", "").split()
        ready = True
        for dep in deps:
            dep_status = id_to_issue.get(dep, {}).get("Status", "")
            if dep_status in ("open", "doing", "blocked"):
                ready = False
        dm = dep_map.get(iid, {})
        entry = {
            "id": iid,
            "path": issue["_path"],
            "legacy_id": int(issue["LegacyId"]) if issue.get("LegacyId", "").isdigit() else None,
            "status": issue.get("Status"),
            "priority": issue.get("Priority", "P3"),
            "labels": issue.get("Labels", "").split(),
            "title": issue.get("Title", ""),
            "summary": issue.get("Summary", "")[:120],
            "next": issue.get("Next", "")[:100],
            "depends_on": [d for d in deps if d],
            "blocked_by": dm.get("blocked_by", []),
            "blocks": dm.get("blocks", []),
            "ready": ready,
            "owner": issue.get("Owner", ""),
            "updated": issue.get("Updated", ""),
        }
        index_issues.append(entry)

    index = {
        "schema_version": 2,
        "generated_at": now,
        "counts": {
            "open": len([issue for issue in issues if issue.get("Status") == "open"]),
            "doing": len([issue for issue in issues if issue.get("Status") == "doing"]),
            "blocked": len([issue for issue in issues if issue.get("Status") == "blocked"]),
            "done": len([issue for issue in issues if issue.get("Status") == "done"]),
            "dropped": len([issue for issue in issues if issue.get("Status") == "dropped"]),
            "total": len(issues),
        },
        "issues": index_issues,
        "omitted": {
            "done": len(done_issues),
        },
    }

    (VIEWS_DIR / "index.json").write_text(json.dumps(index, indent=2, ensure_ascii=False))

    if moved:
        print(f"archived: {', '.join(moved)}")
    print(f"index.json: {len(index_issues)} active + {len(done_issues)} omitted")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
