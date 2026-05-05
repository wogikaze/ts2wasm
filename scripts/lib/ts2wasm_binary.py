"""Resolve the ts2wasm CLI binary path.

Priority: TS2WASM_BINARY env var > newest target/{release,debug}/ts2wasm.
"""

import os
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def _cargo_target_dir() -> Path | None:
    """Parse .cargo/config.toml for a shared target-dir override."""
    cargo_cfg = REPO_ROOT / ".cargo" / "config.toml"
    if not cargo_cfg.is_file():
        return None
    for line in cargo_cfg.read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith("target-dir") and "=" in stripped:
            raw = stripped.split("=", 1)[1].strip().strip('"')
            return Path(raw).expanduser().resolve()
    return None


def resolve_ts2wasm_binary() -> Path:
    """Find the newest workspace-built ts2wasm CLI binary.

    Priority: TS2WASM_BINARY env var > .cargo/config.toml target-dir > default.
    """
    env_bin = os.environ.get("TS2WASM_BINARY")
    if env_bin:
        return Path(env_bin)

    cargo_target = _cargo_target_dir()
    candidates = [
        REPO_ROOT / "target" / "release" / "ts2wasm",
        REPO_ROOT / "target" / "debug" / "ts2wasm",
    ]
    # If .cargo/config.toml overrides target-dir, check there too.
    # target-dir IS the target directory, so no extra "target" subdirectory.
    if cargo_target:
        candidates.insert(0, cargo_target / "release" / "ts2wasm")
        candidates.insert(0, cargo_target / "debug" / "ts2wasm")
    existing = [candidate for candidate in candidates if candidate.is_file()]
    if existing:
        return max(existing, key=lambda path: path.stat().st_mtime)

    print("ERROR: ts2wasm binary not found.", file=sys.stderr)
    print("Build first with: cargo build or cargo build --release", file=sys.stderr)
    sys.exit(1)
