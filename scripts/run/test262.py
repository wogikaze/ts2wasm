#!/usr/bin/env python3
"""Stream G: Test262 Runner with differential comparison

Usage:
  python scripts/manager.py test262 [--sample N] [--category PATTERN] [--jobs N] > test262-results.jsonl

Compiles each test262 file, runs with iwasm, and compares output against Node.js reference.
Outputs one TestRecord per line in JSON Lines format.
"""

import sys
import subprocess
import json
import re
import tempfile
import os
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage: python scripts/manager.py test262 [--sample N] [--category PATTERN] [--jobs N]")
    print()
    print("Options:")
    print("  --sample N          Run up to N files per extracted category.")
    print("  --category PATTERN  Regex matched against extracted category.")
    print("  --jobs N            Number of parallel workers (default: TEST262_JOBS or os.cpu_count or 4).")
    print("  -h, --help          Show this help.")

def escape_json(s):
    """Escape string for JSON."""
    if s is None:
        return ""
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')

def extract_category(path):
    """Extract category from test262 file path."""
    match = re.search(r'test/language/([^/]+)/', str(path))
    return match.group(1) if match else "unknown"

def create_test_record(suite, case_path, target, status, expected=None, actual=None, reason=None, tracking=None):
    """Create a TestRecord JSON object."""
    record = {
        "suite": suite,
        "case": case_path,
        "target": target,
        "status": status
    }
    
    if expected is not None:
        record["expected"] = escape_json(expected)
    if actual is not None:
        record["actual"] = escape_json(actual)
    if reason:
        record["reason"] = escape_json(reason)
    if tracking:
        record["tracking"] = tracking
    
    return json.dumps(record)

def feature_label(diag_code, stderr, test_file):
    """Generate feature label from diagnostic code."""
    # Simplified version - in the full script this would use feature-labels.sh
    feature_map = {
        "UnsupportedSyntax": "feature-unsupported",
        "UnresolvedName": "feature-resolution",
        "UnresolvedFunction": "feature-resolution",
        "TypeError": "type-system",
        "RuntimeError": "runtime",
        "InvariantViolation": "compiler-invariant",
        "BackendIo": "io-backend",
        "CompilationError": "compilation",
    }
    return feature_map.get(diag_code, diag_code.lower())

def compile_and_run_test(test_file, tmp_dir):
    """Compile and run a single test file."""
    tmp_wasm = tmp_dir / f"test-{os.getpid()}-{id(test_file)}.wasm"
    tmp_stdout = tmp_dir / f"stdout-{os.getpid()}-{id(test_file)}.txt"
    tmp_stderr = tmp_dir / f"stderr-{os.getpid()}-{id(test_file)}.txt"
    
    result_status = ""
    result_diag = ""
    result_feature = ""
    result_reason = ""
    result_actual = ""
    
    # Compile with ts2wasm
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", str(test_file), "-o", str(tmp_wasm)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode != 0:
        result_status = "unsupported"
        stderr_content = result.stderr
        diag_match = re.search(r'(UnsupportedSyntax|UnresolvedName|UnresolvedFunction|TypeError|RuntimeError|InvariantViolation|BackendIo|CompilationError)', stderr_content)
        result_diag = diag_match.group(1) if diag_match else "CompilationError"
        
        reason_match = re.search(re.escape(f"[{result_diag}]"), stderr_content)
        result_reason = reason_match.group(0) if reason_match else stderr_content.split('\n')[0] if stderr_content else ""
        result_feature = feature_label(result_diag, stderr_content, str(test_file))
        return result_status, result_diag, result_feature, result_reason, result_actual
    
    # Run with iwasm
    result = subprocess.run(
        ["timeout", "5s", "iwasm", str(tmp_wasm)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode == 0:
        result_status = "pass"
        with open(tmp_stdout, 'w') as f:
            f.write(result.stdout)
        result_actual = result.stdout
    else:
        result_status = "fail"
        result_diag = f"RuntimeError:{result.returncode}"
        result_reason = result.stderr[:200] if result.stderr else ""
        result_actual = result.stdout if result.stdout else ""
    
    return result_status, result_diag, result_feature, result_reason, result_actual

def get_node_reference(test_file, tmp_dir):
    """Get Node.js reference output."""
    tmp_out = tmp_dir / f"node-{os.getpid()}-{id(test_file)}.txt"
    
    result = subprocess.run(
        ["timeout", "5s", "node", str(test_file)],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    with open(tmp_out, 'w') as f:
        f.write(result.stdout + result.stderr)
    
    return result.stdout + result.stderr, result.returncode == 0

def process_one_test(test_file, tmp_dir):
    """Process a single test file and return JSON record and status."""
    print(f"Processing: {test_file}", file=sys.stderr)
    
    result_status, result_diag, result_feature, result_reason, result_actual = compile_and_run_test(test_file, tmp_dir)
    
    if result_status == "pass":
        expected, node_ok = get_node_reference(test_file, tmp_dir)
        
        if node_ok and result_actual == expected:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "pass", expected, result_actual)
            return record, "pass"
        elif node_ok:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "fail", expected, result_actual, "output mismatch")
            return record, "fail"
        else:
            record = create_test_record("test262", str(test_file), "wasm-iwasm", "blocked", expected, result_actual, "node execution failed")
            return record, "blocked"
    
    elif result_status == "unsupported":
        tracking_key = f"feature:{result_feature}"
        reason = f"{result_diag}/{result_feature}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "unsupported", None, None, reason, tracking_key)
        return record, "unsupported"
    
    elif result_status == "fail":
        reason = f"{result_diag}: {result_reason}"
        record = create_test_record("test262", str(test_file), "wasm-iwasm", "fail", None, result_actual, reason)
        return record, "fail"
    
    return "", "fail"

def main():
    args = sys.argv[1:]
    
    sample = None
    category_pattern = "."
    jobs = int(os.environ.get("TEST262_JOBS", "")) if os.environ.get("TEST262_JOBS") else None
    
    i = 0
    while i < len(args):
        if args[i] == "--sample":
            if i + 1 >= len(args):
                print("ERROR: --sample requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                sample = int(args[i + 1])
            except ValueError:
                print("ERROR: --sample must be a non-negative integer", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] == "--category":
            if i + 1 >= len(args):
                print("ERROR: --category requires a value", file=sys.stderr)
                sys.exit(1)
            category_pattern = args[i + 1]
            i += 2
        elif args[i] == "--jobs":
            if i + 1 >= len(args):
                print("ERROR: --jobs requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                jobs = int(args[i + 1])
            except ValueError:
                print("ERROR: --jobs must be a positive integer", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] in ("-h", "--help"):
            usage()
            sys.exit(0)
        else:
            print(f"ERROR: unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    if jobs is None:
        jobs = os.cpu_count() or 4
    
    if jobs < 1:
        print("ERROR: --jobs must be a positive integer", file=sys.stderr)
        sys.exit(1)
    
    if sample == 0:
        print("Starting test262 runner...", file=sys.stderr)
        print(f"Category filter: {category_pattern}", file=sys.stderr)
        print(f"Parallel jobs: {jobs}", file=sys.stderr)
        print("Sample mode: first 0 files per category", file=sys.stderr)
        print("Selected files: 0", file=sys.stderr)
        print("", file=sys.stderr)
        print("=== Test262 Summary ===", file=sys.stderr)
        print("Pass: 0", file=sys.stderr)
        print("Fail: 0", file=sys.stderr)
        print("Unsupported: 0", file=sys.stderr)
        print("Blocked: 0", file=sys.stderr)
        print("Total: 0", file=sys.stderr)
        sys.exit(0)
    
    print("Starting test262 runner...", file=sys.stderr)
    print(f"Category filter: {category_pattern}", file=sys.stderr)
    print(f"Parallel jobs: {jobs}", file=sys.stderr)
    if sample:
        print(f"Sample mode: first {sample} files per category", file=sys.stderr)
    
    # Find test files
    test_files = sorted(REPO_ROOT.glob("reference/test262/test/language/**/*.js"))
    
    # Filter by category
    category_seen = {}
    selected_files = []
    
    for test_file in test_files:
        category = extract_category(test_file)
        
        try:
            if not re.search(category_pattern, category):
                continue
        except re.error:
            print(f"ERROR: Invalid category pattern: {category_pattern}", file=sys.stderr)
            sys.exit(1)
        
        if sample:
            seen = category_seen.get(category, 0)
            if seen >= sample:
                continue
            category_seen[category] = seen + 1
        
        selected_files.append(test_file)
    
    print(f"Selected files: {len(selected_files)}", file=sys.stderr)
    
    passed = 0
    failed = 0
    unsupported = 0
    blocked = 0
    
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_dir = Path(tmp_dir)
        
        with ThreadPoolExecutor(max_workers=jobs) as executor:
            futures = {executor.submit(process_one_test, f, tmp_dir): f for f in selected_files}
            
            for future in as_completed(futures):
                record, status = future.result()
                
                if record:
                    print(record)
                
                if status == "pass":
                    passed += 1
                elif status == "fail":
                    failed += 1
                elif status == "unsupported":
                    unsupported += 1
                elif status == "blocked":
                    blocked += 1
    
    print("", file=sys.stderr)
    print("=== Test262 Summary ===", file=sys.stderr)
    print(f"Pass: {passed}", file=sys.stderr)
    print(f"Fail: {failed}", file=sys.stderr)
    print(f"Unsupported: {unsupported}", file=sys.stderr)
    print(f"Blocked: {blocked}", file=sys.stderr)
    print(f"Total: {passed + failed + unsupported + blocked}", file=sys.stderr)
    
    # Save test results for site generation
    results = {
        "suite": "test262",
        "passed": passed,
        "failed": failed,
        "unsupported": unsupported,
        "blocked": blocked,
        "total": passed + failed + unsupported + blocked,
        "timestamp": datetime.now().isoformat()
    }
    
    results_dir = REPO_ROOT / "artifacts" / "coverage" / "results"
    results_dir.mkdir(parents=True, exist_ok=True)
    results_file = results_dir / "test262.json"
    results_file.write_text(json.dumps(results, indent=2), encoding="utf-8")
    print(f"Results saved to {results_file}", file=sys.stderr)
    
    # Auto-generate site after test completion
    print("Generating documentation site...", file=sys.stderr)
    gen_site_script = REPO_ROOT / "scripts" / "gen-site.py"
    if gen_site_script.exists():
        subprocess.run([sys.executable, str(gen_site_script)], cwd=REPO_ROOT)
        print("Site generation complete. Run 'mise run build-site' to build the site.", file=sys.stderr)

if __name__ == "__main__":
    main()
