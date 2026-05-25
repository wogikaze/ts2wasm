#!/usr/bin/env python3
"""Summarize native LoweredProgram emitter unsupported reasons from JSONL records."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_JSONL = Path("artifacts/coverage/results/test262-results.jsonl")
NATIVE_REASON_RE = re.compile(r"native LoweredProgram emitter[^\n\r\"']+")


def record_text(record: dict[str, Any]) -> str:
    parts = []
    for key in (
        "reason",
        "diagnostic_message",
        "message",
        "detail",
        "stderr",
        "error",
    ):
        value = record.get(key)
        if value is not None:
            parts.append(str(value))
    diagnostic = record.get("diagnostic")
    if isinstance(diagnostic, dict):
        parts.extend(str(value) for value in diagnostic.values() if value is not None)
    return "\n".join(parts)


def record_case(record: dict[str, Any]) -> str:
    for key in ("case", "path", "file", "input"):
        value = record.get(key)
        if value:
            return str(value)
    return ""


def summarize(path: Path) -> tuple[Counter[str], dict[str, str], int]:
    counts: Counter[str] = Counter()
    examples: dict[str, str] = {}
    total = 0
    with path.open(encoding="utf-8", errors="replace") as handle:
        for line_no, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError as exc:
                raise SystemExit(f"{path}:{line_no}: invalid JSONL record: {exc}") from exc
            total += 1
            text = record_text(record)
            seen_reasons: set[str] = set()
            for match in NATIVE_REASON_RE.finditer(text):
                reason = match.group(0).strip().rstrip(".")
                if reason in seen_reasons:
                    continue
                seen_reasons.add(reason)
                counts[reason] += 1
                examples.setdefault(reason, record_case(record))
    return counts, examples, total


def print_text(counts: Counter[str], examples: dict[str, str], total: int) -> None:
    native_total = sum(counts.values())
    print(f"records: {total}")
    print(f"native_unsupported: {native_total}")
    for reason, count in counts.most_common():
        example = examples.get(reason, "")
        if example:
            print(f"{count}\t{reason}\t{example}")
        else:
            print(f"{count}\t{reason}")


def print_markdown(counts: Counter[str], examples: dict[str, str], total: int) -> None:
    native_total = sum(counts.values())
    print(f"- records: {total}")
    print(f"- native_unsupported: {native_total}")
    print()
    print("| Count | Reason | Example |")
    print("|---:|---|---|")
    for reason, count in counts.most_common():
        example = examples.get(reason, "")
        print(f"| {count} | `{reason}` | `{example}` |")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("jsonl", nargs="?", type=Path, default=DEFAULT_JSONL)
    parser.add_argument(
        "--format",
        choices=("text", "markdown", "json"),
        default="text",
        help="output format",
    )
    args = parser.parse_args(argv)

    if not args.jsonl.exists():
        raise SystemExit(f"JSONL file not found: {args.jsonl}")

    counts, examples, total = summarize(args.jsonl)
    if args.format == "json":
        payload = {
            "records": total,
            "native_unsupported": sum(counts.values()),
            "reasons": [
                {
                    "reason": reason,
                    "count": count,
                    "example": examples.get(reason, ""),
                }
                for reason, count in counts.most_common()
            ],
        }
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    elif args.format == "markdown":
        print_markdown(counts, examples, total)
    else:
        print_text(counts, examples, total)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
