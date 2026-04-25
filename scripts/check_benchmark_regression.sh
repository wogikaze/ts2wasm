#!/usr/bin/env bash
# Benchmark gate (placeholder). Wire to benchmark tracker + stored baselines in a follow-up.
#
# Usage: scripts/check_benchmark_regression.sh [--sample N]
set -euo pipefail

while [[ $# -gt 0 ]]; do
  case "$1" in
    --sample)
      shift
      [[ -n "${1:-}" && "$1" != -* ]] && shift || true
      ;;
    -h|--help)
      echo "Usage: scripts/check_benchmark_regression.sh [--sample N]" >&2
      echo "Note: enforcement not yet implemented; exits 0." >&2
      exit 0
      ;;
    *)
      echo "check_benchmark_regression: unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

echo "check_benchmark_regression: OK (no baseline gate yet; use scripts/perf/benchmark-tracker.sh for data)" >&2
