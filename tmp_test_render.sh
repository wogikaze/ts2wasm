#!/usr/bin/env bash
set -euo pipefail
cd /home/wogikaze/ts2wasm

# Source the actual functions from update-issue-index.sh by extracting them
# Or just replicate render_done_table exactly as in the script

_ts2wasm_entry_dir="scripts/gen"
source "${_ts2wasm_entry_dir}/../lib/common.sh"
TS2WASM_REPO_ROOT="."

# Call render_done_table - it should be defined by sourcing update-issue-index.sh
# But wait, update-issue-index.sh has code at the end that actually runs.
# Let's extract just the functions.

# Alternative: just source the relevant parts
source <(sed -n '55,301p' scripts/gen/update-issue-index.sh)

echo "=== render_done_table output ==="
render_done_table
