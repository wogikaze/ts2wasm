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
    "evidence", "linked_issue",
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


def validate_file(path: str, strict: bool = False) -> list[str]:
    text = Path(path).read_text()
    records = json.loads(text)
    if not isinstance(records, list):
        records = [records]

    errors = []
    for i, record in enumerate(records):
        errors.extend(validate_record(record, i))
        if strict:
            kind = record.get("failure_kind", "")
            if kind == "Unclassified":
                errors.append(f"record[{i}]: Unclassified is STRICT REJECT — classify properly")
            status = record.get("status", "")
            if status == "unclassified":
                errors.append(f"record[{i}]: status=unclassified is STRICT REJECT")
            if not record.get("expected_trace", ""):
                errors.append(f"record[{i}]: missing expected_trace (STRICT)")
            if not record.get("actual_trace", ""):
                errors.append(f"record[{i}]: missing actual_trace (STRICT)")
            if kind == "OptimizationGap":
                errors.append(
                    f"record[{i}]: OptimizationGap is not a coverage blocker — "
                    f"use performance tracking instead"
                )
    return errors


def run_self_test():
    errors = 0

    # Test: Unclassified reject
    bad = [{"suite": "test262", "case": "x.js", "failure_kind": "Unclassified",
            "owning_layer": "??", "first_missing_capability": "??",
            "required_change": "??", "expected_trace": "??", "actual_trace": "??",
            "status": "unclassified"}]
    r = validate_file.__wrapped__ if hasattr(validate_file, '__wrapped__') else validate_file
    # Direct test
    import tempfile, json
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump(bad, f)
        fname = f.name
    errs = validate_file(fname, strict=True)
    import os
    os.unlink(fname)
    if not any("Unclassified" in e for e in errs):
        print("FAIL: Unclassified not rejected in strict mode", file=sys.stderr)
        errors += 1

    # Test: empty trace reject
    bad2 = [{"suite": "test262", "case": "y.js", "failure_kind": "SpecOpGap",
             "owning_layer": "spec-kernel", "first_missing_capability": "Get",
             "required_change": "impl Get", "expected_trace": "", "actual_trace": "",
             "status": "classified"}]
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump(bad2, f)
        fname = f.name
    errs2 = validate_file(fname, strict=True)
    os.unlink(fname)
    if not any("expected_trace" in e for e in errs2):
        print("FAIL: empty trace not rejected in strict mode", file=sys.stderr)
        errors += 1

    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print("self-test: OK", file=sys.stderr)


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
    strict = "--strict" in args
    if "--strict" in args:
        args.remove("--strict")
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--schema" in args:
        print_schema()
        return
    if "--self-test" in args:
        run_self_test()
        return

    if not args:
        print("Usage: python scripts/check/coverage-classification.py [--strict] <file.json>", file=sys.stderr)
        sys.exit(1)

    errors = validate_file(args[0], strict=strict)
    for e in errors:
        print(f"coverage_classification: {e}", file=sys.stderr)

    if errors:
        print(f"coverage_classification: FAILED ({len(errors)} errors)", file=sys.stderr)
        sys.exit(1)

    print("coverage_classification: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
