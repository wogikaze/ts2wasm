#!/usr/bin/env bash
set -euo pipefail
cd /home/wogikaze/ts2wasm

issue_field() {
  local file="$1"
  local field="$2"
  grep -m1 "^\*\*${field}\*\*:" "$file" 2>/dev/null | sed "s/^\*\*${field}\*\*: *//" | sed 's/[[:space:]]*$//' || true
}

issue_title() {
  local file="$1"
  grep -m1 '^# ' "$file" 2>/dev/null | sed 's/^# //' || true
}

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

echo "=== render_done_table output ==="
render_done_table
