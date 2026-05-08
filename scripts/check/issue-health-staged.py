#!/usr/bin/env python3
"""Staged-only issue health check for pre-commit hook.

Reads a list of file paths (relative to repo root) from stdin and validates
only those files.  Exits 0 unless a staged issue file has a violation that
is INTRODUCED by this commit (not pre-existing).

Checks for each staged issue file:
1. If in issues/done/: has ## Completion evidence section (non-template)
2. YAML id: matches filename prefix
3. Backticked paths (when the file itself references them — skip path
   existence checks for issues/ refs since those may be cross-stage)

Exit code: 0 if all staged files are clean, 1 otherwise.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

YAML_ID_RE = re.compile(r"^id:\s*\"?([0-9]+[a-z]?)\"?\s*$", re.M)
COMPLETION_EVIDENCE_RE = re.compile(
    r"## Completion evidence(.*?)(?:^## |\Z)", re.DOTALL | re.M
)


def id_from_name(name: str) -> str:
    m = re.match(r"^(\d+[a-z]?)-", name)
    return m.group(1) if m else ""


def is_template_placeholder(evidence: str) -> bool:
    ev = evidence.strip().lower()
    if not ev:
        return True
    if "fill only when moving" in ev:
        return True
    return False


def check_file(rel_path: str) -> list[str]:
    errors: list[str] = []
    abspath = REPO_ROOT / rel_path

    if not abspath.exists():
        return errors  # file was deleted — no check needed

    text = abspath.read_text(encoding="utf-8")
    name = abspath.name
    name_id = id_from_name(name)

    # Extract title for migration detection
    title_m = re.search(r"^title:\s*\"(.+?)\"\s*$", text, re.M)
    title = title_m.group(1).lower() if title_m else ""

    # 1. YAML id matches filename
    m = YAML_ID_RE.search(text)
    body_id = m.group(1).strip() if m else ""
    if name_id and body_id and name_id != body_id:
        errors.append(f"{rel_path}: id mismatch: filename {name_id} vs body {body_id}")

    # 2. Done issues need completion evidence
    if "/done/" in rel_path:
        cm = COMPLETION_EVIDENCE_RE.search(text)
        if not cm:
            errors.append(f"{rel_path}: in done/ but has no ## Completion evidence section")
        elif is_template_placeholder(cm.group(1)):
            errors.append(f"{rel_path}: in done/ but completion evidence is template/empty")

    # 3. Check backticked paths that DON'T start with issues/
    # (issues/ refs span open/done and may legitimately cross-reference)
    if "migrate" not in title:  # migration issues reference old paths
        path_re = re.compile(r"`((?:crates|docs|fixtures|scripts|reference|reports)/[^` ]+)")
        for p in path_re.findall(text):
            p = p.strip().rstrip("),")
            if "..." in p or "|" in p or "*" in p or "YYYY" in p or "xxxx" in p:
                continue
            if not (REPO_ROOT / p).exists():
                errors.append(f"{rel_path}: missing path: {p}")

    return errors


def main() -> int:
    staged = [line.strip() for line in sys.stdin if line.strip()]

    all_errors: list[str] = []
    for rel_path in sorted(staged):
        if not rel_path.startswith("issues/"):
            continue
        errors = check_file(rel_path)
        all_errors.extend(errors)

    if all_errors:
        for msg in all_errors:
            print(msg, file=sys.stderr)
        print("check_issue_health: failed (staged-file check)", file=sys.stderr)
        return 1

    print("check_issue_health: OK (staged files clean)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
