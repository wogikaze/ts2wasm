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


REPO_ROOT = Path(__file__).resolve().parents[2]


def resolve_path(raw, fallback):
    if not raw:
        return fallback
    path = Path(raw)
    if path.is_absolute():
        return path
    return REPO_ROOT / path


def sync_tree(source: Path, destination: Path) -> bool:
    if not source.exists():
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
        # Default dashboard publish target from this repo. If already built with
        # build-dashboard-site into a custom location, copy from there as well.
        dashboard_src = Path(os.environ.get("TS2WASM_DASHBOARD_SRC_DIR", ""))
        if not dashboard_src:
            dashboard_src = REPO_ROOT / "site" / "docs" / "public" / "dashboard"
        dashboard_src = resolve_path(str(dashboard_src), dashboard_src)
        copied = sync_tree(dashboard_src, docs_repo / "dashboard") or copied

    if sync_webui_data:
        # Keep precedence consistent with gen/web-ui-data and reference-coverage:
        # direct web-ui data dir overrides docs repo path + standard subdir.
        web_ui_dir = resolve_path(
            os.environ.get("TS2WASM_WEB_UI_DATA_DIR", ""),
            docs_repo / "coverage" / "web-ui" / "public" / "data",
        )
        copied = sync_tree(web_ui_dir, docs_repo / "coverage" / "web-ui" / "public" / "data") or copied

    if not copied:
        print(
            "No source files found to sync. Ensure dashboard/data generation and docs sources exist.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
