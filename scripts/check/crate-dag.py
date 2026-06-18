#!/usr/bin/env python3
"""Crate dependency DAG enforcement.

Validates the inter-crate dependency graph satisfies architectural layering:
  - backend-wasm must NOT depend on semantic-ir
  - spec-kernel must NOT depend on backend-wasm
  - runtime-core must NOT depend on backend-wasm
  - No cycles in the crate dependency graph
  - backend-wasm dependency count stays within limit

Usage: mise run check crate-dag
       python scripts/check/crate-dag.py
"""

import re
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

# Forbidden dependency edges: (from_crate_dir, denied_dep_name)
FORBIDDEN_EDGES = [
    ("crates/backend-wasm", "ts2wasm-semantic-ir"),
    ("crates/spec-kernel", "ts2wasm-backend-wasm"),
    ("crates/runtime-core", "ts2wasm-backend-wasm"),
]

# Maximum normal dependencies for backend-wasm (fan-out gate)
BACKEND_WASM_MAX_DEPS = 10


def parse_cargo_deps(crate_dir: str) -> list[str]:
    """Parse [dependencies] from a crate's Cargo.toml, returning dep crate names."""
    cargo_path = REPO_ROOT / crate_dir / "Cargo.toml"
    if not cargo_path.exists():
        return []
    text = cargo_path.read_text()
    deps_match = re.search(
        r"^\[dependencies\]\s*$(.+?)(?=^\s*\[|\Z)",
        text,
        re.MULTILINE | re.DOTALL,
    )
    if not deps_match:
        return []
    deps_section = deps_match.group(1)
    dep_names = []
    for m in re.finditer(r'^\s+([a-zA-Z][a-zA-Z0-9_-]*)\s*=\s*{?\s*$', deps_section, re.MULTILINE):
        dep_names.append(m.group(1))
    return dep_names


def check_forbidden_edges() -> list[str]:
    """Check that forbidden dependency edges are not present."""
    violations = []
    for from_dir, denied_dep in FORBIDDEN_EDGES:
        deps = parse_cargo_deps(from_dir)
        if denied_dep in deps:
            violations.append(
                f"ERROR {from_dir}/Cargo.toml depends on {denied_dep} "
                f"— forbidden edge in crate dependency DAG"
            )
    return violations


def build_dependency_graph() -> dict[str, list[str]]:
    """Build the crate dependency graph from Cargo.toml files."""
    graph: dict[str, list[str]] = {}
    crates_dir = REPO_ROOT / "crates"
    for cargo_path in crates_dir.glob("*/Cargo.toml"):
        crate_dir = str(cargo_path.parent.relative_to(REPO_ROOT))
        crate_name_match = re.search(r'^name\s*=\s*"([^"]+)"', cargo_path.read_text(), re.MULTILINE)
        if not crate_name_match:
            continue
        crate_name = crate_name_match.group(1)
        deps = parse_cargo_deps(crate_dir)
        graph[crate_name] = deps
    return graph


def check_no_cycles() -> list[str]:
    """Check that the crate dependency graph is a DAG (no cycles)."""
    violations = []
    graph = build_dependency_graph()
    if not graph:
        return violations

    # DFS-based cycle detection
    WHITE, GRAY, BLACK = 0, 1, 2
    color: dict[str, int] = {node: WHITE for node in graph}
    path: list[str] = []

    def dfs(node: str) -> bool:
        color[node] = GRAY
        path.append(node)
        for neighbor in graph.get(node, []):
            if neighbor not in color:
                continue  # external dep, skip
            if color[neighbor] == GRAY:
                # Found cycle
                cycle_start = path.index(neighbor)
                cycle = path[cycle_start:] + [neighbor]
                violations.append(
                    f"ERROR crate dependency cycle: {' -> '.join(cycle)}"
                )
                return True
            if color[neighbor] == WHITE:
                if dfs(neighbor):
                    return True
        path.pop()
        color[node] = BLACK
        return False

    for node in graph:
        if color[node] == WHITE:
            dfs(node)

    return violations


def check_backend_wasm_fan_out() -> list[str]:
    """Check that backend-wasm does not exceed the dependency fan-out limit."""
    violations = []
    deps = parse_cargo_deps("crates/backend-wasm")
    if len(deps) > BACKEND_WASM_MAX_DEPS:
        violations.append(
            f"ERROR crates/backend-wasm has {len(deps)} dependencies "
            f"(max {BACKEND_WASM_MAX_DEPS})"
        )
    return violations


def check_layering() -> list[str]:
    """Check that the crate layering is consistent with the documented architecture.

    Layer 0: source, runtime-abi, backend-core
    Layer 1: diagnostic, syntax, runtime-core
    Layer 2: shared, frontend, spec-kernel
    Layer 3: resolve, semantics, opt-mir
    Layer 4: runtime-catalog, ir
    Layer 5: semantic-ir, backend-wasm, backend-correctness
    Layer 6: compiler
    Layer 7: cli

    A crate at layer N must not depend on a crate at layer > N.
    """
    LAYERS = {
        "ts2wasm-source": 0,
        "ts2wasm-runtime-abi": 0,
        "ts2wasm-backend-core": 0,
        "ts2wasm-diagnostic": 1,
        "ts2wasm-syntax": 1,
        "ts2wasm-runtime-core": 1,
        "ts2wasm-shared": 2,
        "ts2wasm-frontend": 2,
        "ts2wasm-spec-kernel": 2,
        "ts2wasm-resolve": 3,
        "ts2wasm-semantics": 3,
        "ts2wasm-opt-mir": 3,
        "ts2wasm-runtime-catalog": 4,
        "ts2wasm-ir": 4,
        "ts2wasm-semantic-ir": 5,
        "ts2wasm-backend-wasm": 5,
        "ts2wasm-backend-correctness": 5,
        "ts2wasm-compiler": 6,
        "ts2wasm-cli": 7,
    }
    violations = []
    graph = build_dependency_graph()
    for crate_name, deps in graph.items():
        crate_layer = LAYERS.get(crate_name)
        if crate_layer is None:
            continue  # unknown crate, skip
        for dep in deps:
            dep_layer = LAYERS.get(dep)
            if dep_layer is None:
                continue  # external dep, skip
            if dep_layer > crate_layer:
                violations.append(
                    f"ERROR {crate_name} (layer {crate_layer}) depends on "
                    f"{dep} (layer {dep_layer}) — upward dependency violates layering"
                )
    return violations


def main():
    if sys.argv[1:] and sys.argv[1] in ("-h", "--help"):
        print(__doc__.strip())
        sys.exit(0)

    violations: list[str] = []
    check_fns = [
        ("forbidden edges", check_forbidden_edges),
        ("cycles", check_no_cycles),
        ("backend-wasm fan-out", check_backend_wasm_fan_out),
        ("layering", check_layering),
    ]

    for name, fn in check_fns:
        violations.extend(fn())

    for v in violations:
        print(f"crate_dag: {v}", file=sys.stderr)

    if any(v.startswith("ERROR") for v in violations):
        err_count = sum(1 for v in violations if v.startswith("ERROR"))
        print(f"crate_dag: FAILED ({err_count} errors)", file=sys.stderr)
        sys.exit(1)

    print("crate_dag: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
