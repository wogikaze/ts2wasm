#!/usr/bin/env python3
"""Rustfmt check that skips explicitly frozen legacy files.

This is used by the fast architecture gate because full `cargo fmt --all --check`
currently conflicts with the legacy freeze policy: several frozen files predate
the formatting baseline and must not be touched just to make a gate green.

Usage:
  python scripts/check/rustfmt-legacy-aware.py
  python scripts/check/rustfmt-legacy-aware.py --fix
"""

import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]

FROZEN_FILES = {
    "crates/backend-wasm/src/native_lowered.rs",
    "crates/backend-wasm/src/runtime/core/typed.rs",
    "crates/backend-wasm/src/native_runtime_embed.rs",
    "crates/runtime-catalog/src/runtime_fn.rs",
}

NON_STANDALONE_RUST_FILES = {
    "crates/runtime-catalog/src/runtime/manifest/all.rs",
    "crates/runtime-catalog/src/runtime/spec/all.rs",
}

EXCLUDED_DIRS = {
    ".agents",
    ".claude",
    ".git",
    ".worktrees",
    "target",
    "reference",
    "reports",
    "node_modules",
    ".venv",
    "venv",
}


def iter_rust_files() -> list[Path]:
    files: list[Path] = []
    for path in sorted(REPO_ROOT.rglob("*.rs")):
        rel = path.relative_to(REPO_ROOT)
        if any(part in EXCLUDED_DIRS for part in rel.parts):
            continue
        if str(rel) in FROZEN_FILES or str(rel) in NON_STANDALONE_RUST_FILES:
            continue
        files.append(path)
    return files


def main() -> None:
    args = sys.argv[1:]
    fix = "--fix" in args
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        return
    unknown = [arg for arg in args if arg not in {"--fix"}]
    if unknown:
        print(f"rustfmt_legacy_aware: unknown arguments: {' '.join(unknown)}", file=sys.stderr)
        sys.exit(2)

    files = iter_rust_files()
    if not files:
        print("rustfmt_legacy_aware: no Rust files found", file=sys.stderr)
        return
    cmd = [
        "rustfmt",
        "--edition",
        "2024",
        "--config",
        "skip_children=true",
        *[str(path) for path in files],
    ]
    if not fix:
        cmd.insert(3, "--check")
    result = subprocess.run(cmd, cwd=REPO_ROOT)
    if result.returncode != 0:
        print("rustfmt_legacy_aware: FAILED", file=sys.stderr)
        sys.exit(result.returncode)
    mode = "fixed" if fix else "OK"
    print(
        f"rustfmt_legacy_aware: {mode} ({len(files)} files, {len(FROZEN_FILES)} frozen skipped)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
