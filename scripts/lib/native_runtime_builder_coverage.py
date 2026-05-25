#!/usr/bin/env python3
"""Static native RuntimeFn builder coverage helpers."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
RUNTIME_FN_SOURCE = REPO_ROOT / "crates/runtime-catalog/src/runtime_fn.rs"
NATIVE_RUNTIME_SOURCE = REPO_ROOT / "crates/backend-wasm/src/native_runtime_embed.rs"


def _extract_block(source: str, start_pattern: str, end_pattern: str) -> str:
    start = source.find(start_pattern)
    if start == -1:
        raise RuntimeError(f"missing start pattern: {start_pattern}")
    end = source.find(end_pattern, start)
    if end == -1:
        raise RuntimeError(f"missing end pattern after {start_pattern}: {end_pattern}")
    return source[start:end]


def runtime_fn_variants(runtime_fn_source: Path = RUNTIME_FN_SOURCE) -> list[str]:
    source = runtime_fn_source.read_text(encoding="utf-8")
    block = _extract_block(source, "pub enum RuntimeFn {", "\n}\n\n#[derive")
    variants: list[str] = []
    for line in block.splitlines():
        line = line.split("//", 1)[0].strip()
        match = re.match(r"^([A-Z][A-Za-z0-9_]*)\s*,", line)
        if match:
            variants.append(match.group(1))
    if not variants:
        raise RuntimeError(f"no RuntimeFn variants found in {runtime_fn_source}")
    return variants


def pseudo_runtime_functions(native_runtime_source: Path = NATIVE_RUNTIME_SOURCE) -> set[str]:
    source = native_runtime_source.read_text(encoding="utf-8")
    block = _extract_block(
        source,
        "const PSEUDO_RUNTIME_FUNCTIONS: &[RuntimeFn] = &[",
        "\n];",
    )
    return set(re.findall(r"RuntimeFn::([A-Za-z0-9_]+)", block))


def available_native_runtime_functions(
    native_runtime_source: Path = NATIVE_RUNTIME_SOURCE,
) -> set[str]:
    source = native_runtime_source.read_text(encoding="utf-8")
    block = _extract_block(
        source,
        "pub(crate) fn native_runtime_function_available(runtime_fn: RuntimeFn) -> bool {",
        "\nfn build_native_runtime_function(",
    )
    available = set(re.findall(r"RuntimeFn::([A-Za-z0-9_]+)", block))
    if not available:
        raise RuntimeError(f"no native RuntimeFn builders found in {native_runtime_source}")
    return available


def summarize_native_runtime_builder_coverage(
    *,
    runtime_fn_source: Path = RUNTIME_FN_SOURCE,
    native_runtime_source: Path = NATIVE_RUNTIME_SOURCE,
) -> dict[str, Any]:
    variants = runtime_fn_variants(runtime_fn_source)
    variant_set = set(variants)
    pseudo = pseudo_runtime_functions(native_runtime_source)
    available = available_native_runtime_functions(native_runtime_source)
    unknown_pseudo = sorted(pseudo - variant_set)
    unknown_available = sorted(available - variant_set)
    if unknown_pseudo or unknown_available:
        raise RuntimeError(
            "native runtime builder coverage references unknown RuntimeFn variants: "
            f"pseudo={unknown_pseudo}, available={unknown_available}"
        )

    missing = [
        variant for variant in variants
        if variant not in pseudo and variant not in available
    ]
    non_pseudo_total = len(variants) - len(pseudo)
    available_non_pseudo = len([variant for variant in variants if variant in available])
    coverage_percent = (
        f"{(available_non_pseudo / non_pseudo_total) * 100:.2f}"
        if non_pseudo_total
        else "100.00"
    )
    return {
        "schema_version": 1,
        "runtime_fn_total": len(variants),
        "non_pseudo_total": non_pseudo_total,
        "available": available_non_pseudo,
        "pseudo": len(pseudo),
        "missing_non_pseudo": len(missing),
        "coverage_percent": coverage_percent,
        "missing": missing,
        "sources": [
            runtime_fn_source.relative_to(REPO_ROOT).as_posix(),
            native_runtime_source.relative_to(REPO_ROOT).as_posix(),
        ],
    }
