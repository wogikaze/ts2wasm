#!/usr/bin/env python3
"""Sync generated docs/dashboard assets to an external documentation repo.

This is intended for a split-docs workflow:
- keep this main repo as source of truth for docs source and coverage artifacts
- publish generated docs assets into TS2WASM_DOCS_REPO_PATH
"""

from pathlib import Path
import os
import shutil
import sys


REPO_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT / "scripts" / "lib"))
from path_env import normalize_env_path


def resolve_path(raw, fallback):
    raw = normalize_env_path(raw)
    if not raw:
        return fallback
    path = Path(raw)
    if path.is_absolute():
        return path
    return REPO_ROOT / path


def sync_tree(source: Path, destination: Path) -> bool:
    if not source.exists():
        return False

    # Prevent recursive copy: destination must not overlap with source.
    # Both must be resolved (realpath) to detect cases like
    #   source=…/ts2wasm-docs/coverage/web-ui/public/data
    #   dest  =…/ts2wasm-docs/coverage/web-ui/public/data   (same → self-copy)
    # or source=…/ts2wasm-docs/dashboard  dest=…/ts2wasm-docs  (source inside dest).
    try:
        src_resolved = source.resolve(strict=False)
        dst_resolved = destination.resolve(strict=False)
        if src_resolved == dst_resolved:
            print(
                f"WARNING: source and destination are the same path: {src_resolved}",
                file=sys.stderr,
            )
            return False
        # destination must not be a parent of source
        if dst_resolved in src_resolved.parents:
            print(
                f"WARNING: destination {dst_resolved} is a parent of source {src_resolved} "
                "(would cause recursive copy)",
                file=sys.stderr,
            )
            return False
        # source must not be a parent of destination
        if src_resolved in dst_resolved.parents:
            print(
                f"WARNING: source {src_resolved} is a parent of destination {dst_resolved} "
                "(would cause recursive copy)",
                file=sys.stderr,
            )
            return False
    except (ValueError, OSError) as exc:
        print(f"WARNING: could not resolve paths ({exc}); skipping sync_tree", file=sys.stderr)
        return False

    if destination.exists():
        shutil.rmtree(destination)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, destination)
    return True


def parse_bool_env(name: str, default: bool = True) -> bool:
    value = os.environ.get(name)
    if value is None:
        return default
    return value.strip().lower() not in {"0", "false", "f", "no", "off"}


def main() -> int:
    docs_repo_path = os.environ.get("TS2WASM_DOCS_REPO_PATH")
    if not docs_repo_path:
        print("TS2WASM_DOCS_REPO_PATH is not set", file=sys.stderr)
        return 1

    docs_repo = resolve_path(docs_repo_path, None)
    if docs_repo is None:
        print("TS2WASM_DOCS_REPO_PATH is invalid", file=sys.stderr)
        return 1

    docs_repo.mkdir(parents=True, exist_ok=True)

    sync_docs = parse_bool_env("TS2WASM_SYNC_DOCS_MD", True)
    sync_matrix = parse_bool_env("TS2WASM_SYNC_COVERAGE_MATRIX", True)
    sync_dashboard = parse_bool_env("TS2WASM_SYNC_DASHBOARD_ASSETS", True)
    sync_webui_data = parse_bool_env("TS2WASM_SYNC_WEB_UI_DATA", True)

    copied = False
    if sync_docs:
        copied = sync_tree(REPO_ROOT / "docs", docs_repo / "docs") or copied

    if sync_matrix:
        matrix_src = REPO_ROOT / "artifacts" / "coverage" / "reference-coverage-matrix.md"
        if matrix_src.is_file():
            shutil.copy2(matrix_src, docs_repo / "reference-coverage-matrix.md")
            copied = True

    if sync_dashboard:
        dashboard_src_raw = os.environ.get("TS2WASM_DASHBOARD_SRC_DIR", "")
        if dashboard_src_raw:
            dashboard_src = resolve_path(dashboard_src_raw, None)
        else:
            dashboard_src = REPO_ROOT / "site" / "docs" / "public" / "dashboard"
        if dashboard_src:
            copied = sync_tree(dashboard_src, docs_repo / "dashboard") or copied

    if sync_webui_data:
        web_ui_src_raw = os.environ.get("TS2WASM_WEB_UI_DATA_DIR", "")
        # When the env var is unset the source is inside the main repo, not inside
        # docs_repo — the default is the generated data under REPO_ROOT.
        if not web_ui_src_raw:
            web_ui_src = REPO_ROOT / "site" / "docs" / "coverage" / "web-ui" / "public" / "data"
        else:
            web_ui_src = resolve_path(web_ui_src_raw, None)
        if web_ui_src:
            copied = sync_tree(web_ui_src, docs_repo / "coverage" / "web-ui" / "public" / "data") or copied

    if not copied:
        print(
            "No source files found to sync. Ensure dashboard/data generation and docs sources exist.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
