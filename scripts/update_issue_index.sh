#!/usr/bin/env bash
# Regenerate only the marked regions in issues/index.md from issues/open/*.md
# and issues/done/*.md. Do not edit generated regions by hand.
#
# Behavior:
#   Rewrites the HTML comment bounded tables in issues/index.md (Ready / Blocked / Done).
#
# Options:
#   --check   Compare against the current index and exit nonzero if it would change.
#   -h, --help
#
# Dependencies: bash, awk, cat, cmp, cp, diff, grep, mktemp, mv, sed, sort, xargs
#
# Output:
#   Human messages on stderr. No machine-readable stdout contract.
set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib/common.sh
source "${_ts2wasm_entry_dir}/lib/common.sh"
# This script is in scripts/ directly, so repo root is one level up
TS2WASM_REPO_ROOT="$(cd "${_ts2wasm_entry_dir}/.." && pwd)"
export TS2WASM_REPO_ROOT
cd "$TS2WASM_REPO_ROOT"

ts2wasm_require_cmds awk cat cmp cp diff grep mktemp mv sed sort xargs

if ts2wasm_parse_common_args "$@"; then
  :
else
  case $? in
    1)
      ts2wasm_usage "scripts/update_issue_index.sh" \
        "Regenerate the marked regions in issues/index.md from issues/open/*.md and issues/done/*.md." \
        "Dependencies: bash, awk, cat, cmp, cp, diff, grep, mktemp, mv, sed, sort, xargs" \
        "Human status is printed to stderr. Exit 0 on success, nonzero on error or stale --check."
      exit 0
      ;;
    2)
      ts2wasm_usage "scripts/update_issue_index.sh" \
        "Regenerate the marked regions in issues/index.md from issues/open/*.md and issues/done/*.md."
      exit 1
      ;;
  esac
fi

check_mode="$TS2WASM_CHECK_MODE"

index_path="issues/index.md"
if [[ ! -f "$index_path" ]]; then
  echo "missing $index_path" >&2
  exit 1
fi

# Read a **Field**: value line from an issue markdown file.
issue_field() {
  local file="$1"
  local field="$2"
  grep -m1 "^\*\*${field}\*\*:" "$file" 2>/dev/null | sed "s/^\*\*${field}\*\*: *//" | sed 's/[[:space:]]*$//' || true
}

# First markdown H1 title in the file.
issue_title() {
  local file="$1"
  grep -m1 '^# ' "$file" 2>/dev/null | sed 's/^# //' || true
}

# One-line Problem: summary (fallback: title).
issue_problem_summary() {
  local file="$1"
  local line
  line="$(grep -m1 '^Problem:' "$file" 2>/dev/null | sed 's/^Problem:[[:space:]]*//' || true)"
  if [[ -z "$line" ]]; then
    line="$(issue_title "$file")"
  fi
  echo "$line"
}

# Numeric ID from **ID**:, leading digits in filename, then YAML `id:` (last resort).
issue_id_from_file() {
  local file="$1"
  local id base
  id="$(issue_field "$file" "ID")"
  if [[ -n "$id" ]]; then
    echo "$id"
    return
  fi
  base="$(basename "$file" .md)"
  if [[ "$base" =~ ^([0-9]+)- ]]; then
    echo "${BASH_REMATCH[1]}"
    return
  fi
  id="$(grep -m1 '^id:' "$file" 2>/dev/null | sed 's/^id:[[:space:]]*//' | tr -d '"' || true)"
  echo "${id:-}"
}

# Split **Depends on**: "none" | "001" | "002, 003" into space-separated IDs (empty if none).
parse_depends_ids() {
  local raw="$1"
  raw="${raw//,/ }"
  raw="$(echo "$raw" | xargs)"
  if [[ -z "$raw" || "$raw" == "none" ]]; then
    echo ""
    return
  fi
  echo "$raw"
}

# Sorted list of issue markdown paths (stable, locale-independent).
sorted_issue_paths() {
  local dir="$1"
  shopt -s nullglob
  local -a files=("$dir"/*.md)
  shopt -u nullglob
  if [[ ${#files[@]} -eq 0 ]]; then
    return
  fi
  printf '%s\n' "${files[@]}" | LC_ALL=C sort
}

# Collect open issue IDs (numeric or sub-issue like 017a) from issues/open/*.md
collect_open_ids() {
  local f id
  while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    id="$(issue_id_from_file "$f")"
    [[ -n "$id" ]] || continue
    echo "$id"
  done < <(sorted_issue_paths issues/open) | LC_ALL=C sort -V -u
}

# True (exit 0) if id is in the open set (bash 5+).
id_in_list() {
  local needle="$1"
  shift
  local x
  for x in "$@"; do
    if [[ "$x" == "$needle" ]]; then
      return 0
    fi
  done
  return 1
}

# Compute blocked open IDs: blocked if class is blocked OR any dependency is still open.
compute_blocked_ids() {
  local -a open_ids
  mapfile -t open_ids < <(collect_open_ids)

  local f id deps_raw class blocked=0
  while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    id="$(issue_id_from_file "$f")"
    [[ -n "$id" ]] || continue
    class="$(issue_field "$f" "Orchestration class" | tr '[:upper:]' '[:lower:]' | xargs)"
    deps_raw="$(issue_field "$f" "Depends on")"
    blocked=0
    if [[ "$class" == "blocked" ]]; then
      blocked=1
    else
      local dep_token
      for dep_token in $(parse_depends_ids "$deps_raw"); do
        if id_in_list "$dep_token" "${open_ids[@]}"; then
          blocked=1
          break
        fi
      done
    fi
    if [[ "$blocked" -eq 1 ]]; then
      echo "$id"
    fi
  done < <(sorted_issue_paths issues/open) | LC_ALL=C sort -V -u
}

render_ready_table() {
  local -a open_ids blocked_ids
  mapfile -t open_ids < <(collect_open_ids)
  mapfile -t blocked_ids < <(compute_blocked_ids)

  echo "| ID | Title | Type | Area | Class | Priority | Depends on | Summary |"
  echo "|---:|---|---|---|---|---|---|---|"

  if [[ ${#open_ids[@]} -eq 0 ]]; then
    echo "| — | No open issues | — | — | — | — | — | Create issues from \`issues/templates/issue.md\` |"
    return
  fi

  local id f ready_any=0
  for id in "${open_ids[@]}"; do
    if id_in_list "$id" "${blocked_ids[@]}"; then
      continue
    fi
    ready_any=1
    while IFS= read -r f; do
      [[ -n "$f" ]] || continue
      [[ "$(issue_id_from_file "$f")" == "$id" ]] || continue
      local title type area orch_class priority depends summary
      title="$(issue_title "$f")"
      type="$(issue_field "$f" "Type")"
      area="$(issue_field "$f" "Area")"
      orch_class="$(issue_field "$f" "Orchestration class")"
      priority="$(issue_field "$f" "Priority")"
      depends="$(issue_field "$f" "Depends on")"
      summary="$(issue_problem_summary "$f")"
      title="${title//|/\\|}"
      summary="${summary//|/\\|}"
      if [[ ${#summary} -gt 120 ]]; then
        summary="${summary:0:117}..."
      fi
      echo "| $id | $title | $type | $area | $orch_class | $priority | $depends | $summary |"
      break
    done < <(sorted_issue_paths issues/open)
  done

  if [[ "$ready_any" -eq 0 && ${#open_ids[@]} -gt 0 ]]; then
    echo "| — | No ready issues (all blocked) | — | — | — | — | — | See Blocked queue |"
  fi
}

render_blocked_table() {
  local -a blocked_ids
  mapfile -t blocked_ids < <(compute_blocked_ids)

  echo "| ID | Title | Type | Area | Blocker | Summary |"
  echo "|---:|---|---|---|---|---|"

  if [[ ${#blocked_ids[@]} -eq 0 ]]; then
    echo "| — | No blocked issues | — | — | — | — |"
    return
  fi

  local id f deps_raw blockers summary title type area class
  for id in "${blocked_ids[@]}"; do
    while IFS= read -r f; do
      [[ -n "$f" ]] || continue
      [[ "$(issue_id_from_file "$f")" == "$id" ]] || continue
      title="$(issue_title "$f")"
      type="$(issue_field "$f" "Type")"
      area="$(issue_field "$f" "Area")"
      deps_raw="$(issue_field "$f" "Depends on")"
      summary="$(issue_problem_summary "$f")"
      title="${title//|/\\|}"
      summary="${summary//|/\\|}"
      if [[ ${#summary} -gt 100 ]]; then
        summary="${summary:0:97}..."
      fi
      blockers="$deps_raw"
      class="$(issue_field "$f" "Orchestration class" | tr '[:upper:]' '[:lower:]' | xargs)"
      if [[ "$class" == "blocked" ]]; then
        blockers="class: $(issue_field "$f" "Orchestration class")"
      fi
      blockers="${blockers//|/\\|}"
      echo "| $id | $title | $type | $area | $blockers | $summary |"
      break
    done < <(sorted_issue_paths issues/open)
  done
}

render_done_table() {
  echo "| ID | Title | Type | Area | Completed evidence |"
  echo "|---:|---|---|---|---|"

  local rows_tmp count=0
  rows_tmp="$(mktemp)"
  trap "rm -f -- \"$rows_tmp\"" RETURN

  local f id title type area
  while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    id="$(issue_id_from_file "$f")"
    if [[ -z "$(issue_field "$f" "ID")" ]]; then
      title="$(grep -m1 '^title:' "$f" 2>/dev/null | sed 's/^title:[[:space:]]*//' | sed 's/^"//;s/"$//' || true)"
    else
      title="$(issue_title "$f")"
    fi
    if [[ -z "$title" ]]; then
      title="$(issue_title "$f")"
    fi
    type="$(issue_field "$f" "Type")"
    if [[ -z "$type" ]]; then
      type="$(grep -m1 '^type:' "$f" 2>/dev/null | sed 's/^type:[[:space:]]*//' | sed 's/[[:space:]]*|.*//' | tr -d '"' || true)"
    fi
    area="$(issue_field "$f" "Area")"
    if [[ -z "$area" ]]; then
      area="$(grep -m1 '^area:' "$f" 2>/dev/null | sed 's/^area:[[:space:]]*//' | sed 's/[[:space:]]*|.*//' || true)"
    fi
    [[ -n "$id" ]] || continue
    count=$((count + 1))
    title="${title//|/\\|}"
    local evidence="see file"
    if grep -q '^## Completion evidence' "$f" 2>/dev/null; then
      evidence="see \`issues/done/$(basename "$f")\`"
    fi
    echo "| $id | $title | ${type:-—} | ${area:-—} | $evidence |" >>"$rows_tmp"
  done < <(sorted_issue_paths issues/done)

  if [[ "$count" -eq 0 ]]; then
    echo "| — | No completed issues | — | — | — |"
    return
  fi
  LC_ALL=C sort -t'|' -k2,2n "$rows_tmp"
}

replace_generated_block() {
  local start_marker="$1"
  local end_marker="$2"
  local content_file="$3"
  local infile="$4"
  local out="$5"
  local cf_q
  cf_q="$(printf '%q' "$content_file")"
  awk -v s="$start_marker" -v e="$end_marker" -v cfq="$cf_q" '
    BEGIN { mode = 0; fence = 0 }
    /^```/ { fence = !fence; print; next }
    fence == 1 { print; next }
    $0 == s { print; mode = 1; system("cat " cfq); next }
    $0 == e { print; mode = 0; next }
    mode == 0 { print }
    mode == 1 { next }
  ' "$infile" >"$out"
}

tmp_ready="$(mktemp)"
tmp_blocked="$(mktemp)"
tmp_done="$(mktemp)"
tmp_index="$(mktemp)"
trap 'rm -f "$tmp_ready" "$tmp_blocked" "$tmp_done" "$tmp_index" "${tmp_index}.tmp"' EXIT

render_ready_table >"$tmp_ready"
render_blocked_table >"$tmp_blocked"
render_done_table >"$tmp_done"

cp "$index_path" "$tmp_index"
replace_generated_block "<!-- generated:ready:start -->" "<!-- generated:ready:end -->" "$tmp_ready" "$tmp_index" "${tmp_index}.tmp"
mv "${tmp_index}.tmp" "$tmp_index"
replace_generated_block "<!-- generated:blocked:start -->" "<!-- generated:blocked:end -->" "$tmp_blocked" "$tmp_index" "${tmp_index}.tmp"
mv "${tmp_index}.tmp" "$tmp_index"
replace_generated_block "<!-- generated:done:start -->" "<!-- generated:done:end -->" "$tmp_done" "$tmp_index" "${tmp_index}.tmp"
mv "${tmp_index}.tmp" "$tmp_index"

if [[ "$check_mode" -eq 1 ]]; then
  if ! cmp -s "$index_path" "$tmp_index"; then
    ts2wasm_log "issues/index.md is stale; run scripts/update_issue_index.sh"
    diff -u "$index_path" "$tmp_index" >&2 || true
    exit 1
  fi
  ts2wasm_log "issues/index.md OK (up to date)"
  exit 0
fi

cp "$tmp_index" "$index_path"
ts2wasm_log "Updated $index_path"
