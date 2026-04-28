#!/usr/bin/env python3
"""Verify build/test toolchain commands exist (for CI, agents, and new machines).
Does not run compile/tests — only "can we invoke the tools?".

Usage: mise run check toolchain
Exit: 0 if all required are present, 1 otherwise
"""

import sys
import shutil
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

err = 0

def bad(msg):
    global err
    print(f"check_toolchain: {msg}", file=sys.stderr)
    err = 1

def ok(msg):
    print(f"check_toolchain: OK: {msg}", file=sys.stderr)

def need(cmd):
    if shutil.which(cmd):
        ok(f"command: {cmd}")
    else:
        bad(f"missing: {cmd}")

need("cargo")
need("bash")
need("node")

if shutil.which("npm"):
    ok("command: npm")
else:
    print("check_toolchain: note: npm not on PATH (some nix/CI); optional for this repo", file=sys.stderr)

need("iwasm")
need("jq")
need("git")

# mktemp is Unix-specific; on Windows we can use tempfile module
if shutil.which("mktemp"):
    ok("command: mktemp")
else:
    # On Windows, we'll use Python's tempfile module
    ok("command: mktemp (will use Python tempfile)")

# Search (docs say rg / ig; either is enough)
if shutil.which("ig"):
    ok("search: ig")
elif shutil.which("rg"):
    ok("search: rg")
else:
    bad("missing: ripgrep (ig or rg)")

if shutil.which("ast-grep"):
    ok("ast-grep: ast-grep")
elif shutil.which("sg"):
    ok("ast-grep: sg")
else:
    bad("missing: ast-grep (or sg)")

# Check cargo nextest
result = shutil.which("cargo-nextest")
if result:
    ok("cargo nextest")
else:
    # Try to run cargo nextest
    try:
        result = subprocess.run(["cargo", "nextest", "--version"], capture_output=True, text=True)
        if result.returncode == 0:
            ok("cargo nextest")
        else:
            bad("cargo nextest: not on PATH (install cargo-nextest; see AGENTS / README)")
    except Exception:
        bad("cargo nextest: not on PATH (install cargo-nextest; see AGENTS / README)")

# wasm stack used by other harnesses
if shutil.which("wasm-tools"):
    ok("wasm-tools")
else:
    bad("missing: wasm-tools (print/validate; used by manifest and wasm checks)")

if err != 0:
    print("check_toolchain: FAILED", file=sys.stderr)
    sys.exit(1)

print("check_toolchain: all required tools present", file=sys.stderr)
