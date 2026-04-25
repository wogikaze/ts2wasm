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

f="issues/done/002-emit-canonical-capability-manifest-schema.md"
echo "file=$f"
echo "id=$(issue_id_from_file "$f")"
echo "has_id_field='$(issue_field "$f" "ID")'"
if [[ -z "$(issue_field "$f" "ID")" ]]; then
  title="$(grep -m1 '^title:' "$f" 2>/dev/null | sed 's/^title:[[:space:]]*//' | sed 's/^"//;s/"$//' || true)"
  echo "title=$title"
else
  echo "title_from_h1=$(issue_title "$f")"
fi
type="$(issue_field "$f" "Type")"
if [[ -z "$type" ]]; then type="$(grep -m1 '^type:' "$f" 2>/dev/null | sed 's/^type:[[:space:]]*//' | sed 's/[[:space:]]*|.*//' | tr -d '"' || true)"; fi
echo "type=$type"
area="$(issue_field "$f" "Area")"
if [[ -z "$area" ]]; then area="$(grep -m1 '^area:' "$f" 2>/dev/null | sed 's/^area:[[:space:]]*//' | sed 's/[[:space:]]*|.*//' || true)"; fi
echo "area=$area"
