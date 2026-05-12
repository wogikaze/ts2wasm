#!/usr/bin/env python3
"""Send a concise development report to Discord via webhook.

Usage:
  python scripts/manager.py discord-report <report_path> [--run-id <run_id>]
  python scripts/manager.py discord-report <report_path> --dry-run

The webhook URL is read from DISCORD_WEBHOOK_URL or repo-local .env.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
REPORTS_DIR = REPO_ROOT / "reports"
REGISTRY_PATH = REPORTS_DIR / "discord-sent-registry.json"
DISCORD_CONTENT_LIMIT = 2000
CONTENT_CHUNK_LIMIT = 1850
MAX_MESSAGES = 2


def load_env() -> None:
    env_file = REPO_ROOT / ".env"
    if not env_file.is_file():
        return
    for raw_line in env_file.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        os.environ.setdefault(key.strip(), value.strip().strip("\"'"))


def load_registry(path: Path) -> dict[str, Any]:
    if not path.is_file():
        return {"sent": {}}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {"sent": {}}
    if not isinstance(data, dict) or not isinstance(data.get("sent"), dict):
        return {"sent": {}}
    return data


def save_registry(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def report_key(report_path: Path) -> str:
    resolved = report_path.resolve()
    digest = hashlib.sha256(resolved.read_bytes()).hexdigest()
    return f"{resolved}:{digest}"


def validate_markdown(text: str, report_path: Path) -> None:
    stripped = text.strip()
    if not stripped:
        raise ValueError(f"report is empty: {report_path}")
    if stripped.count("未記入") > 0:
        raise ValueError(f"report still contains 未記入 placeholders: {report_path}")
    if len(stripped) < 40:
        raise ValueError(f"report is too short to be useful: {report_path}")


def chunk_content(text: str) -> list[str]:
    content = text.strip()
    if len(content) <= DISCORD_CONTENT_LIMIT:
        return [content]

    chunks: list[str] = []
    remaining = content
    while remaining and len(chunks) < MAX_MESSAGES:
        if len(remaining) <= CONTENT_CHUNK_LIMIT:
            chunks.append(remaining)
            remaining = ""
            break
        split_at = remaining.rfind("\n", 0, CONTENT_CHUNK_LIMIT)
        if split_at < CONTENT_CHUNK_LIMIT // 2:
            split_at = CONTENT_CHUNK_LIMIT
        chunks.append(remaining[:split_at].strip())
        remaining = remaining[split_at:].strip()

    if remaining:
        raise ValueError("report is too large for the two-message Discord limit")
    return chunks


def payloads_from_markdown(text: str, run_id: str | None) -> list[dict[str, Any]]:
    header = f"**ts2wasm 開発レポート**"
    if run_id:
        header += f" `{run_id}`"
    body = f"{header}\n{text.strip()}"
    return [{"content": chunk} for chunk in chunk_content(body)]


def payloads_from_json(path: Path) -> list[dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(data, dict) and ("content" in data or "embeds" in data):
        return [data]
    pretty = json.dumps(data, indent=2, ensure_ascii=False)
    return [{"content": chunk} for chunk in chunk_content(f"```json\n{pretty}\n```")]


def build_payloads(report_path: Path, run_id: str | None) -> list[dict[str, Any]]:
    suffix = report_path.suffix.lower()
    if suffix == ".json":
        return payloads_from_json(report_path)
    text = report_path.read_text(encoding="utf-8")
    validate_markdown(text, report_path)
    return payloads_from_markdown(text, run_id)


def send_payload(webhook_url: str, payload: dict[str, Any]) -> None:
    encoded = json.dumps(payload, ensure_ascii=False).encode("utf-8")
    request = urllib.request.Request(
        webhook_url,
        data=encoded,
        headers={
            "Content-Type": "application/json",
            "User-Agent": "ts2wasm-discord-report/1.0",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=15) as response:
            if response.status not in (200, 204):
                raise RuntimeError(f"Discord returned HTTP {response.status}")
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"Discord returned HTTP {error.code}: {detail}") from error
    except (urllib.error.URLError, OSError) as error:
        raise RuntimeError(f"Discord webhook send failed: {error}") from error


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report_path", help="Markdown or JSON report path")
    parser.add_argument("--run-id", default=None, help="Run identifier shown in Discord")
    parser.add_argument("--dry-run", action="store_true", help="Print payload JSON without sending")
    parser.add_argument(
        "--allow-resend",
        action="store_true",
        help="Allow sending the same report file content more than once",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    report_path = (REPO_ROOT / args.report_path).resolve()
    if not report_path.is_file():
        print(f"discord-report: report not found: {report_path}", file=sys.stderr)
        return 2

    payloads = build_payloads(report_path, args.run_id)
    if args.dry_run:
        print(json.dumps({"payloads": payloads}, indent=2, ensure_ascii=False))
        return 0

    load_env()
    webhook_url = os.environ.get("DISCORD_WEBHOOK_URL", "").strip()
    if not webhook_url:
        print("discord-report: DISCORD_WEBHOOK_URL is not set", file=sys.stderr)
        return 2

    registry = load_registry(REGISTRY_PATH)
    key = report_key(report_path)
    if key in registry["sent"] and not args.allow_resend:
        print(f"discord-report: report was already sent: {report_path}", file=sys.stderr)
        return 2

    for payload in payloads:
        send_payload(webhook_url, payload)

    registry["sent"][key] = {
        "sent_at": datetime.now(timezone.utc).isoformat(),
        "path": str(report_path.relative_to(REPO_ROOT)),
        "run_id": args.run_id,
        "messages": len(payloads),
    }
    save_registry(REGISTRY_PATH, registry)
    print(f"discord-report: sent {len(payloads)} message(s) from {report_path.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
