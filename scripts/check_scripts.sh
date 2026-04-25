#!/usr/bin/env bash
# Deprecated: use scripts/check/shell-syntax.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/check/shell-syntax.sh" "$@"
