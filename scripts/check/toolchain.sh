#!/usr/bin/env bash
# Verify build/test toolchain commands exist (for CI, agents, and new machines).
# Does not run compile/tests — only "can we invoke the tools?".
#
# Usage: scripts/check_toolchain.sh
# Exit: 0 if all required are present, 1 otherwise
set -euo pipefail

err=0
bad() { echo "check_toolchain: $*" >&2; err=1; }
ok()  { echo "check_toolchain: OK: $*"; }

need() {
  command -v "$1" >/dev/null 2>&1 && ok "command: $1" || bad "missing: $1"
}

need cargo
need bash
need node
if command -v npm >/dev/null 2>&1; then
  ok "command: npm"
else
  echo "check_toolchain: note: npm not on PATH (some nix/CI); optional for this repo" >&2
fi
need iwasm
need jq
need git
need mktemp

# Search (docs say rg / ig; either is enough)
if command -v ig >/dev/null 2>&1; then
  ok "search: ig"
elif command -v rg >/dev/null 2>&1; then
  ok "search: rg"
else
  bad "missing: ripgrep (ig or rg)"
fi

if command -v ast-grep >/dev/null 2>&1; then
  ok "ast-grep: ast-grep"
elif command -v sg >/dev/null 2>&1; then
  ok "ast-grep: sg"
else
  bad "missing: ast-grep (or sg)"
fi

if ! cargo nextest --version >/dev/null 2>&1; then
  bad "cargo nextest: not on PATH (install cargo-nextest; see AGENTS / README)"
else
  ok "cargo nextest"
fi

# wasm stack used by other harnesses
if command -v wasm-tools >/dev/null 2>&1; then
  ok "wasm-tools"
else
  bad "missing: wasm-tools (print/validate; used by manifest and wasm checks)"
fi

if [[ "$err" -ne 0 ]]; then
  echo "check_toolchain: FAILED" >&2
  exit 1
fi
echo "check_toolchain: all required tools present" >&2
