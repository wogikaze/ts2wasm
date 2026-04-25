#!/usr/bin/env bash
# Validate .agents/state/current_task.json against its JSON schema

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CURRENT_TASK="$PROJECT_ROOT/.agents/state/current_task.json"
SCHEMA="$PROJECT_ROOT/.agents/state/schemas/current_task.schema.json"

# Check if ajv-cli is available
if ! command -v ajv &> /dev/null; then
    echo "ERROR: ajv-cli not found. Install with: npm install -g ajv-cli" >&2
    exit 1
fi

# Validate
if ! ajv validate -s "$SCHEMA" -d "$CURRENT_TASK" 2>&1; then
    echo "ERROR: current_task.json does not match schema" >&2
    exit 1
fi

echo "OK: current_task.json validates against schema"
