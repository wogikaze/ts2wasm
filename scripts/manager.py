#!/usr/bin/env python3
"""ts2wasm script manager - cross-platform entry point for development scripts.

Usage: python scripts/manager.py <command> [args...]

This manager dispatches to the appropriate script (Python or bash) based on the command.
"""

import os
import shutil
import sys
import platform
import subprocess
from pathlib import Path

# Repository root
REPO_ROOT = Path(__file__).parent.parent.resolve()
PYTHON_BIN = os.environ.get("PYTHON_BIN", sys.executable)

# Command mapping: command -> (script_type, script_path, interpreter)
COMMANDS = {
    "check-scripts": ("python", "scripts/check/shell-syntax.py"),
    "check-fast-gate": ("python", "scripts/gate/fast-gate.py"),
    "check-manifest-imports": ("python", "scripts/check/manifest-imports.py"),
    "check-test-records-schema": ("python", "scripts/check/test-records-schema.py"),
    "check-fixture-catalog": ("python", "scripts/check/fixture-catalog.py"),
    "check-architecture-rules": ("python", "scripts/check/architecture-rules.py"),
    "check-compiler-diagnostics": ("python", "scripts/check/compiler-diagnostics.py"),
    "check-harness-installation": ("python", "scripts/check/harness-installation.py"),
    "check-toolchain": ("python", "scripts/check/toolchain.py"),
    "check-fixture-differential": ("python", "scripts/check/fixture-differential.py"),
    "check-host-deny": ("python", "scripts/check/host-deny.py"),
    "check-runtimefn-invariants": ("python", "scripts/check/runtimefn-invariants.py"),
    "check-wasm-validation": ("python", "scripts/check/wasm-validation.py"),
    "check-agent-state": ("python", "scripts/check/agent-state.py"),
    "check-issue-index": ("python", "scripts/check/issue-health.py"),
    "check-issue-health": ("python", "scripts/check/issue-health.py"),
    "check-issue-readiness": ("python", "scripts/check/issue-readiness.py"),
    "update-issue-index": ("python", "scripts/gen/update-issue-index.py"),
    "install-hooks": ("bash", "scripts/dev/install-git-hooks.sh"),
    "check-coverage-gate": ("python", "scripts/gate/coverage.py"),
    "update-coverage-matrix": ("python", "scripts/gen/coverage-matrix.py"),
    "coverage-report": ("python", "scripts/gen/coverage-report.py"),
    "reference-coverage": ("python", "scripts/run/reference-coverage.py"),
    "benchmark-tracker": ("python", "scripts/perf/benchmark-tracker.py"),
    "test262": ("python", "scripts/run/test262.py"),
    "test-differential-reporter": ("python", "scripts/report/differential.py"),
    "test-regression-gate": ("python", "scripts/gate/regression.py"),
    "gen-issues-from-coverage": ("python", "scripts/gen/issues-from-coverage.py"),
    "create-run-dir": ("python", "scripts/gen/create-run-dir.py"),
    "discord-report": ("python", "scripts/report/discord-report.py"),
    "fmt": ("cargo", "fmt --all --check"),
    "clippy": ("cargo", "clippy --all-targets -- -D warnings"),
    "nextest": ("cargo", "nextest run"),
}

def usage():
    """Print usage information."""
    print("ts2wasm — script manager (one entry; arguments pass through to the underlying script)")
    print()
    print("Usage:")
    print("  python scripts/manager.py [help]")
    print("  python scripts/manager.py <command> [args...]")
    print()
    print("Examples:")
    print("  python scripts/manager.py check-issue-health")
    print("  python scripts/manager.py update-issue-index --check")
    print("  python scripts/manager.py nextest -- --no-fail-fast")
    print()
    print("Commands:")
    
    # Format command list
    cmd_list = [
        ("check-scripts", "Bash -n on scripts/*.sh (syntax)"),
        ("check-fast-gate", "fmt + issues + architecture + coverage matrix + nextest"),
        ("check-manifest-imports", "Manifest JSON imports vs wasm import section"),
        ("check-test-records-schema", "Validate TestRecord JSONL lines"),
        ("check-fixture-catalog", "Fixtures/ top-level layout rules"),
        ("check-architecture-rules", "Lightweight crate boundary and file-size checks"),
        ("check-compiler-diagnostics", "No panic! in backend/runtime/main.rs"),
        ("check-harness-installation", "Full harness baseline: P0 tools + nextest"),
        ("check-toolchain", "Verify rust/node/iwasm/wasm-tools exist"),
        ("check-fixture-differential", "Node vs iwasm: runs nextest m2_node_diff"),
        ("check-host-deny", "Standalone fixtures must not emit wasm (import \"host\")"),
        ("check-runtimefn-invariants", "Unit tests: runtime_link_plan invariants"),
        ("check-wasm-validation", "Build sample fixtures; wasm-tools validate"),
        ("check-agent-state", "Validate .agents/state JSON files against schemas"),
        ("check-issue-index", "Fail if issues/index.md is stale"),
        ("check-issue-health", "Mechanical invariants: ids, paths, index+tables"),
        ("check-issue-readiness", "Score open issues for actionability and measurement quality"),
        ("update-issue-index", "Regenerate index tables (add --check to verify only)"),
        ("install-hooks", "Install .githooks via git config core.hooksPath"),
        ("check-coverage-gate", "Compare two coverage matrix docs"),
        ("update-coverage-matrix", "Refresh reference coverage table"),
        ("reference-coverage", "Reference suite coverage runner"),
        ("coverage-report", "Language coverage report from language-reference"),
        ("benchmark-tracker", "Performance metrics JSON"),
        ("test262", "test262 JSONL to stdout"),
        ("test-differential-reporter", "Report from test262 JSONL (stdin)"),
        ("test-regression-gate", "JSONL vs baseline"),
        ("gen-issues-from-coverage", "Generate issues from reference-coverage --detail"),
        ("create-run-dir", "Create reports/runs/<run_id>/ directory"),
        ("discord-report", "Send a Markdown report or Discord JSON payload to Discord"),
        ("fmt", "cargo fmt --all --check"),
        ("clippy", "cargo clippy --all-targets -- -D warnings"),
        ("nextest", "cargo nextest run"),
        ("check-repo-smoke", "cargo fmt + check-scripts + check-issue-health"),
    ]
    
    max_cmd_len = max(len(cmd) for cmd, _ in cmd_list)
    for cmd, desc in cmd_list:
        print(f"  {cmd:<{max_cmd_len}}  {desc}")
    
    print()
    print("Mise: run `mise tasks` / `mise run <task>` for the same set (mise optional).")
    print("Scripts: scripts/{check,gate,gen,run,...}/* and scripts/manager.py; this dispatches to them.")

def run_command(script_type, script_path, args):
    """Execute a command based on its type."""
    full_path = REPO_ROOT / script_path
    
    if script_type == "bash":
        if not shutil.which("bash"):
            print("Error: bash not found on PATH", file=sys.stderr)
            sys.exit(1)
        cmd = ["bash", str(full_path)] + args
    elif script_type == "python":
        cmd = [PYTHON_BIN, str(full_path)] + args
    elif script_type == "cargo":
        cmd = ["cargo"] + script_path.split() + args
    else:
        print(f"Error: unknown script type: {script_type}", file=sys.stderr)
        sys.exit(1)
    
    # Execute command
    result = subprocess.run(cmd, cwd=REPO_ROOT)
    sys.exit(result.returncode)

def main():
    """Main entry point."""
    if len(sys.argv) < 2 or sys.argv[1] in ("help", "-h", "--help"):
        usage()
        sys.exit(0)
    
    target = sys.argv[1]
    args = sys.argv[2:]
    
    # Special case: check-repo-smoke is a composite command
    if target == "check-repo-smoke":
        # Run cargo fmt
        result = subprocess.run(["cargo", "fmt", "--all", "--check"], cwd=REPO_ROOT)
        if result.returncode != 0:
            sys.exit(result.returncode)
        
        # Run check-scripts (Python version)
        result = subprocess.run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/shell-syntax.py")], cwd=REPO_ROOT)
        if result.returncode != 0:
            sys.exit(result.returncode)
        
        # Run check-issue-health
        result = subprocess.run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/issue-health.py")], cwd=REPO_ROOT)
        sys.exit(result.returncode)
    
    # Look up command
    if target not in COMMANDS:
        print(f"Unknown command: {target}", file=sys.stderr)
        print("Run: python scripts/manager.py help", file=sys.stderr)
        sys.exit(1)
    
    # Special case: install-hooks uses different scripts on Windows vs Unix
    if target == "install-hooks":
        if platform.system() == "Windows":
            script_type = "python"
            script_info = "scripts/dev/install-git-hooks.py"
        else:
            script_type, script_info = COMMANDS[target]
    else:
        script_type, script_info = COMMANDS[target]
    
    # For cargo commands, script_info is the full cargo command
    if script_type == "cargo":
        run_command("cargo", script_info, args)
    else:
        run_command(script_type, script_info, args)

if __name__ == "__main__":
    main()
