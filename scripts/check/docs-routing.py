#!/usr/bin/env python3
"""Docs routing validator.

Checks that AGENTS.md, CLAUDE.md, README.md contain required routing
rules and no forbidden old routing patterns.

Usage:
  python scripts/check/docs-routing.py
  python scripts/check/docs-routing.py --self-test
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

REQUIRED_ROUTING = [
    "semantic-ir",
    "spec-kernel",
    "runtime-core",
    "backend-correctness",
    "opt-mir",
]

FORBIDDEN_PATTERNS = [
    # Patterns that INSTRUCT adding to legacy paths
    # "しない" negates, "禁止" prohibits — these are safe
    # "SpecOp に追加" is about new crates, not legacy — safe
    r"coverage.*RuntimeFn.*追加\b(?!.*(しない|禁止))",
    r"\bRuntimeFn\b.{0,20}(variant|spec|catalog).{0,20}追加\b(?!.*しない)",
    r"native_lowered.*helper.*追加\b(?!.*(しない|禁止))",
    r"typed\.rs.*builtin.*追加\b(?!.*(しない|禁止))",
    r"backend-wasm.*semantics.*(置く|書く|実装)\b(?!.*(ない|禁止))",
]

CHECK_PATHS = [
    "AGENTS.md",
    "README.md",
]


def check_routing() -> list[str]:
    violations = []

    # Check CLAUDE.md exists
    claude = REPO_ROOT / "CLAUDE.md"
    if not claude.exists():
        violations.append("ERROR: CLAUDE.md not found")

    for rel_path in CHECK_PATHS:
        path = REPO_ROOT / rel_path
        if not path.exists():
            violations.append(f"ERROR: {rel_path} not found")
            continue
        text = path.read_text()
        for route in REQUIRED_ROUTING:
            if route not in text:
                violations.append(f"WARN: {rel_path} missing routing for '{route}'")

    # Check for forbidden patterns across all routing docs
    import re
    for rel_path in CHECK_PATHS + ["CLAUDE.md", "AGENTS.md"]:
        path = REPO_ROOT / rel_path
        if not path.exists():
            continue
        text = path.read_text()
        for pattern in FORBIDDEN_PATTERNS:
            if re.search(pattern, text):
                violations.append(
                    f"ERROR: {rel_path} contains forbidden pattern '{pattern}'"
                )

    return violations


def run_self_test():
    errors = 0
    for p in CHECK_PATHS + ["CLAUDE.md"]:
        path = REPO_ROOT / p
        if not path.exists():
            print(f"FAIL: {p} not found", file=sys.stderr)
            errors += 1
    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print("self-test: OK", file=sys.stderr)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--self-test" in args:
        run_self_test()
        return

    violations = check_routing()
    for v in violations:
        print(f"docs_routing: {v}", file=sys.stderr)

    errors = [v for v in violations if v.startswith("ERROR")]
    if errors:
        print(f"docs_routing: FAILED ({len(errors)} errors)", file=sys.stderr)
        sys.exit(1)
    print(f"docs_routing: OK ({len(violations)} warnings)", file=sys.stderr)


if __name__ == "__main__":
    main()
