#!/usr/bin/env python3
"""Show an issue by ID, legacy ID, or partial ID."""

import os, sys, json

ISSUES_DIR = "issues"
VIEWS_DIR = "issue-views"

def find_issue(query):
    # Direct match
    path = os.path.join(ISSUES_DIR, f"{query}.md")
    if os.path.exists(path):
        return path

    # Strip leading #
    if query.startswith("#"):
        query = query[1:]

    # Search by LegacyId
    for fn in os.listdir(ISSUES_DIR):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        p = os.path.join(ISSUES_DIR, fn)
        with open(p) as f:
            for line in f:
                if line.startswith("Id:") and line.strip() == f"Id: {query}":
                    return p
                if line.startswith("LegacyId:") and line.strip() == f"LegacyId: {query}":
                    return p
                if line.startswith("---"):
                    break

    # Search by partial ID
    for fn in os.listdir(ISSUES_DIR):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        if query in fn:
            return os.path.join(ISSUES_DIR, fn)

    # Search by title word
    for fn in os.listdir(ISSUES_DIR):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        p = os.path.join(ISSUES_DIR, fn)
        with open(p) as f:
            for line in f:
                if line.startswith("Title:"):
                    if query.lower() in line.lower():
                        return p
                if line.startswith("---"):
                    break

    return None

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: issue-show <id|legacy|partial>", file=sys.stderr)
        sys.exit(1)

    path = find_issue(sys.argv[1])
    if not path:
        print(f"Not found: {sys.argv[1]}", file=sys.stderr)
        sys.exit(1)

    with open(path) as f:
        print(f.read(), end="")
