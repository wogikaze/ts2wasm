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
    "check-tracking-consistency": ("python", "scripts/check/tracking-consistency.py"),
    "check-diagnostic-codes": ("python", "scripts/check/diagnostic-codes.py"),
    "check-complexity": ("python", "scripts/check/complexity.py"),
    "check-arch-dag": ("python", "scripts/check/check-arch-dag.py"),
    "check-runtimefn-deprecation": ("python", "scripts/check/check-runtimefn-deprecation.py"),
    "check-legacy-freeze": ("python", "scripts/check/legacy-freeze.py"),
    "check-specop-dispatch": ("python", "scripts/check/specop-dispatch.py"),
    "check-coverage-classification": ("python", "scripts/check/coverage-classification.py"),
    "check-trace-contract": ("python", "scripts/check/trace-contract.py"),
    "check-architecture-exceptions": ("python", "scripts/check/architecture-exceptions.py"),
    "check-docs-routing": ("python", "scripts/check/docs-routing.py"),
    "check-rustfmt-legacy-aware": ("python", "scripts/check/rustfmt-legacy-aware.py"),
    "check-compiler-source-truth": ("python", "scripts/check/compiler-source-truth.py"),
    "check-host-import-baseline": ("python", "scripts/check/host-import-baseline.py"),
    "check-host-import-boundary": ("python", "scripts/check/host-import-boundary.py"),
    "check-crate-dag": ("python", "scripts/check/check-arch-dag.py"),
    "issue-create": ("python", "scripts/issue-create.py"),
    "issue-index": ("python", "scripts/issue-index.py"),
    "update-issue-index": ("python", "scripts/issue-index.py"),
    "issue-lint": ("python", "scripts/issue-lint.py"),
    "issue-show": ("python", "scripts/issue-show.py"),
    "issue-status": ("python", "scripts/issue-status.py"),
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
    "reference-corpus": ("python", "scripts/run/reference-corpus.py"),
    "check-reference-coverage-triage": ("python", "scripts/check/reference-coverage-triage.py"),
    "check-coverage-runner-parity": ("python", "scripts/check/coverage-runner-parity.py"),
    "abc451-runtime-costs": ("python", "scripts/run/abc451-runtime-costs.py"),
    "repo-metrics": ("python", "scripts/run/repo-metrics.py"),
    "benchmark-tracker": ("python", "scripts/perf/benchmark-tracker.py"),
    "check-perf-regression": ("python", "scripts/gate/perf-regression.py"),
    "test-differential-reporter": ("python", "scripts/report/differential.py"),
    "development-report": ("python", "scripts/report/development-report.py"),
    "discord-report": ("python", "scripts/report/discord-report.py"),
    "native-emitter-unsupported": ("python", "scripts/report/native-emitter-unsupported.py"),
    "native-runtime-builder-coverage": ("python", "scripts/report/native-runtime-builder-coverage.py"),
    "gen-site": ("python", "scripts/gen-site.py"),
    "create-run-dir": ("python", "scripts/gen/create-run-dir.py"),
    "generate-differential-tests": ("python", "scripts/gen/generate-differential-tests.py"),
    # Next-architecture gate aliases
    "next-coverage-gate": ("python", "scripts/gate/coverage.py"),
    "next-capability-gate": ("python", "scripts/check/manifest-imports.py"),
    "next-rtgc-gate": ("python", "scripts/gate/fast-gate.py"),
    "next-reference-gate": ("python", "scripts/run/reference-corpus.py"),
    "next-abi-gate": ("python", "scripts/check/architecture-rules.py"),
    "next-architecture-gate": ("python", "scripts/gate/fast-gate.py"),
    "fmt": ("python", "scripts/check/rustfmt-legacy-aware.py"),
    "clippy": ("cargo", "clippy --all-targets -- -D warnings"),
    "nextest": ("cargo", "nextest run"),
}

CHECK_ALL_PARTS = [
    "scripts", "manifest", "records", "fixtures",
    "architecture", "diagnostics", "coverage",
    "toolchain", "ast-grep", "host",
    "runtimefn", "wasm", "assert-true",
    "tracking", "issues", "native-runtime-builder",
    "host-baseline", "host-boundary",
    "diagnostic-codes", "complexity", "runtimefn-deprecation", "arch-dag", "legacy-freeze", "specop-dispatch", "trace-contract", "architecture-exceptions", "docs-routing", "rustfmt-legacy-aware", "compiler-source-truth",
    "crate-dag",
]

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
    "native-runtime-builder": "native-runtime-builder-coverage",
    "native-runtime-builder-coverage": "native-runtime-builder-coverage",
    "wasm": "check-wasm-validation",
    "wasm-validation": "check-wasm-validation",
    "assert-true": "check-assert-true-detect",
    "assert-true-detect": "check-assert-true-detect",
    "host-baseline": "check-host-import-baseline",
    "host-import-baseline": "check-host-import-baseline",
    "host-boundary": "check-host-import-boundary",
    "host-import-boundary": "check-host-import-boundary",
    "tracking": "check-tracking-consistency",
    "tracking-consistency": "check-tracking-consistency",
    "issues": "issue-lint",
    "issue-lint": "issue-lint",
    "triage": "check-reference-coverage-triage",
    "reference-coverage-triage": "check-reference-coverage-triage",
    "coverage-parity": "check-coverage-runner-parity",
    "coverage-runner-parity": "check-coverage-runner-parity",
    "reference-lock": "next-reference-gate",
    "reference-subsets": "next-reference-gate",
    "evidence-check": "next-reference-gate",
    "replay-set": "next-reference-gate",
    "diagnostic-codes": "check-diagnostic-codes",
    "complexity": "check-complexity",
    "arch-dag": "check-arch-dag",
    "legacy-freeze": "check-legacy-freeze",
    "trace-contract": "check-trace-contract",
    "architecture-exceptions": "check-architecture-exceptions",
    "docs-routing": "check-docs-routing",
    "rustfmt-legacy-aware": "check-rustfmt-legacy-aware",
    "compiler-source-truth": "check-compiler-source-truth",
    "specop-dispatch": "check-specop-dispatch",
    "runtimefn-deprecation": "check-runtimefn-deprecation",
    "crate-dag": "check-crate-dag",
    "crate-dag-check": "check-crate-dag",
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
        ("issue-create", "Create a repo-local issue file"),
        ("issue-index", "Generate issue-views/index.json"),
        ("issue-lint", "Validate issue files"),
        ("issue-show", "Show an issue by id, legacy id, partial id, or title word"),
        ("issue-status", "Change issue status and optionally append evidence"),
        ("update-coverage-matrix", "Refresh reference coverage table"),
        ("reference-corpus", "Reference corpus lock verification and management"),
        ("reference-coverage", "Reference suite coverage runner"),
        ("reference-triage", "Rich single-case reference diagnostic report"),
        ("abc451-runtime-costs", "Default-off ABC451 depth-8 runtime cost diagnostic"),
        ("repo-metrics", "Repository line, byte, and content-kind metrics"),
        ("verify", "Full verify: fmt + toolchain + complexity + docs + architecture"),
        ("verify --quick", "Quick verify: fmt + syntax + complexity (quick) + toolchain"),
        ("verify --size", "Size verify: full complexity analysis"),
        ("verify --docs", "Docs verify: diagnostic code alignment"),
        ("check-diagnostic-codes", "Diagnostic code ↔ docs alignment check"),
        ("check-complexity", "Rust code complexity metrics (cyclomatic, nesting, args)"),
        ("check-ast-grep", "Run ast-grep rule tests and repository scan"),
        ("check-host-deny", "Host import deny matrix and policy checker"),
        ("check-host-import-baseline", "Host import baseline checker"),
        ("check-host-import-boundary", "Host import and target string boundary checker"),
        ("coverage-report", "Language coverage report from language-reference"),
        ("coverage-dashboard-data", "Generate coverage dashboard JSON from coverage artifacts"),
        ("benchmark-tracker", "Performance metrics JSON"),
        ("test262", "[use 'mise run test262' instead — alias for reference-coverage test262 --jsonl]"),
        ("test-differential-reporter", "Report from test262 JSONL (stdin)"),
        ("development-report", "Generate a concise Japanese development report"),
        ("discord-report", "Send a report markdown/json file to Discord webhook"),
        ("native-runtime-builder-coverage", "Report native RuntimeFn builder coverage"),
        ("create-run-dir", "Create reports/runs/<run_id>/ directory"),
        ("generate-differential-tests", "Generate Rust differential test code from fixture catalog"),
        ("next-coverage-gate", "REQ-COV-001/002 gates (outcome schema + top-reasons)"),
        ("next-capability-gate", "REQ-CAP-001/002 gates (link-plan validation + manifest parity)"),
        ("next-rtgc-gate", "REQ-RTGC-001/002/003/004/005/006 gates (typed alloc, scanner, roots, thresholds, layout, closures)"),
        ("next-reference-gate", "REQ-REF-001/002 gates (lock manifest + lock identity validation)"),
        ("next-abi-gate", "REQ-ABI-001/002 gates (ABI metadata export + TargetSpec)"),
        ("next-architecture-gate", "Composite of all 5 next-architecture theme gates"),
        ("fmt", "rustfmt legacy-aware check"),
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
    """Run the lightweight repository smoke check (CI entry point)."""
    run_sequence([
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/rustfmt-legacy-aware.py")],
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/shell-syntax.py")],
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/check-arch-dag.py")],
        [PYTHON_BIN, str(REPO_ROOT / "scripts/check/architecture-rules.py")],
    ])

def configure_reference_coverage_defaults(target, args):
    """Apply audited defaults for manager-launched semantic coverage."""
    if target != "reference-coverage":
        return
    if not args:
        return
    if args[0] == "test262":
        if "--no-semantic" in args:
            return
        if "--semantic" in args:
            idx = args.index("--semantic")
            if idx + 1 < len(args) and args[idx + 1] == "fast":
                return
        os.environ.setdefault("TS2WASM_TEST262_NODE_ORACLE", "always")
        os.environ.setdefault("TS2WASM_DISABLE_TEST262_PREPROCESSOR_STUBS", "1")
    elif args[0] in ("tsc", "tsgo"):
        # tsc/tsgo need explicit --semantic to enable oracle (no default)
        if "--semantic" in args:
            os.environ.setdefault("TS2WASM_TEST262_NODE_ORACLE", "always")

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
    if args and args[0] == "--":
        args = args[1:]

    if target == "gate":
        run_command("python", "scripts/gate/fast-gate.py", args)

    if target == "gate-fast":
        run_command("python", "scripts/gate/fast-gate.py", ["--skip-nextest"] + args)

    if target == "gate-all":
        run_command("python", "scripts/check/harness-installation.py", args)

    if target == "verify":
        verify_mode = args[0] if args else "full"
        verify_args = args[1:] if args and args[0] == verify_mode else []
        quick_cmds = [
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/shell-syntax.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/complexity.py"), "--quick", "--exclude-legacy"],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/check-arch-dag.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/toolchain.py")],
        ]
        full_cmds = [
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/rustfmt-legacy-aware.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/shell-syntax.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/complexity.py"), "--full"],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/toolchain.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/diagnostic-codes.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/check-arch-dag.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/tracking-consistency.py")],
            [PYTHON_BIN, str(REPO_ROOT / "scripts/check/architecture-rules.py")],
        ]

        if verify_mode == "--quick":
            run_sequence(quick_cmds)
        elif verify_mode == "--size":
            run_sequence([
                [PYTHON_BIN, str(REPO_ROOT / "scripts/check/complexity.py"), "--full"],
            ])
        elif verify_mode == "--docs":
            run_sequence([
                [PYTHON_BIN, str(REPO_ROOT / "scripts/check/diagnostic-codes.py")],
            ])
        else:
            run_sequence(full_cmds)

    if target == "check":
        if not args:
            run_repo_smoke()
        if args[0] in ("-h", "--help"):
            check_usage()
            sys.exit(0)
        if args[0] == "all":
            cmds = []
            extra = args[1:]
            for name in CHECK_ALL_PARTS:
                part_cmd = CHECK_PARTS[name]
                script_type, script_info = COMMANDS[part_cmd]
                if script_type == "python":
                    cmds.append([PYTHON_BIN, str(REPO_ROOT / script_info)] + extra)
                elif script_type == "cargo":
                    cmds.append(["cargo"] + script_info.split() + extra)
                else:
                    cmds.append(["bash", str(REPO_ROOT / script_info)] + extra)
            run_sequence(cmds)
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
