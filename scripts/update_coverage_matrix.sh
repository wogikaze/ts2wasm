#!/usr/bin/env bash
# Deprecated: use scripts/gen/coverage-matrix.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/gen/coverage-matrix.sh" "$@"
