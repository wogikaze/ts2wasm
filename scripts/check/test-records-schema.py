#!/usr/bin/env python3
"""Validate JSONL TestRecord lines (suite, case, target, status + required reason/tracking).

Usage:
  mise run check records [file.jsonl]
  mise run check records -- --self-test
  some-runner | mise run check records
  some-runner | mise run check records -

When no file arg or file is "-", reads stdin.

One JSON object per line. Each line must include suite, case, target, status.
status must be one of: pass fail unsupported blocked skip-with-reason
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
    print("status must be one of: pass fail unsupported blocked skip-with-reason")
    print("For unsupported, blocked, skip-with-reason: non-empty reason and tracking required.")
    print()
    print("Exit 1 on first invalid line.")

def validate_stream(lines):
    line_no = 0
    valid_statuses = {"pass", "fail", "unsupported", "blocked", "skip-with-reason"}
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
                print(f"check_test_records_schema: line {line_no}: missing or empty string field: {key}", file=sys.stderr)
                return False
        
        # Check status validity
        status = data["status"]
        if status not in valid_statuses:
            print(f"check_test_records_schema: line {line_no}: invalid status: {status}", file=sys.stderr)
            return False
        
        # Check reason and tracking for specific statuses
        if status in reason_required_statuses:
            if "reason" not in data or not isinstance(data["reason"], str) or not data["reason"]:
                print(f"check_test_records_schema: line {line_no}: status {status} requires non-empty reason", file=sys.stderr)
                return False
            if "tracking" not in data or not isinstance(data["tracking"], str) or not data["tracking"]:
                print(f"check_test_records_schema: line {line_no}: status {status} requires non-empty tracking", file=sys.stderr)
                return False
    
    print(f"check_test_records_schema: OK ({line_no} lines checked)", file=sys.stderr)
    return True

def main():
    args = sys.argv[1:]
    
    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)
    
    if args and args[0] == "--self-test":
        test_data = [
            '{"suite":"self","case":"pass","target":"wasm","status":"pass"}',
            '{"suite":"self","case":"unsup","target":"node","status":"unsupported","reason":"r","tracking":"t"}'
        ]
        if validate_stream(test_data):
            print("check_test_records_schema: self-test OK", file=sys.stderr)
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
