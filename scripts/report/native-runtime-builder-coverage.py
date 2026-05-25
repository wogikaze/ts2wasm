#!/usr/bin/env python3
"""Report native RuntimeFn builder coverage."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts" / "lib"))

from native_runtime_builder_coverage import summarize_native_runtime_builder_coverage


def print_text(summary: dict[str, object]) -> None:
    print(f"runtime_fn_total: {summary['runtime_fn_total']}")
    print(f"non_pseudo_total: {summary['non_pseudo_total']}")
    print(f"available: {summary['available']}")
    print(f"pseudo: {summary['pseudo']}")
    print(f"missing_non_pseudo: {summary['missing_non_pseudo']}")
    print(f"coverage_percent: {summary['coverage_percent']}")
    for name in summary["missing"]:
        print(f"- {name}")


def print_markdown(summary: dict[str, object]) -> None:
    print("| Metric | Value |")
    print("|---|---:|")
    for key in (
        "runtime_fn_total",
        "non_pseudo_total",
        "available",
        "pseudo",
        "missing_non_pseudo",
        "coverage_percent",
    ):
        print(f"| `{key}` | `{summary[key]}` |")
    missing = summary["missing"]
    if missing:
        print()
        print("| Missing RuntimeFn |")
        print("|---|")
        for name in missing:
            print(f"| `{name}` |")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--format",
        choices=("text", "markdown", "json"),
        default="text",
        help="output format",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail when any non-pseudo RuntimeFn is missing a native builder",
    )
    args = parser.parse_args(argv)

    summary = summarize_native_runtime_builder_coverage()
    if args.format == "json":
        print(json.dumps(summary, indent=2, sort_keys=True))
    elif args.format == "markdown":
        print_markdown(summary)
    else:
        print_text(summary)

    if args.check and int(summary["missing_non_pseudo"]) != 0:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
