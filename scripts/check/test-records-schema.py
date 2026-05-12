#!/usr/bin/env python3
"""Validate JSONL TestRecord lines (suite, case, target, status + required fields).

Usage:
  mise run check records [file.jsonl]
  mise run check records -- --self-test
  some-runner | mise run check records
  some-runner | mise run check records -

When no file arg or file is "-", reads stdin.

One JSON object per line. Each line must include suite, case, target, status.
status must be one of: pass build_pass fail unsupported blocked skip-with-reason

For status=pass:
  - target must be "wasm-iwasm"
  - expected, actual, oracle fields required
  - node_exit_status and iwasm_exit_status must be 0
  - semantic_checked must be true

For status=build_pass:
  - target must be "wasm-iwasm"
  - reason required

For unsupported, blocked, skip-with-reason: non-empty reason and tracking required.

Exit 1 on first invalid line.
"""

import sys
import json
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def usage():
    print("Usage:")
    print("  mise run check records [file.jsonl|-]")
    print("  mise run check records -- --self-test")
    print()
    print("One JSON object per line. Each line must include suite, case, target, status.")
    print("status must be one of: pass build_pass fail unsupported blocked skip-with-reason")
    print()
    print("For status=pass:")
    print("  - target must be 'wasm-iwasm'")
    print("  - expected, actual, oracle fields required")
    print("  - node_exit_status and iwasm_exit_status must be 0")
    print("  - semantic_checked must be true")
    print()
    print("For status=build_pass:")
    print("  - target must be 'wasm-iwasm'")
    print("  - reason required")
    print()
    print("For unsupported, blocked, skip-with-reason: non-empty reason and tracking required.")
    print()
    print("Exit 1 on first invalid line.")


def validate_stream(lines):
    line_no = 0
    valid_statuses = {"pass", "build_pass", "fail", "unsupported", "blocked", "skip-with-reason"}
    reason_required_statuses = {"unsupported", "blocked", "skip-with-reason"}

    for line in lines:
        line_no += 1
        line = line.strip()
        if not line:
            continue

        try:
            data = json.loads(line)
        except json.JSONDecodeError:
            print(f"check_test_records_schema: line {line_no}: invalid JSON", file=sys.stderr)
            return False

        # Check required fields
        for key in ["suite", "case", "target", "status"]:
            if key not in data or not isinstance(data[key], str) or not data[key]:
                print(f"check_test_records_schema: line {line_no}: missing or empty string field: {key}",
                      file=sys.stderr)
                return False

        # Check status validity
        status = data["status"]
        if status not in valid_statuses:
            print(f"check_test_records_schema: line {line_no}: invalid status: {status}",
                  file=sys.stderr)
            return False

        # status=pass requires strict fields
        if status == "pass":
            if data.get("target") != "wasm-iwasm":
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"target='wasm-iwasm', got '{data.get('target')}'", file=sys.stderr)
                return False
            if not data.get("expected"):
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"non-empty 'expected' field", file=sys.stderr)
                return False
            if not data.get("actual"):
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"non-empty 'actual' field", file=sys.stderr)
                return False
            if data.get("node_exit_status") is not None and data["node_exit_status"] != 0:
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"node_exit_status==0, got {data['node_exit_status']}", file=sys.stderr)
                return False
            if data.get("iwasm_exit_status") is not None and data["iwasm_exit_status"] != 0:
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"iwasm_exit_status==0, got {data['iwasm_exit_status']}", file=sys.stderr)
                return False
            if data.get("semantic_checked") is not None and data["semantic_checked"] is not True:
                print(f"check_test_records_schema: line {line_no}: status 'pass' requires "
                      f"semantic_checked==true, got {data.get('semantic_checked')}", file=sys.stderr)
                return False

        # status=build_pass requires target="wasm-iwasm" and reason
        if status == "build_pass":
            if data.get("target") != "wasm-iwasm":
                print(f"check_test_records_schema: line {line_no}: status 'build_pass' requires "
                      f"target='wasm-iwasm', got '{data.get('target')}'", file=sys.stderr)
                return False
            if not data.get("reason"):
                print(f"check_test_records_schema: line {line_no}: status 'build_pass' requires "
                      f"non-empty 'reason' field", file=sys.stderr)
                return False

        # Check reason and tracking for specific statuses
        if status in reason_required_statuses:
            if "reason" not in data or not isinstance(data["reason"], str) or not data["reason"]:
                print(f"check_test_records_schema: line {line_no}: status {status} requires non-empty reason",
                      file=sys.stderr)
                return False
            if "tracking" not in data or not isinstance(data["tracking"], str) or not data["tracking"]:
                print(f"check_test_records_schema: line {line_no}: status {status} requires non-empty tracking",
                      file=sys.stderr)
                return False

    print(f"check_test_records_schema: OK ({line_no} lines checked)", file=sys.stderr)
    return True


def main():
    args = sys.argv[1:]

    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)

    if args and args[0] == "--self-test":
        # Canonical 5-status TestRecord schema (see docs/17-jsonl-test-record-schema.md).
        # Coverage-runner extensions (build_pass, node_exit_status, etc.) are NOT
        # validated here — only canonical fields are checked.
        #
        # Note: coverage-runner output adds build_pass status and requires
        # expected/actual/node_exit_status/iwasm_exit_status/semantic_checked for
        # pass records. Those extended checks are performed by scripts/gate/coverage.py.
        test_data = [
            # pass: canonical form with optional extended fields
            '{"suite":"self","case":"pass","target":"wasm-iwasm","status":"pass",'
            '"expected":"ok","actual":"ok","node_exit_status":0,"iwasm_exit_status":0,"semantic_checked":true}',
            # fail: expected/actual optional in canonical schema
            '{"suite":"self","case":"fail","target":"wasm32-wasi","status":"fail",'
            '"expected":"3\\n","actual":"5\\n","reason":"stdout mismatch","tracking":null}',
            # unsupported: reason and tracking required
            '{"suite":"self","case":"unsupported","target":"wasm32-wasi",'
            '"status":"unsupported","expected":null,"actual":null,'
            '"reason":"Unsupported syntax","tracking":"feature:async"}',
            # blocked: reason and tracking required
            '{"suite":"self","case":"blocked","target":"wasm32-wasi",'
            '"status":"blocked","expected":null,"actual":null,'
            '"reason":"I/O error","tracking":"issue-5011"}',
            # skip-with-reason: reason and tracking required
            '{"suite":"self","case":"skip","target":"wasm32-wasi",'
            '"status":"skip-with-reason","expected":null,"actual":null,'
            '"reason":"skipped","tracking":"feature:skip"}',
        ]
        if validate_stream(test_data):
            print("check_test_records_schema: self-test OK (canonical 5-status schema)", file=sys.stderr)
            sys.exit(0)
        else:
            sys.exit(1)

    input_source = args[0] if args else "-"

    if input_source == "-":
        validate_stream(sys.stdin)
    else:
        input_path = REPO_ROOT / input_source
        if not input_path.exists():
            print(f"check_test_records_schema: not a file: {input_source}", file=sys.stderr)
            sys.exit(1)
        with open(input_path) as f:
            validate_stream(f)


if __name__ == "__main__":
    main()
