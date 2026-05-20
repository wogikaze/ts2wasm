#!/usr/bin/env python3
"""Change issue status via script (safer than manual edit)."""

import os, re, sys
from datetime import datetime, timezone, timedelta

ISSUES_DIR = "issues"
DONE_DIR = os.path.join(ISSUES_DIR, "done")
JST = timezone(timedelta(hours=9))
ALLOWED = {"open", "doing", "blocked", "done", "dropped"}

def find_file(query):
    if query.startswith("#"):
        query = query[1:]
    for base in (ISSUES_DIR, DONE_DIR):
        path = os.path.join(base, f"{query}.md")
        if os.path.exists(path):
            return path
    for base in (ISSUES_DIR, DONE_DIR):
        if not os.path.isdir(base):
            continue
        for fn in os.listdir(base):
            if not fn.endswith(".md") or fn == "README.md":
                continue
            if query in fn:
                return os.path.join(base, fn)
    return None

def has_test_requirements(body):
    """Check if body contains ## Test-Requirements with non-empty content."""
    m = re.search(r"## Test-Requirements\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if not m:
        return False
    content = m.group(1).strip()
    return bool(content) and content != "-"

def is_implementation_issue(content):
    """Heuristic: labels or title suggest implementation."""
    return any(tag in content for tag in (
        "type:feature", "type:bug", "area:runtime", "area:backend",
        "area:frontend", "area:compiler", "area:semantics", "area:ir",
        "area:host", "feature:"
    ))

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Change issue status")
    parser.add_argument("id", help="Issue ID (full, legacy#, or partial)")
    parser.add_argument("status", choices=list(ALLOWED))
    parser.add_argument("--owner", "-o", default=None, help="Owner (required for doing)")
    parser.add_argument("--reason", "-r", default=None, help="BlockedReason")
    parser.add_argument("--evidence", "-e", default=None, help="Evidence text for done")
    args = parser.parse_args()

    path = find_file(args.id)
    if not path:
        print(f"Not found: {args.id}", file=sys.stderr)
        sys.exit(1)

    with open(path) as f:
        content = f.read()

    parts = content.split("\n---\n", 1)
    header_lines = parts[0].strip().split("\n")
    body = parts[1] if len(parts) > 1 else ""

    new_status = args.status

    # Gate: done requires evidence
    if new_status == "done" and not args.evidence:
        print("ERROR: --evidence is required for status=done", file=sys.stderr)
        sys.exit(1)

    # Gate: implementation issues require Test-Requirements
    if new_status == "done" and is_implementation_issue(content):
        if not has_test_requirements(body):
            print("ERROR: Implementation issues need ## Test-Requirements section for status=done", file=sys.stderr)
            sys.exit(1)

    now = datetime.now(JST).strftime("%Y-%m-%dT%H:%M:%S+09:00")

    new_lines = []
    for line in header_lines:
        if line.startswith("Status:"):
            new_lines.append(f"Status: {new_status}")
        elif line.startswith("Updated:"):
            new_lines.append(f"Updated: {now}")
        elif line.startswith("Owner:"):
            if args.owner is not None:
                new_lines.append(f"Owner: {args.owner}")
            elif new_status in ("done", "dropped"):
                new_lines.append(f"Owner: ")
            else:
                new_lines.append(line)
        elif line.startswith("BlockedReason:"):
            if args.reason is not None:
                new_lines.append(f"BlockedReason: {args.reason}")
            elif new_status != "blocked":
                new_lines.append(f"BlockedReason: ")
            else:
                new_lines.append(line)
        elif line.startswith("Next:"):
            if new_status in ("done", "dropped"):
                new_lines.append(f"Next: ")
            else:
                new_lines.append(line)
        else:
            new_lines.append(line)

    # Add evidence if provided
    if args.evidence:
        if "## Evidence" not in body:
            body = body.rstrip() + f"\n\n## Evidence\n- {args.evidence}\n"
        else:
            body = body.replace("## Evidence", f"## Evidence\n- {args.evidence}\n", 1)

    with open(path, "w") as f:
        f.write("\n".join(new_lines))
        f.write("\n---\n\n")
        f.write(body.strip())
        f.write("\n")

    if os.path.dirname(path) == DONE_DIR and new_status not in ("done", "dropped"):
        new_path = os.path.join(ISSUES_DIR, os.path.basename(path))
        os.replace(path, new_path)
        path = new_path

    print(f"Status: {new_status}")
