#!/usr/bin/env python3
"""Validate TRACKING.yaml structural consistency and ID sequence integrity.

Also checks for issue-NNN namespace conflicts between source/diagnostic
references and active/reserved issue ranges.

Read-only: this script must never modify TRACKING.yaml.
"""
import glob as glob_module
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
TRACKING = ROOT / "TRACKING.yaml"
DEFAULT_CLOSED_SOURCE = ROOT / "docs" / "done-tracking.yaml"

# Known historical gaps in ID sequence (never existed, not lost).
# Add exceptions here with a comment explaining why.
KNOWN_ID_GAPS = {
    108,  # 108 was never assigned (historical gap before 109)
    243,
    244,
    251,
    252,
    253,
    286,
    296,
}

# Reserved issue-number ranges that should NOT appear in diagnostic/comment
# references — these are active tracker IDs for current roadmap items.
RESERVED_ISSUE_RANGES: list[tuple[int, int, str]] = [
    (369, 389, "P15 architecture decoupling / test hardening issues"),
]

# Allowlist for issue-NNN references in source code that do NOT conflict with
# reserved ranges. Each entry: (file_glob_suffix, issue_number, reason).
# These are historical references (e.g. BigInt triage splits) that predate
# the P15 issue range and cannot be easily replaced without breaking tests.
ISSUE_REF_ALLOWLIST: list[tuple[str, int, str]] = [
    # BigInt diagnostic references — these are BigInt implementation gap labels
    # that happen to share numbers with P15 issues. They are not P15 tracker refs.
    ("builtin_resolver_bigint_ops.rs", 369, "BigInt triage: dynamic arithmetic outside first-limb slice"),
    ("builtin_resolver_bigint_ops.rs", 370, "BigInt triage: negative exponent / div-by-zero RangeError"),
    ("builtin_resolver_bigint_ops.rs", 376, "BigInt triage: exponentiation beyond literal folding"),
    ("builtin_resolver_bigint_ops.rs", 378, "BigInt triage: shift operators and unsigned right shift"),
    ("builtin_resolver_bigint_ops.rs", 387, "BigInt triage: bitwise outside signed-i64 helper slice"),
    ("builtin_resolver_bigint.rs", 373, "BigInt triage: object ToPrimitive toString boundary"),
    ("builtin_resolver_bigint.rs", 374, "BigInt triage: object ToPrimitive for mixed comparison"),
    ("crates/diagnostic/src/lib.rs", 370, "BigInt triage: mixed Number/BigInt TypeError"),
    ("crates/diagnostic/src/lib.rs", 378, "BigInt triage: shift operators TypeError"),
    ("m2_node_diff_fixture_tests.rs", 370, "BigInt triage: mixed Number/BigInt test fixture"),
    ("m2_node_diff_fixture_tests.rs", 373, "BigInt triage: ToPrimitive toString boundary test"),
    ("m2_node_diff_fixture_tests.rs", 374, "BigInt triage: ToPrimitive mixed comparison test"),
    ("binary.rs", 370, "BigInt triage: mixed Number/BigInt arithmetic TypeError"),
    ("consts.rs", 375, "BigInt triage: BigInt/String comparison boundary"),
    ("for-loop.snapshot.json", 375, "BigInt triage: BigInt/String comparison boundary snapshot"),
    ("14-runtime-abi.md", 382, "runtime ABI doc mentions deferred BigInt work"),
    ("14-runtime-abi.md", 383, "runtime ABI doc mentions deferred BigInt work"),
    ("14-runtime-abi.md", 384, "runtime ABI doc mentions deferred BigInt work"),
    ("javascript-features.md", 373, "JS features doc mentions BigInt object ToPrimitive gap"),
    ("05-compatibility-and-semantics.md", 373, "compat doc mentions BigInt object ToPrimitive gap"),
    ("25-robust-test-design.md", 369, "P15 design doc: namespace reconciliation planning"),
    ("25-robust-test-design.md", 370, "P15 design doc: namespace reconciliation planning"),
]

SECTIONS = ["open", "active", "done"]
REQUIRED = {
    "open": ["id", "title", "priority", "type", "area", "status", "created", "updated", "acceptance"],
    "active": ["id", "title", "priority", "type", "area", "status", "created", "updated", "acceptance"],
    "done": ["id", "title", "priority", "type", "area", "status", "created", "updated", "closed", "acceptance", "evidence"],
}

VALID_PRIORITIES = {"P1", "P2", "P3", "P4"}
VALID_STATUSES = {"open", "active", "done", "blocked"}
VALID_TYPES = {
    "feature",
    "runtime-builtin",
    "diagnostic",
    "infra",
    "design",
    "docs",
    "bug",
    "test",
    "task",
    "refactor",
    "tooling",
}
VALID_AREAS = {
    "frontend",
    "ir",
    "runtime",
    "backend",
    "scripts",
    "docs",
    "shared",
    "cli",
    "coverage",
    "cross",
}

# Words that suggest an item is roadmap-scale (too large for a single session)
ROADMAP_WORDS = {"all", "complete", "full", "entire", "every", "comprehensive", "11 types", "13 traps"}


def fail(msg: str) -> None:
    print(f"tracking: invalid: {msg}", file=sys.stderr)
    sys.exit(1)


def warn(msg: str) -> None:
    print(f"tracking: warning: {msg}", file=sys.stderr)


def load_closed_ids(path: Path) -> set[int]:
    """Load historical done IDs used for sequence-integrity checks."""
    if not path.exists():
        return set()
    try:
        data = yaml.safe_load(path.read_text())
    except Exception as e:
        fail(f"{path.relative_to(ROOT)} YAML parse error: {e}")
    if data is None:
        return set()
    if not isinstance(data, list):
        fail(f"{path.relative_to(ROOT)} must be a list")

    ids: set[int] = set()
    for i, item in enumerate(data):
        if not isinstance(item, dict):
            fail(f"{path.relative_to(ROOT)}[{i}] must be a mapping")
        item_id = item.get("id")
        if item_id is None:
            continue
        if not isinstance(item_id, int):
            fail(f"{path.relative_to(ROOT)}[{i}] id must be an integer")
        if item_id in ids:
            fail(f"{path.relative_to(ROOT)} duplicate id: {item_id}")
        ids.add(item_id)
    return ids


# ---------------------------------------------------------------------------
# Issue-number namespace conflict checks
# ---------------------------------------------------------------------------

# Directories to scan for issue-NNN references.
NAMESPACE_SCAN_DIRS = ["crates", "docs", "scripts", "fixtures"]

# Skip patterns for namespace scan.
NAMESPACE_SKIP = [
    "target/",
    ".git/",
    "node_modules/",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
    ".venv",
]


def scan_issue_refs(path: Path) -> list[tuple[int, int, str]]:
    """Scan a single file for issue-NNN references.

    Returns list of (line_number, issue_number, snippet) tuples.
    """
    matches = []
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except (OSError, UnicodeDecodeError):
        return matches

    for i, line in enumerate(text.splitlines(), 1):
        for m in re.finditer(r"issue-(\d{3,})", line):
            num = int(m.group(1))
            matches.append((i, num, line.strip()[:120]))
    return matches


def check_issue_namespace() -> int:
    """Check for issue-NNN references that conflict with reserved ranges.

    Returns number of violations found.
    """
    violations = 0
    seen_refs: dict[int, list[tuple[Path, int, str]]] = {}

    for dirname in NAMESPACE_SCAN_DIRS:
        scan_dir = ROOT / dirname
        if not scan_dir.is_dir():
            continue
        for path in scan_dir.rglob("*"):
            if not path.is_file():
                continue
            rel = path.relative_to(ROOT).as_posix()
            if any(p in rel for p in NAMESPACE_SKIP):
                continue
            for line_no, num, snippet in scan_issue_refs(path):
                if num not in seen_refs:
                    seen_refs[num] = []
                seen_refs[num].append((path, line_no, snippet))

    # Check reserved ranges
    for lo, hi, label in RESERVED_ISSUE_RANGES:
        for num in range(lo, hi + 1):
            if num not in seen_refs:
                continue
            for path, line_no, snippet in seen_refs[num]:
                rel = path.relative_to(ROOT).as_posix()
                allowlisted = any(
                    rel.endswith(sfx) and num == n
                    for sfx, n, _ in ISSUE_REF_ALLOWLIST
                )
                if not allowlisted:
                    violations += 1
                    print(
                        f"namespace: {rel}:{line_no}: issue-{num} conflicts with {label}",
                        file=sys.stderr,
                    )
                    print(f"  {snippet}", file=sys.stderr)

    return violations


# ---------------------------------------------------------------------------
# Plan files existence checks
# ---------------------------------------------------------------------------


def _check_plan_files_item(item: dict, root: Path) -> None:
    """Check that plan.files entries for a single item reference real paths.

    Handles:
    - Exact file paths (must exist or parent must exist)
    - Glob patterns (must have at least one match or parent must exist)
    - Directory references (must exist or parent must exist)
    """
    item_id = item.get("id", "?")
    plan = item.get("plan")
    if not isinstance(plan, dict):
        return
    files = plan.get("files")
    if not isinstance(files, list):
        return

    for entry in files:
        if not isinstance(entry, str) or not entry.strip():
            continue
        entry = entry.strip()

        # Build absolute path from repo root
        full = root / entry

        # Check exact path existence
        if full.exists():
            continue

        # Check glob pattern (handles **, *, ? patterns)
        normalized = str(full)
        matches = glob_module.glob(normalized, recursive=True)
        if matches:
            continue

        # Check parent directory exists — catches typos while allowing
        # planned files that haven't been created yet
        parent = full.parent
        if parent.exists():
            continue

        fail(
            f"id {item_id}: plan.files entry '{entry}' references a path that "
            f"does not exist and whose parent directory also does not exist"
        )


def check_plan_files(data: dict, root: Path) -> None:
    """Validate plan.files paths for all sections."""
    for section in SECTIONS:
        items = data.get(section, []) or []
        for item in items:
            _check_plan_files_item(item, root)


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
    closed_source = meta.get("closed_source")
    closed_path = ROOT / closed_source if closed_source else DEFAULT_CLOSED_SOURCE
    closed_ids = load_closed_ids(closed_path)

    ids: set[int] = set()
    has_warnings = False
    depends_on_refs: list[tuple[int, int]] = []  # (referrer_id, referenced_id)

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
            if item_id in closed_ids:
                fail(f"id {item_id}: appears in both TRACKING.yaml and {closed_path.relative_to(ROOT)}")
            ids.add(item_id)

            # Collect depends_on references for cross-reference validation
            deps = item.get("depends_on")
            if deps is not None:
                if not isinstance(deps, list):
                    fail(f"id {item_id}: depends_on must be a list")
                for dep_id in deps:
                    if not isinstance(dep_id, int):
                        fail(f"id {item_id}: depends_on entry '{dep_id}' must be an integer issue ID")
                    depends_on_refs.append((item_id, dep_id))

            if item.get("status") != section:
                # Allow "blocked" status in the "open" section
                if not (item.get("status") == "blocked" and section == "open"):
                    fail(f"id {item_id}: status must be '{section}' (got '{item.get('status')}')")

            # Priority validation
            priority = item.get("priority")
            if priority not in VALID_PRIORITIES:
                fail(f"id {item_id}: invalid priority '{priority}'; must be P1/P2/P3/P4")

            # Status validation (explicit enum check)
            item_status = item.get("status")
            if item_status not in VALID_STATUSES:
                fail(f"id {item_id}: invalid status '{item_status}'; must be open/active/done/blocked")

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

    # depends_on cross-reference validation: every referenced ID must exist
    if depends_on_refs:
        all_known_ids = ids | closed_ids
        missing_deps = [(referrer, dep) for referrer, dep in depends_on_refs if dep not in all_known_ids]
        if missing_deps:
            msg = "; ".join(
                f"id {referrer} depends_on {dep} (not found)"
                for referrer, dep in missing_deps
            )
            fail(f"depends_on references non-existent IDs: {msg}")

    # plan.files existence check
    check_plan_files(data, ROOT)

    # ID sequence integrity check: no gaps, no deletions
    all_ids = ids | closed_ids
    if all_ids:
        sorted_ids = sorted(all_ids)
        expected = range(sorted_ids[0], sorted_ids[-1] + 1)
        all_ids = set(sorted_ids)
        missing = set(expected) - all_ids
        known_and_missing = missing & KNOWN_ID_GAPS
        unexpected = missing - KNOWN_ID_GAPS
        if unexpected:
            fail(
                f"ID sequence broken: missing IDs {sorted(unexpected)}. "
                "Items were likely deleted. Restore them or add to KNOWN_ID_GAPS "
                "if the gap is intentional."
            )
        if known_and_missing:
            warn(f"ID sequence: known gaps {sorted(known_and_missing)} (listed in KNOWN_ID_GAPS)")

    # Issue-number namespace check
    ns_violations = check_issue_namespace()
    if ns_violations > 0:
        fail(f"{ns_violations} issue-number namespace conflict(s) found (see above)")

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
