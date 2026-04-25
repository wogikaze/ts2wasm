#!/usr/bin/env bash
# Deprecated: use scripts/dev/install-git-hooks.sh
set -euo pipefail
exec "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/dev/install-git-hooks.sh" "$@"
