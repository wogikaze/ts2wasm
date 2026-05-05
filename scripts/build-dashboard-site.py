#!/usr/bin/env python3
"""Build the web-ui dashboard and publish it to site dashboard route."""

from pathlib import Path
import sys
import shutil
import subprocess
import os

# /// script
# requires-python = ">=3.8"
# dependencies = []
# ///

PROJECT_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(PROJECT_ROOT / "scripts" / "lib"))
from path_env import normalize_env_path, resolve_env_path

WEB_UI_DIR = PROJECT_ROOT / "web-ui"
DOCS_REPO = resolve_env_path(
    os.environ.get("TS2WASM_DOCS_REPO_PATH"),
    PROJECT_ROOT,
    PROJECT_ROOT / "site" / "docs",
)
SITE_DASHBOARD_DIR = DOCS_REPO / "public" / "dashboard"
DATA_SRC_DIR = resolve_env_path(
    os.environ.get("TS2WASM_WEB_UI_DATA_DIR"),
    PROJECT_ROOT,
    DOCS_REPO / "coverage" / "web-ui" / "public" / "data",
)
DATA_DST_DIR = SITE_DASHBOARD_DIR / "data"


def run(command: str, cwd: Path) -> None:
    subprocess.check_call(command, cwd=cwd, shell=True)


def detect_base() -> str:
    """Detect the correct Vite base URL for the deployment target.

    - TS2WASM_PAGES_BASE: explicit override (e.g. /ts2wasm-docs/dashboard/)
    - TS2WASM_DOCS_REPO_PATH set → building for external Pages repo
    - Otherwise → local VitePress dev (base=/dashboard/)
    """
    explicit = os.environ.get("TS2WASM_PAGES_BASE")
    if explicit:
        return explicit
    docs_repo_path = normalize_env_path(os.environ.get("TS2WASM_DOCS_REPO_PATH"))
    if docs_repo_path:
        # Determine Pages subpath from the docs repo name.
        # e.g. wogikaze/ts2wasm-docs → /ts2wasm-docs/dashboard/
        # Fallback: check git remote of the target dir.
        repo_path = Path(docs_repo_path)
        pages_repo = os.environ.get("TS2WASM_PAGES_REPO", "")
        if pages_repo:
            repo_name = pages_repo.rstrip(".git").split("/")[-1]
        elif (repo_path / ".git").exists():
            try:
                result = subprocess.run(
                    ["git", "remote", "get-url", "origin"],
                    cwd=repo_path, capture_output=True, text=True, timeout=5,
                )
                remote = result.stdout.strip()
                repo_name = remote.rstrip(".git").split("/")[-1]
            except Exception:
                repo_name = ""
        else:
            repo_name = ""
        if repo_name:
            return f"/{repo_name}/dashboard/"
    return "/dashboard/"


def main() -> None:
    base = detect_base()
    print(f"Building web-ui dashboard (base={base})")
    run(f"npm run build:dashboard -- --base {base}", WEB_UI_DIR)

    print(f"Publishing dashboard bundle to {SITE_DASHBOARD_DIR}")
    if SITE_DASHBOARD_DIR.exists():
        shutil.rmtree(SITE_DASHBOARD_DIR)
    shutil.copytree(WEB_UI_DIR / "dist", SITE_DASHBOARD_DIR)

    print(f"Copying dashboard data into {DATA_DST_DIR}")
    if DATA_DST_DIR.exists():
        shutil.rmtree(DATA_DST_DIR)
    if DATA_SRC_DIR.exists():
        shutil.copytree(DATA_SRC_DIR, DATA_DST_DIR)
    else:
        DATA_DST_DIR.mkdir(parents=True, exist_ok=True)

    print("Dashboard published to /dashboard/")


if __name__ == "__main__":
    main()
