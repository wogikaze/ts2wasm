#!/usr/bin/env bash
# Beyond bash -n: smoke that the script entry point runs.
#
# Usage: scripts/manager check-scripts-behavior
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

help_output="$(bash "${repo_root}/scripts/manager" help)"
if ! grep -q ts2wasm <<<"$help_output"; then
  echo "check_scripts_behavior: scripts/manager help did not look right" >&2
  exit 1
fi
echo "check_scripts_behavior: OK" >&2
