#!/usr/bin/env bash
# Fail if obvious non-test panic sites appear in production compiler directories.
#
# Policy (incremental): backend/, runtime/, and main.rs must not contain `panic!(`.
# Tests and lib.rs monolith may still use panics inside #[cfg(test)]; tighten over time.
#
# Usage: scripts/check_compiler_diagnostics.sh
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/check_compiler_diagnostics.sh

Fails if `panic!(` appears under crates/cli/src/backend, crates/cli/src/runtime, or main.rs.
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

hits=0
while IFS= read -r line; do
  echo "check_compiler_diagnostics: $line" >&2
  hits=1
done < <(git grep -n 'panic!(' -- crates/cli/src/backend crates/cli/src/runtime crates/cli/src/main.rs 2>/dev/null || true)

if [[ "$hits" -ne 0 ]]; then
  exit 1
fi

echo "check_compiler_diagnostics: OK" >&2
