#!/usr/bin/env bash
# Ensure local git hooks are not bypassing agreed checks (lightweight).
#
# Usage: scripts/manager check-agent-policy
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/../.." && pwd)"
cd "$repo_root"

if [[ -d .githooks ]]; then
  if grep -RIn -- '--no-verify' .githooks 2>/dev/null; then
    echo "check_agent_policy: found --no-verify in .githooks (review required)" >&2
    exit 1
  fi
fi

echo "check_agent_policy: OK" >&2
