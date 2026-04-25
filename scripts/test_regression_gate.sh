#!/usr/bin/env bash
# Deprecated: use scripts/gate/regression.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/gate/regression.sh" "$@"
