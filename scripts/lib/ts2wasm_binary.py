"""Resolve the ts2wasm CLI binary path.

Priority: TS2WASM_BINARY env var > target/release/ts2wasm > target/debug/ts2wasm.
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
    """Find a usable ts2wasm CLI binary.

    Priority: TS2WASM_BINARY env var > release builds > debug builds.
    Within each location (default target/ or cargo target-dir), prefer
    release over debug regardless of mtime — debug builds are dramatically
    slower for compilation and should only be used when release is absent.
    """
    env_bin = os.environ.get("TS2WASM_BINARY")
    if env_bin:
        return Path(env_bin)

    cargo_target = _cargo_target_dir()
    # Check order: release before debug within each location.
    locations = []
    if cargo_target:
        locations.append(cargo_target / "release" / "ts2wasm")
        locations.append(cargo_target / "debug" / "ts2wasm")
    locations.append(REPO_ROOT / "target" / "release" / "ts2wasm")
    locations.append(REPO_ROOT / "target" / "debug" / "ts2wasm")
    for path in locations:
        if path.is_file():
            return path

    print("ERROR: ts2wasm binary not found.", file=sys.stderr)
    print("Build first with: cargo build --release", file=sys.stderr)
    sys.exit(1)
