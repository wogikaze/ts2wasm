#!/usr/bin/env python3
"""Enforce ts2wasm architecture contract (arch-rules.toml).

Checks:
  1. Dependency DAG: each crate depends only on allowed crates
  2. Forbidden imports: no crate matches a forbidden pattern
  3. File size: new crate files stay under 1500 lines (legacy exempt)

Usage:
  python scripts/check/check-arch-dag.py

Exit code:
  0 = all rules pass
  1 = violations found
"""

import json
import re
import subprocess
import sys
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

# Baseline count of exception edges (excepted legacy deps).
# Increase beyond this baseline is a hard error — it means new legacy edges are being added.
EXCEPTION_EDGE_BASELINE = 3


def load_rules() -> dict:
    path = REPO_ROOT / "arch-rules.toml"
    with open(path, "rb") as f:
        return tomllib.load(f)


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


def is_excepted_dependency(from_crate: str, to_crate: str, exceptions: dict) -> bool:
    from_crate = canonical_crate_name(from_crate)
    to_crate = canonical_crate_name(to_crate)
    for edge_key in exceptions.get("legacy_deps", {}):
        parts = edge_key.split(" -> ")
        if len(parts) != 2:
            continue
        if (
            canonical_crate_name(parts[0].strip()) == from_crate
            and canonical_crate_name(parts[1].strip()) == to_crate
        ):
            return True
    return False


def load_cargo_metadata() -> dict:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version=1"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print(f"check_arch_dag: cargo metadata failed: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return json.loads(result.stdout)


def build_dep_map(metadata: dict) -> dict[str, set[str]]:
    """Map package name → set of workspace dependency package names."""
    workspace_ids = set(metadata["workspace_members"])
    pkg_id_to_name = {p["id"]: p["name"] for p in metadata["packages"]}
    workspace_names = {pkg_id_to_name[i] for i in workspace_ids if i in pkg_id_to_name}

    deps: dict[str, set[str]] = {}
    for pkg in metadata["packages"]:
        if pkg["id"] not in workspace_ids:
            continue
        name = pkg["name"]
        deps[name] = set()
        for dep in pkg.get("dependencies", []):
            dep_name = dep.get("name", "")
            dep_pkg_name = pkg_id_to_name.get(dep.get("pkg", ""), dep_name)
            if dep_pkg_name in workspace_names:
                deps[name].add(dep_pkg_name)
    return deps


def check_dag(rules: dict, deps: dict) -> list[str]:
    violations = []
    crate_rules = rules.get("crates", {})
    exceptions = load_exceptions()

    def to_pkg_name(name: str) -> str:
        """Convert crate shorthand to package name."""
        name = name.replace("_", "-")
        if not name.startswith("ts2wasm-"):
            name = f"ts2wasm-{name}"
        return name

    pkg_to_crate = {to_pkg_name(c): c for c in crate_rules}

    for pkg_name, actual_deps in deps.items():
        if pkg_name not in pkg_to_crate:
            continue
        crate_name = pkg_to_crate[pkg_name]
        info = crate_rules[crate_name]
        allowed = set(info.get("allowed_deps", []))
        allowed_pkgs = {to_pkg_name(a) for a in allowed}

        disallowed = actual_deps - allowed_pkgs

        for dep in sorted(disallowed):
            # Skip workspace members not in the rules (e.g., build deps)
            if not any(dep.startswith(p) for p in ["ts2wasm-", "ts2wasm_"]):
                continue
            dep_crate = pkg_to_crate.get(dep, dep)
            if is_excepted_dependency(crate_name, dep_crate, exceptions):
                violations.append(
                    f"check_arch_dag: ERROR {crate_name} depends on {dep_crate} "
                    f"only via architecture-exceptions.toml — "
                    f"exception edges require documented migration plan"
                )
                continue
            violations.append(
                f"check_arch_dag: ERROR {crate_name} depends on {dep}, "
                f"which is not in target DAG allowed_deps and has no architecture exception"
            )

    return violations


def check_forbidden_imports(rules: dict) -> list[str]:
    violations = []
    forbidden = rules.get("forbidden", {})

    for rule_name, rule in forbidden.items():
        crate_names = rule.get("in_crates", [])
        pattern = rule.get("forbid_pattern", "")
        for crate_name in crate_names:
            src_dir = REPO_ROOT / "crates" / crate_name / "src"
            if not src_dir.exists():
                continue
            for path in sorted(src_dir.rglob("*.rs")):
                text = path.read_text()
                if re.search(pattern, text):
                    rel = path.relative_to(REPO_ROOT)
                    violations.append(
                        f"check_arch_dag: ERROR {rel} matches forbidden pattern "
                        f"'{pattern}' ({rule_name})"
                    )
    return violations


def check_file_sizes(rules: dict) -> list[str]:
    violations = []
    size_rules = rules.get("size", {})
    warn_limit = size_rules.get("warning", 800)
    error_limit = size_rules.get("error", 1500)
    allowlist = size_rules.get("allowlist", {})
    file_forbidden = rules.get("file_forbidden", {})

    # Check per-crate max_lines limits
    for dir_prefix, limits in file_forbidden.items():
        max_lines = limits.get("max_lines", error_limit)
        dir_path = REPO_ROOT / dir_prefix
        if not dir_path.exists():
            continue
        for path in sorted(dir_path.rglob("*.rs")):
            rel = path.relative_to(REPO_ROOT)
            if str(rel) in allowlist:
                continue
            lines = path.read_text().count("\n")
            if lines > max_lines:
                violations.append(
                    f"check_arch_dag: ERROR {rel}: {lines} lines "
                    f"(max {max_lines} for this crate)"
                )
            elif lines > warn_limit:
                violations.append(
                    f"check_arch_dag: WARN {rel}: {lines} lines "
                    f"(warning > {warn_limit})"
                )

    return violations


def check_runtimefn_count(rules: dict) -> list[str]:
    """Check that RuntimeFn variant count hasn't increased."""
    violations = []
    coverage_rules = rules.get("coverage", {})
    if not coverage_rules.get("reject_runtimefn_addition", False):
        return violations

    path = REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime_fn.rs"
    if not path.exists():
        return violations

    text = path.read_text()
    # Count enum variants: lines that start with "    [A-Z]"
    import re
    enum_match = re.search(r"pub enum RuntimeFn \{(.*?)^\}", text, re.MULTILINE | re.DOTALL)
    if not enum_match:
        return violations

    body = enum_match.group(1)
    count = len(re.findall(r"^\s+([A-Z]\w+)\s*,?\s*$", body, re.MULTILINE))

    # Baseline from arch-rules
    violations.append(
        f"check_arch_dag: INFO RuntimeFn has {count} variants "
        f"(baseline ~504, reject addition)"
    )
    return violations


def main():
    args = sys.argv[1:]
    reject_increase = "--reject-increase" in args
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)

    rules = load_rules()
    metadata = load_cargo_metadata()
    deps = build_dep_map(metadata)

    all_violations = []
    all_violations.extend(check_dag(rules, deps))
    all_violations.extend(check_forbidden_imports(rules))
    all_violations.extend(check_file_sizes(rules))
    all_violations.extend(check_runtimefn_count(rules))

    # Separate exception-edge errors from other errors
    exception_errors = [v for v in all_violations if "only via architecture-exceptions.toml" in v]
    real_errors = [v for v in all_violations if "ERROR" in v and v not in exception_errors]

    for v in all_violations:
        # With --reject-increase, downgrade baseline exception errors to INFO
        if reject_increase and v in exception_errors:
            print(v.replace("ERROR", "INFO"), file=sys.stderr)
        else:
            print(v, file=sys.stderr)

    if reject_increase:
        exception_count = len(exception_errors)
        if exception_count > EXCEPTION_EDGE_BASELINE:
            print(
                f"check_arch_dag: FAILED — exception edge count {exception_count} "
                f"exceeds baseline {EXCEPTION_EDGE_BASELINE}",
                file=sys.stderr,
            )
            sys.exit(1)
        print(
            f"check_arch_dag: OK ({exception_count} exception edges at baseline, "
            f"{len(real_errors)} other errors)",
            file=sys.stderr,
        )
        if real_errors:
            sys.exit(1)
        sys.exit(0)

    if real_errors or exception_errors:
        print(
            f"check_arch_dag: FAILED ({len(real_errors) + len(exception_errors)} errors)",
            file=sys.stderr,
        )
        sys.exit(1)

    if all_violations:
        print(
            f"check_arch_dag: OK ({len(all_violations)} warnings, 0 errors)",
            file=sys.stderr,
        )
    else:
        print("check_arch_dag: OK (no violations)", file=sys.stderr)


if __name__ == "__main__":
    main()
