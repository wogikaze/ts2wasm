#!/usr/bin/env bash
# Stream G: Test262 Runner with differential comparison
#
# Usage:
#   scripts/run/test262.sh [--sample N] [--category PATTERN] [--jobs N] > test262-results.jsonl
#
# Compiles each test262 file, runs with iwasm, and compares output against Node.js reference.
# Outputs one TestRecord per line in JSON Lines format.

set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "${_ts2wasm_entry_dir}/../lib/common.sh"
cd "$TS2WASM_REPO_ROOT"

SAMPLE=""
CATEGORY_PATTERN="."
JOBS="${TEST262_JOBS:-}"
TMP_DIR=$(mktemp -d)

PASSED=0
FAILED=0
UNSUPPORTED=0
BLOCKED=0

trap 'rm -rf "$TMP_DIR"' EXIT

usage() {
    cat <<'EOF'
Usage: scripts/test262_runner.sh [--sample N] [--category PATTERN] [--jobs N]

Options:
  --sample N          Run up to N files per extracted category.
  --category PATTERN  Regex matched against extracted category.
  --jobs N            Number of parallel workers (default: TEST262_JOBS or nproc or 4).
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --sample)
            [[ $# -ge 2 ]] || { echo "ERROR: --sample requires a value" >&2; exit 1; }
            SAMPLE="$2"
            shift 2
            ;;
        --category)
            [[ $# -ge 2 ]] || { echo "ERROR: --category requires a value" >&2; exit 1; }
            CATEGORY_PATTERN="$2"
            shift 2
            ;;
        --jobs)
            [[ $# -ge 2 ]] || { echo "ERROR: --jobs requires a value" >&2; exit 1; }
            JOBS="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "ERROR: unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [[ -n "$SAMPLE" ]] && ! [[ "$SAMPLE" =~ ^[0-9]+$ ]]; then
    echo "ERROR: --sample must be a non-negative integer" >&2
    exit 1
fi

if [[ -z "$JOBS" ]]; then
    if command -v nproc >/dev/null 2>&1; then
        JOBS=$(nproc)
    else
        JOBS=4
    fi
fi

if ! [[ "$JOBS" =~ ^[0-9]+$ ]] || (( JOBS < 1 )); then
    echo "ERROR: --jobs must be a positive integer" >&2
    exit 1
fi

if [[ "$SAMPLE" == "0" ]]; then
    echo "Starting test262 runner..." >&2
    echo "Category filter: $CATEGORY_PATTERN" >&2
    echo "Parallel jobs: $JOBS" >&2
    echo "Sample mode: first 0 files per category" >&2
    echo "Selected files: 0" >&2
    echo "" >&2
    echo "=== Test262 Summary ===" >&2
    echo "Pass: 0" >&2
    echo "Fail: 0" >&2
    echo "Unsupported: 0" >&2
    echo "Blocked: 0" >&2
    echo "Total: 0" >&2
    exit 0
fi

escape_json() {
    sed 's/\\/\\\\/g; s/"/\\"/g; s/$/\\n/' | tr -d '\n'
}

extract_category() {
    local path="$1"
    echo "$path" | sed -E 's|.*/test/language/([^/]+)/.*|\1|'
}

create_test_record() {
    local suite="$1"
    local case_path="$2"
    local target="$3"
    local status="$4"
    local expected="$5"
    local actual="$6"
    local reason="$7"
    local tracking="$8"

    local json="{"
    json="$json\"suite\":\"$suite\","
    json="$json\"case\":\"$case_path\","
    json="$json\"target\":\"$target\","
    json="$json\"status\":\"$status\""

    if [[ -n "$expected" ]]; then
        local exp_esc
        exp_esc=$(echo -n "$expected" | escape_json)
        json="$json,\"expected\":\"$exp_esc\""
    fi

    if [[ -n "$actual" ]]; then
        local act_esc
        act_esc=$(echo -n "$actual" | escape_json)
        json="$json,\"actual\":\"$act_esc\""
    fi

    if [[ -n "$reason" ]]; then
        local reason_esc
        reason_esc=$(echo -n "$reason" | escape_json)
        json="$json,\"reason\":\"$reason_esc\""
    fi

    if [[ -n "$tracking" ]]; then
        json="$json,\"tracking\":\"$tracking\""
    fi

    json="$json}"
    echo "$json"
}

RESULT_STATUS=""
RESULT_DIAG=""
RESULT_REASON=""
RESULT_ACTUAL=""

compile_and_run_test() {
    local test_file="$1"
    local tmp_wasm
    local tmp_stdout
    local tmp_stderr

    tmp_wasm=$(mktemp "$TMP_DIR/test-XXXXXX.wasm")
    tmp_stdout=$(mktemp "$TMP_DIR/stdout-XXXXXX.txt")
    tmp_stderr=$(mktemp "$TMP_DIR/stderr-XXXXXX.txt")

    RESULT_STATUS=""
    RESULT_DIAG=""
    RESULT_REASON=""
    RESULT_ACTUAL=""

    if ! cargo run -q -p ts2wasm-cli -- build "$test_file" -o "$tmp_wasm" >"$tmp_stderr" 2>&1; then
        RESULT_STATUS="unsupported"
        RESULT_DIAG=$(grep -oE 'UnsupportedSyntax|UnresolvedName|UnresolvedFunction|TypeError|RuntimeError|InvariantViolation|BackendIo|CompilationError' "$tmp_stderr" | head -1 || true)
        RESULT_REASON=$(head -1 "$tmp_stderr" || true)
        if [[ -z "$RESULT_DIAG" ]]; then
            RESULT_DIAG="CompilationError"
        fi
        return 0
    fi

    if timeout 5s iwasm "$tmp_wasm" >"$tmp_stdout" 2>"$tmp_stderr"; then
        RESULT_STATUS="pass"
        RESULT_ACTUAL=$(cat "$tmp_stdout")
        return 0
    fi

    RESULT_STATUS="fail"
    RESULT_DIAG="RuntimeError:$?"
    RESULT_REASON=$(head -c 200 "$tmp_stderr" || true)
    RESULT_ACTUAL=$(cat "$tmp_stdout" || true)
}

get_node_reference() {
    local test_file="$1"
    local tmp_out

    tmp_out=$(mktemp "$TMP_DIR/node-XXXXXX.txt")
    if timeout 5s node "$test_file" >"$tmp_out" 2>&1; then
        cat "$tmp_out"
        return 0
    fi

    cat "$tmp_out"
    return 1
}

process_one_test() {
    local test_file="$1"
    local json_out="$2"
    local status_out="$3"

    echo "Processing: $test_file" >&2
    compile_and_run_test "$test_file"

    case "$RESULT_STATUS" in
        pass)
            local expected_file
            local expected
            expected_file=$(mktemp "$TMP_DIR/expected-XXXXXX.txt")
            expected=""
            if get_node_reference "$test_file" >"$expected_file"; then
                expected=$(cat "$expected_file")
                if [[ "$RESULT_ACTUAL" == "$expected" ]]; then
                    create_test_record "test262" "$test_file" "wasm-iwasm" "pass" "$expected" "$RESULT_ACTUAL" "" "" >"$json_out"
                    echo "pass" >"$status_out"
                else
                    create_test_record "test262" "$test_file" "wasm-iwasm" "fail" "$expected" "$RESULT_ACTUAL" "output mismatch" "" >"$json_out"
                    echo "fail" >"$status_out"
                fi
            else
                expected=$(cat "$expected_file")
                create_test_record "test262" "$test_file" "wasm-iwasm" "blocked" "$expected" "$RESULT_ACTUAL" "node execution failed" "" >"$json_out"
                echo "blocked" >"$status_out"
            fi
            ;;
        unsupported)
            local tracking_key
            tracking_key="feature:$(echo "$RESULT_DIAG" | tr '[:upper:]' '[:lower:]')"
            create_test_record "test262" "$test_file" "wasm-iwasm" "unsupported" "" "" "$RESULT_DIAG: $RESULT_REASON" "$tracking_key" >"$json_out"
            echo "unsupported" >"$status_out"
            ;;
        fail)
            create_test_record "test262" "$test_file" "wasm-iwasm" "fail" "" "$RESULT_ACTUAL" "$RESULT_DIAG: $RESULT_REASON" "" >"$json_out"
            echo "fail" >"$status_out"
            ;;
    esac
}

echo "Starting test262 runner..." >&2
echo "Category filter: $CATEGORY_PATTERN" >&2
echo "Parallel jobs: $JOBS" >&2
if [[ -n "$SAMPLE" ]]; then
    echo "Sample mode: first $SAMPLE files per category" >&2
fi

declare -A CATEGORY_SEEN
SELECTED_FILES=()

while IFS= read -r test_file; do
    [[ -f "$test_file" ]] || continue

    category=$(extract_category "$test_file")
    if [[ -z "$category" ]]; then
        category="unknown"
    fi

    if ! echo "$category" | grep -Eq "$CATEGORY_PATTERN"; then
        continue
    fi

    if [[ -n "$SAMPLE" ]]; then
        seen=${CATEGORY_SEEN["$category"]:-0}
        if (( seen >= SAMPLE )); then
            continue
        fi
        CATEGORY_SEEN["$category"]=$((seen + 1))
    fi

    SELECTED_FILES+=("$test_file")
done < <(find reference/test262/test/language -type f -name '*.js' | sort)

echo "Selected files: ${#SELECTED_FILES[@]}" >&2

batch_pids=()
idx=0

for test_file in "${SELECTED_FILES[@]}"; do
    json_out="$TMP_DIR/result-$idx.json"
    status_out="$TMP_DIR/status-$idx.txt"

    process_one_test "$test_file" "$json_out" "$status_out" &
    batch_pids+=("$!")

    if (( ${#batch_pids[@]} >= JOBS )); then
        for pid in "${batch_pids[@]}"; do
            wait "$pid"
        done
        batch_pids=()
    fi

    idx=$((idx + 1))
done

if (( ${#batch_pids[@]} > 0 )); then
    for pid in "${batch_pids[@]}"; do
        wait "$pid"
    done
fi

for ((i = 0; i < idx; i++)); do
    status_file="$TMP_DIR/status-$i.txt"
    result_file="$TMP_DIR/result-$i.json"

    if [[ -f "$result_file" ]]; then
        cat "$result_file"
    fi

    if [[ ! -f "$status_file" ]]; then
        FAILED=$((FAILED + 1))
        continue
    fi

    status=$(cat "$status_file")
    case "$status" in
        pass)
            PASSED=$((PASSED + 1))
            ;;
        fail)
            FAILED=$((FAILED + 1))
            ;;
        unsupported)
            UNSUPPORTED=$((UNSUPPORTED + 1))
            ;;
        blocked)
            BLOCKED=$((BLOCKED + 1))
            ;;
    esac
done

echo "" >&2
echo "=== Test262 Summary ===" >&2
echo "Pass: $PASSED" >&2
echo "Fail: $FAILED" >&2
echo "Unsupported: $UNSUPPORTED" >&2
echo "Blocked: $BLOCKED" >&2
echo "Total: $((PASSED + FAILED + UNSUPPORTED + BLOCKED))" >&2
