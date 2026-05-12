#!/usr/bin/env python3
"""Generate a concise Japanese development report for Discord.

Usage:
  python scripts/manager.py development-report [--output <path>] [--run-id <run_id>]
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
ISSUES_DIR = REPO_ROOT / "issues"


def run_git(args: list[str], default: str = "") -> str:
    try:
        result = subprocess.run(
            ["git", *args],
            cwd=REPO_ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError:
        return default
    if result.returncode != 0:
        return default
    return result.stdout.strip()


def parse_issue_header(path: Path) -> dict[str, str]:
    header: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if line == "---":
            break
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        header[key.strip()] = value.strip()
    return header


def issue_summary() -> tuple[Counter[str], list[dict[str, str]]]:
    counts: Counter[str] = Counter()
    ready: list[dict[str, str]] = []
    for path in sorted(ISSUES_DIR.glob("I-*.md")):
        header = parse_issue_header(path)
        status = header.get("Status", "unknown")
        counts[status] += 1
        if status == "open" and header.get("Priority") in {"P0", "P1"}:
            ready.append(header)
    ready.sort(key=lambda item: (item.get("Priority", "P9"), item.get("Updated", "")))
    return counts, ready[:5]


def recent_commits() -> list[str]:
    output = run_git(["log", "--since=24 hours ago", "--pretty=format:%h %s", "-n", "8"])
    if not output:
        output = run_git(["log", "--pretty=format:%h %s", "-n", "5"])
    return [line for line in output.splitlines() if line.strip()]


def workflow_url() -> str | None:
    server = os.environ.get("GITHUB_SERVER_URL")
    repo = os.environ.get("GITHUB_REPOSITORY")
    run_id = os.environ.get("GITHUB_RUN_ID")
    if server and repo and run_id:
        return f"{server}/{repo}/actions/runs/{run_id}"
    return None


def build_report(run_id: str | None) -> str:
    now = datetime.now(timezone.utc).isoformat()
    branch = run_git(["branch", "--show-current"], default="unknown")
    commit = run_git(["rev-parse", "--short", "HEAD"], default="unknown")
    status_lines = run_git(["status", "--short"]).splitlines()
    dirty_count = len([line for line in status_lines if line.strip()])
    counts, ready = issue_summary()
    commits = recent_commits()
    url = workflow_url()

    lines = [
        f"# 開発定期レポート: {run_id or now}",
        "",
        "## 状態",
        f"- 生成時刻(UTC): {now}",
        f"- ブランチ: `{branch}`",
        f"- HEAD: `{commit}`",
        f"- 作業ツリー差分: {dirty_count} 件",
        f"- Issue: open {counts.get('open', 0)} / doing {counts.get('doing', 0)} / blocked {counts.get('blocked', 0)} / done {counts.get('done', 0)}",
    ]
    if url:
        lines.append(f"- Workflow: {url}")

    lines.extend(["", "## 直近の変更"])
    if commits:
        lines.extend(f"- `{line.split(' ', 1)[0]}` {line.split(' ', 1)[1] if ' ' in line else ''}".rstrip() for line in commits)
    else:
        lines.append("- 直近 24 時間の commit はありません。")

    lines.extend(["", "## 優先候補"])
    if ready:
        for item in ready:
            lines.append(
                f"- `{item.get('Id', 'unknown')}` {item.get('Priority', '')}: {item.get('Title', '')}"
            )
    else:
        lines.append("- ready な P0/P1 open issue はありません。")

    lines.extend(
        [
            "",
            "## 次アクション",
            "- ready な P0/P1 issue を 1 件選び、focused gate と local commit まで進める。",
        ]
    )
    return "\n".join(lines) + "\n"


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", "-o", default=None, help="Write report to this path")
    parser.add_argument("--run-id", default=None, help="Report run identifier")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    report = build_report(args.run_id)
    if args.output:
        output = (REPO_ROOT / args.output).resolve()
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(report, encoding="utf-8")
        print(f"development-report: wrote {output.relative_to(REPO_ROOT)}")
    else:
        print(report, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
