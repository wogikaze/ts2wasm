#!/usr/bin/env python3
"""Generate issue-views/index.json from issues/*.md files."""

import os, json

ISSUES_DIR = "issues"
VIEWS_DIR = "issue-views"

os.makedirs(VIEWS_DIR, exist_ok=True)

def parse_issue(path):
    with open(path) as f:
        content = f.read()
    parts = content.split("\n---\n", 1)
    header = {}
    for line in parts[0].strip().split("\n"):
        line = line.strip()
        if ":" in line:
            k, _, v = line.partition(":")
            header[k.strip()] = v.strip().strip('"')
    return header

issues = []
for fn in sorted(os.listdir(ISSUES_DIR)):
    if fn in ("README.md",):
        continue
    if not fn.endswith(".md"):
        continue
    h = parse_issue(os.path.join(ISSUES_DIR, fn))
    if "Id" in h:
        issues.append(h)

sorted_issues = sorted(issues, key=lambda x: (
    0 if x.get("Status") == "open" else 1 if x.get("Status") in ("doing","blocked") else 2,
    {"P0":0,"P1":1,"P2":2,"P3":3,"P4":4}.get(x.get("Priority","P9"),5),
    x.get("Created",""),
))

index = []
for h in sorted_issues:
    depends_on = h.get("DependsOn", "").split()
    index.append({
        "id": h.get("Id", ""),
        "old_id": int(h["OldId"]) if h.get("OldId","").isdigit() else 0,
        "status": h.get("Status", "open"),
        "priority": h.get("Priority", "P3"),
        "labels": h.get("Labels", "").split(),
        "title": h.get("Title", ""),
        "summary": h.get("Summary", "")[:120],
        "next": h.get("Next", "")[:80],
        "depends_on": [d for d in depends_on if d],
        "updated": h.get("Updated", ""),
    })

with open(os.path.join(VIEWS_DIR, "index.json"), "w") as f:
    json.dump(index, f, indent=2, ensure_ascii=False)

print(f"index.json: {len(index)} items")
