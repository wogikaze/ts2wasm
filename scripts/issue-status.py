#!/usr/bin/env python3
"""Change issue status via script (safer than manual edit)."""

import os, sys
from datetime import datetime, timezone, timedelta

ISSUES_DIR = "issues"
JST = timezone(timedelta(hours=9))
ALLOWED = {"open", "doing", "blocked", "done", "dropped"}

def find_file(query):
    if query.startswith("#"):
        query = query[1:]
    path = os.path.join(ISSUES_DIR, f"{query}.md")
    if os.path.exists(path):
        return path
    for fn in os.listdir(ISSUES_DIR):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        if query in fn:
            return os.path.join(ISSUES_DIR, fn)
    return None

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

    print(f"Status: {new_status}")
