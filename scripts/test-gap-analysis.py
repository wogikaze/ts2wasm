#!/usr/bin/env python3
"""Phase 1-2: Test Gap Analysis — map existing tests to implementation issues.

For each implementation issue:
  1. Check if any fixture in fixtures/catalog.yaml matches the issue
  2. Check if test functions reference the issue ID (covers: comment)
  3. Check if evidence references passing test commands
  4. Determine coverage_decision

Output:
  - artifacts/test-gap-analysis.json — per-issue coverage analysis
  - artifacts/test-gap-summary.md — summary report
"""

import json, os, re, sys
from collections import defaultdict

ISSUES_DIR = "issues"
DONE_DIR = os.path.join(ISSUES_DIR, "done")
CATALOG_PATH = "fixtures/catalog.yaml"
OUT_DIR = "artifacts"

# Implementation issue label patterns
IMPL_PATTERNS = [
    re.compile(r"\btype:(?:feature|bug)\b", re.I),
    re.compile(r"\barea:(?:runtime|backend|frontend|compiler|semantics|ir|host)\b", re.I),
    re.compile(r"\bfeature:", re.I),
]


def is_implementation_issue(meta):
    """Check if issue is an implementation issue based on labels/title."""
    text = meta.get("Labels", "") + " " + meta.get("Title", "")
    for p in IMPL_PATTERNS:
        if p.search(text):
            return True
    return False


def parse_header(path):
    """Parse issue header."""
    meta = {}
    with open(path) as f:
        content = f.read()
    parts = content.split("\n---\n", 1)
    if len(parts) < 2:
        body = ""
    else:
        body = parts[1]
    for line in parts[0].strip().split("\n"):
        line = line.strip()
        if ":" in line:
            key, _, val = line.partition(":")
            meta[key.strip()] = val.strip()
    return meta, body, content


def extract_test_commands(body):
    """Extract test commands from Evidence section."""
    commands = []
    m = re.search(r"## Evidence\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if m:
        for line in m.group(1).strip().split("\n"):
            line = line.strip().lstrip("- ").strip()
            if line and not line.startswith("```"):
                commands.append(line)
    return commands


def has_passing_evidence(body):
    """Check if Evidence contains PASS results or test commands."""
    ev_m = re.search(r"## Evidence\s*\n(.*?)(?:\n##|\Z)", body, re.DOTALL)
    if not ev_m:
        return False
    ev = ev_m.group(1)
    return bool(re.search(r"PASS|exit 0|pass", ev, re.I))


def find_fixture_matches(meta, fixture_db, test_refs):
    """Find fixture and test matches for an issue."""
    issue_id = meta.get("Id", "")
    title = meta.get("Title", "")
    labels = meta.get("Labels", "")
    keywords = title.lower() + " " + labels.lower()

    fixture_matches = []
    test_matches = []
    issue_id_matches = []

    # Check fixture DB for issue_id references
    for entry in fixture_db:
        if entry.get("issue_ids") and issue_id in entry["issue_ids"]:
            issue_id_matches.append(entry)

    # Check test file references
    for ref in test_refs:
        if issue_id in ref.get("issue_ids", []):
            test_matches.append(ref)

    return {
        "fixture_matches": fixture_matches,
        "test_matches": test_matches,
        "issue_id_matches": issue_id_matches,
    }


def get_issue_id_from_comment(line):
    """Extract issue ID from a '// covers: I-XXXX' comment."""
    m = re.search(r"// covers:\s*(I-\d{8}-[A-HJ-NP-Z2-9]{6})", line)
    return m.group(1) if m else None


def scan_test_files():
    """Scan test files for covers: annotations and fixture references."""
    test_files = [
        "crates/cli/tests/m6_builtin_methods.rs",
        "crates/cli/tests/m7_control_flow.rs",
        "crates/cli/tests/m8_oop_classes.rs",
        "crates/cli/tests/m9_modules.rs",
        "crates/cli/tests/m10_node_apis.rs",
        "crates/cli/tests/m11_host_deny.rs",
        "crates/cli/tests/m12_async_await.rs",
        "crates/cli/tests/m2_node_diff.rs",
        "crates/cli/tests/m2_node_diff_fixture_tests.rs",
        "crates/cli/tests/m2_node_diff.rs",
        "crates/cli/tests/parser_ast_structures.rs",
        "crates/cli/tests/ir_lowering.rs",
        "crates/cli/tests/parser_keywords.rs",
        "crates/cli/tests/gc_layout_structural.rs",
        "crates/cli/tests/command_contract.rs",
    ]
    test_refs = []
    for tf_path in test_files:
        if not os.path.exists(tf_path):
            continue
        with open(tf_path) as f:
            content = f.read()
        # Find covers: annotations
        for line in content.split("\n"):
            iid = get_issue_id_from_comment(line)
            if iid:
                test_refs.append({
                    "file": tf_path,
                    "issue_ids": [iid],
                    "annotation_line": line.strip(),
                })
        # Find test function names referencing fixtures
        for m in re.finditer(r"fn\s+(build_smoke_|semantic_diff_|assert_)?(\w+)", content):
            pass  # We'll use this for mapping
    return test_refs


def load_fixture_catalog():
    """Load fixture catalog and build a reverse index by keyword."""
    entries = []
    if not os.path.exists(CATALOG_PATH):
        return entries
    with open(CATALOG_PATH) as f:
        content = f.read()
    # Simple YAML-like parse for fixture entries
    current_dir = None
    for line in content.split("\n"):
        dir_m = re.match(r"^\s{2}(\w[-/\w]*):", line)
        if dir_m and not line.startswith("  -") and not line.startswith("    "):
            current_dir = dir_m.group(1)
        fixture_m = re.match(r"\s{4}- fixtures/(.+)", line)
        if fixture_m and current_dir:
            fpath = fixture_m.group(1).strip()
            entries.append({
                "path": fpath,
                "directory": current_dir,
                "issue_ids": [],
            })
    return entries


def main():
    os.makedirs(OUT_DIR, exist_ok=True)

    # Load fixture catalog and test annotations
    fixture_db = load_fixture_catalog()
    test_refs = scan_test_files()
    coverage_issues = []

    # Scan done issues
    for fn in sorted(os.listdir(DONE_DIR)):
        if not fn.endswith(".md"):
            continue
        path = os.path.join(DONE_DIR, fn)
        meta, body, content = parse_header(path)
        if not is_implementation_issue(meta):
            continue
        issue_id = meta.get("Id", fn.replace(".md", ""))
        status = meta.get("Status", "")
        ev_cmds = extract_test_commands(body)
        has_pass = has_passing_evidence(body)

        # Determine coverage decision
        matches = find_fixture_matches(meta, fixture_db, test_refs)

        # Check if evidence has passing test results
        if has_pass:
            decision = "covered"
            if matches["test_matches"]:
                decision = "covered"
        elif ev_cmds:
            decision = "red_pending"
        else:
            decision = "missing"

        entry = {
            "issue_id": issue_id,
            "status": status,
            "priority": meta.get("Priority", ""),
            "title": meta.get("Title", "")[:80],
            "category": "implementation",
            "coverage_decision": decision,
            "has_acceptance": bool(re.search(r"## Acceptance", body)),
            "has_test_requirements": bool(re.search(r"## Test-Requirements", body)),
            "evidence_commands": ev_cmds[:5],
            "has_passing_evidence": has_pass,
            "test_annotations": matches["test_matches"],
            "fixture_matches": [m["path"] for m in matches["fixture_matches"]],
        }
        coverage_issues.append(entry)

    # Scan open issues
    for fn in sorted(os.listdir(ISSUES_DIR)):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        path = os.path.join(ISSUES_DIR, fn)
        meta, body, content = parse_header(path)
        if not is_implementation_issue(meta):
            continue
        issue_id = meta.get("Id", fn.replace(".md", ""))
        status = meta.get("Status", "")
        ev_cmds = extract_test_commands(body)
        has_pass = has_passing_evidence(body)

        matches = find_fixture_matches(meta, fixture_db, test_refs)
        decision = "missing"
        if ev_cmds:
            decision = "red_pending"
        if matches["test_matches"]:
            decision = "covered"

        # Reopened false-done issues
        if "False-done audit" in body:
            decision = "missing"

        entry = {
            "issue_id": issue_id,
            "status": status,
            "priority": meta.get("Priority", ""),
            "title": meta.get("Title", "")[:80],
            "category": "implementation",
            "coverage_decision": decision,
            "has_acceptance": bool(re.search(r"## Acceptance", body)),
            "has_test_requirements": bool(re.search(r"## Test-Requirements", body)),
            "evidence_commands": ev_cmds[:5],
            "has_passing_evidence": has_pass,
            "test_annotations": matches["test_matches"],
            "fixture_matches": [m["path"] for m in matches["fixture_matches"]],
        }
        coverage_issues.append(entry)

    # Write JSON
    json_path = os.path.join(OUT_DIR, "test-gap-analysis.json")
    with open(json_path, "w") as f:
        json.dump(coverage_issues, f, indent=2, ensure_ascii=False)

    # Summary
    decisions = defaultdict(int)
    for i in coverage_issues:
        decisions[i["coverage_decision"]] += 1

    lines = [
        "# Test Gap Analysis Summary",
        "",
        f"Generated: 2026-05-15",
        "",
        "## Coverage Decisions",
        f"| Decision | Count |",
        f"|---:|---|",
    ]
    for d in sorted(decisions):
        lines.append(f"| {d} | {decisions[d]} |")
    lines.append(f"| **Total** | **{len(coverage_issues)}** |")

    lines.append("")
    lines.append("## Uncovered Implementation Issues")
    missing = [i for i in coverage_issues if i["coverage_decision"] == "missing"]
    lines.append("| Issue | Status | Priority | Title |")
    lines.append("|---|---|---|---|")
    for i in missing:
        lines.append(f"| {i['issue_id']} | {i['status']} | {i['priority']} | {i['title']} |")

    lines.append("")
    lines.append("## RED (test exists but fails)")
    reds = [i for i in coverage_issues if i["coverage_decision"] == "red_pending"]
    for r in reds:
        lines.append(f"- {r['issue_id']}: {r['title']}")

    summary_path = os.path.join(OUT_DIR, "test-gap-summary.md")
    with open(summary_path, "w") as f:
        f.write("\n".join(lines) + "\n")

    print(f"Implementation issues analyzed: {len(coverage_issues)}")
    print(f"  Covered: {decisions.get('covered', 0)}")
    print(f"  Missing: {decisions.get('missing', 0)}")
    print(f"  RED:     {decisions.get('red_pending', 0)}")
    print(f"Output: {json_path}")
    print(f"Output: {summary_path}")


if __name__ == "__main__":
    main()
