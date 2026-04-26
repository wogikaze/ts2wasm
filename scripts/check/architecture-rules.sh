#!/usr/bin/env bash
# Lightweight dependency-direction checks (complement to docs/12 + ast-grep rules).
#
# Usage: scripts/manager check-architecture-rules
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/manager check-architecture-rules

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

command -v cargo >/dev/null 2>&1 || {
  echo "check_architecture_rules: cargo is required" >&2
  exit 1
}

if cargo tree -p ts2wasm-shared --edges normal,build 2>/dev/null | grep -qF 'ts2wasm-cli'; then
  echo "check_architecture_rules: ts2wasm-shared must not depend on ts2wasm-cli" >&2
  cargo tree -p ts2wasm-shared --edges normal,build >&2
  exit 1
fi

echo "check_architecture_rules: OK" >&2
