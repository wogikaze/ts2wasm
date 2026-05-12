#!/usr/bin/env python3
"""Coverage Gate: suite-level delta comparison and per-category shard metrics.

Two operating modes:

1. Suite-level delta gate (default, args: <base-doc> <current-doc>)
   Compares base and current coverage matrix documents and fails if:
   - executed count decreased per suite
   - build_pass count decreased per suite
   - semantic_pass count decreased per suite
   - fail count increased per suite

2. Shard metrics (--shards)
   Reads JSONL result files and prints per-category shard breakdown.

3. Regression check (--check-regression)
   Compares current coverage metrics against a stored baseline.
   Fails if coverage drops below the baseline.
   Updates the baseline on success.
"""

import argparse
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
RESULTS_DIR = REPO_ROOT / "artifacts" / "coverage" / "results"
BASELINES_DIR = REPO_ROOT / "artifacts" / "coverage" / "baselines"
SHARD_BASELINE_FILE = BASELINES_DIR / "shard-baseline.json"


def usage():
    print("Usage:")
    print("  scripts/gate/coverage.py <base-doc> <current-doc>")
    print("  scripts/gate/coverage.py --shards [--jsonl-file PATH]")
    print("  scripts/gate/coverage.py --check-regression [--jsonl-file PATH]")
    print()
    print("Checks (suite-level delta):")
    print("  - executed count must not decrease per suite")
    print("  - build_pass count must not decrease per suite")
    print("  - semantic_pass count must not decrease per suite")
    print("  - fail count must not increase per suite")
    print()
    print("Options:")
    print("  --shards               Print per-category shard metrics from JSONL results")
    print("  --check-regression     Compare current coverage against stored baseline")
    print("  --jsonl-file PATH      Path to JSONL results file (default: <suite>-results.jsonl)")


# ---------------------------------------------------------------------------
# Suite-level delta gate (existing logic, updated column indices)
# ---------------------------------------------------------------------------

def extract_col(file_path, suite, col):
    """Extract a column value from the coverage table for a given suite."""
    with open(file_path) as f:
        lines = f.readlines()

    in_table = False
    for line in lines:
        if "<!-- coverage-table:start -->" in line:
            in_table = True
            continue
        if "<!-- coverage-table:end -->" in line:
            in_table = False
            continue
        if in_table and line.startswith("|"):
            parts = [p.strip() for p in line.split("|")]
            if len(parts) > 2 and parts[1] == suite:
                if col < len(parts):
                    return parts[col]
    return None


def run_suite_delta_gate(args: list[str]) -> int:
    """Suite-level comparison between base and current coverage matrix docs."""
    if len(args) < 2:
        print("ERROR: base-doc and current-doc required for suite-level gate", file=sys.stderr)
        usage()
        return 1

    base_doc = args[0]
    current_doc = args[1]

    current_path = REPO_ROOT / current_doc
    if not current_path.exists():
        print(f"missing current doc: {current_doc}", file=sys.stderr)
        return 1

    base_path = REPO_ROOT / base_doc
    if not base_path.exists():
        print(f"base doc not found, skipping delta gate: {base_doc}", file=sys.stderr)
        return 0

    status = 0
    suites = ["test262", "tsc", "tsgo"]

    for suite in suites:
        base_executed = extract_col(base_path, suite, 3)
        base_build_pass = extract_col(base_path, suite, 6)
        base_semantic_pass = extract_col(base_path, suite, 7)
        base_fail = extract_col(base_path, suite, 8)

        current_executed = extract_col(current_path, suite, 3)
        current_build_pass = extract_col(current_path, suite, 6)
        current_semantic_pass = extract_col(current_path, suite, 7)
        current_fail = extract_col(current_path, suite, 8)

        if not all([base_executed, base_build_pass, base_semantic_pass, base_fail,
                   current_executed, current_build_pass, current_semantic_pass, current_fail]):
            print(f"ERROR: incomplete coverage row for suite: {suite}", file=sys.stderr)
            status = 1
            continue

        try:
            base_executed = int(base_executed)
            base_build_pass = int(base_build_pass)
            base_semantic_pass = int(base_semantic_pass)
            base_fail = int(base_fail)
            current_executed = int(current_executed)
            current_build_pass = int(current_build_pass)
            current_semantic_pass = int(current_semantic_pass)
            current_fail = int(current_fail)
        except ValueError:
            print(f"ERROR: non-integer values for suite: {suite}", file=sys.stderr)
            status = 1
            continue

        if current_executed < base_executed:
            print(f"gate failure: executed decreased for {suite} ({base_executed} -> {current_executed})", file=sys.stderr)
            status = 1

        if current_build_pass < base_build_pass:
            print(f"gate failure: build_pass decreased for {suite} ({base_build_pass} -> {current_build_pass})", file=sys.stderr)
            status = 1

        if current_semantic_pass < base_semantic_pass:
            print(f"gate failure: semantic_pass decreased for {suite} ({base_semantic_pass} -> {current_semantic_pass})", file=sys.stderr)
            status = 1

        if current_fail > base_fail:
            print(f"gate failure: fail increased for {suite} ({base_fail} -> {current_fail})", file=sys.stderr)
            status = 1

    return status


# ---------------------------------------------------------------------------
# Shard metrics: per-category breakdown from JSONL results
# ---------------------------------------------------------------------------

def extract_category(case_path: str) -> str:
    """Extract the test262 category from a case file path.

    Category is the first path segment after 'test/'.
    E.g., 'reference/test262/test/language/asi/foo.js' -> 'language'
    """
    m = re.search(r'/test/([^/]+)/', case_path)
    return m.group(1) if m else "unknown"


def read_jsonl_results(jsonl_path: Path) -> list[dict]:
    """Read JSONL results file."""
    records = []
    if not jsonl_path.exists():
        print(f"WARNING: JSONL results not found: {jsonl_path}", file=sys.stderr)
        return records
    with open(jsonl_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                records.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    return records


def compute_shard_metrics(jsonl_path: Path) -> dict:
    """Compute per-category shard metrics from JSONL results.

    Returns a dict mapping category -> { total, pass, fail, unsupported, blocked,
    build_pass, semantic_pass }
    """
    records = read_jsonl_results(jsonl_path)
    categories: dict[str, dict] = {}

    for rec in records:
        case_path = rec.get("case", "")
        cat = extract_category(case_path)
        if cat not in categories:
            categories[cat] = {
                "total": 0,
                "pass": 0,
                "build_pass": 0,
                "semantic_pass": 0,
                "fail": 0,
                "unsupported": 0,
                "blocked": 0,
            }
        categories[cat]["total"] += 1
        status = rec.get("status", "unknown")

        if status == "pass":
            categories[cat]["pass"] += 1
            categories[cat]["build_pass"] += 1
            categories[cat]["semantic_pass"] += 1
        elif status == "build_pass":
            categories[cat]["build_pass"] += 1
        elif status == "fail":
            categories[cat]["fail"] += 1
        elif status == "unsupported":
            categories[cat]["unsupported"] += 1
        elif status == "blocked":
            categories[cat]["blocked"] += 1
        elif status == "mismatch":
            categories[cat]["build_pass"] += 1
        elif status == "runtime_error":
            categories[cat]["fail"] += 1

    return dict(sorted(categories.items()))


def run_shard_metrics(args: list[str]) -> int:
    """Print per-category shard metrics."""
    jsonl_file = None
    i = 0
    while i < len(args):
        if args[i] == "--jsonl-file":
            if i + 1 >= len(args):
                print("ERROR: --jsonl-file requires a value", file=sys.stderr)
                return 1
            jsonl_file = args[i + 1]
            i += 2
        else:
            print(f"ERROR: Unknown option: {args[i]}", file=sys.stderr)
            return 1

    if jsonl_file is None:
        jsonl_file = str(RESULTS_DIR / "test262-results.jsonl")

    jsonl_path = REPO_ROOT / jsonl_file
    if not jsonl_path.exists():
        print(f"ERROR: JSONL results not found: {jsonl_path}", file=sys.stderr)
        return 1

    shards = compute_shard_metrics(jsonl_path)

    if not shards:
        print("No shard data available")
        return 1

    # Print header
    print(f"{'Category':<30} {'Total':>8} {'Pass':>8} {'BuildPass':>10} {'SemPass':>9} {'Fail':>6} {'Unsupp':>8} {'Blocked':>8}")
    print("-" * 90)
    totals = {"total": 0, "pass": 0, "build_pass": 0, "semantic_pass": 0, "fail": 0, "unsupported": 0, "blocked": 0}
    for cat, metrics in shards.items():
        print(f"{cat:<30} {metrics['total']:>8} {metrics['pass']:>8} {metrics['build_pass']:>10} {metrics['semantic_pass']:>9} {metrics['fail']:>6} {metrics['unsupported']:>8} {metrics['blocked']:>8}")
        for k in totals:
            totals[k] += metrics[k]
    print("-" * 90)
    print(f"{'TOTAL':<30} {totals['total']:>8} {totals['pass']:>8} {totals['build_pass']:>10} {totals['semantic_pass']:>9} {totals['fail']:>6} {totals['unsupported']:>8} {totals['blocked']:>8}")

    return 0


# ---------------------------------------------------------------------------
# Regression detection: compare against stored baseline
# ---------------------------------------------------------------------------

def load_baseline() -> dict | None:
    """Load stored shard baseline, or return None."""
    if not SHARD_BASELINE_FILE.exists():
        return None
    with open(SHARD_BASELINE_FILE) as f:
        return json.load(f)


def save_baseline(shards: dict) -> None:
    """Save current shard metrics as the new baseline."""
    BASELINES_DIR.mkdir(parents=True, exist_ok=True)
    baseline = {
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "shards": shards,
        "totals": {},
    }
    # Compute totals
    for cat, metrics in shards.items():
        for k in ("total", "pass", "build_pass", "semantic_pass", "fail", "unsupported", "blocked"):
            baseline["totals"][k] = baseline["totals"].get(k, 0) + metrics[k]
    with open(SHARD_BASELINE_FILE, "w") as f:
        json.dump(baseline, f, indent=2)
    print(f"Baseline saved to {SHARD_BASELINE_FILE}", file=sys.stderr)


def run_regression_check(args: list[str]) -> int:
    """Compare current coverage against stored baseline.

    Uses JSONL results when available; falls back to seeds file counts.
    Returns non-zero if coverage dropped.
    """
    jsonl_file = None
    i = 0
    while i < len(args):
        if args[i] == "--jsonl-file":
            if i + 1 >= len(args):
                print("ERROR: --jsonl-file requires a value", file=sys.stderr)
                return 1
            jsonl_file = args[i + 1]
            i += 2
        else:
            print(f"ERROR: Unknown option: {args[i]}", file=sys.stderr)
            return 1

    if jsonl_file is None:
        jsonl_file = str(RESULTS_DIR / "test262-results.jsonl")

    jsonl_path = REPO_ROOT / jsonl_file

    # Try JSONL results first; fall back to seeds file counts
    if jsonl_path.exists():
        shards = compute_shard_metrics(jsonl_path)
        source = "JSONL"
    else:
        seeds_shards = parse_seeds_shards()
        # Convert seeds shards to the same format as JSONL shards
        shards = {}
        for name, data in seeds_shards.items():
            shards[name] = {
                "total": data["total"],
                "pass": data["total"],
                "build_pass": data["total"],
                "semantic_pass": data["total"],
                "fail": 0,
                "unsupported": 0,
                "blocked": 0,
            }
        source = "seeds"

    if not shards:
        print("ERROR: No shard data to compare", file=sys.stderr)
        return 1

    baseline = load_baseline()

    if baseline is None:
        print(f"No baseline found. Creating baseline from {source} data.", file=sys.stderr)
        save_baseline(shards)
        return 0

    baseline_shards = baseline.get("shards", {})
    regression = 0

    # Compare per-category metrics
    all_categories = sorted(set(list(shards.keys()) + list(baseline_shards.keys())))
    for cat in all_categories:
        cur = shards.get(cat, {})
        base = baseline_shards.get(cat, {})

        cur_pass = cur.get("pass", 0)
        base_pass = base.get("pass", 0)
        cur_build = cur.get("build_pass", 0)
        base_build = base.get("build_pass", 0)
        cur_sem = cur.get("semantic_pass", 0)
        base_sem = base.get("semantic_pass", 0)
        cur_fail = cur.get("fail", 0)
        base_fail = base.get("fail", 0)
        cur_unsupp = cur.get("unsupported", 0)
        base_unsupp = base.get("unsupported", 0)

        if cur_pass < base_pass:
            print(f"REGRESSION [{cat}]: pass decreased from {base_pass} to {cur_pass}", file=sys.stderr)
            regression = 1
        if cur_build < base_build:
            print(f"REGRESSION [{cat}]: build_pass decreased from {base_build} to {cur_build}", file=sys.stderr)
            regression = 1
        if cur_sem < base_sem:
            print(f"REGRESSION [{cat}]: semantic_pass decreased from {base_sem} to {cur_sem}", file=sys.stderr)
            regression = 1
        if cur_fail > base_fail:
            print(f"REGRESSION [{cat}]: fail increased from {base_fail} to {cur_fail}", file=sys.stderr)
            regression = 1
        if cur_unsupp > base_unsupp:
            # Unsupported increasing can be a regression or new coverage data
            # It's only a regression if the total also didn't increase
            cur_total = cur.get("total", 0)
            base_total = base.get("total", 0)
            if cur_total <= base_total:
                print(f"REGRESSION [{cat}]: unsupported increased from {base_unsupp} to {cur_unsupp} (total {base_total} -> {cur_total})", file=sys.stderr)
                regression = 1

    if regression == 0:
        print(f"No regressions detected (source: {source}).", file=sys.stderr)
        # Update baseline on success
        save_baseline(shards)
        return 0

    print("Regression detected! Baseline NOT updated.", file=sys.stderr)
    return 1


# ---------------------------------------------------------------------------
# Seeds-based shard metrics (default no-arg mode)
# ---------------------------------------------------------------------------

SEEDS_FILE = REPO_ROOT / "scripts" / "data" / "test262-semantic-core-seeds.txt"


def parse_seeds_shards() -> dict:
    """Parse seeds file into shards by subcategory.

    Seeds file has:
        # Category: <top-level>
        #   Subcategory: <path> (N seeds)
        <test-path>

    Returns dict mapping subcategory_name -> { total, paths }.
    """
    shards: dict[str, dict] = {}
    current_subcat = None

    with open(SEEDS_FILE) as f:
        for line in f:
            stripped = line.rstrip()

            if stripped.startswith("#   Subcategory:"):
                subcat_raw = stripped.split(":", 1)[1].strip()
                # Extract leaf name: "language/asi" -> "asi"
                current_subcat = subcat_raw.split("/")[-1].split()[0]
                if current_subcat not in shards:
                    shards[current_subcat] = {"total": 0, "paths": []}
                continue

            if not stripped or stripped.startswith("#"):
                continue

            if current_subcat is not None:
                shards[current_subcat]["paths"].append(stripped)
                shards[current_subcat]["total"] += 1

    return dict(sorted(shards.items()))


def format_seeds_shards(shards: dict) -> str:
    """Format seeds-based shard metrics as a human-readable table."""
    total = sum(s["total"] for s in shards.values())
    lines = []
    lines.append(f"Coverage shards (from seeds file): {len(shards)} shards, {total} total seeds")
    lines.append("")
    lines.append(f"{'Shard':<25} {'Seeds':>6}")
    lines.append("-" * 33)
    for name, data in shards.items():
        lines.append(f"{name:<25} {data['total']:>6}")
    lines.append("-" * 33)
    lines.append(f"{'TOTAL':<25} {total:>6}")
    return "\n".join(lines)


def run_seeds_shards() -> int:
    """Print shard metrics parsed from the seeds file (no JSONL required)."""
    shards = parse_seeds_shards()
    if not shards:
        print("No shard data found in seeds file.", file=sys.stderr)
        return 1
    print(format_seeds_shards(shards))
    return 0


# ---------------------------------------------------------------------------
# Main entry
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="Coverage gate: delta comparison and shard metrics",
        add_help=False,
    )
    parser.add_argument("--shards", action="store_true", help="Print per-category shard metrics")
    parser.add_argument("--check-regression", action="store_true", help="Compare against stored baseline")
    parser.add_argument("--jsonl-file", type=str, default=None, help="Path to JSONL results file")
    parser.add_argument("args", nargs=argparse.REMAINDER, help="Base and current doc paths")

    parsed, unknown = parser.parse_known_args()

    # Collect positional args
    positional = parsed.args + unknown

    if parsed.shards:
        return run_shard_metrics(positional)

    if parsed.check_regression:
        # Pass any --jsonl-file in positional args
        if parsed.jsonl_file:
            positional = ["--jsonl-file", parsed.jsonl_file] + positional
        return run_regression_check(positional)

    # No positional args: show seeds-based shard metrics
    if not positional:
        return run_seeds_shards()

    # Default: suite-level delta gate
    return run_suite_delta_gate(positional)


if __name__ == "__main__":
    sys.exit(main())
