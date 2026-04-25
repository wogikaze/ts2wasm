#!/usr/bin/env bash
# Deprecated: use scripts/run/reference-coverage.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/run/reference-coverage.sh" "$@"
