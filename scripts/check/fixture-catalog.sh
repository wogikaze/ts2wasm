#!/usr/bin/env bash
# Mechanical fixture layout rules (taxonomy hygiene).
#
# Rules:
#   - Top-level entries under fixtures/ must be directories (no loose .ts at fixtures root).
#   - Directory names: lowercase ASCII, digits, hyphen only (kebab-case prefix style).
#
# Usage: scripts/manager check-fixture-catalog
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/manager check-fixture-catalog

Validates fixtures/ directory layout conventions.
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

errors=0
err() {
  echo "check_fixture_catalog: $*" >&2
  errors=1
}

if [[ ! -d fixtures ]]; then
  err "missing fixtures/"
  exit 1
fi

shopt -s nullglob
for p in fixtures/*; do
  [[ -e "$p" ]] || continue
  base="$(basename "$p")"
  if [[ -f "$p" ]]; then
    err "fixtures/ must not contain loose files at top level: $base"
  fi
  if [[ ! -d "$p" ]]; then
    err "fixtures/ top-level entry is not a directory: $base"
    continue
  fi
  if [[ ! "$base" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
    err "fixtures/ directory name must be kebab-case [a-z0-9-]+ only: $base"
  fi
done
shopt -u nullglob

if [[ "$errors" -ne 0 ]]; then
  exit 1
fi

echo "check_fixture_catalog: OK" >&2
