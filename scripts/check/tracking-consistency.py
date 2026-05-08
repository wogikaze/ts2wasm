#!/usr/bin/env python3
"""Validate TRACKING.yaml structural consistency.

Read-only: this script must never modify TRACKING.yaml.
"""
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
TRACKING = ROOT / "TRACKING.yaml"

SECTIONS = ["open", "active", "done"]
REQUIRED = {
    "open": ["id", "title", "priority", "type", "area", "status", "created", "updated", "acceptance"],
    "active": ["id", "title", "priority", "type", "area", "status", "created", "updated", "acceptance"],
    "done": ["id", "title", "priority", "type", "area", "status", "created", "updated", "closed", "acceptance", "evidence"],
}

VALID_PRIORITIES = {"P0", "P1", "P2", "P3"}
VALID_TYPES = {"feature", "runtime-builtin", "diagnostic", "infra", "design", "docs", "bug", "test"}
VALID_AREAS = {"frontend", "ir", "runtime", "backend", "scripts", "docs", "shared", "cli"}

# Words that suggest an item is roadmap-scale (too large for a single session)
ROADMAP_WORDS = {"all", "complete", "full", "entire", "every", "comprehensive", "11 types", "13 traps"}


def fail(msg: str) -> None:
    print(f"tracking: invalid: {msg}", file=sys.stderr)
    sys.exit(1)


def warn(msg: str) -> None:
    print(f"tracking: warning: {msg}", file=sys.stderr)


def main() -> None:
    if not TRACKING.exists():
        fail("TRACKING.yaml does not exist")

    try:
        data = yaml.safe_load(TRACKING.read_text())
    except Exception as e:
        fail(f"YAML parse error: {e}")

    if not isinstance(data, dict):
        fail("root must be a mapping")

    meta = data.get("meta", {})
    if not isinstance(meta, dict):
        fail("meta must be a mapping")

    open_limit = int(meta.get("open_limit", 50))
    active_limit = int(meta.get("active_limit", 1))

    ids: set[int] = set()
    has_warnings = False

    for section in SECTIONS:
        items = data.get(section, [])
        if items is None:
            items = []
        if not isinstance(items, list):
            fail(f"{section} must be a list")

        if section == "open" and len(items) > open_limit:
            fail(f"open has {len(items)} items; limit is {open_limit}")
        if section == "active" and len(items) > active_limit:
            fail(f"active has {len(items)} items; limit is {active_limit}")

        for i, item in enumerate(items):
            if not isinstance(item, dict):
                fail(f"{section}[{i}] must be a mapping")

            for field in REQUIRED[section]:
                if field not in item:
                    fail(f"{section}[{i}] missing required field: {field}")

            item_id = item["id"]
            if item_id in ids:
                fail(f"duplicate id: {item_id}")
            ids.add(item_id)

            if item.get("status") != section:
                fail(f"id {item_id}: status must be '{section}'")

            # Priority validation
            priority = item.get("priority")
            if priority not in VALID_PRIORITIES:
                fail(f"id {item_id}: invalid priority '{priority}'; must be P0/P1/P2/P3")

            # Type validation
            item_type = item.get("type")
            if item_type not in VALID_TYPES:
                fail(f"id {item_id}: invalid type '{item_type}'")

            # Area validation
            area = item.get("area")
            if area not in VALID_AREAS:
                fail(f"id {item_id}: invalid area '{area}'")

            # Acceptance must be non-empty list
            acceptance = item.get("acceptance")
            if not isinstance(acceptance, list) or not acceptance:
                fail(f"id {item_id}: acceptance must be a non-empty list")

            # Warn on observation-only acceptance (grep -c without threshold)
            for cmd in acceptance:
                if "grep -c" in cmd and "--max" not in cmd and "--min" not in cmd:
                    warn(f"id {item_id}: acceptance uses grep -c without threshold — observation only")

            # Warn on roadmap-scale words in title
            title_lower = item.get("title", "").lower()
            for word in ROADMAP_WORDS:
                if word in title_lower:
                    warn(f"id {item_id}: title contains roadmap-scale word '{word}'")
                    break

            # ready check for active items
            if section == "active":
                if not item.get("ready", False):
                    fail(f"id {item_id}: active item must have ready: true")

            if section == "done":
                evidence = item.get("evidence")
                if evidence is None:
                    fail(f"id {item_id}: evidence is required for done items")

                if not isinstance(evidence, dict):
                    fail(f"id {item_id}: evidence must be a mapping")

                if not evidence.get("commit"):
                    fail(f"id {item_id}: evidence.commit is required")

                commands = evidence.get("commands")
                if not isinstance(commands, list) or not commands:
                    fail(f"id {item_id}: evidence.commands must be a non-empty list")

                for cmd in commands:
                    if not isinstance(cmd, dict):
                        fail(f"id {item_id}: each evidence command must be a mapping")
                    if not cmd.get("command"):
                        fail(f"id {item_id}: evidence command missing 'command'")
                    if cmd.get("exit") != 0:
                        fail(f"id {item_id}: evidence command did not exit 0")

    # Summary
    open_count = len(data.get("open", []) or [])
    active_count = len(data.get("active", []) or [])
    done_count = len(data.get("done", []) or [])

    if has_warnings:
        print(f"tracking: valid with warnings ({open_count} open, {active_count} active, {done_count} done)")
    else:
        print(f"tracking: valid ({open_count} open, {active_count} active, {done_count} done)")


if __name__ == "__main__":
    main()
