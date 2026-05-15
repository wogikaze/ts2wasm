#!/usr/bin/env python3
"""Phase 1-2: map existing test coverage + identify gaps.

For each implementation issue, determine:
- What test type is required (based on area/semantic domain)
- Is there test coverage already? (from fixture catalog + test functions)
- What is the coverage decision?

Output: artifacts/test-gap-analysis.json
"""

import os, sys, json, re
from datetime import datetime

CRATES_DIR = "crates"
CATALOG_PATH = "fixtures/catalog.yaml"
CLASSIFICATION_PATH = "artifacts/issue-classification.json"
OUTPUT_PATH = "artifacts/test-gap-analysis.json"

# Test type assignment rules
def assign_test_type(labels_str, title):
    labels = labels_str.lower()
    title = title.lower()

    if any(k in labels for k in ["area:runtime", "area:semantics", "feature:", "area:host"]):
        return "differential"
    if any(k in labels for k in ["area:backend", "area:compiler", "area:abi"]):
        return "snapshot+unit"
    if "area:frontend" in labels:
        return "parser-snapshot"
    if "area:ir" in labels:
        return "hir-mir-snapshot"
    if "type:bug" in labels:
        return "regression"

    # Bare label fallback
    bare_labels = labels.split()
    for bl in bare_labels:
        bl_clean = bl.strip()
        if bl_clean in ("abi", "backend", "capability", "cli", "runtime"):
            return "differential"
        if bl_clean in ("compiler",):
            return "snapshot+unit"

    # Title fallback
    if any(k in title for k in ["runtime", "semantic", "regexp", "weakref",
                                  "console", "intl", "function.prototype",
                                  "string.prototype", "promise", "proxy"]):
        return "differential"
    if any(k in title for k in ["backend", "compiler", "cli", "abi",
                                  "target", "metadata"]):
        return "snapshot+unit"
    if any(k in title for k in ["parser", "lexer", "ast", "comma expression"]):
        return "parser-snapshot"
    if any(k in title for k in ["hir", "mir", "lowering"]):
        return "hir-mir-snapshot"

    return "unit-test"


def parse_header(content):
    """Parse issue file header."""
    parts = content.split("\n---\n", 1)
    header_text = parts[0].strip()
    body = parts[1] if len(parts) > 1 else ""
    header = {}
    for line in header_text.split("\n"):
        line = line.strip()
        if not line:
            continue
        m = re.match(r"^([A-Za-z]+):\s*(.*)", line)
        if m:
            header[m.group(1)] = m.group(2).strip()
    return header, body


def extract_acceptance_commands(body):
    """Extract cargo test / mise run commands from Acceptance section."""
    commands = []
    if not body:
        return commands
    # Find Acceptance section
    m = re.search(r"## Acceptance\s*\n(.*?)(?:\n## |\Z)", body, re.DOTALL)
    if m:
        section = m.group(1)
        for line in section.split("\n"):
            line = line.strip().lstrip("- ").strip()
            if re.match(r"(cargo|mise|python|ts2wasm|npm)", line):
                commands.append(line)
    return commands


def extract_feature_labels(content):
    """Extract feature: labels from issue file header."""
    labels = []
    for line in content.split("\n"):
        if line.startswith("Labels:"):
            parts = line[7:].strip().split()
            for p in parts:
                if p.startswith("feature:"):
                    labels.append(p[8:])
    return labels


def main():
    if not os.path.exists(CLASSIFICATION_PATH):
        print(f"Error: Run Phase 0 first (missing {CLASSIFICATION_PATH})", file=sys.stderr)
        sys.exit(1)

    with open(CLASSIFICATION_PATH) as f:
        classification = json.load(f)

    # Build test function index from crate test files
    test_index = {}  # test_name -> file_path
    covers_index = {}  # issue_id -> [test_names]

    # Walk crate test directories
    for root, dirs, files in os.walk(CRATES_DIR):
        for fn in files:
            if not fn.endswith(".rs"):
                continue
            path = os.path.join(root, fn)
            with open(path) as f:
                content = f.read()
            # Collect test functions
            for m in re.finditer(r'#\[test\]\s*\n\s*(?:pub\s+)?fn\s+(\w+)', content):
                test_index[m.group(1)] = path
            # Collect covers: annotations
            for m in re.finditer(r'//\s*covers:\s*([\w-]+(?:\s*,\s*[\w-]+)*)', content):
                for item in m.group(1).split(","):
                    item_id = item.strip()
                    if item_id not in covers_index:
                        covers_index[item_id] = []
                    covers_index[item_id].append(path)

    # Load fixture catalog features
    catalog_features = set()
    fixture_entries = {}
    try:
        import yaml
        with open(CATALOG_PATH) as f:
            catalog = yaml.safe_load(f)
        # Extract feature_matrix data
        fm = catalog.get("feature_matrix", {})
        if isinstance(fm, dict):
            for dir_key, dir_data in fm.items():
                if isinstance(dir_data, dict):
                    for feat in dir_data.get("feature", []):
                        catalog_features.add(feat)
                    fixtures = dir_data.get("fixtures", [])
                    if isinstance(fixtures, list):
                        for f_entry in fixtures:
                            if isinstance(f_entry, dict):
                                fixture_entries[f_entry.get("path", str(f_entry))] = dir_data.get("status", "unknown")
    except ImportError:
        print("Warning: PyYAML not installed. Cannot parse fixture catalog.", file=sys.stderr)
    except Exception as e:
        print(f"Warning: Could not parse catalog: {e}", file=sys.stderr)

    gap_entries = []

    for issue in classification.get("issues", []):
        if issue.get("category") != "implementation":
            continue

        iid = issue["id"]
        labels = issue.get("labels", "")
        title = issue.get("title", "")
        directory = issue.get("directory", "issues")
        status = issue.get("status", "")
        priority = issue.get("priority", "")

        # Determine required test type
        test_type = assign_test_type(labels, title)

        # Extract feature keywords from labels & title
        feature_keywords = set()
        for label in labels.replace(",", " ").split():
            if label.startswith("feature:"):
                feature_keywords.add(label[8:])
            if label.startswith("area:"):
                feature_keywords.add(label[5:])

        # Check for direct test coverage (covers: annotation)
        direct_coverage = covers_index.get(iid, [])

        # Check test function name coverage
        name_coverage = []
        for test_name, test_file in test_index.items():
            # Match by issue ID (without I- prefix)
            short_id = iid.replace("I-", "", 1)
            if short_id in test_name:
                name_coverage.append({"test": test_name, "file": test_file, "match": "exact"})
                continue
            # Match by feature keywords
            for kw in feature_keywords:
                kw_normalized = kw.lower().replace("-", "_").replace(":", "_")
                if kw_normalized and kw_normalized in test_name.lower():
                    name_coverage.append({"test": test_name, "file": test_file, "match": "keyword"})
                    break

        # Check fixture coverage
        fixture_match = None
        for kw in feature_keywords:
            if kw in catalog_features:
                fixture_match = kw
                break

        # Determine coverage_decision
        if direct_coverage:
            coverage_decision = "covered"
            coverage_detail = f"covers annotation in {direct_coverage[0]}"
        elif name_coverage:
            coverage_decision = "covered"
            coverage_detail = f"test {name_coverage[0]['test']} in {name_coverage[0]['file']}"
        elif fixture_match:
            coverage_decision = "covered"
            coverage_detail = f"fixture feature: {fixture_match}"
        else:
            coverage_decision = "missing"
            coverage_detail = "no test coverage found"

        evidence_status = issue.get("has_evidence", "False")

        entry = {
            "issue_id": iid,
            "status_at_audit": status,
            "directory": directory,
            "priority": priority,
            "title": title,
            "labels": labels,
            "required_test_type": test_type,
            "coverage_decision": coverage_decision,
            "coverage_detail": coverage_detail,
            "feature_keywords": sorted(feature_keywords),
            "evidence_status": evidence_status,
            "test_refs": [],
        }

        # Add test refs if covered
        if direct_coverage:
            for cov_path in direct_coverage:
                entry["test_refs"].append({
                    "kind": test_type,
                    "path": cov_path,
                    "match": "covers_annotation",
                })
        elif name_coverage:
            for nc in name_coverage[:5]:
                entry["test_refs"].append({
                    "kind": test_type,
                    "path": nc["file"],
                    "match": nc["match"],
                })

        gap_entries.append(entry)

    # Sort by priority (P0 first) then by status (open before done)
    priority_order = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}
    gap_entries.sort(key=lambda e: (
        priority_order.get(e["priority"], 99),
        0 if e["status_at_audit"] == "open" else 1,
    ))

    output = {
        "schema_version": 1,
        "generated_at": datetime.now().isoformat(),
        "summary": {
            "total_implementation": len(gap_entries),
            "covered": sum(1 for e in gap_entries if e["coverage_decision"] == "covered"),
            "missing": sum(1 for e in gap_entries if e["coverage_decision"] == "missing"),
        },
        "entries": gap_entries,
    }

    with open(OUTPUT_PATH, "w") as f:
        json.dump(output, f, indent=2, ensure_ascii=False)

    summary = output["summary"]
    print(f"Implementation issues: {summary['total_implementation']}")
    print(f"  Covered: {summary['covered']}")
    print(f"  Missing: {summary['missing']}")

    # Print top 5 missing by priority
    missing = [e for e in gap_entries if e["coverage_decision"] == "missing"]
    print(f"\nTop 5 missing by priority (first = highest):")
    for e in missing[:5]:
        print(f"  [{e['priority']}] {e['issue_id']}: {e['title'][:60]}")
        print(f"    needed: {e['required_test_type']}")

    print(f"\nWritten to {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
