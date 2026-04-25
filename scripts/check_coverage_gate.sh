#!/usr/bin/env bash
# Deprecated: use scripts/gate/coverage.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/gate/coverage.sh" "$@"
