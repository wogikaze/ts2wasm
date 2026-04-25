#!/usr/bin/env bash
# Deprecated: use scripts/perf/benchmark-tracker.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/perf/benchmark-tracker.sh" "$@"
