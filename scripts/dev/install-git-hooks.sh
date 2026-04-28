#!/usr/bin/env bash
set -euo pipefail

_ts2wasm_entry_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/common.sh
source "${_ts2wasm_entry_dir}/../lib/common.sh"
cd "$TS2WASM_REPO_ROOT"

git config core.hooksPath .githooks
git config pull.ff only
git config merge.ff only
chmod +x .githooks/pre-commit .githooks/pre-push .githooks/pre-merge-commit

echo "Installed git hooks path: .githooks"
echo "Active hooks: .githooks/pre-commit, .githooks/pre-push, .githooks/pre-merge-commit"
echo "Merge policy: pull.ff=only, merge.ff=only"
