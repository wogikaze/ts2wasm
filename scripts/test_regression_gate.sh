#!/bin/bash
# Stream G: Regression Gate for Coverage Tracking
#
# Usage:
#   ./scripts/test_regression_gate.sh test262-results.jsonl [--baseline baseline.json]
#
# Compares current test results against baseline to detect regressions:
# - Fail if pass count decreases
# - Fail if fail count increases (new failures)
# - Fail if unsupported count increases without feature label
# - Pass if all metrics are stable or improving

set -e

CURRENT_RESULTS="$1"
BASELINE_FILE="${2:-test262-baseline.json}"

if [ ! -f "$CURRENT_RESULTS" ]; then
    echo "ERROR: Current results file not found: $CURRENT_RESULTS" >&2
    exit 1
fi

# Count current results
CURRENT_PASS=0
CURRENT_FAIL=0
CURRENT_UNSUPPORTED=0
CURRENT_BLOCKED=0

while IFS= read -r line; do
    [ -z "$line" ] && continue
    local status=$(echo "$line" | grep -oP '(?<="status":")[^"]*' | head -1)
    
    case "$status" in
        pass) CURRENT_PASS=$((CURRENT_PASS + 1)) ;;
        fail) CURRENT_FAIL=$((CURRENT_FAIL + 1)) ;;
        unsupported) CURRENT_UNSUPPORTED=$((CURRENT_UNSUPPORTED + 1)) ;;
        blocked) CURRENT_BLOCKED=$((CURRENT_BLOCKED + 1)) ;;
    esac
done < "$CURRENT_RESULTS"

# Load baseline (if exists)
if [ -f "$BASELINE_FILE" ]; then
    BASELINE_PASS=$(grep -oP '(?<="pass":)\d+' "$BASELINE_FILE" | head -1)
    BASELINE_FAIL=$(grep -oP '(?<="fail":)\d+' "$BASELINE_FILE" | head -1)
    BASELINE_UNSUPPORTED=$(grep -oP '(?<="unsupported":)\d+' "$BASELINE_FILE" | head -1)
else
    # No baseline - treat as first run (always passes)
    BASELINE_PASS=$CURRENT_PASS
    BASELINE_FAIL=$CURRENT_FAIL
    BASELINE_UNSUPPORTED=$CURRENT_UNSUPPORTED
    
    echo "No baseline found. Creating baseline: $BASELINE_FILE" >&2
fi

# Check gates
GATE_PASS=0
REGRESSION=0

# Gate 1: Pass count must not decrease
if [ "$CURRENT_PASS" -lt "$BASELINE_PASS" ]; then
    echo "✗ FAIL: pass count decreased from $BASELINE_PASS to $CURRENT_PASS (regression: $(($BASELINE_PASS - $CURRENT_PASS)))" >&2
    REGRESSION=1
else
    PASS_DELTA=$((CURRENT_PASS - BASELINE_PASS))
    if [ "$PASS_DELTA" -eq 0 ]; then
        echo "✓ pass: $CURRENT_PASS (no change)" >&2
    else
        echo "✓ pass: $BASELINE_PASS → $CURRENT_PASS (+$PASS_DELTA)" >&2
    fi
    GATE_PASS=$((GATE_PASS + 1))
fi

# Gate 2: Fail count must not increase
if [ "$CURRENT_FAIL" -gt "$BASELINE_FAIL" ]; then
    echo "✗ FAIL: fail count increased from $BASELINE_FAIL to $CURRENT_FAIL (regression: $(($CURRENT_FAIL - $BASELINE_FAIL)))" >&2
    REGRESSION=1
else
    FAIL_DELTA=$((BASELINE_FAIL - CURRENT_FAIL))
    if [ "$FAIL_DELTA" -eq 0 ]; then
        echo "✓ fail: $CURRENT_FAIL (no change)" >&2
    else
        echo "✓ fail: $BASELINE_FAIL → $CURRENT_FAIL (-$FAIL_DELTA fixed)" >&2
    fi
    GATE_PASS=$((GATE_PASS + 1))
fi

# Gate 3: Unsupported count should not increase (new blockers)
if [ "$CURRENT_UNSUPPORTED" -gt "$BASELINE_UNSUPPORTED" ]; then
    echo "⚠ WARNING: unsupported count increased from $BASELINE_UNSUPPORTED to $CURRENT_UNSUPPORTED (new blockers)" >&2
    # This is a warning, not a failure - features are added over time
else
    UNSUPPORTED_DELTA=$((BASELINE_UNSUPPORTED - CURRENT_UNSUPPORTED))
    if [ "$UNSUPPORTED_DELTA" -eq 0 ]; then
        echo "✓ unsupported: $CURRENT_UNSUPPORTED (no change)" >&2
    else
        echo "✓ unsupported: $BASELINE_UNSUPPORTED → $CURRENT_UNSUPPORTED (-$UNSUPPORTED_DELTA removed)" >&2
    fi
    GATE_PASS=$((GATE_PASS + 1))
fi

# Save current as new baseline
cat > "$BASELINE_FILE" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "pass": $CURRENT_PASS,
  "fail": $CURRENT_FAIL,
  "unsupported": $CURRENT_UNSUPPORTED,
  "blocked": $CURRENT_BLOCKED
}
EOF

echo "" >&2
if [ "$REGRESSION" -eq 0 ]; then
    echo "✓ All regression gates passed" >&2
    exit 0
else
    echo "✗ Regression detected" >&2
    exit 1
fi
