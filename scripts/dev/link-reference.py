#!/usr/bin/env python3
"""Link the ignored reference corpus into git worktrees.

The repository keeps reference/* ignored because the corpora are large external
checkouts. Git worktrees do not share ignored files, so autonomous child
worktrees otherwise miss reference/test262 and similar suites. This script
creates a worktree-local `reference` symlink pointing at the parent repository's
`reference` directory.
"""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REFERENCE = REPO_ROOT / "reference"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Symlink the repo reference/ corpus into one or more worktrees."
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
        default=DEFAULT_REFERENCE,
        help="Source reference directory. Defaults to this repo's reference/.",
    )
    parser.add_argument(
        "--replace-broken",
        action="store_true",
        help="Replace an existing broken reference symlink.",
    )
    return parser.parse_args()


def same_path(left: Path, right: Path) -> bool:
    try:
        return left.resolve(strict=True) == right.resolve(strict=True)
    except FileNotFoundError:
        return False


def replaceable_stub_reference_dir(path: Path) -> bool:
    if not path.is_dir() or path.is_symlink():
        return False
    entries = list(path.iterdir())
    if not entries:
        return True
    return len(entries) == 1 and entries[0].name == "README.md" and entries[0].is_file()


def remove_stub_reference_dir(path: Path) -> None:
    readme = path / "README.md"
    if readme.exists():
        readme.unlink()
    path.rmdir()


def link_reference(worktree: Path, reference_root: Path, replace_broken: bool) -> bool:
    worktree = worktree.resolve()
    reference_root = reference_root.resolve()
    link_path = worktree / "reference"

    if not reference_root.is_dir():
        print(f"link-reference: source reference root is missing: {reference_root}", file=sys.stderr)
        return False
    if not worktree.is_dir():
        print(f"link-reference: worktree is missing: {worktree}", file=sys.stderr)
        return False

    if link_path.is_symlink():
        target = Path(os.readlink(link_path))
        if not target.is_absolute():
            target = (link_path.parent / target).resolve()
        if same_path(target, reference_root):
            print(f"link-reference: ok existing {link_path} -> {reference_root}")
            return True
        if not link_path.exists() and replace_broken:
            link_path.unlink()
        else:
            print(
                f"link-reference: refusing to replace existing symlink {link_path} -> {target}; "
                "remove it or pass --replace-broken if it is broken",
                file=sys.stderr,
            )
            return False

    if link_path.exists():
        if same_path(link_path, reference_root):
            print(f"link-reference: ok existing directory {link_path}")
            return True
        if replaceable_stub_reference_dir(link_path):
            remove_stub_reference_dir(link_path)
            os.symlink(reference_root, link_path, target_is_directory=True)
            print(f"link-reference: replaced stub {link_path} -> {reference_root}")
            return True
        print(
            f"link-reference: refusing to replace existing non-symlink path: {link_path}",
            file=sys.stderr,
        )
        return False

    os.symlink(reference_root, link_path, target_is_directory=True)
    print(f"link-reference: created {link_path} -> {reference_root}")
    return True


def main() -> int:
    args = parse_args()
    worktrees = args.worktrees or [REPO_ROOT]
    ok = True
    for worktree in worktrees:
        ok = link_reference(worktree, args.reference_root, args.replace_broken) and ok
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
