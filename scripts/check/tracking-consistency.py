#!/usr/bin/env python3
"""Validate issue-NNN namespace consistency across source/diagnostic refs.

The old TRACKING.yaml is replaced by issues/<id>.md files per issue.
This script checks that issue-NNN references in source code and diagnostic
messages are not stale or conflicting.

Use mise run issue-lint for per-issue structural validation.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ISSUES_DIR = ROOT / "issues"

# Reserved issue-number ranges that should NOT appear in diagnostic/comment
# references — these are active tracker IDs for current roadmap items.
RESERVED_ISSUE_RANGES: list[tuple[int, int, str]] = [
    (369, 389, "P15 architecture decoupling / test hardening issues"),
]

# Allowlist for issue-NNN references in source code that do NOT conflict with
# reserved ranges. Each entry: (file_glob_suffix, issue_number, reason).
ISSUE_REF_ALLOWLIST: list[tuple[str, int, str]] = [
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


def fail(msg: str) -> None:
    print(f"tracking: invalid: {msg}", file=sys.stderr)
    sys.exit(1)


def warn(msg: str) -> None:
    print(f"tracking: warning: {msg}", file=sys.stderr)


# ---------------------------------------------------------------------------
# Issue-number namespace conflict checks
# ---------------------------------------------------------------------------

NAMESPACE_SCAN_DIRS = ["crates", "docs", "scripts", "fixtures"]

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


def main() -> None:
    if not ISSUES_DIR.exists():
        fail("issues/ dir does not exist — run mise run issue-lint instead")

    # Collect issue IDs from issues/ directory
    issue_files = sorted(ISSUES_DIR.glob("I-*.md"))
    issue_ids: set[str] = set()
    issue_count = len(issue_files)

    for f in issue_files:
        content = f.read_text(encoding="utf-8")
        header_text = content.split("\n---\n", 1)[0]
        header = {}
        for line in header_text.strip().split("\n"):
            if ":" in line:
                k, _, v = line.partition(":")
                header[k.strip()] = v.strip()
        iid = header.get("Id", "")
        if iid:
            if iid in issue_ids:
                fail(f"duplicate issue Id: {iid} (in {f.name})")
            issue_ids.add(iid)

    # Issue-number namespace check
    ns_violations = check_issue_namespace()
    if ns_violations > 0:
        fail(f"{ns_violations} issue-number namespace conflict(s) found (see above)")

    print(f"tracking: valid ({issue_count} issues, namespace OK)")


if __name__ == "__main__":
    main()
