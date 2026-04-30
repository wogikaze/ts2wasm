"""Resolve the ts2wasm CLI binary path.

Priority: TS2WASM_BINARY env var > target/release/ts2wasm > target/debug/ts2wasm.
"""

import os
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def resolve_ts2wasm_binary() -> Path:
    """Find the ts2wasm CLI binary, falling back from env var to release to debug."""
    env_bin = os.environ.get("TS2WASM_BINARY")
    if env_bin:
        return Path(env_bin)

    candidates = [
        REPO_ROOT / "target" / "release" / "ts2wasm",
        REPO_ROOT / "target" / "debug" / "ts2wasm",
    ]
    for candidate in candidates:
        if candidate.is_file():
            return candidate

    print("ERROR: ts2wasm binary not found.", file=sys.stderr)
    print("Build first with: cargo build or cargo build --release", file=sys.stderr)
    sys.exit(1)
