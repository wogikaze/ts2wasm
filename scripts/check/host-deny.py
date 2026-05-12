#!/usr/bin/env python3
"""Host-import deny matrix and policy checker.

Reads fixtures/catalog.yaml, analyzes each fixture source file for Node host
import patterns, compiles a representative subset, and reports an explicit
allow/deny status per fixture. Exits with 0 after reporting (info-only gate).

Usage:
  python scripts/check/host-deny.py
  python scripts/check/host-deny.py --update-catalog

Options:
  --update-catalog   Add host_imports field to catalog.yaml entries.
  --compile          Force compilation-based check (slower but more accurate).
  -v, --verbose      Print per-fixture analysis details.

Dependencies: cargo, wasm-tools (for --compile mode only).
"""

import sys
import subprocess
import shutil
import tempfile
import os
import re
import yaml
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"

# Node.js built-in modules that route through the host shim.
NODE_BUILTIN_MODULES = {
    "crypto", "fs", "path", "os", "child_process", "http", "https",
    "net", "tls", "dns", "url", "querystring", "stream", "zlib",
    "util", "events", "assert", "buffer", "string_decoder", "timers",
    "punycode", "readline", "cluster", "domain", "vm", "tty",
    "dgram", "module", "wasi",
}

# Global function names that route through Node host shims.
HOST_SHIM_GLOBALS = {
    "encodeURI", "decodeURI", "encodeURIComponent", "decodeURIComponent",
    "escape", "unescape",
}

# Source patterns that indicate Node host dependency.
HOST_PATTERNS = [
    re.compile(r'require\s*\(\s*["\'](node:)?(' + '|'.join(re.escape(m) for m in sorted(NODE_BUILTIN_MODULES, key=len, reverse=True)) + r')["\']\s*\)'),
    re.compile(r'(import|export)\s+.*\s+from\s+["\']node:(' + '|'.join(re.escape(m) for m in sorted(NODE_BUILTIN_MODULES, key=len, reverse=True)) + r')["\']'),
    re.compile(r'\bprocess\s*\.\s*(exit|env|argv|cwd|chdir|pid|ppid|platform|arch|version|versions|stdout|stderr|stdin|hrtime|nextTick|umask|uptime|memoryUsage)\b'),
    re.compile(r'\b(' + '|'.join(re.escape(g) for g in sorted(HOST_SHIM_GLOBALS, key=len, reverse=True)) + r')\s*\('),
    re.compile(r'\bimport\s*\('),
    re.compile(r'\.toISOString\s*\('),
    re.compile(r'\.getTimezoneOffset\s*\('),
    re.compile(r'\.toLocaleString\s*\('),
    re.compile(r'\.toLocaleDateString\s*\('),
    re.compile(r'\.toLocaleTimeString\s*\('),
]


def load_catalog(path):
    with open(path) as f:
        return yaml.safe_load(f)


def get_compilable_categories(catalog):
    non_compilable = {"negative", "parser", "test-infrastructure"}
    return {name for name, desc in catalog.get("categories", {}).items()
            if name not in non_compilable}


def discover_fixtures(catalog):
    for dir_name, info in catalog.get("directories", {}).items():
        cat = info.get("category", "")
        status = info.get("status", "unknown")
        for fname in info.get("fixtures", []):
            rel = f"fixtures/{dir_name}/{fname}"
            yield rel, dir_name, cat, status


def scan_source_for_host_imports(source_path):
    try:
        with open(source_path) as f:
            source = f.read()
    except (FileNotFoundError, IOError):
        return ["<file not readable>"]
    matches = []
    for pattern in HOST_PATTERNS:
        if pattern.search(source):
            matches.append(pattern.pattern)
    return matches


def categorize_host_imports(matches):
    if not matches:
        return "allow"
    return "deny"


def get_catalog_explicit_status(catalog, rel_path):
    parts = rel_path.replace("fixtures/", "").split("/")
    if len(parts) < 2:
        return None
    dir_name = parts[0]
    fname = "/".join(parts[1:])
    for dname, info in catalog.get("directories", {}).items():
        if dname == dir_name and fname in info.get("fixtures", []):
            return info.get("host_imports")
    return None


def check_wasm_for_host_imports(wasm_path):
    result = subprocess.run(
        ["wasm-tools", "print", str(wasm_path)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        return None, f"wasm-tools print failed: {result.stderr.strip()}"
    has_host = '(import "host"' in result.stdout
    return has_host, None


def build_fixture(ts2wasm_bin, fixture_path, wasm_path):
    result = subprocess.run(
        [str(ts2wasm_bin), "build", str(fixture_path), "-o", str(wasm_path)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        err = result.stderr.strip() or result.stdout.strip()
        return False, err[:200]
    return True, ""


def build_fixture_compile_check(catalog, selected_rel_paths, verbose=False):
    if not shutil.which("wasm-tools"):
        print("  (skipping compile check: wasm-tools not available)", file=sys.stderr)
        return {}
    print("  Building ts2wasm-cli...", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print("  (skipping compile check: cargo build failed)", file=sys.stderr)
        return {}
    ts2wasm_bin = REPO_ROOT / "target/debug/ts2wasm"
    if not ts2wasm_bin.exists():
        print("  (skipping compile check: ts2wasm binary not found)", file=sys.stderr)
        return {}
    results = {}
    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        for rel in selected_rel_paths:
            fixture_path = REPO_ROOT / rel
            if not fixture_path.exists():
                if verbose:
                    print(f"    SKIP (not found): {rel}", file=sys.stderr)
                continue
            wasm_path = tmpd / f"{rel.replace('/', '_')}.wasm"
            ok, log = build_fixture(ts2wasm_bin, fixture_path, wasm_path)
            if not ok:
                results[rel] = {"allow": False, "detail": f"build_error: {log}"}
                if verbose:
                    print(f"    BUILD FAIL: {rel}", file=sys.stderr)
                continue
            has_host, err = check_wasm_for_host_imports(wasm_path)
            if err:
                results[rel] = {"allow": False, "detail": err}
                if verbose:
                    print(f"    WASM FAIL: {rel}", file=sys.stderr)
                continue
            verdict = not has_host
            detail = "allow" if verdict else "deny: has (import \"host\" ...)"
            results[rel] = {"allow": verdict, "detail": detail}
            if verbose:
                print(f"    {'OK' if verdict else 'DENY'}: {rel} -> {detail}", file=sys.stderr)
    return results


def main():
    args = sys.argv[1:]
    update_catalog = "--update-catalog" in args
    force_compile = "--compile" in args
    verbose = "-v" in args or "--verbose" in args

    if not CATALOG_PATH.exists():
        print(f"host-deny: catalog not found: {CATALOG_PATH}", file=sys.stderr)
        sys.exit(1)

    catalog = load_catalog(CATALOG_PATH)
    compilable_cats = get_compilable_categories(catalog)

    # Phase 1: Source-level analysis (fast)
    print("host-deny: Phase 1 -- source-level analysis", file=sys.stderr)
    analysis = {}
    missing_status = []
    fixture_count = 0

    for rel, dir_name, cat, status in discover_fixtures(catalog):
        if cat not in compilable_cats:
            continue
        fixture_count += 1
        source_path = REPO_ROOT / rel
        matches = scan_source_for_host_imports(source_path)
        verdict = categorize_host_imports(matches)
        analysis[rel] = {"matches": matches, "verdict": verdict}

        explicit = get_catalog_explicit_status(catalog, rel)
        dir_info = catalog.get("directories", {}).get(dir_name, {})
        dir_explicit = dir_info.get("host_imports")
        if explicit is None and dir_explicit is None:
            missing_status.append(rel)

    # Phase 2: Compilation-based verification
    compile_results = {}
    if force_compile:
        print("host-deny: Phase 2 -- compilation-based verification (all fixtures)", file=sys.stderr)
        compile_results = build_fixture_compile_check(catalog, list(analysis.keys()), verbose)
    else:
        subset = set()
        seen_dirs = set()
        for rel, dir_name, cat, status in discover_fixtures(catalog):
            if cat not in compilable_cats:
                continue
            if dir_name not in seen_dirs:
                subset.add(rel)
                seen_dirs.add(dir_name)
        if subset:
            print(f"host-deny: Phase 2 -- representative subset ({len(subset)} fixtures)", file=sys.stderr)
            compile_results = build_fixture_compile_check(catalog, list(subset), verbose)

    # Phase 3: Report matrix
    total = fixture_count
    allow_count = sum(1 for v in analysis.values() if v["verdict"] == "allow")
    deny_count = sum(1 for v in analysis.values() if v["verdict"] == "deny")

    print(file=sys.stderr)
    print("=" * 70, file=sys.stderr)
    print("HOST-IMPORT DENY MATRIX", file=sys.stderr)
    print("=" * 70, file=sys.stderr)

    if verbose:
        for rel in sorted(analysis.keys()):
            v = analysis[rel]
            icon = "ALLOW" if v["verdict"] == "allow" else "DENY"
            if v["matches"]:
                patterns = "; ".join(v["matches"][:3])
                print(f"  {icon:5s}  {rel}  [{patterns}]", file=sys.stderr)
            else:
                print(f"  {icon:5s}  {rel}", file=sys.stderr)

    print(file=sys.stderr)
    print(f"  Total fixtures analyzed:  {total}", file=sys.stderr)
    print(f"  Allow (no host imports):  {allow_count}", file=sys.stderr)
    print(f"  Deny (has host imports):  {deny_count}", file=sys.stderr)
    print(file=sys.stderr)

    if missing_status:
        print(f"  Fixtures without explicit host_imports status: {len(missing_status)}", file=sys.stderr)
        if verbose:
            for rel in sorted(missing_status)[:20]:
                verdict = analysis.get(rel, {}).get("verdict", "unknown")
                print(f"    {verdict:5s}  {rel}", file=sys.stderr)
            if len(missing_status) > 20:
                print(f"    ... and {len(missing_status) - 20} more", file=sys.stderr)
    else:
        print("  All fixtures have explicit host_imports status", file=sys.stderr)
    print(file=sys.stderr)

    if compile_results:
        compile_allow = sum(1 for v in compile_results.values() if v.get("allow"))
        compile_deny = sum(1 for v in compile_results.values() if not v.get("allow"))
        print(f"  Compilation check ({len(compile_results)} fixtures):", file=sys.stderr)
        print(f"    Host-free (allow): {compile_allow}", file=sys.stderr)
        print(f"    Host imports:      {compile_deny}", file=sys.stderr)
        print(file=sys.stderr)
        for rel in sorted(compile_results.keys()):
            v = compile_results[rel]
            if not v.get("allow"):
                print(f"    FAIL: {rel}  [{v.get('detail', 'unknown')}]", file=sys.stderr)
        print(file=sys.stderr)

    if update_catalog:
        print("host-deny: Updating catalog.yaml with host_imports status...", file=sys.stderr)
        for rel, info in analysis.items():
            parts = rel.replace("fixtures/", "").split("/")
            dir_name = parts[0]
            fname = "/".join(parts[1:])
            dir_info = catalog.get("directories", {}).get(dir_name)
            if dir_info is None:
                continue
            if fname in dir_info.get("fixtures", []):
                dir_info["host_imports"] = info["verdict"]
        with open(CATALOG_PATH, "w") as f:
            yaml.dump(catalog, f, default_flow_style=False, sort_keys=False)
        print("host-deny: catalog.yaml updated", file=sys.stderr)

    print("host-deny: OK (exiting 0 -- info-only gate)", file=sys.stderr)
    sys.exit(0)


if __name__ == "__main__":
    main()
