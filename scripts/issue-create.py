#!/usr/bin/env python3
"""Create a new issue file with auto-generated ID."""

import os, re, sys, random, string, json
from datetime import date

ISSUES_DIR = "issues"
VIEWS_DIR = "issue-views"

def gen_id():
    date_part = date.today().strftime("%Y%m%d")
    rand = "".join(random.choices(string.ascii_uppercase + string.digits, k=4))
    return f"I-{date_part}-{rand}"

def get_max_old_id():
    max_id = 0
    if not os.path.isdir(ISSUES_DIR):
        return max_id
    for fn in os.listdir(ISSUES_DIR):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        with open(os.path.join(ISSUES_DIR, fn)) as f:
            for line in f:
                if line.startswith("OldId:"):
                    try:
                        mid = int(line.split(":")[1].strip())
                        if mid > max_id:
                            max_id = mid
                    except: pass
                if line.startswith("---"):
                    break
    return max_id

def esc(val):
    """Auto-quote value if it contains unquoted colon."""
    if ":" in val and not (val.startswith('"') and val.endswith('"')):
        return f'"{val}"'
    return val

def write_issue(title, priority="P2", labels="", summary="", next_step="",
                 depends_on="", status="open"):
    # Ensure unique ID
    for _ in range(10):
        iid = gen_id()
        path = os.path.join(ISSUES_DIR, f"{iid}.md")
        if not os.path.exists(path):
            break
    else:
        print("error: could not generate unique ID", file=sys.stderr)
        sys.exit(1)

    old_id = get_max_old_id() + 1
    today = date.today().isoformat()

    # Auto-escape colons in header values
    title = esc(title)
    summary = esc(summary)
    next_step = esc(next_step)

    with open(path, "w") as f:
        f.write(f"Id: {iid}\n")
        f.write(f"OldId: {old_id}\n")
        f.write(f"Status: {status}\n")
        f.write(f"Priority: {priority}\n")
        if labels:
            f.write(f"Labels: {labels}\n")
        if depends_on:
            f.write(f"DependsOn: {depends_on}\n")
        f.write(f"Created: {today}\n")
        f.write(f"Updated: {today}\n")
        f.write(f"Title: {title}\n")
        if summary:
            f.write(f"Summary: {summary}\n")
        if next_step:
            f.write(f"Next: {next_step}\n")
        f.write("\n---\n")
        f.write(f"\n## Notes\n\n")

    print(f"Created: {path}")
    print(f"  Id: {iid}")
    print(f"  OldId: {old_id}")
    return iid

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Create a new issue")
    parser.add_argument("title", help="Issue title")
    parser.add_argument("--priority", "-p", default="P2", choices=["P0","P1","P2","P3","P4"])
    parser.add_argument("--labels", "-l", default="", help="Space-separated labels")
    parser.add_argument("--summary", "-s", default="", help="One-line summary (max 150 chars)")
    parser.add_argument("--next", "-n", default="", help="Next action")
    parser.add_argument("--depends-on", "-d", default="", help="Space-separated issue IDs")
    parser.add_argument("--reindex", "-r", action="store_true", help="Re-run issue-index after creation")

    args = parser.parse_args()

    if len(args.title) > 120:
        print(f"error: title too long ({len(args.title)} chars)", file=sys.stderr)
        sys.exit(1)

    write_issue(
        title=args.title,
        priority=args.priority,
        labels=args.labels,
        summary=args.summary,
        next_step=args.next,
        depends_on=args.depends_on,
    )

    if args.reindex:
        os.system("python scripts/issue-index.py")
