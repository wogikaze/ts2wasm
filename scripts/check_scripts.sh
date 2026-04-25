#!/bin/bash
# Validate shell scripts before running coverage/test workflows.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

echo "Running bash -n on scripts/*.sh"
for script in scripts/*.sh; do
    [[ -f "$script" ]] || continue
    bash -n "$script"
    echo "OK: $script"
done

echo "All shell syntax checks passed"
