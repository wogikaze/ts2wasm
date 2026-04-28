#!/usr/bin/env python3
"""Stream G: Regression Gate for Coverage Tracking

Usage:
  python scripts/manager.py test-regression-gate <results.jsonl> [--baseline FILE]

Compares current test results against baseline to detect regressions:
- Fail if pass count decreases
- Fail if fail count increases (new failures)
- Fail if unsupported count increases
"""

import sys
import json
import re
from pathlib import Path
from datetime import datetime

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage: python scripts/manager.py test-regression-gate <results.jsonl> [--baseline FILE]")

def main():
    if len(sys.argv) < 2:
        usage()
        sys.exit(1)
    
    current_results = sys.argv[1]
    args = sys.argv[2:]
    
    baseline_file = "test262-baseline.json"
    i = 0
    while i < len(args):
        if args[i] == "--baseline":
            if i + 1 >= len(args):
                print("ERROR: --baseline requires a value", file=sys.stderr)
                sys.exit(1)
            baseline_file = args[i + 1]
            i += 2
        else:
            print(f"ERROR: Unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    current_path = REPO_ROOT / current_results
    if not current_path.exists():
        print(f"ERROR: Current results file not found: {current_results}", file=sys.stderr)
        sys.exit(1)
    
    current_pass = 0
    current_fail = 0
    current_unsupported = 0
    current_blocked = 0
    
    with open(current_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            
            # Extract status from JSON
            match = re.search(r'"status":\s*"([^"]+)"', line)
            if match:
                status = match.group(1)
                if status == "pass":
                    current_pass += 1
                elif status == "fail":
                    current_fail += 1
                elif status == "unsupported":
                    current_unsupported += 1
                elif status == "blocked":
                    current_blocked += 1
    
    baseline_path = REPO_ROOT / baseline_file
    
    if baseline_path.exists():
        with open(baseline_path) as f:
            baseline = json.load(f)
        baseline_pass = baseline.get("pass", 0)
        baseline_fail = baseline.get("fail", 0)
        baseline_unsupported = baseline.get("unsupported", 0)
    else:
        baseline_pass = current_pass
        baseline_fail = current_fail
        baseline_unsupported = current_unsupported
        print(f"No baseline found. Creating baseline: {baseline_file}", file=sys.stderr)
    
    regression = 0
    
    if current_pass < baseline_pass:
        print(f"FAIL: pass count decreased from {baseline_pass} to {current_pass}", file=sys.stderr)
        regression = 1
    else:
        delta = current_pass - baseline_pass
        if delta == 0:
            print(f"OK: pass={current_pass} (no change)", file=sys.stderr)
        else:
            print(f"OK: pass {baseline_pass} -> {current_pass} (+{delta})", file=sys.stderr)
    
    if current_fail > baseline_fail:
        print(f"FAIL: fail count increased from {baseline_fail} to {current_fail}", file=sys.stderr)
        regression = 1
    else:
        delta = baseline_fail - current_fail
        if delta == 0:
            print(f"OK: fail={current_fail} (no change)", file=sys.stderr)
        else:
            print(f"OK: fail {baseline_fail} -> {current_fail} (-{delta} fixed)", file=sys.stderr)
    
    if current_unsupported > baseline_unsupported:
        print(f"ERROR: unsupported increased from {baseline_unsupported} to {current_unsupported}", file=sys.stderr)
        regression = 1
    else:
        delta = baseline_unsupported - current_unsupported
        if delta == 0:
            print(f"OK: unsupported={current_unsupported} (no change)", file=sys.stderr)
        else:
            print(f"OK: unsupported {baseline_unsupported} -> {current_unsupported} (-{delta})", file=sys.stderr)
    
    # Update baseline
    baseline_data = {
        "timestamp": datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
        "pass": current_pass,
        "fail": current_fail,
        "unsupported": current_unsupported,
        "blocked": current_blocked
    }
    
    with open(baseline_path, 'w') as f:
        json.dump(baseline_data, f, indent=2)
    
    if regression == 0:
        print("All regression gates passed", file=sys.stderr)
        sys.exit(0)
    
    print("Regression detected", file=sys.stderr)
    sys.exit(1)

if __name__ == "__main__":
    main()
