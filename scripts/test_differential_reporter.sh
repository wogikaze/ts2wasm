#!/usr/bin/env bash
# Deprecated: use scripts/report/differential.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/report/differential.sh" "$@"
