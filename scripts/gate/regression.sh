#!/usr/bin/env bash
# Stream G: Regression Gate for Coverage Tracking
#
# Usage:
#   scripts/gate/regression.sh test262-results.jsonl [--baseline baseline.json]
#
# Compares current test results against baseline to detect regressions:
# - Fail if pass count decreases
# - Fail if fail count increases (new failures)
# - Warn if unsupported count increases

set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "${_ts2wasm_entry_dir}/../lib/common.sh"
cd "$TS2WASM_REPO_ROOT"

usage() {
    cat <<'EOF'
Usage: scripts/gate/regression.sh <results.jsonl> [--baseline FILE]
EOF
}

if [[ $# -lt 1 ]]; then
    usage >&2
    exit 1
fi

CURRENT_RESULTS="$1"
shift
BASELINE_FILE="test262-baseline.json"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --baseline)
            [[ $# -ge 2 ]] || { echo "ERROR: --baseline requires a value" >&2; exit 1; }
            BASELINE_FILE="$2"
            shift 2
            ;;
        *)
            echo "ERROR: Unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [[ ! -f "$CURRENT_RESULTS" ]]; then
    echo "ERROR: Current results file not found: $CURRENT_RESULTS" >&2
    exit 1
fi

CURRENT_PASS=0
CURRENT_FAIL=0
CURRENT_UNSUPPORTED=0
CURRENT_BLOCKED=0

while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    status=$(echo "$line" | grep -oP '(?<="status":")[^"]*' | head -1 || true)

    case "$status" in
        pass) CURRENT_PASS=$((CURRENT_PASS + 1)) ;;
        fail) CURRENT_FAIL=$((CURRENT_FAIL + 1)) ;;
        unsupported) CURRENT_UNSUPPORTED=$((CURRENT_UNSUPPORTED + 1)) ;;
        blocked) CURRENT_BLOCKED=$((CURRENT_BLOCKED + 1)) ;;
    esac
done < "$CURRENT_RESULTS"

if [[ -f "$BASELINE_FILE" ]]; then
    BASELINE_PASS=$(grep -oP '(?<="pass":)\d+' "$BASELINE_FILE" | head -1 || echo 0)
    BASELINE_FAIL=$(grep -oP '(?<="fail":)\d+' "$BASELINE_FILE" | head -1 || echo 0)
    BASELINE_UNSUPPORTED=$(grep -oP '(?<="unsupported":)\d+' "$BASELINE_FILE" | head -1 || echo 0)
else
    BASELINE_PASS=$CURRENT_PASS
    BASELINE_FAIL=$CURRENT_FAIL
    BASELINE_UNSUPPORTED=$CURRENT_UNSUPPORTED
    echo "No baseline found. Creating baseline: $BASELINE_FILE" >&2
fi

REGRESSION=0

if [[ "$CURRENT_PASS" -lt "$BASELINE_PASS" ]]; then
    echo "FAIL: pass count decreased from $BASELINE_PASS to $CURRENT_PASS" >&2
    REGRESSION=1
else
    delta=$((CURRENT_PASS - BASELINE_PASS))
    if [[ "$delta" -eq 0 ]]; then
        echo "OK: pass=$CURRENT_PASS (no change)" >&2
    else
        echo "OK: pass $BASELINE_PASS -> $CURRENT_PASS (+$delta)" >&2
    fi
fi

if [[ "$CURRENT_FAIL" -gt "$BASELINE_FAIL" ]]; then
    echo "FAIL: fail count increased from $BASELINE_FAIL to $CURRENT_FAIL" >&2
    REGRESSION=1
else
    delta=$((BASELINE_FAIL - CURRENT_FAIL))
    if [[ "$delta" -eq 0 ]]; then
        echo "OK: fail=$CURRENT_FAIL (no change)" >&2
    else
        echo "OK: fail $BASELINE_FAIL -> $CURRENT_FAIL (-$delta fixed)" >&2
    fi
fi

if [[ "$CURRENT_UNSUPPORTED" -gt "$BASELINE_UNSUPPORTED" ]]; then
    echo "WARN: unsupported increased from $BASELINE_UNSUPPORTED to $CURRENT_UNSUPPORTED" >&2
else
    delta=$((BASELINE_UNSUPPORTED - CURRENT_UNSUPPORTED))
    if [[ "$delta" -eq 0 ]]; then
        echo "OK: unsupported=$CURRENT_UNSUPPORTED (no change)" >&2
    else
        echo "OK: unsupported $BASELINE_UNSUPPORTED -> $CURRENT_UNSUPPORTED (-$delta)" >&2
    fi
fi

cat > "$BASELINE_FILE" <<EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "pass": $CURRENT_PASS,
  "fail": $CURRENT_FAIL,
  "unsupported": $CURRENT_UNSUPPORTED,
  "blocked": $CURRENT_BLOCKED
}
EOF

if [[ "$REGRESSION" -eq 0 ]]; then
    echo "All regression gates passed" >&2
    exit 0
fi

echo "Regression detected" >&2
exit 1
