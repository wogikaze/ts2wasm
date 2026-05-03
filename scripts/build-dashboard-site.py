#!/usr/bin/env python3
"""Build the web-ui dashboard and publish it to site dashboard route."""

from pathlib import Path
import shutil
import subprocess

# /// script
# requires-python = ">=3.8"
# dependencies = []
# ///

PROJECT_ROOT = Path(__file__).resolve().parent.parent
WEB_UI_DIR = PROJECT_ROOT / "web-ui"
SITE_DASHBOARD_DIR = PROJECT_ROOT / "site" / "docs" / ".vitepress" / "public" / "dashboard"
DATA_SRC_DIR = PROJECT_ROOT / "site" / "docs" / "coverage" / "web-ui" / "public" / "data"
DATA_DST_DIR = SITE_DASHBOARD_DIR / "data"


def run(command: str, cwd: Path) -> None:
    subprocess.check_call(command, cwd=cwd, shell=True)


def main() -> None:
    print("Building web-ui dashboard for /dashboard")
    run("npm run build:dashboard", WEB_UI_DIR)

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
