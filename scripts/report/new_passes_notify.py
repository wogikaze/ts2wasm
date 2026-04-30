#!/usr/bin/env python3
"""Notify Discord of newly passing tests by comparing against baseline."""

import json
import os
import sys
import urllib.request
import urllib.error
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BASELINE_DIR = REPO_ROOT / "artifacts" / "coverage" / "baselines"
MAX_DISPLAY_TESTS = 20
FIELD_CHAR_LIMIT = 1900


def load_env():
    """Load environment variables from .env file."""
    env_file = REPO_ROOT / ".env"
    if env_file.is_file():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, value = line.split("=", 1)
            os.environ.setdefault(key.strip(), value.strip())


def load_baseline(baseline_path):
    """Load previous baseline, return (dict, prev_pass_count) or ({}, 0)."""
    if not baseline_path.is_file():
        return {}, 0
    try:
        data = json.loads(baseline_path.read_text())
        prev_pass = (data.pop("_meta", {}) or {}).get("pass_count", 0)
        return data, prev_pass
    except (json.JSONDecodeError, KeyError):
        return {}, 0


def save_baseline(baseline_path, baseline, total, pass_count):
    """Save updated baseline."""
    baseline_path.parent.mkdir(parents=True, exist_ok=True)
    data = dict(baseline)
    data["_meta"] = {
        "updated_at": datetime.now(timezone.utc).isoformat(),
        "total": total,
        "pass_count": pass_count,
    }
    baseline_path.write_text(
        json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8"
    )


def send_discord(webhook_url, embed):
    """Send Discord embed via webhook."""
    payload = json.dumps({"embeds": [embed]}).encode("utf-8")
    req = urllib.request.Request(
        webhook_url,
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            if resp.status not in (200, 204):
                print(f"WARNING: Discord returned {resp.status}", file=sys.stderr)
    except (urllib.error.URLError, urllib.error.HTTPError, OSError) as e:
        print(f"WARNING: Discord send failed: {e}", file=sys.stderr)


def build_embed(suite, prev_pass, curr_pass, delta, new_passes):
    """Build Discord embed dict."""
    short_paths = []
    for p in new_passes[:MAX_DISPLAY_TESTS]:
        # Strip prefix to get short relative path
        short = p
        for prefix in [
            "reference/test262/test/",
            "reference/typescript/tests/cases/compiler/",
            "reference/typescript-go/testdata/tests/",
        ]:
            if prefix in p:
                short = p.split(prefix, 1)[1]
                break
        short_paths.append(short)

    value_lines = "\n".join(short_paths)
    remaining = len(new_passes) - MAX_DISPLAY_TESTS
    if remaining > 0:
        value_lines += f"\n... and {remaining} more"

    # Truncate if over Discord limit
    if len(value_lines) > FIELD_CHAR_LIMIT:
        value_lines = value_lines[: FIELD_CHAR_LIMIT - 20] + "\n... (truncated)"

    return {
        "title": "New Passing Tests",
        "color": 5814783,
        "fields": [
            {
                "name": "Suite",
                "value": f"{suite} | 前回 {prev_pass:,} pass → 今回 {curr_pass:,} pass (+{delta:,})",
                "inline": False,
            },
            {
                "name": f"Newly passing tests ({delta})",
                "value": value_lines or "(none)",
                "inline": False,
            },
        ],
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


def notify_new_passes(jsonl_path, suite="test262"):
    """
    Compare JSONL against baseline and send Discord notification if new passes found.
    """
    jsonl_path = Path(jsonl_path)
    if not jsonl_path.is_file():
        raise FileNotFoundError(f"JSONL file not found: {jsonl_path}")

    baseline_dir = DEFAULT_BASELINE_DIR
    baseline_path = baseline_dir / f"{suite}-statuses.json"

    # Load previous baseline
    baseline, prev_pass_count = load_baseline(baseline_path)

    # Read JSONL and compare
    new_baseline = {}
    new_passes = []
    curr_pass_count = 0
    total = 0

    with open(jsonl_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            record = json.loads(line)
            case = record.get("case", "")
            status = record.get("status", "")
            key = f"{suite}::{case}"

            new_baseline[key] = status
            total += 1

            if status == "pass":
                curr_pass_count += 1
                prev_status = baseline.get(key)
                if prev_status is not None and prev_status != "pass":
                    new_passes.append(case)

    # Save updated baseline
    save_baseline(baseline_path, new_baseline, total, curr_pass_count)

    # Notify if new passes found
    delta = curr_pass_count - prev_pass_count
    if delta > 0 and new_passes:
        load_env()
        webhook_url = os.environ.get("DISCORD_WEBHOOK_URL")
        if not webhook_url:
            print(
                f"INFO: DISCORD_WEBHOOK_URL not set, skipping notification ({delta} new passes)",
                file=sys.stderr,
            )
            return

        embed = build_embed(suite, prev_pass_count, curr_pass_count, delta, new_passes)
        send_discord(webhook_url, embed)
        print(
            f"Discord notification sent: {delta} new passes in {suite}",
            file=sys.stderr,
        )
    elif delta > 0:
        print(
            f"INFO: {delta} new passes in {suite} (all from initial baseline, no previous data)",
            file=sys.stderr,
        )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(
            "Usage: python scripts/report/new-passes-notify.py <jsonl_path> [--suite <suite>]",
            file=sys.stderr,
        )
        sys.exit(1)

    jsonl_path = sys.argv[1]
    suite = "test262"
    for i, arg in enumerate(sys.argv[2:], start=2):
        if arg == "--suite" and i + 1 < len(sys.argv):
            suite = sys.argv[i + 1]

    notify_new_passes(jsonl_path, suite)
