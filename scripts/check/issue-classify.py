#!/usr/bin/env python3
"""Phase 0: inventory + classify all issue files.

Outputs:
  artifacts/issue-inventory-report.md  — counts, mismatches, warnings
  artifacts/issue-classification.csv   — per-issue flat table
  artifacts/issue-classification.json  — per-issue stable JSON
"""

import os, sys, json, csv, re
from datetime import datetime

ISSUES_DIR = "issues"
DONE_DIR = os.path.join(ISSUES_DIR, "done")
ALLOWED_STATUS = {"open", "doing", "blocked", "done", "dropped"}
HEADER_RE = re.compile(r"^([A-Za-z]+):\s*(.*)")

# Implementation heuristics (strict)
IMPL_LABEL_PREFIXES = [
    "type:feature", "type:bug",
    "area:runtime", "area:backend", "area:frontend",
    "area:compiler", "area:semantics", "area:ir", "area:host",
]
IMPL_LABEL_CONTAINS = ["feature:"]

IMPL_TITLE_KEYWORDS = [
    "implement", "support", "add ", "runtime:",
    "frontend:", "compiler:", "backend:", "abi:",
    "fix ", "bug", "feature:",
]

CATEGORY_RULES = [
    ("test", ["type:test", "tooling test", "test "]),
    ("refactor", ["refactor", "type:techdebt"]),
    ("docs", ["type:docs", "docs "]),
    ("tooling", ["type:tooling", "type:infrastructure", "tooling "]),
    ("architecture", ["type:architecture", "type:epic"]),
]

# Bare implementation labels (labels without type:/area: prefix that clearly indicate feature work)
BARE_IMPL_LABELS = ["abi", "backend", "runtime", "manifest", "capability", "cli"]

def classify_issue(labels_str, title):
    labels_lower = labels_str.lower()
    title_lower = title.lower()

    # Check non-implementation categories first
    for cat, keywords in CATEGORY_RULES:
        if any(k in labels_lower for k in keywords):
            return cat

    # Check implementation labels
    for prefix in IMPL_LABEL_PREFIXES:
        if prefix in labels_lower:
            return "implementation"
    for substr in IMPL_LABEL_CONTAINS:
        if substr in labels_lower:
            return "implementation"

    # Check bare implementation labels (labels without type:/area: prefix)
    for bare_label in BARE_IMPL_LABELS:
        if bare_label in labels_lower.split():
            return "implementation"

    # Check implementation title keywords
    for kw in IMPL_TITLE_KEYWORDS:
        if kw in title_lower:
            return "implementation"

    return "other"

def parse_header(content):
    parts = content.split("\n---\n", 1)
    header_text = parts[0].strip()
    body = parts[1] if len(parts) > 1 else ""
    header = {}
    for line in header_text.split("\n"):
        line = line.strip()
        if not line:
            continue
        m = HEADER_RE.match(line)
        if m:
            header[m.group(1)] = m.group(2).strip()
    return header, body

def main():
    os.makedirs("artifacts", exist_ok=True)

    # Inventory
    inventory = {
        "total": 0,
        "by_directory": {"issues": 0, "issues/done": 0},
        "by_status": {},
        "mismatches": [],
    }
    rows = []

    for directory, dir_label in [(ISSUES_DIR, "issues"), (DONE_DIR, "issues/done")]:
        if not os.path.isdir(directory):
            continue
        for fn in sorted(os.listdir(directory)):
            if not fn.endswith(".md") or fn == "README.md":
                continue
            path = os.path.join(directory, fn)
            with open(path) as f:
                content = f.read()
            header, body = parse_header(content)

            iid = header.get("Id", fn.replace(".md", ""))
            status = header.get("Status", "unknown")
            priority = header.get("Priority", "")
            labels = header.get("Labels", "")
            title = header.get("Title", "")

            inventory["total"] += 1
            inventory["by_directory"][dir_label] += 1
            inventory["by_status"][status] = inventory["by_status"].get(status, 0) + 1

            # Detect mismatch: status=done file in issues/ or status!=done in issues/done/
            if dir_label == "issues" and status == "done":
                inventory["mismatches"].append(f"{iid}: done file in issues/")
            if dir_label == "issues/done" and status != "done":
                inventory["mismatches"].append(f"{iid}: non-done file in issues/done/ (status={status})")

            category = classify_issue(labels, title)
            has_acceptance = "## Acceptance" in body
            has_evidence = "## Evidence" in body

            rows.append({
                "id": iid,
                "status": status,
                "priority": priority,
                "title": title,
                "labels": labels,
                "category": category,
                "directory": dir_label,
                "has_acceptance": str(has_acceptance),
                "has_evidence": str(has_evidence),
            })

    # Write inventory report
    with open("artifacts/issue-inventory-report.md", "w") as f:
        f.write("# Issue Inventory Report\n\n")
        f.write(f"Generated: {datetime.now().isoformat()}\n\n")
        f.write(f"## Counts\n\n")
        f.write(f"| Metric | Value |\n|---|---|\n")
        f.write(f"| Total files | {inventory['total']} |\n")
        f.write(f"| In issues/ | {inventory['by_directory']['issues']} |\n")
        f.write(f"| In issues/done/ | {inventory['by_directory']['issues/done']} |\n")
        for s, c in sorted(inventory["by_status"].items()):
            f.write(f"| Status={s} | {c} |\n")
        f.write(f"\n## Mismatches\n\n")
        if inventory["mismatches"]:
            for m in inventory["mismatches"]:
                f.write(f"- {m}\n")
        else:
            f.write("None\n")
        f.write(f"\n## Category Counts\n\n")
        cat_counts = {}
        for r in rows:
            cat_counts[r["category"]] = cat_counts.get(r["category"], 0) + 1
        for cat, cnt in sorted(cat_counts.items()):
            f.write(f"- {cat}: {cnt}\n")

    # Write CSV
    with open("artifacts/issue-classification.csv", "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=[
            "id", "status", "priority", "title", "labels",
            "category", "directory", "has_acceptance", "has_evidence",
        ])
        w.writeheader()
        w.writerows(rows)

    # Write JSON
    with open("artifacts/issue-classification.json", "w") as f:
        json.dump({
            "schema_version": 1,
            "generated_at": datetime.now().isoformat(),
            "inventory": inventory,
            "issues": rows,
        }, f, indent=2, ensure_ascii=False)

    # Summary
    impl_count = sum(1 for r in rows if r["category"] == "implementation")
    print(f"Total: {inventory['total']} issues")
    print(f"Implementation: {impl_count}")
    print(f"Mismatches: {len(inventory['mismatches'])}")
    print(f"Artifacts written to artifacts/")

if __name__ == "__main__":
    main()
