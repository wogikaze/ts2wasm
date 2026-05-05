#!/usr/bin/env python3
"""
Send Japanese development loop report to Discord via webhook.

Usage:
    cat cycle_report.md | scripts/manager discord-report [--run-id <run_id>]
    scripts/manager discord-report reports/runs/<run_id>/cycle_report.md
    scripts/manager discord-report reports/runs/<run_id>/discord_payload.json
"""

import argparse
import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Optional

import requests

REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_SENT_REGISTRY = REPO_ROOT / ".discord-report-sent.json"
CONTENT_LIMIT = 1900
EMBED_TOTAL_LIMIT = 5600
FIELD_VALUE_LIMIT = 900
SECTION_LINE_LIMIT = 2
PLACEHOLDER_VALUES = {"未記入", "なし", "-", "n/a", "N/A"}
JAPANESE_RE = re.compile(r"[\u3040-\u30ff\u3400-\u9fff]")
ASCII_WORD_RE = re.compile(r"\b[A-Za-z]{4,}\b")


def load_env() -> dict[str, str]:
    """Load .env file if it exists."""
    env_file = REPO_ROOT / ".env"
    env = dict(os.environ)
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, value = line.split("=", 1)
                env.setdefault(key.strip(), value.strip())
    return env


def parse_cycle_report(content: str) -> dict[str, str]:
    """Parse Japanese cycle report markdown and extract report fields."""
    # Default values
    fields = {
        "status": "未記入",
        "purpose": "未記入",
        "actions": "未記入",
        "reasoning": "未記入",
        "blockers": "なし",
        "risks": "未記入",
        "next": "未記入",
        "issues": "未記入",
    }

    # Parse sections using regex
    patterns = {
        "status": r"## 状態\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "purpose": r"## (?:目的|今回の目的)\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "actions": r"## 実施内容\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "reasoning": r"## 判断と根拠\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "blockers": r"## 詰まり・ロス\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "risks": r"## リスク\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "next": r"## 次にやるべきこと\s*\n+(.+?)(?=\n##|\n---|\Z)",
        "issues": r"## 完了(?:・|\s*/\s*)追加\s*\n+(.+?)(?=\n##|\n---|\Z)",
    }

    for key, pattern in patterns.items():
        match = re.search(pattern, content, re.DOTALL)
        if match:
            # Clean up: remove markdown formatting, limit length
            value = match.group(1).strip()
            # Remove leading/trailing whitespace and empty lines
            lines = [line.strip() for line in value.split("\n") if line.strip()]
            if len(lines) > SECTION_LINE_LIMIT:
                lines = lines[:SECTION_LINE_LIMIT]
            fields[key] = "\n".join(lines)[:FIELD_VALUE_LIMIT]

    return fields


def reject_placeholder_report(fields: dict[str, str]) -> None:
    meaningful = [
        value.strip()
        for value in fields.values()
        if value.strip() and value.strip() not in PLACEHOLDER_VALUES
    ]
    placeholder_count = sum(1 for value in fields.values() if value.strip() in PLACEHOLDER_VALUES)
    if len(meaningful) < 3 or placeholder_count >= 5:
        print(
            "エラー: Discord レポートが未記入または実質空です。"
            " 状態、目的、実施内容、検証、blocker/next action を簡潔に埋めてください。",
            file=sys.stderr,
        )
        sys.exit(1)


def reject_non_japanese_text(text: str) -> None:
    japanese_chars = len(JAPANESE_RE.findall(text))
    ascii_words = len(ASCII_WORD_RE.findall(text))
    if ascii_words >= 4 and japanese_chars < 4:
        print(
            "エラー: Discord レポート本文が英語中心です。"
            " レポートは日本語で簡潔に書き、コマンド/パス/issue ID のみ英字を使ってください。",
            file=sys.stderr,
        )
        sys.exit(1)


def reject_non_japanese_report(fields: dict[str, str]) -> None:
    text = "\n".join(value for value in fields.values() if value.strip())
    reject_non_japanese_text(text)


def collect_payload_text(data: Any) -> list[str]:
    if isinstance(data, str):
        return [data]
    if isinstance(data, dict):
        parts: list[str] = []
        for key in ("content", "title", "description", "name", "value", "text"):
            value = data.get(key)
            if isinstance(value, str):
                parts.append(value)
        for value in data.values():
            if isinstance(value, (dict, list)):
                parts.extend(collect_payload_text(value))
        return parts
    if isinstance(data, list):
        parts = []
        for value in data:
            parts.extend(collect_payload_text(value))
        return parts
    return []


def create_discord_embed(fields: dict[str, str], run_id: Optional[str]) -> dict[str, Any]:
    """Create Japanese Discord embed from report fields."""
    embed = {
        "username": "ts2wasm-report",
        "embeds": [
            {
                "title": "ts2wasm 開発レポート",
                "color": 5814783,  # Blue
                "fields": [
                    {
                        "name": "📊 状態",
                        "value": fields["status"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "🎯 目的",
                        "value": fields["purpose"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "🔄 実施内容",
                        "value": fields["actions"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "🧠 判断と根拠",
                        "value": fields["reasoning"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "⚠️ 詰まり・ロス",
                        "value": fields["blockers"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "📉 リスク",
                        "value": fields["risks"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "➡️ 次にやるべきこと",
                        "value": fields["next"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                    {
                        "name": "📌 完了 / 追加",
                        "value": fields["issues"][:FIELD_VALUE_LIMIT],
                        "inline": False,
                    },
                ],
            }
        ]
    }
    if run_id:
        embed["embeds"][0]["footer"] = {"text": f"run: {run_id}"}
    return embed


def create_json_payload(content: str, source_path: Path) -> dict[str, Any]:
    """Create a Discord payload from a JSON file."""
    try:
        data = json.loads(content)
    except json.JSONDecodeError as e:
        print(f"エラー: JSON を解析できません: {source_path}: {e}", file=sys.stderr)
        sys.exit(1)

    if isinstance(data, dict) and ("content" in data or "embeds" in data):
        payload = dict(data)
        payload.setdefault("username", "ts2wasm-report")
        reject_non_japanese_text("\n".join(collect_payload_text(payload)))
        return payload

    text = json.dumps(data, indent=2, ensure_ascii=False)
    reject_non_japanese_text(text)
    rel = display_path(source_path)
    return {
        "username": "ts2wasm-report",
        "content": f"Discord JSON レポート: {rel}\n```json\n{text}\n```",
    }


def create_markdown_payload(content: str, run_id: Optional[str]) -> dict[str, Any]:
    fields = parse_cycle_report(content)
    reject_placeholder_report(fields)
    reject_non_japanese_report(fields)
    return create_discord_embed(fields, run_id)


def send_discord_webhook(webhook_url: str, payload: dict[str, Any]) -> bool:
    """Send payload to Discord webhook."""
    try:
        response = requests.post(webhook_url, json=payload, timeout=10)
        response.raise_for_status()
        return True
    except requests.RequestException as e:
        print(f"Error sending to Discord: {e}", file=sys.stderr)
        return False


def text_chunks(text: str, limit: int) -> list[str]:
    if len(text) <= limit:
        return [text]
    first = text[:limit].rstrip()
    second = text[limit : limit * 2].rstrip()
    if len(text) > limit * 2:
        suffix = "\n[truncated]"
        second = second[: limit - len(suffix)].rstrip() + suffix
    return [first, second or "[continued]"]


def embed_text_size(embed: dict[str, Any]) -> int:
    total = len(str(embed.get("title", ""))) + len(str(embed.get("description", "")))
    footer = embed.get("footer")
    if isinstance(footer, dict):
        total += len(str(footer.get("text", "")))
    for field in embed.get("fields", []):
        if isinstance(field, dict):
            total += len(str(field.get("name", ""))) + len(str(field.get("value", "")))
    return total


def split_payload_for_discord(payload: dict[str, Any]) -> list[dict[str, Any]]:
    """Keep messages concise and split oversized payloads into at most two sends."""
    content = payload.get("content")
    if isinstance(content, str) and len(content) > CONTENT_LIMIT:
        chunks = text_chunks(content, CONTENT_LIMIT - 14)
        return [
            {**payload, "content": f"({i}/{len(chunks)}) {chunk}"}
            for i, chunk in enumerate(chunks, 1)
        ]

    embeds = payload.get("embeds")
    if not isinstance(embeds, list) or not embeds:
        return [payload]

    if len(embeds) > 1:
        return [{**payload, "embeds": embeds[:5]}, {**payload, "embeds": embeds[5:10]}] if len(embeds) > 5 else [payload]

    embed = embeds[0]
    if not isinstance(embed, dict):
        return [payload]

    fields = embed.get("fields")
    if not isinstance(fields, list) or embed_text_size(embed) <= EMBED_TOTAL_LIMIT:
        return [payload]

    midpoint = max(1, (len(fields) + 1) // 2)
    parts = [fields[:midpoint], fields[midpoint:]]
    split_payloads = []
    for i, part_fields in enumerate(parts, 1):
        split_embed = dict(embed)
        split_embed["fields"] = part_fields
        split_embed["title"] = f"{embed.get('title', 'Discord report')} ({i}/2)"
        split_payloads.append({**payload, "embeds": [split_embed]})
    return split_payloads


def display_path(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO_ROOT))
    except ValueError:
        return str(path.resolve())


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_sent_registry(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"version": 1, "files": {}}
    try:
        data = json.loads(path.read_text())
    except json.JSONDecodeError as e:
        print(f"エラー: sent registry が壊れています: {path}: {e}", file=sys.stderr)
        sys.exit(1)
    if not isinstance(data, dict):
        print(f"エラー: sent registry が object ではありません: {path}", file=sys.stderr)
        sys.exit(1)
    files = data.setdefault("files", {})
    if not isinstance(files, dict):
        print(f"エラー: sent registry の files が object ではありません: {path}", file=sys.stderr)
        sys.exit(1)
    data.setdefault("version", 1)
    return data


def fail_if_already_sent(registry: dict[str, Any], source_path: Path) -> None:
    key = display_path(source_path)
    record = registry.get("files", {}).get(key)
    if not record:
        return
    sent_at = record.get("sent_at", "unknown") if isinstance(record, dict) else "unknown"
    run_id = record.get("run_id", "unknown") if isinstance(record, dict) else "unknown"
    print(
        f"エラー: このレポートファイルは既に Discord へ送信済みです: {key} "
        f"(sent_at={sent_at}, run_id={run_id})",
        file=sys.stderr,
    )
    sys.exit(1)


def mark_sent(
    registry_path: Path,
    registry: dict[str, Any],
    source_path: Path,
    run_id: Optional[str],
    payload_kind: str,
) -> None:
    key = display_path(source_path)
    files = registry.setdefault("files", {})
    files[key] = {
        "sha256": file_sha256(source_path),
        "sent_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "run_id": run_id,
        "kind": payload_kind,
    }
    tmp_path = registry_path.with_suffix(registry_path.suffix + ".tmp")
    tmp_path.write_text(json.dumps(registry, indent=2, sort_keys=True, ensure_ascii=False) + "\n")
    tmp_path.replace(registry_path)


def load_input_payload(args: argparse.Namespace) -> tuple[dict[str, Any], Optional[Path], str]:
    if args.input_file:
        source_path = Path(args.input_file)
        if not source_path.is_absolute():
            source_path = REPO_ROOT / source_path
        if not source_path.exists():
            print(f"エラー: 入力ファイルが存在しません: {source_path}", file=sys.stderr)
            sys.exit(1)
        if not source_path.is_file():
            print(f"エラー: 入力パスがファイルではありません: {source_path}", file=sys.stderr)
            sys.exit(1)

        suffix = source_path.suffix.lower()
        if suffix not in (".md", ".json"):
            print("エラー: 入力ファイルは .md または .json のみ対応です", file=sys.stderr)
            sys.exit(1)

        content = source_path.read_text()
        if not content.strip():
            print(f"エラー: 入力ファイルが空です: {source_path}", file=sys.stderr)
            sys.exit(1)

        if suffix == ".json":
            return create_json_payload(content, source_path), source_path, "json"
        return create_markdown_payload(content, args.run_id), source_path, "markdown"

    content = sys.stdin.read()
    if not content:
        print("エラー: 標準入力にレポート本文がありません", file=sys.stderr)
        sys.exit(1)
    return create_markdown_payload(content, args.run_id), None, "stdin-markdown"


def main():
    parser = argparse.ArgumentParser(description="Discord に日本語の開発ループレポートを送信する")
    parser.add_argument("input_file", nargs="?", help=".md cycle report or .json Discord payload file")
    parser.add_argument("--run-id", help="Run ID (e.g., 20260426-120000) for footer")
    parser.add_argument("--dry-run", action="store_true", help="送信せずに embed JSON を表示する")
    parser.add_argument(
        "--sent-registry",
        default=str(DEFAULT_SENT_REGISTRY),
        help="送信済みファイルを記録する registry path",
    )
    args = parser.parse_args()

    payload, source_path, payload_kind = load_input_payload(args)
    payloads = split_payload_for_discord(payload)

    registry_path = Path(args.sent_registry)
    if not registry_path.is_absolute():
        registry_path = REPO_ROOT / registry_path
    registry = load_sent_registry(registry_path)
    if source_path is not None:
        fail_if_already_sent(registry, source_path)

    if args.dry_run:
        print("Discord embed (dry run):")
        print(json.dumps(payloads, indent=2, ensure_ascii=False))
        sys.exit(0)

    env = load_env()
    webhook_url = env.get("DISCORD_WEBHOOK_URL")
    if not webhook_url:
        print("エラー: DISCORD_WEBHOOK_URL が環境変数または .env に設定されていません", file=sys.stderr)
        sys.exit(1)

    for index, payload_part in enumerate(payloads, 1):
        if not send_discord_webhook(webhook_url, payload_part):
            print(f"エラー: Discord 送信に失敗しました ({index}/{len(payloads)})", file=sys.stderr)
            sys.exit(1)

    if source_path is not None:
        mark_sent(registry_path, registry, source_path, args.run_id, payload_kind)
    print(f"Discord にレポートを送信しました ({len(payloads)} message(s))")
    sys.exit(0)


if __name__ == "__main__":
    main()
