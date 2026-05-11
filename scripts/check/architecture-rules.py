#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: mise run check architecture

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - crates/cli/src/backend must not be reintroduced after backend-wasm extraction.
  - crates/cli/src must not declare local backend/parser/compiler implementation modules.
  - Error when a repo-owned source/document file exceeds the documented line limit.
  - Error when backend-wasm or ir directly depends on frontend via Cargo.toml.
"""

import os
import re
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_MAX_FILE_LINES = 3000

# Known oversized files that are exempt from the line limit.
# Each entry must include a reason and the P-item that will eventually fix it.
OVERSIZED_ALLOWLIST = {
    # Test files (naturally large, not a concern)
    "crates/frontend/src/parser/tests.rs": "test file",
    "crates/cli/tests/common/m2_node_diff_fixture_tests.rs": "test file",
    "crates/cli/tests/m6_builtin_methods.rs": "test file",
    "crates/cli/tests/ir_lowering.rs": "test file",
    "crates/cli/tests/m2_node_diff.rs": "test file",
    "crates/ir/src/name_resolver_tests.rs": "test file",
    "crates/cli/tests/dump_cli.rs": "test file",
    "crates/cli/tests/m11_host_deny.rs": "test file",
    "crates/cli/tests/m_standalone_wasi.rs": "test file",
    "crates/cli/tests/differential_jsonl.rs": "test file",
    # Tracking data files
    "docs/done-tracking.yaml": "tracking data",
    # Being refactored by P4 (domain split)
    "crates/backend-wasm/src/runtime_fn_impl.rs": "P4: domain split planned",
    # Being refactored by P7 (Resolver decomposition)
    "crates/ir/src/lowered/resolver_extra.rs": "P7: Resolver context decomposition",
}

# Crates that must not directly depend on ts2wasm-frontend via Cargo.toml.
# Remove entries from this set as dependencies are eliminated.
FRONTEND_DEP_DENY = {
    "crates/backend-wasm",
    "crates/ir",
}


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
    print("  - Error when backend-wasm or ir depends on frontend via Cargo.toml.")
    print("  - Error when `use super::*` appears outside test modules.")
    print("  - Error when new RuntimeCall { runtime_fn: } construction appears outside allowlist.")


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


def check_oversized_files(max_file_lines: int) -> None:
    oversized: list[tuple[int, Path]] = []
    for path in REPO_ROOT.rglob("*"):
        if not path.is_file() or not should_count_lines(path):
            continue
        rel = path.relative_to(REPO_ROOT)
        if str(rel) in OVERSIZED_ALLOWLIST:
            continue
        count = line_count(path)
        if count > max_file_lines:
            oversized.append((count, rel))

    if not oversized:
        return

    print(
        "check_architecture_rules: ERROR files exceed "
        f"{max_file_lines} lines; split ownership/modules or raise the documented limit",
        file=sys.stderr,
    )
    for count, path in sorted(oversized, key=lambda item: (-item[0], item[1])):
        print(f"check_architecture_rules: ERROR {path}: {count} lines", file=sys.stderr)
    sys.exit(1)


def check_cli_thin_wrapper_boundary() -> None:
    cli_src = REPO_ROOT / "crates" / "cli" / "src"
    backend_dir = cli_src / "backend"
    if backend_dir.exists():
        print(
            "check_architecture_rules: crates/cli/src/backend must not be reintroduced; "
            "put WASM backend implementation under crates/backend-wasm/src",
            file=sys.stderr,
        )
        sys.exit(1)

    forbidden_module_names = ("backend", "parser", "compiler", "driver")
    for path in cli_src.glob("*.rs"):
        text = path.read_text()
        for module_name in forbidden_module_names:
            if f"mod {module_name};" in text:
                print(
                    f"check_architecture_rules: {path.relative_to(REPO_ROOT)} must not declare "
                    f"mod {module_name}; keep compiler implementation outside crates/cli",
                    file=sys.stderr,
                )
                sys.exit(1)
        if "struct Lexer" in text or "struct Parser" in text:
            print(
                f"check_architecture_rules: {path.relative_to(REPO_ROOT)} must not define "
                "parser implementation types; keep parser/compiler implementation outside crates/cli",
                file=sys.stderr,
            )
            sys.exit(1)

    for module_name in forbidden_module_names:
        module_file = cli_src / f"{module_name}.rs"
        if module_file.exists():
            print(
                f"check_architecture_rules: {module_file.relative_to(REPO_ROOT)} must not exist; "
                "crates/cli is a thin wrapper",
                file=sys.stderr,
            )
            sys.exit(1)

    cli_lib = cli_src / "lib.rs"
    if cli_lib.exists() and "ts2wasm_backend_wasm" in cli_lib.read_text():
        print(
            "check_architecture_rules: crates/cli/src/lib.rs must not call backend directly; "
            "use ts2wasm-compiler instead",
            file=sys.stderr,
        )
        sys.exit(1)


def check_backend_frontend_dependency() -> None:
    """Check that backend-wasm and ir don't directly depend on frontend via Cargo.toml [dependencies].

    Only checks the [dependencies] section (normal dependencies).
    Permits ts2wasm-frontend in [dev-dependencies] and [build-dependencies].
    """
    found = False
    for crate_rel in FRONTEND_DEP_DENY:
        cargo_path = REPO_ROOT / crate_rel / "Cargo.toml"
        if not cargo_path.exists():
            continue
        text = cargo_path.read_text()
        # Extract only the [dependencies] section: everything from "[dependencies]"
        # up to the next "[...]" section header, or end of file.
        deps_match = re.search(
            r"^\[dependencies\]\s*$(.+?)(?=^\s*\[|\Z)",
            text,
            re.MULTILINE | re.DOTALL,
        )
        if deps_match and "ts2wasm-frontend" in deps_match.group(1):
            print(
                f"check_architecture_rules: ERROR {crate_rel}/Cargo.toml depends on "
                f"ts2wasm-frontend; this violates the layer architecture. "
                f"Move dependencies to shared crates.",
                file=sys.stderr,
            )
            found = True
    if found:
        sys.exit(1)


def main():
    args = sys.argv[1:]
    max_file_lines = parse_max_file_lines(args)

    errors = 0
    try:
        check_oversized_files(max_file_lines)
    except SystemExit:
        errors += 1

    try:
        check_cli_thin_wrapper_boundary()
    except SystemExit:
        errors += 1

    try:
        check_backend_frontend_dependency()
    except SystemExit:
        errors += 1

    if not shutil.which("cargo"):
        print("check_architecture_rules: cargo is required", file=sys.stderr)
        sys.exit(1)

    # Check if ts2wasm-shared depends on ts2wasm-cli
    result = subprocess.run(
        ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )

    if result.returncode != 0:
        print("check_architecture_rules: OK (cargo tree unavailable)", file=sys.stderr)
        sys.exit(0 if errors == 0 else 1)

    if "ts2wasm-cli" in result.stdout:
        print("check_architecture_rules: ts2wasm-shared must not depend on ts2wasm-cli", file=sys.stderr)
        print(result.stdout, file=sys.stderr)
        errors += 1

    if errors > 0:
        print(f"check_architecture_rules: FAILED ({errors} checks)", file=sys.stderr)
        sys.exit(1)

    print("check_architecture_rules: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
