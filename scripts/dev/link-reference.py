#!/usr/bin/env python3
"""Link ignored reference corpus directories into git worktrees.

The repository tracks `reference/README.md`, but ignores large external corpus
subdirectories such as `reference/test262/`. Git worktrees share tracked files
but not ignored directories, so child worktrees otherwise miss reference suites.
This script creates symlinks for each ignored corpus directory under a
worktree's existing `reference/` directory without replacing the tracked
`reference/` directory itself.
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REFERENCE = REPO_ROOT / "reference"


def has_directory(entry: Path) -> bool:
    try:
        return entry.is_dir()
    except OSError:
        return False


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Symlink ignored reference corpus directories into one or more worktrees."
    )
    parser.add_argument(
        "worktrees",
        nargs="*",
        type=Path,
        help="Worktree path(s). Defaults to the current repository root.",
    )
    parser.add_argument(
        "--reference-root",
        type=Path,
        default=None,
        help=(
            "Source reference directory. Defaults to TS2WASM_REFERENCE_ROOT, "
            "then this repo's reference/, then another git worktree's populated reference/."
        ),
    )
    parser.add_argument(
        "--replace-broken",
        action="store_true",
        help="Replace existing broken corpus symlinks under reference/.",
    )
    return parser.parse_args()


def git_worktree_reference_roots() -> list[Path]:
    try:
        result = subprocess.run(
            ["git", "-C", str(REPO_ROOT), "worktree", "list", "--porcelain"],
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError:
        return []
    if result.returncode != 0:
        return []
    roots = []
    for line in result.stdout.splitlines():
        if line.startswith("worktree "):
            roots.append(Path(line.removeprefix("worktree ")) / "reference")
    return roots


def has_corpus_entries(reference_root: Path) -> bool:
    try:
        return any(has_directory(entry) for entry in reference_root.iterdir() if entry.name != "README.md")
    except FileNotFoundError:
        return False


def default_reference_root() -> Path:
    if env_root := os.environ.get("TS2WASM_REFERENCE_ROOT"):
        env_path = Path(env_root)
        if has_corpus_entries(env_path):
            return env_path
        print(
            f"link-reference: TS2WASM_REFERENCE_ROOT has no corpus directory: {env_path}",
            file=sys.stderr,
        )
    if has_corpus_entries(DEFAULT_REFERENCE):
        return DEFAULT_REFERENCE
    for candidate in git_worktree_reference_roots():
        if same_path(candidate, DEFAULT_REFERENCE):
            continue
        if has_corpus_entries(candidate):
            return candidate
    return DEFAULT_REFERENCE


def same_path(left: Path, right: Path) -> bool:
    try:
        return left.resolve(strict=True) == right.resolve(strict=True)
    except FileNotFoundError:
        return False


def reference_entries(reference_root: Path) -> list[Path]:
    return sorted(
        entry
        for entry in reference_root.iterdir()
        if entry.name != "README.md" and has_directory(entry)
    )


def ensure_reference_dir(worktree: Path) -> Path | None:
    reference_dir = worktree / "reference"
    if reference_dir.is_symlink():
        print(
            f"link-reference: refusing to use symlinked tracked directory: {reference_dir}",
            file=sys.stderr,
        )
        return None
    if reference_dir.exists() and not reference_dir.is_dir():
        print(f"link-reference: reference path is not a directory: {reference_dir}", file=sys.stderr)
        return None
    reference_dir.mkdir(exist_ok=True)
    return reference_dir


def link_entry(reference_dir: Path, source: Path, replace_broken: bool) -> bool:
    link_path = reference_dir / source.name
    if link_path.is_symlink():
        target = Path(os.readlink(link_path))
        if not target.is_absolute():
            target = (link_path.parent / target).resolve()
        if same_path(target, source):
            print(f"link-reference: ok existing {link_path} -> {source}")
            return True
        if not link_path.exists() and replace_broken:
            link_path.unlink()
        else:
            print(
                f"link-reference: refusing to replace existing symlink {link_path} -> {target}",
                file=sys.stderr,
            )
            return False
    if link_path.exists():
        if same_path(link_path, source):
            print(f"link-reference: ok existing directory {link_path}")
            return True
        print(f"link-reference: refusing to replace existing path: {link_path}", file=sys.stderr)
        return False
    os.symlink(source, link_path, target_is_directory=True)
    print(f"link-reference: created {link_path} -> {source}")
    return True


def link_reference(worktree: Path, reference_root: Path, replace_broken: bool) -> bool:
    worktree = worktree.resolve()
    reference_root = reference_root.resolve()

    if not reference_root.is_dir():
        print(f"link-reference: source reference root is missing: {reference_root}", file=sys.stderr)
        return False
    if not worktree.is_dir():
        print(f"link-reference: worktree is missing: {worktree}", file=sys.stderr)
        return False

    reference_dir = ensure_reference_dir(worktree)
    if reference_dir is None:
        return False

    ok = True
    for source in reference_entries(reference_root):
        ok = link_entry(reference_dir, source, replace_broken) and ok
    return ok


def main() -> int:
    args = parse_args()
    worktrees = args.worktrees or [REPO_ROOT]
    reference_root = args.reference_root or default_reference_root()
    ok = True
    for worktree in worktrees:
        ok = link_reference(worktree, reference_root, args.replace_broken) and ok
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
