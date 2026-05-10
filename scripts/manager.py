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
    "check-ast-grep": ("python", "scripts/check/ast-grep.py"),
    "check-fixture-differential": ("python", "scripts/check/fixture-differential.py"),
    "check-host-deny": ("python", "scripts/check/host-deny.py"),
    "check-assert-true-detect": ("python", "scripts/check/assert-true-detect.py"),
    "check-runtimefn-invariants": ("python", "scripts/check/runtimefn-invariants.py"),
    "check-wasm-validation": ("python", "scripts/check/wasm-validation.py"),
    "install-hooks": ("bash", "scripts/dev/install-git-hooks.sh"),
    "link-reference": ("python", "scripts/dev/link-reference.py"),
    "spawn-worktrees": ("bash", "scripts/dev/spawn-worktrees.sh"),
    "worktree-status": ("bash", "scripts/dev/worktree-batch-status.sh"),
    "git-worktree": ("bash", "scripts/dev/git-worktree.sh"),
    "check-coverage-gate": ("python", "scripts/gate/coverage.py"),
    "update-coverage-matrix": ("python", "scripts/gen/coverage-matrix.py"),
    "coverage-report": ("python", "scripts/gen/coverage-report.py"),
    "coverage-dashboard-data": ("python", "scripts/gen/web-ui-data.py"),
    "reference-coverage": ("python", "scripts/run/reference-coverage.py"),
    "reference-triage": ("python", "scripts/run/reference-triage.py"),
    "abc451-runtime-costs": ("python", "scripts/run/abc451-runtime-costs.py"),
    "repo-metrics": ("python", "scripts/run/repo-metrics.py"),
    "benchmark-tracker": ("python", "scripts/perf/benchmark-tracker.py"),
    "test-differential-reporter": ("python", "scripts/report/differential.py"),
    "gen-site": ("python", "scripts/gen-site.py"),
    "create-run-dir": ("python", "scripts/gen/create-run-dir.py"),
    "fmt": ("cargo", "fmt --all --check"),
    "clippy": ("cargo", "clippy --all-targets -- -D warnings"),
    "nextest": ("cargo", "nextest run"),
}

CHECK_PARTS = {
    "scripts": "check-scripts",
    "shell-syntax": "check-scripts",
    "manifest": "check-manifest-imports",
    "manifest-imports": "check-manifest-imports",
    "records": "check-test-records-schema",
    "test-records": "check-test-records-schema",
    "test-records-schema": "check-test-records-schema",
    "fixtures": "check-fixture-catalog",
    "fixture-catalog": "check-fixture-catalog",
    "architecture": "check-architecture-rules",
    "architecture-rules": "check-architecture-rules",
    "diagnostics": "check-compiler-diagnostics",
    "compiler-diagnostics": "check-compiler-diagnostics",
    "coverage": "check-coverage-gate",
    "coverage-gate": "check-coverage-gate",
    "toolchain": "check-toolchain",
    "ast-grep": "check-ast-grep",
    "astgrep": "check-ast-grep",
    "differential": "check-fixture-differential",
    "fixture-differential": "check-fixture-differential",
    "host": "check-host-deny",
    "host-deny": "check-host-deny",
    "runtimefn": "check-runtimefn-invariants",
    "runtimefn-invariants": "check-runtimefn-invariants",
    "wasm": "check-wasm-validation",
    "wasm-validation": "check-wasm-validation",
    "assert-true": "check-assert-true-detect",
    "assert-true-detect": "check-assert-true-detect",
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
    print("  python scripts/manager.py gate")
    print("  python scripts/manager.py gate-fast")
    print("  python scripts/manager.py check")
    print("  python scripts/manager.py nextest -- --no-fail-fast")
    print()
    print("Commands:")
    
    # Format command list
    cmd_list = [
        ("gate", "Standard gate: fmt + architecture + coverage matrix + nextest"),
        ("gate-fast", "Fast gate: standard gate without nextest"),
        ("gate-all", "Full gate: harness/toolchain baseline plus project gates"),
        ("check", "Run check-repo-smoke, or run a part with `check <part>`"),
        ("install-hooks", "Install .githooks via git config core.hooksPath"),
        ("link-reference", "Symlink ignored reference/ corpus into worktrees"),
        ("spawn-worktrees", "Create child worktrees and local assignment files"),
        ("worktree-status", "Collect status from child worktrees"),
        ("git-worktree", "Small git worktree helper"),
        ("update-coverage-matrix", "Refresh reference coverage table"),
        ("reference-coverage", "Reference suite coverage runner"),
        ("reference-triage", "Rich single-case reference diagnostic report"),
        ("abc451-runtime-costs", "Default-off ABC451 depth-8 runtime cost diagnostic"),
        ("repo-metrics", "Repository line, byte, and content-kind metrics"),
        ("check-ast-grep", "Run ast-grep rule tests and repository scan"),
        ("coverage-report", "Language coverage report from language-reference"),
        ("coverage-dashboard-data", "Generate coverage dashboard JSON from coverage artifacts"),
        ("benchmark-tracker", "Performance metrics JSON"),
        ("test262", "[use 'mise run test262' instead — alias for reference-coverage test262 --jsonl]"),
        ("test-differential-reporter", "Report from test262 JSONL (stdin)"),
        ("create-run-dir", "Create reports/runs/<run_id>/ directory"),
        ("fmt", "cargo fmt --all --check"),
        ("clippy", "cargo clippy --all-targets -- -D warnings"),
        ("nextest", "cargo nextest run"),
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
        parts = script_path.split()
        cmd = [PYTHON_BIN, str(REPO_ROOT / parts[0])] + parts[1:] + args
    elif script_type == "cargo":
        cmd = ["cargo"] + script_path.split() + args
    else:
        print(f"Error: unknown script type: {script_type}", file=sys.stderr)
        sys.exit(1)
    
    # Execute command
    result = subprocess.run(cmd, cwd=REPO_ROOT)
    sys.exit(result.returncode)

def run_sequence(commands):
    """Run commands in order and stop at the first failure."""
    for cmd in commands:
        print(f"manager: {' '.join(cmd)}", file=sys.stderr)
        result = subprocess.run(cmd, cwd=REPO_ROOT)
        if result.returncode != 0:
            sys.exit(result.returncode)
    sys.exit(0)

def run_repo_smoke():
    """Run the lightweight repository smoke check."""
    run_sequence([
        ["cargo", "fmt", "--all", "--check"],
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/shell-syntax.py")],
    ])

def configure_reference_coverage_defaults(target, args):
    """Apply audited defaults for manager-launched test262 semantic coverage."""
    if target != "reference-coverage":
        return
    if not args or args[0] != "test262":
        return
    if "--no-semantic" in args:
        return

    os.environ.setdefault("TS2WASM_TEST262_NODE_ORACLE", "always")
    os.environ.setdefault("TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS", "1")

def check_usage():
    parts = ", ".join(sorted(CHECK_PARTS))
    print("Usage: python scripts/manager.py check [part] [args...]")
    print()
    print("No part runs check-repo-smoke.")
    print(f"Parts: {parts}")

def main():
    """Main entry point."""
    if len(sys.argv) < 2 or sys.argv[1] in ("help", "-h", "--help"):
        usage()
        sys.exit(0)
    
    target = sys.argv[1]
    args = sys.argv[2:]

    if target == "gate":
        run_command("python", "scripts/gate/fast-gate.py", args)

    if target == "gate-fast":
        run_command("python", "scripts/gate/fast-gate.py", ["--skip-nextest"] + args)

    if target == "gate-all":
        run_command("python", "scripts/check/harness-installation.py", args)

    if target == "check":
        if not args:
            run_repo_smoke()
        if args[0] in ("-h", "--help"):
            check_usage()
            sys.exit(0)
        part = args[0]
        if part not in CHECK_PARTS:
            print(f"Unknown check part: {part}", file=sys.stderr)
            check_usage()
            sys.exit(1)
        target = CHECK_PARTS[part]
        args = args[1:]
        if args and args[0] == "--":
            args = args[1:]
    
    # Special case: check-repo-smoke is a composite command
    if target == "check-repo-smoke":
        run_repo_smoke()
    
    # Look up command
    if target not in COMMANDS:
        print(f"Unknown command: {target}", file=sys.stderr)
        print("Run: python scripts/manager.py help", file=sys.stderr)
        sys.exit(1)

    configure_reference_coverage_defaults(target, args)
    
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
