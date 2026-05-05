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


def load_all_results() -> dict[str, dict]:
    """Load all configured suite results."""
    return {
        suite_key: result
        for suite_key in SUITE_CONFIG
        if (result := load_result(suite_key)) is not None
    }


def suite_rows(result: dict, suite_key: str) -> list[dict]:
    """Return the canonical suite row plus optional additive evidence rows."""
    return [result] + result.get("evidence_rows", [])


def breakdown_columns(results: dict[str, dict], field: str) -> list[str]:
    """Return stable breakdown columns ordered by total descending."""
    totals: dict[str, int] = {}
    for suite_key, result in results.items():
        for row in suite_rows(result, suite_key):
            for key, value in row.get(field, {}).items():
                totals[key] = totals.get(key, 0) + int(value)
    return [key for key, _ in sorted(totals.items(), key=lambda item: (-item[1], item[0]))]


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
    status = result.get("status", "in-progress")
    evidence = result.get("evidence", f"scripts/manager reference-coverage {suite_key} --limit {executed}")

    return f"| {suite_name} | {denominator} | {executed} | {build_cov} | {semantic_cov} | {build_pass} | {semantic_pass} | {fail} | {unsupported} | {blocked} | {skip} | {status} | `{evidence}` |"


def render_breakdown_row(result: dict, suite_key: str, field: str, columns: list[str]) -> str:
    """Render unsupported breakdown counts as one column per breakdown key."""
    suite_name = result.get("suite_name", suite_key)
    executed = result.get("executed", 0)
    unsupported = result.get("unsupported", 0)
    breakdown = result.get(field, {})
    counts = [str(breakdown.get(column, 0)) for column in columns]
    count_cells = f" | {' | '.join(counts)}" if counts else ""
    evidence = result.get("evidence", f"scripts/manager reference-coverage {suite_key} --limit {executed}")

    return f"| {suite_name} | {executed} | {unsupported}{count_cells} | `{evidence}` |"


def render_result_rows(result: dict, suite_key: str) -> list[str]:
    """Render the canonical suite row plus optional additive evidence rows."""
    return [render_row(row, suite_key) for row in suite_rows(result, suite_key)]


def render_empty_row(suite_key: str, config: dict) -> str:
    """Render an empty row when no result exists."""
    suite_name = config["name"]
    step = config["step"]
    return f"| {suite_name} | 0 | 0 | 0.00 | 0.00 | 0 | 0 | 0 | 0 | 0 | 0 | in-progress | `scripts/manager reference-coverage {suite_key} --limit {step}` |"


def render_empty_breakdown_row(suite_key: str, config: dict, columns: list[str]) -> str:
    """Render an empty unsupported breakdown row when no result exists."""
    suite_name = config["name"]
    step = config["step"]
    counts = ["0" for _ in columns]
    count_cells = f" | {' | '.join(counts)}" if counts else ""
    return f"| {suite_name} | 0 | 0{count_cells} | `scripts/manager reference-coverage {suite_key} --limit {step}` |"


def generate_matrix(results: dict[str, dict]) -> str:
    """Generate the main coverage matrix markdown."""
    rows = []
    for suite_key, config in SUITE_CONFIG.items():
        result = results.get(suite_key)
        if result:
            rows.extend(render_result_rows(result, suite_key))
        else:
            rows.append(render_empty_row(suite_key, config))

    header = "| suite | denominator | executed | build_coverage% | semantic_coverage% | build_pass | semantic_pass | fail | unsupported | blocked | skip-with-reason | status | evidence |"
    separator = "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|"

    return "\n".join([header, separator] + rows)


def generate_breakdown_table(
    results: dict[str, dict],
    field: str,
    columns: list[str],
) -> str:
    """Generate an unsupported breakdown table with one column per breakdown key."""
    rows = []
    for suite_key, config in SUITE_CONFIG.items():
        result = results.get(suite_key)
        if result:
            rows.extend(
                render_breakdown_row(row, suite_key, field, columns)
                for row in suite_rows(result, suite_key)
            )
        else:
            rows.append(render_empty_breakdown_row(suite_key, config, columns))

    column_headers = f" | {' | '.join(columns)}" if columns else ""
    column_separators = f"|{'|'.join(['---:' for _ in columns])}" if columns else ""
    header = f"| suite | executed | unsupported{column_headers} | evidence |"
    separator = f"|---|---:|---:{column_separators}|---|"

    return "\n".join([header, separator] + rows)


def replace_section(content: str, start_marker: str, end_marker: str, replacement: str) -> str:
    """Replace a generated section in the content."""
    lines = content.splitlines()
    result = []
    in_section = False
    found = False

    for line in lines:
        if start_marker in line:
            result.append(line)
            result.append(replacement)
            in_section = True
            found = True
            continue
        if end_marker in line:
            in_section = False
            result.append(line)
            continue
        if not in_section:
            result.append(line)

    if not found:
        return content
    return "\n".join(result)


def render_document(main_table: str, diagcode_table: str, feature_table: str) -> str:
    """Render the complete generated matrix document."""
    return f"""# Reference Coverage Matrix (generated)

Generated by scripts/manager update-coverage-matrix.
Do not edit manually.

<!-- coverage-table:start -->
{main_table}
<!-- coverage-table:end -->

## Unsupported Diagnostic Codes

<!-- diagcode-table:start -->
{diagcode_table}
<!-- diagcode-table:end -->

## Unsupported Features

<!-- feature-table:start -->
{feature_table}
<!-- feature-table:end -->
"""


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
| suite | denominator | executed | build_coverage% | semantic_coverage% | build_pass | semantic_pass | fail | unsupported | blocked | skip-with-reason | status | evidence |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
<!-- coverage-table:end -->

## Unsupported Diagnostic Codes

<!-- diagcode-table:start -->
| suite | executed | unsupported | unsupported (DiagCode breakdown) | evidence |
|---|---:|---:|---|---|
<!-- diagcode-table:end -->

## Unsupported Features

<!-- feature-table:start -->
| suite | executed | unsupported | evidence |
|---|---:|---:|---|
<!-- feature-table:end -->
""",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate coverage matrix from JSON results")
    parser.add_argument("--check", action="store_true", help="Check if matrix is stale")
    args = parser.parse_args()

    ensure_matrix_file()

    current_content = MATRIX_PATH.read_text(encoding="utf-8")
    results = load_all_results()
    diagcode_columns = breakdown_columns(results, "unsupported_diagcodes")
    feature_columns = breakdown_columns(results, "unsupported_features")
    main_table = generate_matrix(results)
    diagcode_table = generate_breakdown_table(results, "unsupported_diagcodes", diagcode_columns)
    feature_table = generate_breakdown_table(results, "unsupported_features", feature_columns)
    required_markers = (
        "<!-- coverage-table:start -->",
        "<!-- diagcode-table:start -->",
        "<!-- feature-table:start -->",
    )
    if any(marker not in current_content for marker in required_markers):
        new_content = render_document(main_table, diagcode_table, feature_table)
    else:
        new_content = replace_section(
            current_content,
            "<!-- coverage-table:start -->",
            "<!-- coverage-table:end -->",
            main_table,
        )
        new_content = replace_section(
            new_content,
            "<!-- diagcode-table:start -->",
            "<!-- diagcode-table:end -->",
            diagcode_table,
        )
        new_content = replace_section(
            new_content,
            "<!-- feature-table:start -->",
            "<!-- feature-table:end -->",
            feature_table,
        )
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
