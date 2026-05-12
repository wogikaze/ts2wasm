#!/usr/bin/env python3
"""Generate issue-views/index.json from issues/*.md (active issues only)."""

import os, json
from datetime import datetime, timezone

ISSUES_DIR = "issues"
VIEWS_DIR = "issue-views"
HEADER_RE = __import__("re").compile(r"^([A-Za-z]+):\s*(.*)")

os.makedirs(VIEWS_DIR, exist_ok=True)

def parse(path):
    with open(path) as f:
        content = f.read()
    parts = content.split("\n---\n", 1)
    header = {}
    for line in parts[0].strip().split("\n"):
        m = HEADER_RE.match(line.strip())
        if m:
            header[m.group(1)] = m.group(2).strip()
    return header

issues = []
for fn in sorted(os.listdir(ISSUES_DIR)):
    if fn in ("README.md",):
        continue
    if not fn.endswith(".md"):
        continue
    h = parse(os.path.join(ISSUES_DIR, fn))
    if "Id" in h:
        issues.append(h)

# Build dependency maps
id_to_issue = {h["Id"]: h for h in issues}
dep_map = {}  # id -> {"blocks": [...], "blocked_by": [...]}
for h in issues:
    iid = h["Id"]
    dep_map.setdefault(iid, {"blocks": [], "blocked_by": []})
    for dep in h.get("DependsOn", "").split():
        dep = dep.strip()
        if dep:
            dep_map.setdefault(iid, {"blocks": [], "blocked_by": []})
            dep_map[iid]["blocked_by"].append(dep)
            dep_map.setdefault(dep, {"blocks": [], "blocked_by": []})
            dep_map[dep]["blocks"].append(iid)

# Active = open, doing, blocked
active_statuses = {"open", "doing", "blocked"}
active_issues = [h for h in issues if h.get("Status") in active_statuses]
done_issues = [h for h in issues if h.get("Status") in ("done", "dropped")]

now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S+00:00")

def sort_key(h):
    status = h.get("Status", "")
    doing = 0 if status == "doing" else 1 if status == "open" else 2 if status == "blocked" else 9
    pri = {"P0":0,"P1":1,"P2":2,"P3":3,"P4":4}.get(h.get("Priority","P9"),5)
    return (doing, pri, h.get("Created", ""))

index_issues = []
for h in sorted(active_issues, key=sort_key):
    iid = h["Id"]
    deps = h.get("DependsOn", "").split()
    ready = True
    for dep in deps:
        dep_status = id_to_issue.get(dep, {}).get("Status", "")
        if dep_status in ("open", "doing", "blocked"):
            ready = False
    dm = dep_map.get(iid, {})
    entry = {
        "id": iid,
        "path": f"issues/{iid}.md",
        "legacy_id": int(h["LegacyId"]) if h.get("LegacyId","").isdigit() else None,
        "status": h.get("Status"),
        "priority": h.get("Priority", "P3"),
        "labels": h.get("Labels", "").split(),
        "title": h.get("Title", ""),
        "summary": h.get("Summary", "")[:120],
        "next": h.get("Next", "")[:100],
        "depends_on": [d for d in deps if d],
        "blocked_by": dm.get("blocked_by", []),
        "blocks": dm.get("blocks", []),
        "ready": ready,
        "owner": h.get("Owner", ""),
        "updated": h.get("Updated", ""),
    }
    index_issues.append(entry)

index = {
    "schema_version": 2,
    "generated_at": now,
    "counts": {
        "open": len([h for h in issues if h.get("Status") == "open"]),
        "doing": len([h for h in issues if h.get("Status") == "doing"]),
        "blocked": len([h for h in issues if h.get("Status") == "blocked"]),
        "done": len([h for h in issues if h.get("Status") == "done"]),
        "dropped": len([h for h in issues if h.get("Status") == "dropped"]),
        "total": len(issues),
    },
    "issues": index_issues,
    "omitted": {
        "done": len(done_issues),
    },
}

with open(os.path.join(VIEWS_DIR, "index.json"), "w") as f:
    json.dump(index, f, indent=2, ensure_ascii=False)

print(f"index.json: {len(index_issues)} active + {len(done_issues)} omitted")
