#!/usr/bin/env python3
"""Validate all issue files."""

import os, sys, re
from datetime import datetime

ISSUES_DIR = "issues"
DONE_DIR = os.path.join(ISSUES_DIR, "done")
ALLOWED_STATUS = {"open", "doing", "blocked", "done", "dropped"}
ALLOWED_PRIORITY = {"P0", "P1", "P2", "P3", "P4"}
KNOWN_FIELDS = {"Id", "LegacyId", "Status", "Priority", "Labels", "DependsOn",
                "Related", "Owner", "BlockedReason", "Created", "Updated",
                "Title", "Summary", "Next"}
FIELD_ORDER = ["Id", "LegacyId", "Status", "Priority", "Labels", "DependsOn",
               "Related", "Owner", "BlockedReason", "Created", "Updated",
               "Title", "Summary", "Next"]

errors = 0

def error(msg, fn=""):
    global errors
    p = f"{fn}: " if fn else ""
    print(f"  ERROR {p}{msg}")
    errors += 1

def warn(msg, fn=""):
    p = f"{fn}: " if fn else ""
    print(f"  WARN  {p}{msg}")

HEADER_RE = re.compile(r"^([A-Za-z]+):\s*(.*)")
TS_RE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$")
ID_RE = re.compile(r"^I-\d{8}-[A-HJ-NP-Z2-9]{6}$")

seen_ids = set()
seen_legacy = set()
all_parsed = []
check_test_refs = []

def iter_issue_files():
    for root in (ISSUES_DIR, DONE_DIR):
        if not os.path.isdir(root):
            continue
        for fn in sorted(os.listdir(root)):
            if fn in ("README.md",):
                continue
            if not fn.endswith(".md"):
                continue
            yield root, fn, os.path.join(root, fn)


for root, fn, path in iter_issue_files():

    with open(path) as f:
        content = f.read()

    parts = content.split("\n---\n", 1)
    header_text = parts[0].strip()
    body = parts[1] if len(parts) > 1 else ""

    header = {}
    seen_fields = []
    for line in header_text.split("\n"):
        line = line.strip()
        if not line:
            continue
        m = HEADER_RE.match(line)
        if not m:
            error(f"Cannot parse: {line[:40]}", fn)
            continue
        k, v = m.group(1).strip(), m.group(2).strip()
        if k in header:
            error(f"Duplicate: {k}", fn)
        header[k] = v
        seen_fields.append(k)

    iid = header.get("Id", "")

    # Per-file checks
    if not iid:
        error("Missing Id", fn)
    else:
        if not ID_RE.match(iid):
            error(f"Invalid Id format: {iid}", fn)
        if iid in seen_ids:
            error(f"Duplicate Id: {iid}", fn)
        seen_ids.add(iid)
        exp_fn = f"{iid}.md"
        if fn != exp_fn:
            error(f"Filename: expected {exp_fn}", fn)

    lid = header.get("LegacyId", "")
    if lid:
        if not lid.isdigit():
            error(f"LegacyId not numeric: {lid}", fn)
        elif lid in seen_legacy:
            error(f"Duplicate LegacyId: {lid}", fn)
        seen_legacy.add(lid)

    status = header.get("Status", "")
    if not status:
        error("Missing Status", fn)
    elif status not in ALLOWED_STATUS:
        error(f"Bad Status: {status}", fn)
    else:
        if status == "open" and "Next" not in header:
            error("Open needs Next", fn)
        if status == "doing":
            if not header.get("Owner", "").strip():
                error("Doing needs Owner", fn)
            if "Next" not in header:
                error("Doing needs Next", fn)
        if status == "blocked":
            if not header.get("DependsOn", "") and not header.get("BlockedReason", ""):
                error("Blocked needs DependsOn or BlockedReason", fn)
        if status in ("done",):
            if header.get("Next", ""):
                error("Done should not have Next", fn)
            if header.get("Owner", "").strip():
                error("Done should not have Owner", fn)
            if "## Evidence" not in body and "# Evidence" not in body:
                error("Done without Evidence section", fn)
            # Verify evidence references test files
            ev_m = re.search(r"## Evidence\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
            if ev_m:
                ev_text = ev_m.group(1)
                for line in ev_text.split("\n"):
                    line = line.strip().lstrip("- ").strip()
                    # Check cargo test commands reference real tests
                    m2 = re.match(r"cargo (nextest )?run.*?\s(build_smoke_\w+)", line)
                    if m2:
                        test_name = m2.group(2)
                        cmd = f"cargo nextest run -p ts2wasm-cli --test builtin_methods {test_name} --no-fail-fast"
                        check_test_refs.append({"test": test_name, "cmd": cmd, "fn": fn})
        if status == "dropped":
            if not header.get("BlockedReason", "") and "## Notes" not in body:
                error("Dropped needs BlockedReason or Notes", fn)

    pri = header.get("Priority", "")
    if pri and pri not in ALLOWED_PRIORITY:
        error(f"Bad Priority: {pri}", fn)

    created = header.get("Created", "")
    updated = header.get("Updated", "")
    if created and not TS_RE.match(created):
        error(f"Created not RFC3339: {created}", fn)
    if updated and not TS_RE.match(updated):
        error(f"Updated not RFC3339: {updated}", fn)
    if created and updated and updated < created:
        error("Updated < Created", fn)

    if "Title" not in header:
        error("Missing Title", fn)

    summary = header.get("Summary", "")
    if len(summary) > 150:
        error(f"Summary >150 ({len(summary)})", fn)

    next_val = header.get("Next", "")
    if len(next_val) > 200:
        error(f"Next >200 ({len(next_val)})", fn)

    labels = header.get("Labels", "")
    if labels and len(labels.split()) > 8:
        error(f"Labels >8 ({len(labels.split())})", fn)

    field_keys = [k for k in seen_fields if k in KNOWN_FIELDS]
    ordered = [k for k in FIELD_ORDER if k in field_keys]
    if field_keys != ordered:
        error(f"Field order mismatch", fn)

    for k in header:
        if k not in KNOWN_FIELDS:
            error(f"Unknown field: {k}", fn)

    deps = header.get("DependsOn", "")
    if iid in deps.split():
        error(f"Depends on self", fn)

    all_parsed.append({"id": iid, "status": status, "deps": deps.split(), "fn": fn, "root": root})

# Verify evidence test references are runnable
for ref in check_test_refs:
    test = ref["test"]
    fn = ref["fn"]
    # Check that test file exists via grep for the test fn name
    test_files = ["crates/cli/tests/builtin_methods.rs",
                  "crates/cli/tests/control_flow.rs",
                  "crates/cli/tests/classes.rs",
                  "crates/cli/tests/modules.rs",
                  "crates/cli/tests/node_apis.rs",
                  "crates/cli/tests/host_deny.rs",
                  "crates/cli/tests/async_await.rs",
                  "crates/cli/tests/node_diff.rs",
                  "crates/cli/tests/common/node_diff_fixture_tests.rs",
                  "crates/cli/tests/parser_ast_structures.rs"]
    found = False
    for tf in test_files:
        if os.path.exists(tf):
            with open(tf) as f:
                if f"fn {test}" in f.read():
                    found = True
                    break
    if not found:
        warn(f"Evidence references test '{test}' but no test function found in crate tests", fn)

# Cross-file checks
id_map = {d["id"]: d for d in all_parsed if d["id"]}

for d in all_parsed:
    iid = d["id"]
    for dep in d["deps"]:
        dep = dep.strip()
        if not dep:
            continue
        if dep not in id_map:
            error(f"DependsOn unknown: {dep}", d["fn"])
    # Cycle detect (BFS, depth limited)
    visited = set()
    queue = [(iid, 0)]
    while queue:
        cur, depth = queue.pop(0)
        if depth > 50:
            break
        if cur in visited:
            continue
        visited.add(cur)
        for dep in id_map.get(cur, {}).get("deps", []):
            if dep == iid:
                error(f"Cycle involving {iid}", d["fn"])
                break
            if dep not in visited:
                queue.append((dep, depth + 1))

if errors:
    print(f"\n{errors} error(s) found")
    sys.exit(1)
else:
    print("All issues valid")
