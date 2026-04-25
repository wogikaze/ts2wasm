#!/usr/bin/env bash
# Deprecated: use scripts/run/test262.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/run/test262.sh" "$@"
