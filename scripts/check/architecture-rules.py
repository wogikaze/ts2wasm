#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: mise run check architecture

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - crates/cli/src/backend must not be reintroduced after backend-wasm extraction.
  - crates/cli/src must not declare local backend/parser/compiler implementation modules.
  - Error when a repo-owned source/document file exceeds the documented line limit.
  - RuntimeFn import/capability parity: every RuntimeFn with imports must have a capability marker and vice versa.
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_MAX_FILE_LINES = 4100
LINE_COUNT_SUFFIXES = {
    ".md",
    ".py",
    ".rs",
    ".sh",
    ".toml",
    ".yaml",
    ".yml",
}
EXCLUDED_PATH_PARTS = {
    ".claude",
    ".git",
    ".mypy_cache",
    "__pycache__",
    "artifacts",
    "node_modules",
    "reference",
    "reports",
    "target",
    "_worktrees",
}
EXCLUDED_FILENAMES = {
    "Cargo.lock",
}


def usage():
    print("Usage:")
    print("  mise run check architecture -- [--max-file-lines N]")
    print()
    print("Current checks:")
    print("  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).")
    print("  - crates/cli/src/backend must not be reintroduced.")
    print("  - crates/cli/src must not declare local backend/parser/compiler modules.")
    print("  - Error when a repo-owned source/document file exceeds the line limit.")
    print("  - RuntimeFn import/capability parity: every RuntimeFn with imports must have a capability marker and vice versa.")


def parse_max_file_lines(args: list[str]) -> int:
    raw_max_file_lines = os.environ.get("TS2WASM_MAX_FILE_LINES", str(DEFAULT_MAX_FILE_LINES))
    try:
        max_file_lines = int(raw_max_file_lines)
    except ValueError:
        print("check_architecture_rules: TS2WASM_MAX_FILE_LINES must be an integer", file=sys.stderr)
        sys.exit(1)

    while args:
        arg = args.pop(0)
        if arg in ("-h", "--help"):
            usage()
            sys.exit(0)
        if arg == "--max-file-lines":
            if not args:
                print("check_architecture_rules: --max-file-lines requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                max_file_lines = int(args.pop(0))
            except ValueError:
                print("check_architecture_rules: --max-file-lines must be an integer", file=sys.stderr)
                sys.exit(1)
        else:
            print(f"check_architecture_rules: unknown option: {arg}", file=sys.stderr)
            usage()
            sys.exit(1)

    if max_file_lines < 1:
        print("check_architecture_rules: --max-file-lines must be >= 1", file=sys.stderr)
        sys.exit(1)
    return max_file_lines


def should_count_lines(path: Path) -> bool:
    relative = path.relative_to(REPO_ROOT)
    if path.name in EXCLUDED_FILENAMES:
        return False
    if any(part in EXCLUDED_PATH_PARTS for part in relative.parts):
        return False
    if relative == Path("TRACKING.yaml"):
        return False
    return path.suffix in LINE_COUNT_SUFFIXES


def line_count(path: Path) -> int:
    data = path.read_bytes()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def find_oversized_files(max_file_lines: int) -> list[tuple[int, Path]]:
    """Return list of (count, relative_path) for files exceeding the line limit."""
    oversized: list[tuple[int, Path]] = []
    for path in REPO_ROOT.rglob("*"):
        if not path.is_file() or not should_count_lines(path):
            continue
        count = line_count(path)
        if count > max_file_lines:
            oversized.append((count, path.relative_to(REPO_ROOT)))
    return oversized


def find_cli_boundary_violations() -> list[str]:
    """Return list of violation messages for CLI thin-wrapper boundary checks."""
    violations: list[str] = []
    cli_src = REPO_ROOT / "crates" / "cli" / "src"
    backend_dir = cli_src / "backend"
    if backend_dir.exists():
        violations.append(
            "crates/cli/src/backend must not be reintroduced; "
            "put WASM backend implementation under crates/backend-wasm/src"
        )

    forbidden_module_names = ("backend", "parser", "compiler", "driver")
    for path in cli_src.glob("*.rs"):
        text = path.read_text()
        for module_name in forbidden_module_names:
            if f"mod {module_name};" in text:
                violations.append(
                    f"{path.relative_to(REPO_ROOT)} must not declare "
                    f"mod {module_name}; keep compiler implementation outside crates/cli"
                )
        if "struct Lexer" in text or "struct Parser" in text:
            violations.append(
                f"{path.relative_to(REPO_ROOT)} must not define "
                "parser implementation types; keep parser/compiler implementation outside crates/cli"
            )

    for module_name in forbidden_module_names:
        module_file = cli_src / f"{module_name}.rs"
        if module_file.exists():
            violations.append(
                f"{module_file.relative_to(REPO_ROOT)} must not exist; "
                "crates/cli is a thin wrapper"
            )

    cli_lib = cli_src / "lib.rs"
    if cli_lib.exists() and "ts2wasm_backend_wasm" in cli_lib.read_text():
        violations.append(
            "crates/cli/src/lib.rs must not call backend directly; "
            "use ts2wasm-compiler instead"
        )

    return violations


def main():
    args = sys.argv[1:]
    max_file_lines = parse_max_file_lines(args)
    errors: list[str] = []

    # --- check 1: oversized files ---
    oversized = find_oversized_files(max_file_lines)
    for count, path in sorted(oversized, key=lambda item: (-item[0], item[1])):
        errors.append(
            f"{path}: {count} lines exceeds {max_file_lines}; "
            "split ownership/modules or raise the documented limit"
        )

    # --- check 2: CLI thin wrapper boundary ---
    errors.extend(find_cli_boundary_violations())

    # --- check 3: shared depends on cli ---
    if shutil.which("cargo"):
        result = subprocess.run(
            ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
            capture_output=True,
            text=True,
            cwd=REPO_ROOT,
        )
        if result.returncode == 0 and "ts2wasm-cli" in result.stdout:
            errors.append("ts2wasm-shared must not depend on ts2wasm-cli")
    else:
        errors.append("cargo is required")

    # --- check 4: import-capability parity ---
    if shutil.which("cargo"):
        result = subprocess.run(
            [
                "cargo",
                "test",
                "-p",
                "ts2wasm-backend-wasm",
                "--lib",
                "--",
                "import_capability_parity",
            ],
            capture_output=True,
            text=True,
            cwd=REPO_ROOT,
        )
        if result.returncode != 0:
            errors.append("RuntimeFn import/capability parity check FAILED")
            if result.stderr:
                errors.append(result.stderr[:500])
            if result.stdout:
                errors.append(result.stdout[:500])
    else:
        errors.append("cargo is required for import-capability parity check")

    if errors:
        for msg in errors:
            print(f"check_architecture_rules: ERROR: {msg}", file=sys.stderr)
        sys.exit(1)

    print("check_architecture_rules: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
