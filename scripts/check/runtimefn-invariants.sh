#!/usr/bin/env bash
# Run in-tree RuntimeLinkPlan / runtime helper unit tests (no integration / iwasm).
# Reuses crates/cli src/backend runtime_link_plan.rs and runtime_fn.rs #[test] items.
#
# Usage: scripts/manager check-runtimefn-invariants
# Uses: cargo test (not nextest) for simple filter on internal module path.
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

if ! command -v cargo >/dev/null 2>&1; then
  echo "check_runtimefn_invariants: cargo is required" >&2
  exit 1
fi

echo "check_runtimefn_invariants: runtime_link_plan::tests" >&2
cargo test -p ts2wasm-cli --lib 'runtime_link_plan::tests' -- --quiet
echo "check_runtimefn_invariants: runtime_fn::tests" >&2
cargo test -p ts2wasm-cli --lib 'runtime_fn::tests' -- --quiet
echo "check_runtimefn_invariants: OK" >&2
