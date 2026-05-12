#!/usr/bin/env python3
"""Semantic Coverage Dashboard: per-feature coverage report based on fixture catalog.

Scans fixtures/catalog.yaml and fixture directories to produce a per-feature
coverage report. Each fixture directory is treated as a feature area.

Outputs human-readable markdown and JSON for CI display.

Usage:
  python3 scripts/dashboard/semantic-coverage.py                 # default: markdown to stdout
  python3 scripts/dashboard/semantic-coverage.py --json          # JSON output
  python3 scripts/dashboard/semantic-coverage.py --help          # this help
"""

import argparse
import json
import sys
from pathlib import Path
from typing import Any

import yaml  # type: ignore[import-untyped]

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
FIXTURES_DIR = REPO_ROOT / "fixtures"
CATALOG_PATH = FIXTURES_DIR / "catalog.yaml"


def load_catalog() -> dict[str, Any]:
    """Load fixtures/catalog.yaml and return its data."""
    if not CATALOG_PATH.exists():
        print(f"ERROR: catalog not found: {CATALOG_PATH}", file=sys.stderr)
        sys.exit(1)

    with open(CATALOG_PATH, encoding="utf-8") as f:
        data = yaml.safe_load(f)

    if not data or "directories" not in data:
        print("ERROR: catalog.yaml has no 'directories' key", file=sys.stderr)
        sys.exit(1)

    return data


def count_fixture_files(dir_name: str, catalog: dict[str, Any]) -> int:
    """Count fixture .ts files for a directory, excluding parser-errors and negative."""
    dir_data = catalog.get("directories", {}).get(dir_name)
    if not dir_data:
        return 0
    fixtures = dir_data.get("fixtures", [])
    return len(fixtures)


def compute_feature_coverage(catalog: dict[str, Any]) -> dict[str, Any]:
    """Compute per-feature coverage from catalog data.

    Returns a dict with:
      - features: dict mapping directory_name -> {status, category, expected, fixture_count}
      - summary: aggregated totals
    """
    directories = catalog.get("directories", {})
    features: dict[str, dict[str, Any]] = {}
    totals = {
        "total_dirs": 0,
        "pass": 0,
        "unknown": 0,
        "total_fixtures": 0,
        "pass_fixtures": 0,
    }

    # Directories to skip in feature reporting
    skip_dirs = {"parser-errors", "negative", "test-infrastructure"}

    for dir_name, dir_data in sorted(directories.items()):
        if dir_name in skip_dirs:
            continue

        status = dir_data.get("status", "unknown")
        category = dir_data.get("category", "unknown")
        expected = dir_data.get("expected", "")
        fixtures = dir_data.get("fixtures", [])
        fixture_count = len(fixtures)

        features[dir_name] = {
            "status": status,
            "category": category,
            "expected": expected,
            "fixture_count": fixture_count,
        }

        totals["total_dirs"] += 1
        totals["total_fixtures"] += fixture_count

        if status == "pass":
            totals["pass"] += 1
            totals["pass_fixtures"] += fixture_count
        else:
            totals["unknown"] += 1

    return {
        "features": features,
        "summary": totals,
        "timestamp": __import__("datetime").datetime.now(
            __import__("datetime").timezone.utc
        ).strftime("%Y-%m-%dT%H:%M:%SZ"),
    }


def format_markdown_report(data: dict[str, Any]) -> str:
    """Format coverage data as a human-readable markdown report."""
    features = data["features"]
    summary = data["summary"]
    lines: list[str] = []

    lines.append("# Semantic Coverage Dashboard")
    lines.append("")
    lines.append(
        f"_Generated: {data['timestamp']}_"
    )
    lines.append("")
    lines.append(
        f"**Total feature areas:** {summary['total_dirs']}  "
        f"**Pass:** {summary['pass']}  "
        f"**Unknown/incomplete:** {summary['unknown']}  "
        f"**Total fixtures:** {summary['total_fixtures']}  "
        f"**Pass fixtures:** {summary['pass_fixtures']}"
    )
    lines.append("")

    # Per-category breakdown
    category_groups: dict[str, list[tuple[str, dict[str, Any]]]] = {}
    for feat_name, feat_data in features.items():
        cat = feat_data["category"]
        if cat not in category_groups:
            category_groups[cat] = []
        category_groups[cat].append((feat_name, feat_data))

    for category in sorted(category_groups.keys()):
        cat_features = category_groups[category]
        cat_pass = sum(
            1 for _, fd in cat_features if fd["status"] == "pass"
        )
        cat_total = len(cat_features)
        cat_fixtures = sum(fd["fixture_count"] for _, fd in cat_features)

        lines.append(f"## Category: {category} ({cat_pass}/{cat_total} pass, {cat_fixtures} fixtures)")
        lines.append("")
        lines.append("| Feature | Status | Fixtures | Description |")
        lines.append("|---------|--------|----------|-------------|")
        for feat_name, feat_data in sorted(cat_features):
            status_icon = "PASS" if feat_data["status"] == "pass" else "?"
            lines.append(
                f"| {feat_name} | {status_icon} | {feat_data['fixture_count']} | {feat_data['expected']} |"
            )
        lines.append("")

    return "\n".join(lines)


def format_json_report(data: dict[str, Any]) -> str:
    """Format coverage data as JSON."""
    return json.dumps(data, indent=2, ensure_ascii=False)


def get_coverage_baseline(data: dict[str, Any]) -> dict[str, Any]:
    """Compute a baseline dict suitable for comparison in the regression gate.

    The baseline contains:
      - feature_count: number of feature directories
      - pass_count: number of features with status "pass"
      - total_fixtures: total number of fixture files
      - pass_fixtures: number of fixtures in pass-status directories
      - features: dict of feature_name -> status for comparison
    """
    features = data["features"]
    summary = data["summary"]

    feature_statuses = {
        name: feat["status"] for name, feat in sorted(features.items())
    }

    return {
        "timestamp": data["timestamp"],
        "feature_count": summary["total_dirs"],
        "pass_count": summary["pass"],
        "unknown_count": summary["unknown"],
        "total_fixtures": summary["total_fixtures"],
        "pass_fixtures": summary["pass_fixtures"],
        "features": feature_statuses,
    }


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Semantic coverage dashboard: per-feature coverage report",
        add_help=False,
    )
    parser.add_argument("--json", action="store_true", help="Output JSON instead of markdown")
    parser.add_argument("--baseline", action="store_true", help="Output baseline JSON (for regression gate)")
    parser.add_argument("--help", action="store_true", help="Show help and exit")

    args, _unknown = parser.parse_known_args()

    if args.help:
        print(__doc__)
        return 0

    catalog = load_catalog()
    coverage_data = compute_feature_coverage(catalog)

    if args.baseline:
        baseline = get_coverage_baseline(coverage_data)
        print(json.dumps(baseline, indent=2, ensure_ascii=False))
        return 0

    if args.json:
        print(format_json_report(coverage_data))
    else:
        print(format_markdown_report(coverage_data))

    return 0


if __name__ == "__main__":
    sys.exit(main())
