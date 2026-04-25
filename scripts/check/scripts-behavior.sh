#!/usr/bin/env bash
# Beyond bash -n: smoke that the script entry point runs.
#
# Usage: scripts/check_scripts_behavior.sh
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

if ! bash "${repo_root}/scripts/manager" help | head -n 3 | grep -q ts2wasm; then
  echo "check_scripts_behavior: scripts/manager help did not look right" >&2
  exit 1
fi
echo "check_scripts_behavior: OK" >&2
