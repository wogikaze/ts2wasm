#!/bin/bash
# Stream G: Performance Baseline Tracker (optional)
#
# Usage:
#   ./scripts/benchmark_tracker.sh [--output benchmark-results.json]
#
# Tracks per-build metrics:
# - Wasm file size (bytes)
# - Compilation time (seconds)
# - Execution time on first 10 fixtures (milliseconds)

set -e

OUTPUT_FILE="${1:-benchmark-results.json}"
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

echo "Starting benchmark tracking..." >&2

# Initialize results if not exists
if [ ! -f "$OUTPUT_FILE" ]; then
    echo "[]" > "$OUTPUT_FILE"
fi

# Collect metrics
declare -A METRICS

# 1. Wasm file size (build fixtures/arrays-objects/object.ts)
echo "Measuring wasm file size..." >&2
local wasm_file=$(mktemp "$TMP_DIR/test-XXXXXX.wasm")

START_TIME=$(date +%s%N)
cargo run -q -p ts2wasm-cli -- build fixtures/arrays-objects/object.ts -o "$wasm_file" 2>&1
END_TIME=$(date +%s%N)

METRICS["compile_time_ms"]=$(( (END_TIME - START_TIME) / 1000000 ))
METRICS["wasm_size_bytes"]=$(wc -c < "$wasm_file")

echo "  Wasm size: ${METRICS[wasm_size_bytes]} bytes" >&2
echo "  Compile time: ${METRICS[compile_time_ms]} ms" >&2

# 2. Execution time (measure first fixture)
echo "Measuring execution time..." >&2
EXEC_TIMES=()
for fixture in fixtures/arrays-objects/array.ts fixtures/arrays-objects/object.ts fixtures/arrays-objects/string-length.ts; do
    [ -f "$fixture" ] || continue
    
    local test_wasm=$(mktemp "$TMP_DIR/exec-XXXXXX.wasm")
    cargo run -q -p ts2wasm-cli -- build "$fixture" -o "$test_wasm" 2>&1
    
    START_TIME=$(date +%s%N)
    iwasm "$test_wasm" >/dev/null 2>&1 || true
    END_TIME=$(date +%s%N)
    
    EXEC_TIMES+=( $(( (END_TIME - START_TIME) / 1000000 )) )
done

if [ ${#EXEC_TIMES[@]} -gt 0 ]; then
    local avg_time=0
    for t in "${EXEC_TIMES[@]}"; do
        avg_time=$((avg_time + t))
    done
    avg_time=$((avg_time / ${#EXEC_TIMES[@]}))
    METRICS["avg_exec_time_ms"]=$avg_time
    echo "  Average execution time: $avg_time ms" >&2
fi

# 3. Git commit info (for tracking)
METRICS["commit"]=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
METRICS["timestamp"]=$(date -u +%Y-%m-%dT%H:%M:%SZ)
METRICS["branch"]=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

# Append to results file
python3 << EOF
import json
import sys
from datetime import datetime

try:
    with open('$OUTPUT_FILE', 'r') as f:
        results = json.load(f)
except:
    results = []

record = {
    'timestamp': '${METRICS[timestamp]}',
    'commit': '${METRICS[commit]}',
    'branch': '${METRICS[branch]}',
    'wasm_size_bytes': ${METRICS[wasm_size_bytes]},
    'compile_time_ms': ${METRICS[compile_time_ms]},
    'avg_exec_time_ms': ${METRICS[avg_exec_time_ms]:-0},
}

results.append(record)

with open('$OUTPUT_FILE', 'w') as f:
    json.dump(results, f, indent=2)

print(f"Saved benchmark: {json.dumps(record, indent=2)}")
EOF

echo "Benchmarks saved to: $OUTPUT_FILE" >&2
