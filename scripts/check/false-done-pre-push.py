#!/usr/bin/env python3
"""pre-push: reject push when done/ issues cannot prove correct execution.

Scan issues/done/ and flag every issue whose "slice" (test case) claims to
execute correctly but lacks semantic (Node-matching) proof in completion
evidence.

Rules (by YAML `type:` field):

  feature, bug    → MUST have Node-comparison evidence in completion
                    (semantic_pass, node_diff, iwasm, m2_node, etc.)
                    Build-only evidence (build_smoke or nextest) is INSUFFICIENT.

  report          → "emit diagnostic X" tasks.  Done with diagnostic-test
                    evidence; Node runtime comparison NOT required.

  spike, design,
  meta, maintenance,
  test, docs,
  infra, refactor,
  cleanup         → Exempt: do not claim executable correctness.
                    Still must NOT have empty/pending evidence.

  NO_TYPE (missing
  YAML front
  matter)         → Lenient: must have non-empty completion evidence.

Exit code: 0 if clean, 1 if any false-done issues found.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DONE_DIR = REPO_ROOT / "issues" / "done"

# ---------------------------------------------------------------------------
# Regex helpers
# ---------------------------------------------------------------------------
YAML_TYPE_RE = re.compile(r"^type:\s*(\S+)", re.M)

# Completion evidence keywords that prove SEMANTIC correctness
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

# Weak evidence that only proves build (NOT semantic correctness)
WEAK_EVIDENCE_KW = {
    "build_pass",
    "build_smoke",
    "nextest",
    "cargo test",
    "ts2wasm build",
    "cargo fmt",
}

# -- Type classification ---------------------------------------------------

# Types whose job is emitting runnable wasm → MUST have strong evidence
CLAIMS_EXECUTABLE = {"feature", "bug"}

# Types that do NOT claim to produce correct runtime output.
# report = "emit diagnostic X" — diagnostic evidence is sufficient.
# The others are architectural / meta / investigation / infrastructure.
EXEMPT_TYPES = {
    "report",
    "spike",
    "design",
    "meta",
    "maintenance",
    "test",
    "docs",
    "infra",
    "refactor",
    "cleanup",
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def yaml_field(text: str, field_pattern: re.Pattern) -> str:
    m = field_pattern.search(text)
    return m.group(1).strip().lower() if m else ""


def has_strong_evidence(evidence: str) -> bool:
    ev_lower = evidence.lower()
    return any(kw in ev_lower for kw in STRONG_EVIDENCE_KW)


def is_template_placeholder(evidence: str) -> bool:
    ev = evidence.strip().lower()
    if not ev:
        return True
    if "fill only when moving" in ev:
        return True
    if len(ev) < 40 and ("pending" in ev or "to be committed" in ev or "..." in ev):
        return True
    return False


# ---------------------------------------------------------------------------
# Per-issue check
# ---------------------------------------------------------------------------


def check_issue(path: Path) -> tuple[str, str] | None:
    """Return (filename_slug, reason) if the issue is false-done, else None."""
    text = path.read_text(encoding="utf-8")
    name_id = path.name.replace(".md", "")

    type_val = yaml_field(text, YAML_TYPE_RE)

    # -- Section existence ------------------------------------------------
    m = re.search(r"## Completion evidence(.*?)(?:^## |\Z)", text, re.DOTALL | re.M)
    has_section = m is not None
    evidence = m.group(1).strip() if m else ""

    # -- 1. No section at all → always a violation -----------------------
    if not has_section:
        return (name_id, "no `## Completion evidence` section")

    # -- 2. Empty / template evidence → always a violation ---------------
    if is_template_placeholder(evidence):
        return (name_id, "completion evidence is template/empty")

    # -- 3. For NO_TYPE (missing YAML), we're lenient --------------------
    if not type_val:
        return None  # non-empty evidence exists; good enough

    # -- 4. Exempt types → non-empty evidence is sufficient --------------
    if type_val in EXEMPT_TYPES:
        return None

    # -- 5. feature / bug → MUST have strong (Node) evidence ------------
    if type_val in CLAIMS_EXECUTABLE:
        if not has_strong_evidence(evidence):
            return (name_id, "build-only evidence (no Node/semantic comparison)")
        return None

    # -- 6. Unknown type → safe side: require strong evidence ------------
    if not has_strong_evidence(evidence):
        return (name_id, f"unknown type '{type_val}'; requires semantic evidence")

    return None


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> int:
    if not DONE_DIR.is_dir():
        print(f"ERROR: {DONE_DIR} not found", file=sys.stderr)
        return 1

    violations: list[tuple[str, str, str]] = []

    for issue_file in sorted(DONE_DIR.glob("*.md")):
        result = check_issue(issue_file)
        if result is not None:
            name_id, reason = result
            violations.append((name_id, issue_file.name, reason))

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
        "Fix: move each issue to issues/open/ or add semantic "
        "(Node-matching) completion evidence."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
