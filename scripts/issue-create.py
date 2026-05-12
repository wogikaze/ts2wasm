#!/usr/bin/env python3
"""Create a new issue file."""

import os, sys, random, string
from datetime import datetime, timezone, timedelta

ISSUES_DIR = "issues"
CROCKFORD = "ABCDEFGHJKMNPQRSTVWXYZ23456789"  # no 0 O 1 I L
JST = timezone(timedelta(hours=9))
NOW = datetime.now(JST)

def gen_id():
    date_part = NOW.strftime("%Y%m%d")
    rand = "".join(random.choices(CROCKFORD, k=6))
    return f"I-{date_part}-{rand}"

def esc(val):
    return val  # no escaping needed, first : splits key/value

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Create new issue")
    parser.add_argument("title", help="Title")
    parser.add_argument("-p", "--priority", default="P2", choices=["P0","P1","P2","P3","P4"])
    parser.add_argument("-l", "--labels", default="", help="Space-separated labels")
    parser.add_argument("-s", "--summary", default="", help="One-line summary")
    parser.add_argument("-n", "--next", default="", help="Next action")
    parser.add_argument("-d", "--depends-on", default="", help="DependsOn IDs")
    args = parser.parse_args()

    if len(args.title) > 120:
        print("error: title too long", file=sys.stderr)
        sys.exit(1)

    for _ in range(10):
        iid = gen_id()
        path = os.path.join(ISSUES_DIR, f"{iid}.md")
        if not os.path.exists(path):
            break
    else:
        print("error: cannot generate unique ID", file=sys.stderr)
        sys.exit(1)

    ts = NOW.strftime("%Y-%m-%dT%H:%M:%S+09:00")

    with open(path, "w") as f:
        f.write(f"Id: {iid}\n")
        f.write(f"Status: open\n")
        f.write(f"Priority: {args.priority}\n")
        if args.labels:
            f.write(f"Labels: {args.labels}\n")
        if args.depends_on:
            f.write(f"DependsOn: {args.depends_on}\n")
        f.write(f"Owner: \n")
        f.write(f"BlockedReason: \n")
        f.write(f"Created: {ts}\n")
        f.write(f"Updated: {ts}\n")
        f.write(f"Title: {args.title}\n")
        if args.summary:
            f.write(f"Summary: {args.summary}\n")
        if args.next:
            f.write(f"Next: {args.next}\n")
        f.write("\n---\n\n## Notes\n\n")

    print(f"Created: {path}")
