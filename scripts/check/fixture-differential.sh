#!/usr/bin/env bash
# Re-run the Node vs iwasm fixture differential (integration) suite as a standalone gate.
# Wraps: crates/cli/tests/m2_node_diff.rs
#
# Usage: scripts/check_fixture_differential.sh
# Dependencies: cargo, nextest, node, ts2wasm binary (via nextest build)
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

for c in cargo node iwasm; do
  command -v "$c" >/dev/null 2>&1 || {
    echo "check_fixture_differential: missing: $c" >&2
    exit 1
  }
done

# Optional: --sample N (reserved for future subset; full m2_node_diff always runs for now)
while [[ $# -gt 0 ]]; do
  case "$1" in
    --sample)
      if [[ -n "${2:-}" && "$2" != -* ]]; then
        echo "check_fixture_differential: note: --sample $2 ignored; running full m2_node_diff" >&2
        shift 2
      else
        echo "check_fixture_differential: --sample requires a number" >&2
        exit 1
      fi
      ;;
    -h|--help)
      echo "Usage: scripts/check_fixture_differential.sh [--sample N]" >&2
      exit 0
      ;;
    *)
      echo "check_fixture_differential: unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

echo "check_fixture_differential: cargo nextest -p ts2wasm-cli --test m2_node_diff" >&2
exec cargo nextest run -p ts2wasm-cli --test m2_node_diff
