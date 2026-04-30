"""Resolve the ts2wasm CLI binary path.

Priority: TS2WASM_BINARY env var > newest target/{release,debug}/ts2wasm.
"""

import os
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def resolve_ts2wasm_binary() -> Path:
    """Find the newest workspace-built ts2wasm CLI binary."""
    env_bin = os.environ.get("TS2WASM_BINARY")
    if env_bin:
        return Path(env_bin)

    candidates = [
        REPO_ROOT / "target" / "release" / "ts2wasm",
        REPO_ROOT / "target" / "debug" / "ts2wasm",
    ]
    existing = [candidate for candidate in candidates if candidate.is_file()]
    if existing:
        return max(existing, key=lambda path: path.stat().st_mtime)

    print("ERROR: ts2wasm binary not found.", file=sys.stderr)
    print("Build first with: cargo build or cargo build --release", file=sys.stderr)
    sys.exit(1)
