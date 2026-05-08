#!/usr/bin/env python3
"""pre-push: reject push when done/ issues cannot prove correct execution.

Scan issues/done/ and flag every issue whose "slice" (test case) claims to
execute correctly but lacks semantic (Node-matching) proof in completion
evidence.

An issue is "false-done" (cannot prove correct execution) if any of:
1. Has no `## Completion evidence` section.
2. The section body is a template placeholder ("Fill only when moving").
3. The issue YAML type is "feature" or "bug" AND the evidence has no
   reference to Node / iwasm / semantic_diff / semantic_pass / node_diff
   (i.e., it relies solely on build_smoke / build_pass).

Issues whose YAML type is "test" / "docs" / "infra" / "refactor" /
"cleanup" / "spike" are exempt — they do not claim executable correctness.

Exit code: 0 if clean, 1 if any false-done issues found.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DONE_DIR = REPO_ROOT / "issues" / "done"

# Front-matter field extractors
YAML_TYPE_RE = re.compile(r"^type:\s*(\S+)", re.M)
YAML_CLASS_RE = re.compile(r"^class:\s*(\S+)", re.M)

# Completion evidence keywords that prove semantic correctness
# (iwasm output compared against Node.js)
STRONG_EVIDENCE_KW = {
    "semantic_pass",
    "semantic_diff",
    "node_diff",
    "m2_node",
    "m3_semantic",
    "m5_array",  # m5 fixture suite has node-output comparison
    "node_output",
    "node_compar",
    "iwasm",  # implies the wasm was actually run
    "fixtures_match_node",
}

# Weak evidence that only proves build (not semantic correctness)
# Presence of these without STRONG_EVIDENCE_KW → false-done for feature/bug
WEAK_EVIDENCE_KW = {
    "build_pass",
    "build_smoke",
    "nextest",
    "cargo test",
    "ts2wasm build",
    "cargo fmt",
}

# Issue types that explicitly claim to produce runnable output
CLAIMS_EXECUTABLE = {"feature", "bug"}

# Issue types that do NOT claim executable correctness
EXEMPT_TYPES = {"test", "docs", "infra", "refactor", "cleanup", "spike", "spike?"}


def yaml_field(text: str, field_pattern: re.Pattern) -> str:
    m = field_pattern.search(text)
    return m.group(1).strip().lower() if m else ""


def has_strong_evidence(evidence: str) -> bool:
    ev_lower = evidence.lower()
    return any(kw in ev_lower for kw in STRONG_EVIDENCE_KW)


def has_weak_evidence(evidence: str) -> bool:
    ev_lower = evidence.lower()
    return any(kw in ev_lower for kw in WEAK_EVIDENCE_KW)


def is_template_placeholder(evidence: str) -> bool:
    ev = evidence.strip().lower()
    if not ev:
        return True
    if "fill only when moving" in ev:
        return True
    if len(ev) < 40 and ("pending" in ev or "to be committed" in ev or "..." in ev):
        return True
    return False


def check_issue(path: Path) -> tuple[str, str] | None:
    """Return (issue_id, reason) if the issue is false-done, else None."""
    text = path.read_text(encoding="utf-8")

    # Determine issue type from YAML front matter
    type_val = yaml_field(text, YAML_TYPE_RE)

    # Exempt non-executable types
    if type_val in EXEMPT_TYPES:
        return None

    # Build display ID from filename
    name_id = path.name.replace(".md", "")

    # Check for completion evidence section
    m = re.search(r"## Completion evidence(.*?)(?:^## |\Z)", text, re.DOTALL | re.M)
    if not m:
        return (name_id, "no `## Completion evidence` section")

    evidence = m.group(1).strip()

    # Template / empty evidence
    if is_template_placeholder(evidence):
        return (name_id, "completion evidence is template/empty")

    # For feature/bug types, strong evidence is REQUIRED
    if type_val in CLAIMS_EXECUTABLE:
        if not has_strong_evidence(evidence):
            if has_weak_evidence(evidence):
                return (
                    name_id,
                    "build-only evidence (no Node/semantic comparison)",
                )
            else:
                return (
                    name_id,
                    "no verifiable execution evidence",
                )

    # For unknown types, at minimum check evidence is not empty
    return None


def main() -> int:
    if not DONE_DIR.is_dir():
        print(f"ERROR: {DONE_DIR} not found", file=sys.stderr)
        return 1

    violations: list[tuple[str, str, str]] = []  # (id, title_prefix, reason)

    for issue_file in sorted(DONE_DIR.glob("*.md")):
        result = check_issue(issue_file)
        if result is not None:
            name_id, reason = result
            violations.append((name_id, issue_file.name, reason))

    # Summary
    total = sum(1 for _ in DONE_DIR.glob("*.md"))

    if not violations:
        print(
            "pre-push (false-done): OK — "
            f"all {total} done/ issues have valid completion evidence"
        )
        return 0

    print(
        f"pre-push (false-done): FAIL — "
        f"{len(violations)}/{total} issues cannot prove correct execution\n"
    )

    # Group by reason for readability
    from collections import defaultdict

    by_reason: dict[str, list[str]] = defaultdict(list)
    for name_id, fname, reason in violations:
        by_reason[reason].append(fname)

    for reason, files in sorted(by_reason.items()):
        print(f"  [{reason}] ({len(files)} issues)")
        for f in files:
            print(f"    {f}")
        print()

    print(
        "Fix: move each issue back to issues/open/ or add semantic "
        "(Node-matching) completion evidence."
    )
    print("See docs/16-commit-and-push-policy.md §issue close の検証ルール.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
