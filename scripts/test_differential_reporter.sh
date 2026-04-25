#!/bin/bash
# Stream G: Differential Test Reporter
#
# Usage:
#   ./scripts/test262_runner.sh | ./scripts/test_differential_reporter.sh [--html FILE] [--markdown FILE]
#
# Reads JSONL test records from stdin and generates:
# - HTML report with summary table and failure details
# - Markdown report with grouped results

set -e

OUTPUT_HTML="${1:---html /dev/stdout}"
OUTPUT_MD="${2:---markdown /dev/stdout}"

# Parse arguments
HTML_FILE=""
MD_FILE=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --html)
            HTML_FILE="$2"
            shift 2
            ;;
        --markdown)
            MD_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

# Defaults if not specified
if [ -z "$HTML_FILE" ]; then
    HTML_FILE="/tmp/test262-report.html"
fi
if [ -z "$MD_FILE" ]; then
    MD_FILE="/tmp/test262-report.md"
fi

# Temporary work files
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

PASS_COUNT=0
FAIL_COUNT=0
UNSUPPORTED_COUNT=0
BLOCKED_COUNT=0

PASS_DETAILS="$TMP_DIR/pass.txt"
FAIL_DETAILS="$TMP_DIR/fail.txt"
UNSUPPORTED_DETAILS="$TMP_DIR/unsupported.txt"
BLOCKED_DETAILS="$TMP_DIR/blocked.txt"

touch "$PASS_DETAILS" "$FAIL_DETAILS" "$UNSUPPORTED_DETAILS" "$BLOCKED_DETAILS"

# Counters by category
declare -A CATEGORY_PASS
declare -A CATEGORY_FAIL
declare -A CATEGORY_UNSUPPORTED

# Process JSONL records
while IFS= read -r line; do
    [ -z "$line" ] && continue
    
    # Extract fields using simple grep/sed (no jq dependency)
    local case=$(echo "$line" | grep -oP '(?<="case":")[^"]*' | head -1)
    local status=$(echo "$line" | grep -oP '(?<="status":")[^"]*' | head -1)
    local expected=$(echo "$line" | grep -oP '(?<="expected":")[^"]*' | head -1)
    local actual=$(echo "$line" | grep -oP '(?<="actual":")[^"]*' | head -1)
    local reason=$(echo "$line" | grep -oP '(?<="reason":")[^"]*' | head -1)
    
    # Extract category from case path
    local category=$(echo "$case" | sed -E 's|.*/test/language/([^/]+)/.*|\1|')
    [ -z "$category" ] && category="unknown"
    
    case "$status" in
        pass)
            PASS_COUNT=$((PASS_COUNT + 1))
            CATEGORY_PASS["$category"]=$((${CATEGORY_PASS["$category"]:-0} + 1))
            echo "$case" >> "$PASS_DETAILS"
            ;;
        fail)
            FAIL_COUNT=$((FAIL_COUNT + 1))
            CATEGORY_FAIL["$category"]=$((${CATEGORY_FAIL["$category"]:-0} + 1))
            printf "%s | Expected: %s | Actual: %s\n" "$case" "$expected" "$actual" >> "$FAIL_DETAILS"
            ;;
        unsupported)
            UNSUPPORTED_COUNT=$((UNSUPPORTED_COUNT + 1))
            CATEGORY_UNSUPPORTED["$category"]=$((${CATEGORY_UNSUPPORTED["$category"]:-0} + 1))
            printf "%s | Reason: %s\n" "$case" "$reason" >> "$UNSUPPORTED_DETAILS"
            ;;
        blocked)
            BLOCKED_COUNT=$((BLOCKED_COUNT + 1))
            printf "%s | Condition: %s\n" "$case" "$reason" >> "$BLOCKED_DETAILS"
            ;;
    esac
done

TOTAL=$((PASS_COUNT + FAIL_COUNT + UNSUPPORTED_COUNT + BLOCKED_COUNT))
if [ "$TOTAL" -eq 0 ]; then
    TOTAL=1  # Avoid division by zero
fi
PASS_RATE=$((PASS_COUNT * 100 / TOTAL))

# Generate HTML report
cat > "$HTML_FILE" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Test262 Differential Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1, h2 { color: #333; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #4CAF50; color: white; }
        .pass { background-color: #d4edda; }
        .fail { background-color: #f8d7da; }
        .unsupported { background-color: #fff3cd; }
        .summary { font-size: 14px; margin: 10px 0; }
        .metric { display: inline-block; margin-right: 20px; }
        pre { background-color: #f5f5f5; padding: 10px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>Test262 Differential Test Report</h1>
EOF

# Summary metrics
cat >> "$HTML_FILE" << EOF
    <div class="summary">
        <h2>Summary</h2>
        <div class="metric"><strong>Pass:</strong> $PASS_COUNT ($PASS_RATE%)</div>
        <div class="metric"><strong>Fail:</strong> $FAIL_COUNT</div>
        <div class="metric"><strong>Unsupported:</strong> $UNSUPPORTED_COUNT</div>
        <div class="metric"><strong>Blocked:</strong> $BLOCKED_COUNT</div>
        <div class="metric"><strong>Total:</strong> $TOTAL</div>
    </div>

    <h2>Results by Category</h2>
    <table>
        <thead>
            <tr>
                <th>Category</th>
                <th class="pass">Pass</th>
                <th class="fail">Fail</th>
                <th class="unsupported">Unsupported</th>
                <th>Pass Rate</th>
            </tr>
        </thead>
        <tbody>
EOF

# Generate category breakdown
for category in $(echo "${!CATEGORY_PASS[@]} ${!CATEGORY_FAIL[@]} ${!CATEGORY_UNSUPPORTED[@]}" | tr ' ' '\n' | sort -u); do
    local cat_pass=${CATEGORY_PASS["$category"]:-0}
    local cat_fail=${CATEGORY_FAIL["$category"]:-0}
    local cat_unsupported=${CATEGORY_UNSUPPORTED["$category"]:-0}
    local cat_total=$((cat_pass + cat_fail + cat_unsupported))
    
    if [ "$cat_total" -eq 0 ]; then
        cat_total=1
    fi
    
    local cat_rate=$((cat_pass * 100 / cat_total))
    
    cat >> "$HTML_FILE" << EOF
            <tr>
                <td>$category</td>
                <td class="pass">$cat_pass</td>
                <td class="fail">$cat_fail</td>
                <td class="unsupported">$cat_unsupported</td>
                <td>$cat_rate%</td>
            </tr>
EOF
done

cat >> "$HTML_FILE" << 'EOF'
        </tbody>
    </table>

    <h2>Failures</h2>
    <details>
        <summary>Failed Tests (click to expand)</summary>
        <pre>
EOF

if [ -s "$FAIL_DETAILS" ]; then
    cat "$FAIL_DETAILS" >> "$HTML_FILE"
else
    echo "No failures" >> "$HTML_FILE"
fi

cat >> "$HTML_FILE" << 'EOF'
        </pre>
    </details>

    <h2>Unsupported Features</h2>
    <details>
        <summary>Unsupported Tests (click to expand)</summary>
        <pre>
EOF

if [ -s "$UNSUPPORTED_DETAILS" ]; then
    cat "$UNSUPPORTED_DETAILS" >> "$HTML_FILE"
else
    echo "No unsupported features" >> "$HTML_FILE"
fi

cat >> "$HTML_FILE" << 'EOF'
        </pre>
    </details>

</body>
</html>
EOF

# Generate Markdown report
cat > "$MD_FILE" << EOF
# Test262 Differential Test Report

## Summary

| Metric | Count |
|--------|-------|
| Pass | $PASS_COUNT ($PASS_RATE%) |
| Fail | $FAIL_COUNT |
| Unsupported | $UNSUPPORTED_COUNT |
| Blocked | $BLOCKED_COUNT |
| **Total** | **$TOTAL** |

## Results by Category

| Category | Pass | Fail | Unsupported | Pass Rate |
|----------|------|------|-------------|-----------|
EOF

for category in $(echo "${!CATEGORY_PASS[@]} ${!CATEGORY_FAIL[@]} ${!CATEGORY_UNSUPPORTED[@]}" | tr ' ' '\n' | sort -u); do
    local cat_pass=${CATEGORY_PASS["$category"]:-0}
    local cat_fail=${CATEGORY_FAIL["$category"]:-0}
    local cat_unsupported=${CATEGORY_UNSUPPORTED["$category"]:-0}
    local cat_total=$((cat_pass + cat_fail + cat_unsupported))
    
    if [ "$cat_total" -eq 0 ]; then
        cat_total=1
    fi
    
    local cat_rate=$((cat_pass * 100 / cat_total))
    
    cat >> "$MD_FILE" << EOF
| $category | $cat_pass | $cat_fail | $cat_unsupported | $cat_rate% |
EOF
done

cat >> "$MD_FILE" << 'EOF'

## Failures

```
EOF

if [ -s "$FAIL_DETAILS" ]; then
    cat "$FAIL_DETAILS" >> "$MD_FILE"
else
    echo "No failures" >> "$MD_FILE"
fi

cat >> "$MD_FILE" << 'EOF'
```

## Unsupported Features

```
EOF

if [ -s "$UNSUPPORTED_DETAILS" ]; then
    cat "$UNSUPPORTED_DETAILS" >> "$MD_FILE"
else
    echo "No unsupported features" >> "$MD_FILE"
fi

cat >> "$MD_FILE" << 'EOF'
```
EOF

# Output paths
echo "HTML report: $HTML_FILE" >&2
echo "Markdown report: $MD_FILE" >&2

# Also output to stdout for piping
cat "$HTML_FILE"
