#!/usr/bin/env python3
"""
Send development loop report to Discord via webhook.

Usage:
    python scripts/report/discord-report.py --run-id <run_id>
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Optional

import requests

REPO_ROOT = Path(__file__).resolve().parents[3]
REPORTS_DIR = REPO_ROOT / "reports" / "runs"


def load_env() -> dict[str, str]:
    """Load .env file if it exists."""
    env_file = REPO_ROOT / ".env"
    env = {}
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, value = line.split("=", 1)
                env[key.strip()] = value.strip()
    return env


def parse_cycle_report(run_dir: Path) -> dict[str, str]:
    """Parse cycle_report.md and extract report fields."""
    report_file = run_dir / "cycle_report.md"
    if not report_file.exists():
        raise FileNotFoundError(f"cycle_report.md not found in {run_dir}")

    content = report_file.read_text()

    # Default values
    fields = {
        "status": "N/A",
        "purpose": "N/A",
        "actions": "N/A",
        "reasoning": "N/A",
        "blockers": "N/A",
        "risks": "N/A",
        "next": "N/A",
        "issues": "N/A",
    }

    # Parse sections using regex
    patterns = {
        "status": r"## 状態\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "purpose": r"## 目的\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "actions": r"## 実施内容\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "reasoning": r"## 判断と根拠\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "blockers": r"## 詰まり・ロス\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "risks": r"## リスク\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "next": r"## 次にやるべきこと\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "issues": r"## 完了・追加\s*\n+(.+?)(?=\n##|\n---|\Z)",
    }

    for key, pattern in patterns.items():
        match = re.search(pattern, content, re.DOTALL)
        if match:
            # Clean up: remove markdown formatting, limit length
            value = match.group(1).strip()
            # Remove leading/trailing whitespace and empty lines
            lines = [line.strip() for line in value.split("\n") if line.strip()]
            # Limit to 5 lines for Discord field limit
            if len(lines) > 5:
                lines = lines[:5]
            fields[key] = "\n".join(lines)

    return fields


def create_discord_embed(fields: dict[str, str], run_id: str) -> dict:
    """Create Discord embed from report fields."""
    embed = {
        "title": "ts2wasm 開発ループレポート",
        "color": 5814783,  # Blue
        "fields": [
            {
                "name": "📊 状態",
                "value": fields["status"][:1024],
                "inline": False,
            },
            {
                "name": "🎯 目的",
                "value": fields["purpose"][:1024],
                "inline": False,
            },
            {
                "name": "🔄 実施内容",
                "value": fields["actions"][:1024],
                "inline": False,
            },
            {
                "name": "🧠 判断と根拠",
                "value": fields["reasoning"][:1024],
                "inline": False,
            },
            {
                "name": "⚠️ 詰まり・ロス",
                "value": fields["blockers"][:1024],
                "inline": False,
            },
            {
                "name": "📉 リスク",
                "value": fields["risks"][:1024],
                "inline": False,
            },
            {
                "name": "➡️ 次にやるべきこと",
                "value": fields["next"][:1024],
                "inline": False,
            },
            {
                "name": "📌 完了 / 追加",
                "value": fields["issues"][:1024],
                "inline": False,
            },
        ],
        "footer": {
            "text": f"run: {run_id}",
        },
    }
    return {"embeds": [embed]}


def send_discord_webhook(webhook_url: str, embed: dict) -> bool:
    """Send embed to Discord webhook."""
    try:
        response = requests.post(webhook_url, json=embed, timeout=10)
        response.raise_for_status()
        return True
    except requests.RequestException as e:
        print(f"Error sending to Discord: {e}", file=sys.stderr)
        return False


def main():
    parser = argparse.ArgumentParser(description="Send development loop report to Discord")
    parser.add_argument("--run-id", required=True, help="Run ID (e.g., 20260426-120000)")
    parser.add_argument("--dry-run", action="store_true", help="Print embed without sending")
    args = parser.parse_args()

    # Load environment
    env = load_env()
    webhook_url = env.get("DISCORD_WEBHOOK_URL")
    if not webhook_url:
        print("Error: DISCORD_WEBHOOK_URL not set in .env", file=sys.stderr)
        sys.exit(1)

    # Find run directory
    run_dir = REPORTS_DIR / args.run_id
    if not run_dir.exists():
        print(f"Error: Run directory not found: {run_dir}", file=sys.stderr)
        sys.exit(1)

    # Parse cycle report
    try:
        fields = parse_cycle_report(run_dir)
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    # Create Discord embed
    embed = create_discord_embed(fields, args.run_id)

    if args.dry_run:
        print("Discord embed (dry run):")
        print(json.dumps(embed, indent=2, ensure_ascii=False))
        sys.exit(0)

    # Send to Discord
    if send_discord_webhook(webhook_url, embed):
        print(f"Report sent to Discord for run {args.run_id}")
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
