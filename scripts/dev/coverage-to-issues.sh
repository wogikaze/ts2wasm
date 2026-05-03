#!/usr/bin/env bash
set -euo pipefail
cd "$(cd "$(dirname "$0")/../.." && pwd)"
S=""; X=()
while [ $# -gt 0 ]; do case "$1" in --suite) S="$2"; shift;; --limit) X+=("--limit" "$2"); shift;; *) X+=("$1");; esac; shift; done
[ -z "$S" ] && S="test262"
python3 scripts/run/reference-coverage.py "$S" --detail "${X[@]}" 2>/dev/null | python3 scripts/gen/issues-from-coverage.py --suite "$S"
python3 scripts/gen/update-issue-index.py
