#!/usr/bin/env python3
"""Coverage classification schema checker.

Validates that coverage PRs include proper failure classification records.
Also validates the classification JSON/CSV schema.

Usage:
  python scripts/check/coverage-classification.py <classification.json>
  python scripts/check/coverage-classification.py --schema
"""

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

VALID_KINDS = {
    "ParseGap", "ResolveGap", "SemanticIRGap", "SpecOpGap",
    "RuntimeCoreGap", "CorrectnessBackendGap", "LegacyBackendLeak",
    "OptimizationGap", "HarnessGap", "OracleGap", "Unclassified",
}

VALID_STATUSES = {
    "unclassified", "classified", "implemented", "verified", "optimized",
}

REQUIRED_FIELDS = {
    "suite", "case", "failure_kind", "owning_layer",
    "first_missing_capability", "required_change",
    "expected_trace", "actual_trace", "status",
}

VALID_OWNING_LAYERS = {
    "frontend", "resolve", "semantic-ir", "spec-kernel",
    "runtime-core", "backend-correctness", "opt-mir",
    "compiler", "cli", "harness", "legacy",
}


def validate_record(record: dict, index: int) -> list[str]:
    errors = []
    for field in REQUIRED_FIELDS:
        if field not in record:
            errors.append(f"record[{index}]: missing field '{field}'")

    kind = record.get("failure_kind", "")
    if kind not in VALID_KINDS:
        errors.append(f"record[{index}]: invalid failure_kind '{kind}'")

    status = record.get("status", "")
    if status not in VALID_STATUSES:
        errors.append(f"record[{index}]: invalid status '{status}'")

    layer = record.get("owning_layer", "")
    if layer not in VALID_OWNING_LAYERS:
        errors.append(f"record[{index}]: invalid owning_layer '{layer}'")

    if kind == "Unclassified" and status != "unclassified":
        errors.append(f"record[{index}]: Unclassified must have status=unclassified")

    if kind == "LegacyBackendLeak":
        errors.append(f"record[{index}]: LegacyBackendLeak is a DESIGN VIOLATION, not a fix target")

    return errors


def validate_file(path: str) -> list[str]:
    text = Path(path).read_text()
    records = json.loads(text)
    if not isinstance(records, list):
        records = [records]

    errors = []
    for i, record in enumerate(records):
        errors.extend(validate_record(record, i))
    return errors


def print_schema():
    print("Failure Classification Schema")
    print("============================")
    print()
    print("Required fields:")
    for f in REQUIRED_FIELDS:
        print(f"  {f}")
    print()
    print("Valid failure_kinds:")
    for k in sorted(VALID_KINDS):
        print(f"  {k}")
    print()
    print("Valid statuses:")
    for s in sorted(VALID_STATUSES):
        print(f"  {s}")
    print()
    print("Valid owning_layers:")
    for l in sorted(VALID_OWNING_LAYERS):
        print(f"  {l}")
    print()
    print("Example:")
    print(json.dumps({
        "suite": "test262",
        "case": "built-ins/Object/getOwnPropertyDescriptor/basic.js",
        "failure_kind": "SpecOpGap",
        "owning_layer": "spec-kernel",
        "first_missing_capability": "OrdinaryGetOwnProperty",
        "required_change": "Implement SpecOp::GetOwnProperty with OrdinaryGetOwnProperty dispatch",
        "expected_trace": "ToPropertyKey -> GetOwnProperty -> OrdinaryGetOwnProperty",
        "actual_trace": "ToPropertyKey -> GetOwnProperty -> stub(returns undefined)",
        "status": "classified",
    }, indent=2))


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--schema" in args:
        print_schema()
        return

    if not args:
        print("Usage: python scripts/check/coverage-classification.py <file.json>", file=sys.stderr)
        sys.exit(1)

    errors = validate_file(args[0])
    for e in errors:
        print(f"coverage_classification: {e}", file=sys.stderr)

    if errors:
        print(f"coverage_classification: FAILED ({len(errors)} errors)", file=sys.stderr)
        sys.exit(1)

    print("coverage_classification: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
