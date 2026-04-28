#!/usr/bin/env python3
"""Generate coverage matrix from JSON results.

Reads JSON results from artifacts/coverage/results/*.json and generates
the coverage table in artifacts/coverage/reference-coverage-matrix.md.

Usage:
  scripts/gen/coverage-matrix.py [--check]

Options:
  --check   Fail if matrix is stale (does not run coverage sampling)

Note: This script does NOT run reference-coverage.sh. Use the separate
refresh script to generate JSON results.
"""

import argparse
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
RESULTS_DIR = REPO / "artifacts" / "coverage" / "results"
MATRIX_PATH = REPO / "artifacts" / "coverage" / "reference-coverage-matrix.md"

SUITE_CONFIG = {
    "test262": {"name": "test262", "step": 50},
    "tsc": {"name": "TypeScript compiler cases", "step": 30},
    "tsgo": {"name": "typescript-go testdata", "step": 20},
}


def load_result(suite_key: str) -> dict | None:
    """Load JSON result for a suite."""
    result_path = RESULTS_DIR / f"{suite_key}.json"
    if not result_path.exists():
        return None
    try:
        with open(result_path, "r", encoding="utf-8") as f:
            return json.load(f)
    except (json.JSONDecodeError, IOError) as e:
        print(f"Error loading {result_path}: {e}", file=sys.stderr)
        return None


def format_diagcodes(diagcodes: dict) -> str:
    """Format diagcodes dict as comma-separated string."""
    if not diagcodes:
        return "-"
    items = sorted(diagcodes.items(), key=lambda x: (-x[1], x[0]))
    return ",".join(f"{k}:{v}" for k, v in items)


def format_features(features: dict) -> str:
    """Format feature-label counts as comma-separated string."""
    if not features:
        return "-"
    items = sorted(features.items(), key=lambda x: (-x[1], x[0]))
    return ",".join(f"{k}:{v}" for k, v in items)


def render_row(result: dict, suite_key: str) -> str:
    """Render a single table row from result data."""
    suite_name = result.get("suite_name", suite_key)
    denominator = result.get("denominator", 0)
    executed = result.get("executed", 0)
    build_cov = result.get("build_coverage_percent", "0.00")
    semantic_cov = result.get("semantic_coverage_percent", "0.00")
    build_pass = result.get("build_pass", 0)
    semantic_pass = result.get("semantic_pass", 0)
    fail = result.get("fail", 0)
    unsupported = result.get("unsupported", 0)
    blocked = result.get("blocked", 0)
    skip = result.get("skip_with_reason", 0)
    diagcodes = format_diagcodes(result.get("unsupported_diagcodes", {}))
    features = format_features(result.get("unsupported_features", {}))
    status = result.get("status", "in-progress")
    evidence = result.get("evidence", f"scripts/manager reference-coverage {suite_key} --limit {executed}")

    return f"| {suite_name} | {denominator} | {executed} | {build_cov} | {semantic_cov} | {build_pass} | {semantic_pass} | {fail} | {unsupported} | {blocked} | {skip} | {diagcodes} | {features} | {status} | `{evidence}` |"


def render_result_rows(result: dict, suite_key: str) -> list[str]:
    """Render the canonical suite row plus optional additive evidence rows."""
    rows = [render_row(result, suite_key)]
    for evidence_row in result.get("evidence_rows", []):
        rows.append(render_row(evidence_row, suite_key))
    return rows


def render_empty_row(suite_key: str, config: dict) -> str:
    """Render an empty row when no result exists."""
    suite_name = config["name"]
    step = config["step"]
    return f"| {suite_name} | 0 | 0 | 0.00 | 0.00 | 0 | 0 | 0 | 0 | 0 | 0 | - | - | in-progress | `scripts/manager reference-coverage {suite_key} --limit {step}` |"


def generate_matrix() -> str:
    """Generate the full matrix markdown."""
    rows = []
    for suite_key, config in SUITE_CONFIG.items():
        result = load_result(suite_key)
        if result:
            rows.extend(render_result_rows(result, suite_key))
        else:
            rows.append(render_empty_row(suite_key, config))

    header = "| suite | denominator | executed | build_coverage% | semantic_coverage% | build_pass | semantic_pass | fail | unsupported | blocked | skip-with-reason | unsupported (DiagCode breakdown) | unsupported (feature breakdown) | status | evidence |"
    separator = "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---|"

    return "\n".join([header, separator] + rows)


def replace_table(content: str, new_table: str) -> str:
    """Replace the coverage table in the content."""
    lines = content.splitlines()
    result = []
    in_table = False

    for line in lines:
        if "<!-- coverage-table:start -->" in line:
            result.append(line)
            result.append(new_table)
            in_table = True
            continue
        if "<!-- coverage-table:end -->" in line:
            in_table = False
            result.append(line)
            continue
        if not in_table:
            result.append(line)

    return "\n".join(result)


def ensure_matrix_file() -> None:
    """Ensure the matrix file exists with proper structure."""
    if MATRIX_PATH.exists():
        return

    MATRIX_PATH.parent.mkdir(parents=True, exist_ok=True)
    MATRIX_PATH.write_text(
        """# Reference Coverage Matrix (generated)

Generated by scripts/manager update-coverage-matrix.
Do not edit manually.

<!-- coverage-table:start -->
| suite | denominator | executed | build_coverage% | semantic_coverage% | build_pass | semantic_pass | fail | unsupported | blocked | skip-with-reason | unsupported (DiagCode breakdown) | unsupported (feature breakdown) | status | evidence |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---|
<!-- coverage-table:end -->
""",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate coverage matrix from JSON results")
    parser.add_argument("--check", action="store_true", help="Check if matrix is stale")
    args = parser.parse_args()

    ensure_matrix_file()

    current_content = MATRIX_PATH.read_text(encoding="utf-8")
    new_table = generate_matrix()
    new_content = replace_table(current_content, new_table)
    if not new_content.endswith("\n"):
        new_content += "\n"

    if args.check:
        if new_content != current_content:
            print(f"coverage matrix is stale; run refresh script and commit {MATRIX_PATH}", file=sys.stderr)
            return 1
        print("coverage matrix OK (up to date)", file=sys.stderr)
        return 0

    MATRIX_PATH.write_text(new_content, encoding="utf-8")
    print(f"updated {MATRIX_PATH}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
