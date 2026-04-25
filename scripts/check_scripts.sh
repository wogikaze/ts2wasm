#!/usr/bin/env bash
# Validate shell scripts before running coverage/test workflows.
#
# Note: `bash -n` is syntax-only. It does not prove runtime behavior.
# After editing a script, also run a representative command (see
# `.agents/skills/ts2wasm-scripts-workflow/SKILL.md`).

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

echo "Running bash -n on scripts/*.sh and scripts/manager"
for script in scripts/*.sh; do
    [[ -f "$script" ]] || continue
    bash -n "$script"
    echo "OK: $script"
done
if [[ -f scripts/manager ]]; then
    bash -n scripts/manager
    echo "OK: scripts/manager"
fi

echo "All shell syntax checks passed"
