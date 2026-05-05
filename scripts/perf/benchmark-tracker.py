#!/usr/bin/env python3
"""Stream G: Performance Baseline Tracker (optional)

Usage:
  python scripts/manager.py benchmark-tracker [--output artifacts/benchmark-results.json]

Tracks per-build metrics:
- Wasm file size (bytes)
- Compilation time (seconds)
- Execution time on first 10 fixtures (milliseconds)
"""

import sys
import subprocess
import json
import tempfile
import time
from pathlib import Path
from datetime import datetime

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_OUTPUT_FILE = "artifacts/benchmark-results.json"

def main():
    args = sys.argv[1:]
    
    output_file = DEFAULT_OUTPUT_FILE
    i = 0
    while i < len(args):
        if args[i] == "--output":
            if i + 1 >= len(args):
                print("error: --output requires a path", file=sys.stderr)
                sys.exit(1)
            output_file = args[i + 1]
            i += 2
        elif not args[i].startswith("--"):
            # Backward compatible: first positional argument is the output path
            output_file = args[i]
            i += 1
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            sys.exit(1)
    
    output_path = REPO_ROOT / output_file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    print("Starting benchmark tracking...", file=sys.stderr)
    
    # Initialize results if not exists
    if not output_path.exists():
        output_path.write_text("[]")
    
    metrics = {}
    
    # 1. Wasm file size (build fixtures/arrays-objects/object.ts)
    print("Measuring wasm file size...", file=sys.stderr)
    with tempfile.NamedTemporaryFile(suffix=".wasm", delete=False) as wasm_file:
        wasm_path = Path(wasm_file.name)
    
    start_time = time.time_ns()
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", 
         "fixtures/arrays-objects/object.ts", "-o", str(wasm_path)],
        capture_output=True,
        cwd=REPO_ROOT
    )
    end_time = time.time_ns()
    
    metrics["compile_time_ms"] = (end_time - start_time) // 1_000_000
    metrics["wasm_size_bytes"] = wasm_path.stat().st_size
    
    print(f"  Wasm size: {metrics['wasm_size_bytes']} bytes", file=sys.stderr)
    print(f"  Compile time: {metrics['compile_time_ms']} ms", file=sys.stderr)
    
    wasm_path.unlink()
    
    # 2. Execution time (measure first fixture)
    print("Measuring execution time...", file=sys.stderr)
    exec_times = []
    fixtures = [
        "fixtures/arrays-objects/array.ts",
        "fixtures/arrays-objects/object.ts",
        "fixtures/arrays-objects/string-length.ts"
    ]
    
    for fixture in fixtures:
        fixture_path = REPO_ROOT / fixture
        if not fixture_path.exists():
            continue
        
        with tempfile.NamedTemporaryFile(suffix=".wasm", delete=False) as test_wasm:
            test_wasm_path = Path(test_wasm.name)
        
        subprocess.run(
            ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", 
             str(fixture_path), "-o", str(test_wasm_path)],
            capture_output=True,
            cwd=REPO_ROOT
        )
        
        start_time = time.time_ns()
        subprocess.run(["iwasm", str(test_wasm_path)], capture_output=True, cwd=REPO_ROOT)
        end_time = time.time_ns()
        
        exec_times.append((end_time - start_time) // 1_000_000)
        test_wasm_path.unlink()
    
    metrics["avg_exec_time_ms"] = 0
    if exec_times:
        avg_time = sum(exec_times) // len(exec_times)
        metrics["avg_exec_time_ms"] = avg_time
        print(f"  Average execution time: {avg_time} ms", file=sys.stderr)
    
    # 3. Git commit info (for tracking)
    try:
        metrics["commit"] = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, cwd=REPO_ROOT
        ).stdout.strip()
    except Exception:
        metrics["commit"] = "unknown"
    
    try:
        metrics["timestamp"] = datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
    except Exception:
        metrics["timestamp"] = datetime.utcnow().isoformat()
    
    try:
        metrics["branch"] = subprocess.run(
            ["git", "rev-parse", "--abbrev-ref", "HEAD"],
            capture_output=True, text=True, cwd=REPO_ROOT
        ).stdout.strip()
    except Exception:
        metrics["branch"] = "unknown"
    
    # Append to results file
    try:
        with open(output_path) as f:
            results = json.load(f)
    except Exception:
        results = []
    
    record = {
        'timestamp': metrics['timestamp'],
        'commit': metrics['commit'],
        'branch': metrics['branch'],
        'wasm_size_bytes': metrics['wasm_size_bytes'],
        'compile_time_ms': metrics['compile_time_ms'],
        'avg_exec_time_ms': metrics['avg_exec_time_ms'],
    }
    
    results.append(record)
    
    with open(output_path, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"Saved benchmark: {json.dumps(record, indent=2)}")
    print(f"Benchmarks saved to: {output_file}", file=sys.stderr)

if __name__ == "__main__":
    main()
