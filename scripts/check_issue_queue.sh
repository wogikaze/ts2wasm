#!/usr/bin/env bash
# Mechanical invariants for issues/ and the issue index.
#
# Fails (exit 1) on:
#   - duplicate NNN- prefix in issues/open/ or issues/done/
#   - same NNN in both open/ and done/ (id collision)
#   - filename NNN not matching **ID** (or yaml id: / id:) in the file
#   - issues/done/*.md (excluding *sample* and 000-*) containing unchecked items "- [ ]"
#   - **Depends on** listing an id with no issues/open/NNN-*.md or issues/done/NNN-*.md
#   - backticked repo paths (crates|docs|fixtures|scripts|reference|issues|reports|.agents) that
#     do not exist, skipping obvious placeholders
#   - .agents/state/*.json invalid JSON
#   - issues/index.md: scripts/check_issue_index.sh (stale index + queue table contract)
#
# Usage: scripts/check_issue_queue.sh
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
errors=0

err() { echo "check_issue_queue: $*" >&2; errors=1; }

# --- Id from filename: NNN-stuff.md -> 000, NNNa-stuff.md -> 017a
id_from_basename() {
  local b="$1"
  if [[ "$b" =~ ^([0-9]{3}[a-z]?)- ]]; then
    echo "${BASH_REMATCH[1]}"
  else
    echo ""
  fi
}

# --- Id from file body: **ID**: 010 or 017a, or id: 010 (first 40 lines, yaml or markdown)
id_from_body() {
  local f="$1"
  local line idl
  while IFS= read -r line; do
    if [[ "$line" =~ ^\*\*ID\*\*:[[:space:]]*(.+) ]]; then
      idl="${BASH_REMATCH[1]// /}"
      idl="${idl//$'\r'/}"
      echo "$idl"
      return 0
    fi
  done < <(head -n 50 "$f")

  while IFS= read -r line; do
    if [[ "$line" =~ ^(id|ID):[[:space:]]*\"?([0-9]+[a-z]?)\"? ]]; then
      idl="${BASH_REMATCH[2]}"
      idl="${idl//$'\r'/}"
      idl="${idl//[[:space:]]/}"
      if [[ -n "$idl" ]]; then
        # If it's a numeric-only ID, zero-pad to 3 digits
        if [[ "$idl" =~ ^[0-9]+$ ]]; then
          printf '%03d' $((10#$idl))
        else
          echo "$idl"
        fi
        return 0
      fi
    fi
  done < <(head -n 20 "$f")

  echo ""
}

# --- issues using NNN-*.md or NNNa-*.md
collect_ids_in_dir() {
  local d="$1"
  local f base tid
  shopt -s nullglob
  for f in "$d"/*.md; do
    [[ -f "$f" ]] || continue
    base="$(basename "$f")"
    if [[ ! "$base" =~ ^[0-9]{3}[a-z]?- ]]; then
      continue
    fi
    id_from_basename "$base"
  done
}

# --- Duplicates in one directory
check_duplicates_in() {
  local d="$1"
  local f sort_out
  sort_out="$(collect_ids_in_dir "$d" | sort | uniq -d || true)"
  if [[ -n "$sort_out" ]]; then
    err "duplicate id prefix in $d: $(echo "$sort_out" | tr '\n' ' ')"
  fi
}

# --- Collision between open and done
check_open_done_collision() {
  local o d
  o="$(collect_ids_in_dir issues/open | sort -u)"
  d="$(collect_ids_in_dir issues/done | sort -u)"
  if [[ -z "$o" || -z "$d" ]]; then
    return 0
  fi
  local a
  a="$(comm -12 <(echo "$o") <(echo "$d") || true)"
  if [[ -n "$a" ]]; then
    err "id(s) present in both issues/open/ and issues/done/: $(echo "$a" | tr '\n' ' ')"
  fi
}

# --- Filename id vs body
check_id_matches_body() {
  local dir="$1"
  local f base f_id b_id
  shopt -s nullglob
  for f in "$dir"/*.md; do
    [[ -f "$f" ]] || continue
    base="$(basename "$f")"
    [[ "$base" =~ ^[0-9]{3}[a-z]?- ]] || continue
    f_id="$(id_from_basename "$base")"
    b_id="$(id_from_body "$f")"
    if [[ -z "$b_id" ]]; then
      err "$f: missing **ID** or id: in header (expected id $f_id matching filename)"
      continue
    fi
    # For numeric-only IDs, zero-pad to 3 digits for comparison
    if [[ "$f_id" =~ ^[0-9]+$ ]]; then
      f_id="$(printf '%03d' $((10#$f_id)))"
    fi
    if [[ "$b_id" =~ ^[0-9]+$ ]]; then
      b_id="$(printf '%03d' $((10#$b_id)))"
    fi
    if [[ "$b_id" != "$f_id" ]]; then
      err "$f: id mismatch: filename $f_id vs body $b_id"
    fi
  done
}

# --- done/: no unchecked [ ] (exclude samples)
check_done_unchecked() {
  local f base
  shopt -s nullglob
  for f in issues/done/*.md; do
    base="$(basename "$f")"
    if [[ "$base" == *sample* || "$base" == 000-* ]]; then
      continue
    fi
    if grep -n '- \[ \]' "$f" &>/dev/null; then
      err "$f: has unchecked list items - [ ] but file is in issues/done/"
    fi
  done
}

# --- Depends on references (supports NNN and NNNa formats)
depend_ids_from_file() {
  local f="$1"
  local raw
  raw="$( (grep -m1 '^\*\*Depends on\*\*:' "$f" 2>/dev/null || :) | sed 's/^\*\*Depends on\*\*:[[:space:]]*//' | tr -d '\r' )"
  raw="${raw//,/ }"
  if [[ -z "$raw" || "$raw" == "none" || "$raw" == "None" ]]; then
    return 0
  fi
  local t
  for t in $raw; do
    # Extract ID (numeric only or with letter suffix)
    if [[ "$t" =~ ^([0-9]{3}[a-z]?)$ ]]; then
      echo "${BASH_REMATCH[1]}"
    elif [[ "$t" =~ ^([0-9]+)[a-z]?$ ]]; then
      # Zero-pad numeric-only IDs
      printf '%03d' $((10#${BASH_REMATCH[1]}))
    fi
  done
}

check_depends_resolvable() {
  local f d wid ex
  shopt -s nullglob
  for f in issues/open/*.md; do
    while IFS= read -r d; do
      [[ -n "$d" ]] || continue
      ex=0
      for wid in issues/open/${d}-*.md issues/done/${d}-*.md; do
        [[ -e "$wid" ]] && ex=1 && break
      done
      if [[ "$ex" -ne 1 ]]; then
        err "$f: **Depends on** id $d has no matching issues/open/${d}-*.md or issues/done/${d}-*.md"
      fi
    done < <(depend_ids_from_file "$f")
  done
}

# --- Check sub-issue validity: no duplicate sub-ids within same parent, parent exists
check_sub_issue_validity() {
  local dir="$1"
  local f base parent_id sub_id
  local -A parent_map
  local -A sub_map

  shopt -s nullglob
  for f in "$dir"/*.md; do
    [[ -f "$f" ]] || continue
    base="$(basename "$f")"
    [[ "$base" =~ ^[0-9]{3}[a-z]?- ]] || continue
    local full_id
    full_id="$(id_from_basename "$base")"

    # Check if this is a sub-issue (has letter suffix)
    if [[ "$full_id" =~ ^([0-9]{3})([a-z])$ ]]; then
      parent_id="${BASH_REMATCH[1]}"
      sub_id="${BASH_REMATCH[2]}"
      local key="${parent_id}_${sub_id}"
      if [[ -n "${sub_map[$key]:-}" ]]; then
        err "$f: duplicate sub-issue id $full_id (conflicts with ${sub_map[$key]})"
      else
        sub_map[$key]="$f"
      fi
      # Track that this parent has sub-issues
      parent_map[$parent_id]=1
    fi
  done

  # Check that parent issues exist for all sub-issues
  for parent_id in "${!parent_map[@]}"; do
    local parent_found=0
    for f in "$dir"/*.md issues/done/*.md; do
      [[ -f "$f" ]] || continue
      base="$(basename "$f")"
      local pid
      pid="$(id_from_basename "$base")"
      if [[ "$pid" == "$parent_id" ]]; then
        parent_found=1
        break
      fi
    done
    if [[ "$parent_found" -eq 0 ]]; then
      err "sub-issues exist for parent $parent_id but parent issue not found in open/ or done/"
    fi
  done
}

# --- Path existence
should_skip_path() {
  local p="$1"
  case "$p" in
  *'...'*) return 0 ;; # placeholder
  *\|*) return 0 ;;    # type union text in templates
  esac
  [[ ${#p} -lt 4 ]] && return 0
  if [[ ! "$p" =~ ^(crates|docs|fixtures|scripts|reference|issues|reports|\.github|\.agents|artifacts)/ ]]; then
    return 0
  fi
  if [[ "$p" == *'*' ]]; then
    return 0
  fi
  if [[ "$p" =~ YYYY|xxxx ]]; then
    return 0
  fi
  return 1
}

check_paths_in_issues() {
  local f raw p
  shopt -s nullglob
  for f in issues/open/*.md issues/done/*.md; do
    [[ -e "$f" ]] || continue
    while IFS= read -r p; do
      [[ -n "$p" ]] || continue
      p="${p//[[:space:]]/}"
      p="${p%)}"
      p="${p%,}"
      if should_skip_path "$p"; then
        continue
      fi
      if [[ -e "$repo_root/$p" ]]; then
        continue
      fi
      err "$f: missing path: $p"
    done < <(LC_ALL=C grep -ohE '\`(crates|docs|fixtures|scripts|reference|issues|reports|\.github/|\.agents/|artifacts)/[^` ]+' "$f" 2>/dev/null | tr -d '\`' || true)
  done
}

json_ok() {
  if ! command -v jq &>/dev/null; then
    return 0
  fi
  local j
  shopt -s nullglob
  for j in .agents/state/*.json .agents/state/examples/*.json; do
    [[ -e "$j" ]] || continue
    jq empty "$j" 2>/dev/null || err "$j: invalid JSON (jq empty)"
  done
}

# --- run all checks
check_duplicates_in "issues/open"
check_duplicates_in "issues/done"
check_open_done_collision
check_id_matches_body "issues/open"
check_id_matches_body "issues/done"
check_done_unchecked
check_sub_issue_validity "issues/open"
check_sub_issue_validity "issues/done"
check_depends_resolvable
check_paths_in_issues
json_ok

if ! "$repo_root/scripts/check_issue_index.sh"; then
  errors=1
fi

if [[ "$errors" -ne 0 ]]; then
  echo "check_issue_queue: failed (see errors above)" >&2
  exit 1
fi
echo "check_issue_queue: OK"
