from pathlib import Path
from typing import Optional


def normalize_env_path(raw: Optional[str]) -> Optional[str]:
    """Normalize path-like env values loaded by task runners.

    Some dotenv loaders preserve wrapping quotes in values. Path() treats those
    values as relative paths, so strip one matching quote pair before resolving.
    """
    if raw is None:
        return None
    value = raw.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {"'", '"'}:
        value = value[1:-1]
    return value


def resolve_env_path(raw: Optional[str], repo_root: Path, fallback: Optional[Path] = None) -> Optional[Path]:
    value = normalize_env_path(raw)
    if not value:
        return fallback
    path = Path(value)
    return path if path.is_absolute() else repo_root / path
