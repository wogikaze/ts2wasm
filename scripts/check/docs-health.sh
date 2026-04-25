#!/usr/bin/env bash
# Docs hygiene: backticked scripts/ paths in docs must exist in the repo.
#
# Usage: scripts/check_docs_health.sh
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

if [[ ! -d docs ]]; then
  echo "check_docs_health: missing docs/" >&2
  exit 1
fi

bad=0
while IFS= read -r -d '' f; do
  while IFS= read -r p; do
    [[ -z "$p" ]] && continue
    if [[ ! -f "$p" ]]; then
      echo "check_docs_health: $f references missing $p" >&2
      bad=1
    fi
  done < <(grep -ohE 'scripts/[^`[:space:]]+\.sh' "$f" 2>/dev/null | sort -u)
done < <(find docs -name '*.md' -print0 2>/dev/null)

if [[ "$bad" -ne 0 ]]; then
  exit 1
fi
echo "check_docs_health: OK" >&2
