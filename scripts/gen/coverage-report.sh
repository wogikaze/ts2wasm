#!/usr/bin/env bash
# Generate coverage report from language-reference markdown tables
# Usage: scripts/gen/coverage-report.sh [--format text|markdown]

set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

format="text"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --)
      shift
      ;;
    --format)
      format="${2:?--format requires text or markdown}"
      shift 2
      ;;
    text|markdown)
      format="$1"
      shift
      ;;
    -h|--help)
      sed -n '2,3p' "$0"
      exit 0
      ;;
    *)
      echo "unknown option: $1" >&2
      echo "Usage: scripts/gen/coverage-report.sh [--format text|markdown]" >&2
      exit 1
      ;;
  esac
done

case "$format" in
  text|markdown) ;;
  *)
    echo "unknown format: $format" >&2
    echo "Usage: scripts/gen/coverage-report.sh [--format text|markdown]" >&2
    exit 1
    ;;
esac

# Function to extract table data from markdown
extract_table() {
  local file="$1"
  local section="$2"
  
  # Find the section and extract the table
  awk -v section="$section" '
    BEGIN { in_section=0; in_table=0 }
    $0 == "## " section { in_section=1; next }
    in_section && /^## / { in_section=0; next }
    in_section && /^\|/ { in_table=1; print }
    in_table && /^$/ { in_table=0 }
  ' "$file"
}

# Function to count implementation status
count_status() {
  local file="$1"
  local section="$2"
  
  extract_table "$file" "$section" | tail -n +2 | awk -F'|' '
    {
      status = $5
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", status)
      if (status == "実装済み") implemented++
      else if (status == "未実装") unimplemented++
      else if (status == "将来対応") future++
      else if (status == "将来検討") future++
      else if (status ~ /実装済み/) implemented++
      else if (status ~ /未実装/) unimplemented++
      total++
    }
    END {
      printf "%d,%d,%d,%d\n", total, implemented, unimplemented, future
    }
  '
}

# Function to count by priority
count_priority() {
  local file="$1"
  local section="$2"
  
  extract_table "$file" "$section" | tail -n +2 | awk -F'|' '
    {
      priority = $6
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", priority)
      if (priority == "P0") p0++
      else if (priority == "P1") p1++
      else if (priority == "P2") p2++
      else if (priority == "P3") p3++
      else if (priority == "将来検討") future++
      else if (priority == "-") ignore++
      total++
    }
    END {
      printf "%d,%d,%d,%d,%d,%d\n", total, p0, p1, p2, p3, future
    }
  '
}

# Generate report
if [[ "$format" == "markdown" ]]; then
  echo "# Language Coverage Report"
  echo ""
  echo "Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
  echo ""
  
  for file in docs/language-reference/*.md; do
    if [[ ! -f "$file" ]]; then continue; fi
    basename=$(basename "$file" .md)
    echo "## $basename"
    echo ""
    
    # Count by section
    sections=$(grep -n "^## " "$file" | cut -d: -f2 | sed 's/^## //')
    for section in $sections; do
      # Skip non-feature sections
      case "$section" in
        "仕様リファレンス"|"仕様詳細"|"実装方針の原則"|"Capability Mapping") continue ;;
      esac
      
      IFS=',' read -r total implemented unimplemented future <<< "$(count_status "$file" "$section")"
      if [[ $total -eq 0 ]]; then continue; fi
      
      echo "### $section"
      echo ""
      echo "| Total | Implemented | Unimplemented | Future |"
      echo "|-------|-------------|---------------|--------|"
      echo "| $total | $implemented | $unimplemented | $future |"
      echo ""
      
      IFS=',' read -r total p0 p1 p2 p3 future <<< "$(count_priority "$file" "$section")"
      echo "| Priority | P0 | P1 | P2 | P3 | Future |"
      echo "|----------|----|----|----|----|--------|"
      echo "| Count | $p0 | $p1 | $p2 | $p3 | $future |"
      echo ""
    done
  done
else
  # Text format
  printf "%-30s %-20s %-12s %-12s %-12s %-8s\n" "File" "Section" "Total" "Impl" "Unimpl" "Future"
  printf "%s\n" "$(printf '%.0s-' {1..100})"
  
  for file in docs/language-reference/*.md; do
    if [[ ! -f "$file" ]]; then continue; fi
    basename=$(basename "$file" .md)
    
    sections=$(grep -n "^## " "$file" | cut -d: -f2 | sed 's/^## //')
    for section in $sections; do
      case "$section" in
        "仕様リファレンス"|"仕様詳細"|"実装方針の原則"|"Capability Mapping") continue ;;
      esac
      
      IFS=',' read -r total implemented unimplemented future <<< "$(count_status "$file" "$section")"
      if [[ $total -eq 0 ]]; then continue; fi
      
      printf "%-30s %-20s %-12s %-12s %-12s %-8s\n" "$basename" "$section" "$total" "$implemented" "$unimplemented" "$future"
    done
  done
  
  echo ""
  echo "Priority breakdown (unimplemented only):"
  printf "%-30s %-20s %-4s %-4s %-4s %-4s %-8s\n" "File" "Section" "P0" "P1" "P2" "P3" "Future"
  printf "%s\n" "$(printf '%.0s-' {1..100})"
  
  for file in docs/language-reference/*.md; do
    if [[ ! -f "$file" ]]; then continue; fi
    basename=$(basename "$file" .md)
    
    sections=$(grep -n "^## " "$file" | cut -d: -f2 | sed 's/^## //')
    for section in $sections; do
      case "$section" in
        "仕様リファレンス"|"仕様詳細"|"実装方針の原則"|"Capability Mapping") continue ;;
      esac
      
      IFS=',' read -r total p0 p1 p2 p3 future <<< "$(count_priority "$file" "$section")"
      if [[ $((p0 + p1 + p2 + p3 + future)) -eq 0 ]]; then continue; fi
      
      printf "%-30s %-20s %-4s %-4s %-4s %-4s %-8s\n" "$basename" "$section" "$p0" "$p1" "$p2" "$p3" "$future"
    done
  done
fi
