#!/usr/bin/env bash
# Inventory and baseline: toolchain, P0 harness scripts, existing gates, optional P1+ stubs.
#
# Default: planned (P1+) scripts may be missing — warn only. P0 must exist and pass.
#   REQUIRE_ALL_HARNESSES=1  — treat P1+ scripts as required (exist + executable) too.
# Nextest: default is plain `cargo nextest run` (warnings allowed). Strict:
#   TS2WASM_NEXTEST_DENY_WARNINGS=1  —  RUSTFLAGS='-D warnings' (project may fail until #011 is done).
#
# Usage:
#   scripts/check_harness_installation.sh
set -euo pipefail

repo_root="$(cd "$(dirname "${0}")/.." && pwd)"
cd "$repo_root"

fail=0
ok()  { printf 'harness: OK: %s\n' "$1"; }
bad() { printf 'harness: FAIL: %s\n' "$1" >&2; fail=1; }
warn() { printf 'harness: WARN: %s\n' "$1" >&2; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 && ok "command: $1" || bad "missing command: $1"
}

need_exec_required() {
  if [[ -x "$1" ]]; then
    ok "executable: $1"
  else
    bad "missing executable: $1"
  fi
}

need_exec_optional() {
  if [[ -x "$1" ]]; then
    ok "executable: $1"
  else
    warn "not installed (optional in default mode): $1"
  fi
}

run_check() {
  local name="$1"
  shift
  echo "" >&2
  echo "== $name ==" >&2
  if "$@"; then
    ok "$name"
  else
    bad "$name"
  fi
}

echo "== toolchain (quick) ==" >&2
need_cmd cargo
need_cmd bash
need_cmd node
need_cmd iwasm
need_cmd jq
need_cmd git
need_cmd wasm-tools

if command -v ast-grep >/dev/null 2>&1 || command -v sg >/dev/null 2>&1; then
  ok "ast-grep/sg"
else
  bad "ast-grep/sg"
fi

if command -v ig >/dev/null 2>&1 || command -v rg >/dev/null 2>&1; then
  ok "ig/rg"
else
  bad "ig or rg"
fi

cargo nextest --version >/dev/null 2>&1 && ok "cargo nextest" || bad "cargo nextest"

echo "" >&2
echo "== P0 harness (must exist) ==" >&2
for f in \
  scripts/check_toolchain.sh \
  scripts/check_fixture_differential.sh \
  scripts/check_host_deny.sh \
  scripts/check_runtimefn_invariants.sh \
  scripts/check_wasm_validation.sh; do
  need_exec_required "$f"
done

echo "" >&2
echo "== P1+ planned harnesses (default: optional) ==" >&2
if [[ "${REQUIRE_ALL_HARNESSES:-0}" == "1" ]]; then
  check_p=need_exec_required
else
  check_p=need_exec_optional
fi
$check_p scripts/check_docs_health.sh
$check_p scripts/check_agent_policy.sh
$check_p scripts/check_benchmark_regression.sh
$check_p scripts/check_scripts_behavior.sh
$check_p scripts/check_determinism.sh

echo "" >&2
echo "== required repo gates (script files) ==" >&2
for f in \
  scripts/check_scripts.sh \
  scripts/check_issue_queue.sh \
  scripts/update_coverage_matrix.sh \
  scripts/check_fast_gate.sh \
  scripts/check_manifest_imports.sh \
  scripts/check_test_records_schema.sh \
  scripts/check_fixture_catalog.sh \
  scripts/check_architecture_rules.sh \
  scripts/check_compiler_diagnostics.sh; do
  need_exec_required "$f"
done

echo "" >&2
echo "== run P0 harnesses ==" >&2
run_check "P0: check_toolchain" bash scripts/check_toolchain.sh
run_check "P0: check_fixture_differential" bash scripts/check_fixture_differential.sh
run_check "P0: check_host_deny" bash scripts/check_host_deny.sh
run_check "P0: check_runtimefn_invariants" bash scripts/check_runtimefn_invariants.sh
run_check "P0: check_wasm_validation" bash scripts/check_wasm_validation.sh

echo "" >&2
echo "== run aggregate gates (fast gate without nextest first) ==" >&2
# fmt + scripts + issues + coverage matrix (nextest run separately so we can flag RUSTFLAGS once)
run_check "check_fast_gate --skip-nextest" bash scripts/check_fast_gate.sh --skip-nextest

if [[ "${TS2WASM_NEXTEST_DENY_WARNINGS:-0}" == "1" ]]; then
  echo "harness: TS2WASM_NEXTEST_DENY_WARNINGS=1 (RUSTFLAGS=-D warnings)" >&2
  run_check "cargo nextest (RUSTFLAGS=-D warnings)" env RUSTFLAGS='-D warnings' cargo nextest run
else
  echo "harness: (hint) set TS2WASM_NEXTEST_DENY_WARNINGS=1 to fail on Rust warnings (see issues/open/011-*.md)" >&2
  run_check "cargo nextest" cargo nextest run
fi

echo "" >&2
echo "== additional custom harnesses ==" >&2
run_check "check_manifest_imports" bash scripts/check_manifest_imports.sh
run_check "check_test_records_schema (empty)" bash -c ': | scripts/check_test_records_schema.sh'
run_check "check_fixture_catalog" bash scripts/check_fixture_catalog.sh
run_check "check_architecture_rules" bash scripts/check_architecture_rules.sh
run_check "check_compiler_diagnostics" bash scripts/check_compiler_diagnostics.sh

echo "" >&2
if [[ "$fail" -eq 0 ]]; then
  echo "HARNESS BASELINE PASSED" >&2
  exit 0
fi
echo "HARNESS BASELINE FAILED" >&2
exit 1
