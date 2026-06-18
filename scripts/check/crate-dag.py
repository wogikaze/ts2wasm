#!/usr/bin/env python3
"""Crate dependency DAG enforcement (tomllib-based).

Validates the inter-crate dependency graph satisfies architectural layering:
  - backend-wasm must NOT depend on semantic-ir, frontend AST, etc.
  - spec-kernel must NOT depend on backend-wasm
  - runtime-core must NOT depend on backend-wasm, frontend, ir
  - No cycles in the crate dependency graph
  - Uses architecture-exceptions.toml for pre-existing violations

Usage:
  python scripts/check/crate-dag.py
  python scripts/check/crate-dag.py --self-test   # verify self-tests pass
"""

import json
import re
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def load_cargo_toml(crate_dir: str) -> dict:
    path = REPO_ROOT / crate_dir / "Cargo.toml"
    with open(path, "rb") as f:
        return tomllib.load(f)


def parse_dep_names(cargo: dict) -> set[str]:
    """Extract dependency package names from parsed Cargo.toml, handling all formats."""
    deps = cargo.get("dependencies", {})
    names = set()
    for name, spec in deps.items():
        if isinstance(spec, str):
            names.add(name)
        elif isinstance(spec, dict):
            names.add(name)
            if "package" in spec:
                names.add(spec["package"])
        elif isinstance(spec, (list, tuple)):
            for item in spec:
                if isinstance(item, dict) and "package" in item:
                    names.add(item["package"])
    return names


def load_exceptions() -> dict:
    path = REPO_ROOT / "architecture-exceptions.toml"
    if not path.exists():
        return {"legacy_deps": {}}
    with open(path, "rb") as f:
        return tomllib.load(f)


def canonical_crate_name(name: str) -> str:
    name = name.replace("_", "-")
    if name.startswith("ts2wasm-"):
        name = name.removeprefix("ts2wasm-")
    return name


def is_excepted(from_crate: str, to_dep: str) -> bool:
    exc = load_exceptions()
    from_crate = canonical_crate_name(from_crate)
    to_dep = canonical_crate_name(to_dep)
    for edge_key, info in exc.get("legacy_deps", {}).items():
        parts = edge_key.split(" -> ")
        if len(parts) != 2:
            continue
        f = canonical_crate_name(parts[0].strip())
        t = canonical_crate_name(parts[1].strip())
        if f == from_crate and t == to_dep:
            return True
    return False


def get_all_workspace_packages() -> dict[str, str]:
    """Return {package_name: crate_dir} for all workspace crates."""
    result = subprocess.run(
        ["cargo", "metadata", "--format-version=1"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    meta = json.loads(result.stdout)
    workspace_ids = set(meta["workspace_members"])
    pkg_id_to_name = {p["id"]: p["name"] for p in meta["packages"]}
    pkg_id_to_dir = {}
    for pkg in meta["packages"]:
        if pkg["id"] in workspace_ids:
            pkg_id_to_dir[pkg["id"]] = pkg["manifest_path"]
    packages = {}
    for pkg_id in workspace_ids:
        name = pkg_id_to_name.get(pkg_id, "")
        manifest = pkg_id_to_dir.get(pkg_id, "")
        crate_dir = str(Path(manifest).parent.relative_to(REPO_ROOT))
        packages[name] = crate_dir
    return packages


def build_dep_graph(packages: dict[str, str]) -> dict[str, set[str]]:
    graph: dict[str, set[str]] = {}
    for pkg_name, crate_dir in packages.items():
        cargo = load_cargo_toml(crate_dir)
        deps = parse_dep_names(cargo)
        # Filter to only workspace packages
        graph[pkg_name] = {d for d in deps if d in packages}
    return graph


# ── Forbidden edges (target architecture) ────────────────────────────────────
FORBIDDEN_DIRECT = [
    ("ts2wasm-backend-wasm", "ts2wasm-semantic-ir"),
    ("ts2wasm-backend-wasm", "ts2wasm-frontend"),
    ("ts2wasm-backend-wasm", "ts2wasm-syntax"),
    ("ts2wasm-spec-kernel", "ts2wasm-backend-wasm"),
    ("ts2wasm-runtime-core", "ts2wasm-backend-wasm"),
    ("ts2wasm-runtime-core", "ts2wasm-frontend"),
    ("ts2wasm-runtime-core", "ts2wasm-ir"),
    ("ts2wasm-runtime-core", "ts2wasm-resolve"),
    ("ts2wasm-semantic-ir", "ts2wasm-backend-wasm"),
    ("ts2wasm-semantic-ir", "ts2wasm-backend-correctness"),
    ("ts2wasm-semantic-ir", "ts2wasm-opt-mir"),
    ("ts2wasm-opt-mir", "ts2wasm-frontend"),
    ("ts2wasm-opt-mir", "ts2wasm-ir"),
]


def check_forbidden_edges(graph: dict[str, set[str]]) -> list[str]:
    violations = []
    for from_pkg, to_pkg in FORBIDDEN_DIRECT:
        deps = graph.get(from_pkg, set())
        if to_pkg in deps and not is_excepted(from_pkg, to_pkg):
            violations.append(
                f"ERROR {from_pkg} depends on {to_pkg} — forbidden edge"
            )
    return violations


def check_cycles(graph: dict[str, set[str]]) -> list[str]:
    violations = []
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {n: WHITE for n in graph}
    path: list[str] = []

    def dfs(node: str) -> bool:
        color[node] = GRAY
        path.append(node)
        for neighbor in graph.get(node, set()):
            if neighbor not in color:
                continue
            if color[neighbor] == GRAY:
                cycle_start = path.index(neighbor)
                cycle = path[cycle_start:] + [neighbor]
                violations.append(f"ERROR cycle: {' -> '.join(cycle)}")
                return True
            if color[neighbor] == WHITE:
                if dfs(neighbor):
                    return True
        path.pop()
        color[node] = BLACK
        return False

    for n in graph:
        if color[n] == WHITE:
            dfs(n)
    return violations


def check_layering(graph: dict[str, set[str]]) -> list[str]:
    LAYERS = {
        "ts2wasm-source": 0, "ts2wasm-runtime-abi": 0, "ts2wasm-backend-core": 0,
        "ts2wasm-diagnostic": 1, "ts2wasm-syntax": 1, "ts2wasm-runtime-core": 1,
        "ts2wasm-shared": 2, "ts2wasm-frontend": 2, "ts2wasm-spec-kernel": 2,
        "ts2wasm-resolve": 3, "ts2wasm-semantics": 3, "ts2wasm-opt-mir": 3,
        "ts2wasm-runtime-catalog": 4, "ts2wasm-ir": 4,
        "ts2wasm-semantic-ir": 5, "ts2wasm-backend-wasm": 5, "ts2wasm-backend-correctness": 5,
        "ts2wasm-compiler": 6, "ts2wasm-cli": 7,
    }
    violations = []
    for pkg_name, deps in graph.items():
        pkg_layer = LAYERS.get(pkg_name)
        if pkg_layer is None:
            continue
        for dep in deps:
            dep_layer = LAYERS.get(dep)
            if dep_layer is None:
                continue
            if dep_layer > pkg_layer and not is_excepted(pkg_name, dep):
                violations.append(
                    f"ERROR {pkg_name} (layer {pkg_layer}) depends on "
                    f"{dep} (layer {dep_layer}) — upward dependency"
                )
    return violations


def print_dep_table(graph: dict[str, set[str]]):
    """Print dependency table for debugging."""
    for pkg in sorted(graph):
        deps = sorted(graph[pkg])
        if deps:
            print(f"  {pkg}: {', '.join(deps)}")


def run_self_test():
    """Verify the checker catches real violations."""
    errors = 0

    # Test: parse_dep_names handles inline table format
    test_toml = """
[package]
name = "ts2wasm-test"
version = "0.1.0"

[dependencies]
ts2wasm-runtime-abi = { path = "../runtime-abi" }
ts2wasm-backend-core = { path = "../backend-core", features = ["foo"] }
ts2wasm-source = "0.1"
"""
    import tempfile
    t = tomllib.loads(test_toml)
    deps = parse_dep_names(t)
    if "ts2wasm-runtime-abi" not in deps:
        print("FAIL: parse_dep_names missing inline table dep", file=sys.stderr)
        errors += 1
    if "ts2wasm-backend-core" not in deps:
        print("FAIL: parse_dep_names missing inline table dep with features", file=sys.stderr)
        errors += 1

    # Test: FORBIDDEN_DIRECT catches backend-wasm → semantic-ir
    fake_graph = {
        "ts2wasm-backend-wasm": {"ts2wasm-semantic-ir"},
        "ts2wasm-semantic-ir": set(),
    }
    v = check_forbidden_edges(fake_graph)
    if not any("ts2wasm-backend-wasm depends on ts2wasm-semantic-ir" in x for x in v):
        print("FAIL: forbidden edge not detected", file=sys.stderr)
        errors += 1

    # Test: architecture exceptions match both package names and shorthand names.
    if not is_excepted("ts2wasm-backend-wasm", "ts2wasm-ir"):
        print("FAIL: package-name legacy exception not matched", file=sys.stderr)
        errors += 1
    if not is_excepted("backend-wasm", "ir"):
        print("FAIL: shorthand legacy exception not matched", file=sys.stderr)
        errors += 1

    # Test: cycle detection
    cycle_graph = {
        "ts2wasm-a": {"ts2wasm-b"},
        "ts2wasm-b": {"ts2wasm-c"},
        "ts2wasm-c": {"ts2wasm-a"},
    }
    v = check_cycles(cycle_graph)
    if not v:
        print("FAIL: cycle not detected", file=sys.stderr)
        errors += 1

    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print("self-test: OK", file=sys.stderr)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--self-test" in args:
        run_self_test()
        return

    packages = get_all_workspace_packages()
    graph = build_dep_graph(packages)

    violations: list[str] = []
    violations.extend(check_forbidden_edges(graph))
    violations.extend(check_cycles(graph))
    violations.extend(check_layering(graph))

    for v in violations:
        print(f"crate_dag: {v}", file=sys.stderr)

    if violations:
        err_count = sum(1 for v in violations if v.startswith("ERROR"))
        print(f"crate_dag: FAILED ({err_count} errors)", file=sys.stderr)
        sys.exit(1)

    print("crate_dag: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
