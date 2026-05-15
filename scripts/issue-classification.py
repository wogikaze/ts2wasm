#!/usr/bin/env python3
"""Phase 0: Issue Classification — inventory + categorization.

Produces:
  - artifacts/issue-inventory-report.md
  - artifacts/issue-classification.csv
  - artifacts/issue-classification.json
"""

import csv, json, os, re, sys
from collections import defaultdict
from datetime import datetime, timezone

ISSUES_DIR = "issues"
DONE_DIR = os.path.join(ISSUES_DIR, "done")
OUT_DIR = "artifacts"
NOW = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

# Category rules
CATEGORY_RULES = [
    (re.compile(r"\btype:(?:feature|bug)\b", re.I), "implementation"),
    (re.compile(r"\barea:(?:runtime|backend|frontend|compiler|semantics|ir|host)\b", re.I), "implementation"),
    (re.compile(r"\bfeature:", re.I), "implementation"),
    (re.compile(r"\btype:test\b", re.I), "test"),
    (re.compile(r"\btype:techdebt\b", re.I), "refactor"),
    (re.compile(r"\brefactor\b", re.I), "refactor"),
    (re.compile(r"\btype:docs\b", re.I), "docs"),
    (re.compile(r"\btype:tooling\b", re.I), "tooling"),
    (re.compile(r"\btype:infrastructure\b", re.I), "tooling"),
    (re.compile(r"\btype:architecture\b", re.I), "architecture"),
    (re.compile(r"\btype:epic\b", re.I), "architecture"),
]

def parse_header(path):
    """Parse issue header (key: value lines before first standalone ---)."""
    meta = {}
    with open(path) as f:
        content = f.read()
    parts = content.split("\n---\n", 1)
    if len(parts) < 2:
        body = parts[0]
    else:
        header_text, body = parts
    for line in header_text.strip().split("\n"):
        line = line.strip()
        if ":" in line:
            key, _, val = line.partition(":")
            meta[key.strip()] = val.strip()
    return meta, body


def classify(labels_str, title):
    """Classify an issue into category based on labels and title."""
    labels_lower = (labels_str + " " + title).lower()
    for pattern, cat in CATEGORY_RULES:
        if pattern.search(labels_lower):
            return cat
    # Fallback: any area:* label
    if re.search(r"\barea:", labels_lower):
        return "implementation"
    return "other"


def extract_test_commands(body):
    """Extract test commands from Acceptance section."""
    commands = []
    m = re.search(r"## Acceptance\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if m:
        for line in m.group(1).strip().split("\n"):
            line = line.strip().lstrip("- ").strip()
            if line and not line.startswith("[") and not line.startswith("```"):
                commands.append(line)
    return commands


def has_evidence(body):
    """Check if Evidence section exists with content beyond 'Closed in...'."""
    m = re.search(r"## Evidence\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if not m:
        return False
    evidence = m.group(1).strip()
    # Ignore placeholder evidence
    if not evidence or re.match(r"^-?\s*Closed in", evidence):
        return False
    return True


def check_dir_mismatch(status, dirpath):
    """Check if header Status matches directory location."""
    if dirpath.endswith("/done"):
        return status not in ("done", "dropped")
    return status == "done"


def main():
    os.makedirs(OUT_DIR, exist_ok=True)

    issues = []
    mismatches = []
    counts = defaultdict(int)
    cat_counts = defaultdict(int)
    status_counts = defaultdict(int)

    # Scan issues/ (open)
    for fn in sorted(os.listdir(ISSUES_DIR)):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        path = os.path.join(ISSUES_DIR, fn)
        meta, body = parse_header(path)
        status = meta.get("Status", "unknown")
        labels = meta.get("Labels", "")
        title = meta.get("Title", "")
        category = classify(labels, title)
        has_acc = bool(re.search(r"## Acceptance", body))
        has_ev = has_evidence(body)
        test_cmds = extract_test_commands(body)

        counts["open"] += 1
        status_counts[status] += 1
        cat_counts[category] += 1

        if check_dir_mismatch(status, ISSUES_DIR):
            mismatches.append({
                "id": meta.get("Id", fn),
                "path": path,
                "status": status,
                "directory": "issues/",
                "problem": f"Status is '{status}' but located in issues/ (not done/)"
            })

        entry = {
            "issue_id": meta.get("Id", fn.replace(".md", "")),
            "path": path,
            "status": status,
            "priority": meta.get("Priority", ""),
            "title": title,
            "summary": meta.get("Summary", ""),
            "labels": labels,
            "category": category,
            "has_acceptance": has_acc,
            "has_evidence": has_ev,
            "test_commands_found": test_cmds,
        }
        issues.append(entry)

    # Scan issues/done/
    for fn in sorted(os.listdir(DONE_DIR)):
        if not fn.endswith(".md"):
            continue
        path = os.path.join(DONE_DIR, fn)
        meta, body = parse_header(path)
        status = meta.get("Status", "unknown")
        labels = meta.get("Labels", "")
        title = meta.get("Title", "")
        category = classify(labels, title)
        has_acc = bool(re.search(r"## Acceptance", body))
        has_ev = has_evidence(body)
        test_cmds = extract_test_commands(body)

        counts["done"] += 1
        status_counts[status] += 1
        cat_counts[category] += 1

        if check_dir_mismatch(status, DONE_DIR):
            mismatches.append({
                "id": meta.get("Id", fn),
                "path": path,
                "status": status,
                "directory": "issues/done/",
                "problem": f"Status is '{status}' but located in issues/done/"
            })

        entry = {
            "issue_id": meta.get("Id", fn.replace(".md", "")),
            "path": path,
            "status": status,
            "priority": meta.get("Priority", ""),
            "title": title,
            "summary": meta.get("Summary", ""),
            "labels": labels,
            "category": category,
            "has_acceptance": has_acc,
            "has_evidence": has_ev,
            "test_commands_found": test_cmds,
        }
        issues.append(entry)

    total = len(issues)
    counts["total"] = total

    # --- Inventory Report ---
    report = []
    report.append("# Issue Inventory Report\n")
    report.append(f"Generated: {NOW}\n")

    report.append("## Counts\n")
    report.append(f"| Metric | Value |")
    report.append(f"|---|---|")
    report.append(f"| Total issue files | {total} |")
    report.append(f"| issues/ (active) | {counts['open']} |")
    report.append(f"| issues/done/ | {counts['done']} |")

    report.append(f"\n## Status Distribution (from headers)\n")
    report.append(f"| Status | Count |")
    report.append(f"|---|---|")
    for s in sorted(status_counts):
        report.append(f"| {s} | {status_counts[s]} |")

    report.append(f"\n## Category Distribution\n")
    report.append(f"| Category | Count |")
    report.append(f"|---|---|")
    for c in sorted(cat_counts):
        report.append(f"| {c} | {cat_counts[c]} |")

    report.append(f"\n## Implementation Issues (needs test coverage)\n")
    impl_issues = [i for i in issues if i["category"] == "implementation"]
    report.append(f"| Issue ID | Status | Priority | Title | Has Acceptance | Has Evidence |")
    report.append(f"|---|---|---|---|---|---|")
    for i in impl_issues:
        report.append(f"| {i['issue_id']} | {i['status']} | {i['priority']} | {i['title'][:60]} | {i['has_acceptance']} | {i['has_evidence']} |")

    if mismatches:
        report.append(f"\n## Directory/Status Mismatches\n")
        report.append(f"| Issue | Path | Status | Problem |")
        report.append(f"|---|---|---|---|")
        for m in mismatches:
            report.append(f"| {m['id']} | {m['path']} | {m['status']} | {m['problem']} |")
    else:
        report.append(f"\n## Directory/Status Mismatches\n\nNone found.\n")

    report.append(f"\n## Warnings\n")
    done_no_evidence = [i for i in issues if i["status"] == "done" and not i["has_evidence"]]
    if done_no_evidence:
        report.append(f"### Done issues with missing/placeholder evidence ({len(done_no_evidence)})\n")
        for i in done_no_evidence:
            report.append(f"- {i['issue_id']}: {i['title'][:60]}")
    else:
        report.append("All done issues have evidence.\n")

    open_no_next = [i for i in issues if i["status"] in ("open", "doing") and i["category"] == "implementation"]
    # Check for Next field
    for i in open_no_next:
        meta, _ = parse_header(i["path"])
        if not meta.get("Next", "").strip():
            report.append(f"- {i['issue_id']}: open/doing implementation issue without Next field")

    with open(os.path.join(OUT_DIR, "issue-inventory-report.md"), "w") as f:
        f.write("\n".join(report))

    # --- CSV ---
    with open(os.path.join(OUT_DIR, "issue-classification.csv"), "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["issue_id", "status", "priority", "title", "labels", "category",
                         "has_acceptance", "has_evidence", "test_commands_found"])
        for i in issues:
            writer.writerow([
                i["issue_id"], i["status"], i["priority"], i["title"],
                i["labels"], i["category"], i["has_acceptance"],
                i["has_evidence"], "; ".join(i["test_commands_found"])
            ])

    # --- JSON ---
    json_path = os.path.join(OUT_DIR, "issue-classification.json")
    with open(json_path, "w") as f:
        json.dump(issues, f, indent=2, ensure_ascii=False)

    print(f"Total: {total} issues ({counts['open']} active, {counts['done']} done)")
    print(f"Categories: {dict(cat_counts)}")
    print(f"Status counts: {dict(status_counts)}")
    print(f"Mismatches: {len(mismatches)}")
    print(f"Warnings: {len(done_no_evidence)} done without evidence")
    print(f"Output: {os.path.join(OUT_DIR, 'issue-inventory-report.md')}")
    print(f"Output: {os.path.join(OUT_DIR, 'issue-classification.csv')}")
    print(f"Output: {json_path}")


if __name__ == "__main__":
    main()
