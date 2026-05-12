#!/usr/bin/env python3
"""Lint all issue files under issues/."""

import os, re, sys

ISSUES_DIR = "issues"
ALLOWED_STATUSES = {"open", "doing", "blocked", "done", "dropped"}
HEADER_FIELDS = {"Id", "OldId", "Status", "Priority", "Labels", "DependsOn", "Created", "Updated", "Title", "Summary", "Next"}

errors = 0

def error(msg, path=""):
    global errors
    prefix = f"{path}: " if path else ""
    print(f"  ERROR {prefix}{msg}")
    errors += 1

def warn(msg, path=""):
    prefix = f"{path}: " if path else ""
    print(f"  WARN  {prefix}{msg}")

issues = {}
seen_ids = {}

for fn in sorted(os.listdir(ISSUES_DIR)):
    if fn in ("README.md",):
        continue
    if not fn.endswith(".md"):
        continue
    path = os.path.join(ISSUES_DIR, fn)

    with open(path) as f:
        content = f.read()

    # Split header from body
    parts = content.split("\n---\n", 1)
    header_text = parts[0].strip()
    body = parts[1] if len(parts) > 1 else ""

    # Parse header lines
    header = {}
    for line in header_text.split("\n"):
        line = line.strip()
        if not line:
            continue
        if ":" not in line:
            error(f"Header line without colon: {line[:50]}", fn)
            continue
        key, _, val = line.partition(":")
        key = key.strip()
        val = val.strip()
        if not key:
            error(f"Empty key in header line", fn)
            continue
        if key not in HEADER_FIELDS:
            warn(f"Unknown header field: {key}", fn)
        if key in header:
            error(f"Duplicate header field: {key}", fn)
        header[key] = val

    # --- Checks ---

    # Id exists
    if "Id" not in header:
        error("Missing Id header", fn)
        continue

    issue_id = header["Id"]

    # Id matches filename
    expected_fn = f"{issue_id}.md"
    if fn != expected_fn:
        error(f"Filename mismatch: expected {expected_fn}, got {fn}")

    # No duplicate Id
    if issue_id in seen_ids:
        error(f"Duplicate Id: {issue_id} (also in {seen_ids[issue_id]})", fn)
    seen_ids[issue_id] = fn

    # Status
    status = header.get("Status", "")
    if not status:
        error("Missing Status header", fn)
    elif status not in ALLOWED_STATUSES:
        error(f"Invalid Status: '{status}' (allowed: {', '.join(sorted(ALLOWED_STATUSES))})", fn)

    # Unquoted colon in header values
    for k in header:
        v = header[k]
        if ":" in v and not (v.startswith('"') and v.endswith('"')):
            warn(f"Value for '{k}' contains unquoted colon — wrap in double quotes if intentional", fn)

    # DependsOn cycles / self-reference
    deps = header.get("DependsOn", "")
    if issue_id in deps.split():
        error(f"Issue depends on itself: {issue_id}", fn)

    # Required fields
    for req in ("Title", "Created"):
        if req not in header:
            error(f"Missing required header: {req}", fn)

    # Updated format check
    created = header.get("Created", "")
    updated = header.get("Updated", "")
    date_pat = r"^\d{4}-\d{2}-\d{2}$"
    if created and not re.match(date_pat, created):
        error(f"Created date format (expected YYYY-MM-DD): {created}", fn)
    if updated and not re.match(date_pat, updated):
        error(f"Updated date format (expected YYYY-MM-DD): {updated}", fn)

    # Summary length
    summary = header.get("Summary", "")
    if len(summary) > 150:
        error(f"Summary too long ({len(summary)} chars, max 150)", fn)

    # Next exists for open
    if status == "open" and "Next" not in header:
        warn("Open issue without Next field", fn)

    # No YAML-like nesting in header values
    for k, v in header.items():
        if "\n" in v:
            error(f"Multi-line value in header field {k}", fn)

    issues[issue_id] = fn

# Second pass: check DependsOn against known issues
for fn in sorted(os.listdir(ISSUES_DIR)):
    if fn in ("README.md",):
        continue
    if not fn.endswith(".md"):
        continue
    path = os.path.join(ISSUES_DIR, fn)
    with open(path) as f:
        content = f.read()
    header_text = content.split("\n---\n", 1)[0]
    header = {}
    for line in header_text.strip().split("\n"):
        if ":" in line:
            k, _, v = line.partition(":")
            header[k.strip()] = v.strip()
    deps = header.get("DependsOn", "")
    for dep_id in deps.split():
        if dep_id not in seen_ids:
            error(f"DependsOn references unknown issue: {dep_id}", fn)

if errors:
    print(f"\n{errors} error(s) found")
    sys.exit(1)
else:
    print("All issues valid")
