#!/usr/bin/env python3
"""Trace contract validator.

Checks that docs/trace-contract.md contains required trace kinds, samples,
and field schemas. Coverage PRs must reference one of the defined trace kinds.

Usage:
  python scripts/check/trace-contract.py
  python scripts/check/trace-contract.py --self-test
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

REQUIRED_TRACE_KINDS = [
    "SemanticIRTrace",
    "SpecOpTrace",
    "RuntimeCoreTrace",
    "DeoptTrace",
]

REQUIRED_SAMPLES = [
    "ordinary property get",
    "ordinary property set",
    "accessor getter",
    "Proxy get",
    "function call",
    "constructor call",
    "throw / catch",
    "try / finally",
    "deopt",
]

REQUIRED_FIELDS = [
    "trace_kind", "event", "span", "op", "inputs", "outputs",
    "frame", "realm", "object_kind", "shape", "result_status",
]


def check_trace_doc() -> list[str]:
    path = REPO_ROOT / "docs" / "trace-contract.md"
    if not path.exists():
        return ["ERROR: docs/trace-contract.md not found"]

    text = path.read_text()
    violations = []

    for kind in REQUIRED_TRACE_KINDS:
        if kind not in text:
            violations.append(f"ERROR: trace kind '{kind}' not documented")

    for sample in REQUIRED_SAMPLES:
        if sample not in text:
            violations.append(f"ERROR: required sample '{sample}' missing from trace docs")

    for field in REQUIRED_FIELDS:
        if field not in text:
            violations.append(f"ERROR: required trace field '{field}' not documented")

    return violations


def run_self_test():
    errors = 0
    text = (REPO_ROOT / "docs" / "trace-contract.md").read_text()

    for kind in REQUIRED_TRACE_KINDS:
        if kind not in text:
            print(f"FAIL: missing trace kind {kind}", file=sys.stderr)
            errors += 1

    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print(f"self-test: OK ({len(REQUIRED_TRACE_KINDS)} kinds, {len(REQUIRED_SAMPLES)} samples)", file=sys.stderr)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--self-test" in args:
        run_self_test()
        return

    violations = check_trace_doc()
    for v in violations:
        print(f"trace_contract: {v}", file=sys.stderr)

    if violations:
        print(f"trace_contract: FAILED ({len(violations)} errors)", file=sys.stderr)
        sys.exit(1)
    print("trace_contract: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
