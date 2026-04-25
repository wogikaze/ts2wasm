#!/bin/bash
# Stream G: Test262 Runner with differential comparison
#
# Usage:
#   ./scripts/test262_runner.sh [--sample N] [--category PATTERN] > test262-results.jsonl
#
# Compiles each test262 file, runs with iwasm, and compares output against Node.js reference.
# Outputs one TestRecord per line in JSON Lines format.

set -e

SAMPLE=${1:-}
SAMPLE_COUNT=0
CATEGORY_PATTERN="${2:-.}"
RESULTS_FILE="${RESULTS_FILE:-test262-results.jsonl}"
TMP_DIR=$(mktemp -d)
PASSED=0
FAILED=0
UNSUPPORTED=0
BLOCKED=0

# Cleanup on exit
trap "rm -rf $TMP_DIR" EXIT

# Helper: extract category from path
# E.g., reference/test262/test/language/expressions/arithmetic -> expressions
extract_category() {
    local path="$1"
    echo "$path" | sed -E 's|.*/test/language/([^/]+)/.*|\1|'
}

# Helper: compile and run, capturing output and exit code
compile_and_run_test() {
    local test_file="$1"
    local tmp_wasm=$(mktemp "$TMP_DIR/test-XXXXXX.wasm")
    local tmp_stdout=$(mktemp "$TMP_DIR/stdout-XXXXXX.txt")
    local tmp_stderr=$(mktemp "$TMP_DIR/stderr-XXXXXX.txt")

    # Try to compile
    if ! cargo run -q -p ts2wasm-cli -- build "$test_file" -o "$tmp_wasm" \
        >"$tmp_stderr" 2>&1; then
        # Compilation failed - extract diagnostic
        local diag_code=$(grep -oP '(UnsupportedSyntax|UnresolvedName|TypeError|TypeError|RuntimeError)' "$tmp_stderr" | head -1)
        local reason=$(head -1 "$tmp_stderr")
        
        if [ -z "$diag_code" ]; then
            diag_code="CompilationError"
        fi
        
        echo "$test_file" "$diag_code" "$reason" "unsupported"
        return 1
    fi

    # Run with iwasm, capture output
    if iwasm "$tmp_wasm" >"$tmp_stdout" 2>"$tmp_stderr"; then
        local actual=$(cat "$tmp_stdout")
        echo "$test_file" "Pass" "" "pass" "$actual"
        return 0
    else
        local exit_code=$?
        local actual=$(cat "$tmp_stdout")
        local stderr=$(cat "$tmp_stderr" | head -c 100)
        echo "$test_file" "RuntimeError:$exit_code" "$stderr" "fail" "$actual"
        return 2
    fi
}

# Helper: get reference output from Node.js
get_node_reference() {
    local test_file="$1"
    local tmp_out=$(mktemp "$TMP_DIR/node-XXXXXX.txt")
    
    if timeout 5s node "$test_file" >"$tmp_out" 2>&1; then
        cat "$tmp_out"
    else
        echo "(node execution failed or timed out)"
    fi
}

# Helper: create TestRecord JSON
create_test_record() {
    local suite="$1"
    local case="$2"
    local target="$3"
    local status="$4"
    local expected="$5"
    local actual="$6"
    local reason="$7"
    local tracking="$8"

    # Escape JSON strings
    local escape_json() {
        sed 's/\\/\\\\/g; s/"/\\"/g; s/$/\\n' | tr -d '\n'
    }

    local json="{"
    json="$json\"suite\":\"$suite\","
    json="$json\"case\":\"$case\","
    json="$json\"target\":\"$target\","
    json="$json\"status\":\"$status\""
    
    if [ -n "$expected" ]; then
        local exp_esc=$(echo -n "$expected" | escape_json)
        json="$json,\"expected\":\"$exp_esc\""
    fi
    
    if [ -n "$actual" ]; then
        local act_esc=$(echo -n "$actual" | escape_json)
        json="$json,\"actual\":\"$act_esc\""
    fi
    
    if [ -n "$reason" ]; then
        local reas_esc=$(echo -n "$reason" | escape_json)
        json="$json,\"reason\":\"$reas_esc\""
    fi
    
    if [ -n "$tracking" ]; then
        json="$json,\"tracking\":\"$tracking\""
    fi
    
    json="$json}"
    echo "$json"
}

echo "Starting test262 runner..." >&2
echo "Category filter: $CATEGORY_PATTERN" >&2
if [ -n "$SAMPLE" ] && [ "$SAMPLE" != "-q" ]; then
    echo "Sample mode: first $SAMPLE files per category" >&2
fi

# Iterate through test262 files
for test_file in reference/test262/test/language/**/*.js; do
    [ -f "$test_file" ] || continue

    local category=$(extract_category "$test_file")
    
    # Filter by category pattern
    if ! echo "$category" | grep -q "$CATEGORY_PATTERN"; then
        continue
    fi

    # Sample limiting
    if [ -n "$SAMPLE" ] && [ "$SAMPLE" != "-q" ]; then
        SAMPLE_COUNT=$((SAMPLE_COUNT + 1))
        if [ "$SAMPLE_COUNT" -gt "$SAMPLE" ]; then
            break
        fi
    fi

    # Try to process test
    echo "Processing: $test_file" >&2

    local compile_result=$(compile_and_run_test "$test_file" 2>&1)
    local test_status=$(echo "$compile_result" | awk '{print $NF}')
    
    if [ "$test_status" = "pass" ]; then
        local actual=$(echo "$compile_result" | cut -d' ' -f4-)
        local expected=$(get_node_reference "$test_file")
        
        if [ "$actual" = "$expected" ]; then
            create_test_record "test262" "$test_file" "wasm-iwasm" "pass" "$expected" "$actual" "" ""
            PASSED=$((PASSED + 1))
        else
            create_test_record "test262" "$test_file" "wasm-iwasm" "fail" "$expected" "$actual" "output mismatch" ""
            FAILED=$((FAILED + 1))
        fi
    elif [ "$test_status" = "unsupported" ]; then
        local diag_code=$(echo "$compile_result" | awk '{print $2}')
        local reason=$(echo "$compile_result" | cut -d' ' -f3-)
        create_test_record "test262" "$test_file" "wasm-iwasm" "unsupported" "" "" "$diag_code: $reason" "feature:$(echo $diag_code | tr '[:upper:]' '[:lower:]')"
        UNSUPPORTED=$((UNSUPPORTED + 1))
    elif [ "$test_status" = "fail" ]; then
        local reason=$(echo "$compile_result" | awk '{print $2}')
        local stderr=$(echo "$compile_result" | cut -d' ' -f3-5)
        local actual=$(echo "$compile_result" | cut -d' ' -f6-)
        create_test_record "test262" "$test_file" "wasm-iwasm" "fail" "(node ref)" "$actual" "$reason" ""
        FAILED=$((FAILED + 1))
    fi
done

# Summary
echo "" >&2
echo "=== Test262 Summary ===" >&2
echo "Pass: $PASSED" >&2
echo "Fail: $FAILED" >&2
echo "Unsupported: $UNSUPPORTED" >&2
echo "Blocked: $BLOCKED" >&2
echo "Total: $((PASSED + FAILED + UNSUPPORTED + BLOCKED))" >&2
