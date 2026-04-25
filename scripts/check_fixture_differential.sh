#!/usr/bin/env bash
# Re-run the Node vs iwasm fixture differential (integration) suite as a standalone gate.
# Wraps: crates/cli/tests/m2_node_diff.rs
#
# Usage: scripts/check_fixture_differential.sh
# Dependencies: cargo, nextest, node, ts2wasm binary (via nextest build)
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/.." && pwd)"
cd "$repo_root"

for c in cargo node iwasm; do
  command -v "$c" >/dev/null 2>&1 || {
    echo "check_fixture_differential: missing: $c" >&2
    exit 1
  }
done

echo "check_fixture_differential: cargo nextest -p ts2wasm-cli --test m2_node_diff" >&2
exec cargo nextest run -p ts2wasm-cli --test m2_node_diff
