#!/usr/bin/env python3
"""Host-import deny matrix and policy checker.

Reads fixtures/catalog.yaml, analyzes each fixture source file for Node host
import patterns, compiles a representative subset, and reports an explicit
allow/deny status per fixture.

Usage:
  python scripts/check/host-deny.py                                         # info-only report
  python scripts/check/host-deny.py --compile                                # fails on host import violations
  python scripts/check/host-deny.py --compile --report-only                  # info-only even with --compile
  python scripts/check/host-deny.py --strict                                 # strict compile+manifest+wasm gate
  python scripts/check/host-deny.py --strict --fixture PATH.ts               # single fixture in strict mode
  python scripts/check/host-deny.py --strict --limit 10                      # first 10 fixtures in strict mode
  python scripts/check/host-deny.py --self-test                              # run self-test
  python scripts/check/host-deny.py --update-catalog

Options:
  --update-catalog     Add host_imports field to catalog.yaml entries.
  --compile            Force compilation-based check (slower but more accurate).
                       Exits with 1 when a host-free fixture imports (import "host" ...).
  --strict             Strict mode: compile all non-excluded fixtures, emit manifests,
                       inspect wasm import sections, and validate manifest properties
                       (standalone=true, node_host.required=false, no host imports).
  --fixture PATH       Check a single fixture path (relative to repo root).
  --limit N            Maximum number of fixtures to process (for debugging).
  --report-only        Report only, exit 0 even on violations (default with --compile off).
  --self-test          Run self-test with fake manifest/wasm import pairs.
  -v, --verbose        Print per-fixture analysis details.

Strict mode exit code:
  0 if all fixtures pass
  1 if any fixture has host imports, missing manifest reasons, or policy violations

Dependencies: cargo, wasm-tools (for --compile and --strict modes).
"""

import sys
import subprocess
import shutil
import tempfile
import os
import re
import json
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


def build_fixture_with_manifest(ts2wasm_bin, fixture_path, wasm_path, manifest_path):
    """Build a fixture with manifest emission. Returns (success, error_message)."""
    result = subprocess.run(
        [str(ts2wasm_bin), "build", str(fixture_path), "-o", str(wasm_path),
         "--emit-manifest", str(manifest_path)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        err = result.stderr.strip() or result.stdout.strip()
        return False, err[:300]
    return True, ""


def validate_strict_fixture(manifest_path, wasm_path, fixture_rel):
    """Validate a fixture in strict mode.
    Returns (success, list_of_violations)."""
    violations = []

    # Check wasm import section
    result = subprocess.run(
        ["wasm-tools", "print", str(wasm_path)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        violations.append(f"wasm-tools print failed: {result.stderr.strip()}")
        return len(violations) == 0, violations

    has_host_import = '(import "host"' in result.stdout
    if has_host_import:
        violations.append(f"wasm import section contains (import \"host\" ...)")

    # Check manifest
    try:
        with open(manifest_path) as f:
            manifest = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        violations.append(f"manifest read error: {e}")
        return len(violations) == 0, violations

    if manifest.get("node_host", {}).get("required", False):
        violations.append("manifest node_host.required is true")

    if not manifest.get("standalone", True):
        violations.append("manifest standalone is false")

    # Check capability reasons exist for used capabilities
    reasons = manifest.get("capability_reasons", {})
    wasi = manifest.get("wasi", {})
    wasi_reason_keys = {
        "stdout": "wasi.stdout",
        "stdin": "wasi.stdin",
        "stderr": "wasi.stderr",
        "args": "wasi.args",
        "env": "wasi.env",
        "random": "wasi.random",
    }
    for cap_key, reason_key in wasi_reason_keys.items():
        if wasi.get(cap_key) and reason_key not in reasons:
            violations.append(f"manifest missing reason for {reason_key}")

    if wasi.get("clock", {}).get("realtime") and "wasi.clock.realtime" not in reasons:
        violations.append("manifest missing reason for wasi.clock.realtime")

    filesystem = wasi.get("filesystem", {})
    if filesystem.get("read") and "wasi.filesystem.read" not in reasons:
        violations.append("manifest missing reason for wasi.filesystem.read")
    if filesystem.get("write") and "wasi.filesystem.write" not in reasons:
        violations.append("manifest missing reason for wasi.filesystem.write")

    for imp in manifest.get("node_host", {}).get("imports", []):
        if imp not in reasons:
            violations.append(f"manifest missing reason for node_host import {imp}")

    return len(violations) == 0, violations


def run_strict_mode(catalog, fixtures, limit, verbose):
    """Run strict mode: compile all fixtures, emit manifests, validate."""
    if not shutil.which("wasm-tools"):
        print("host-deny: error: wasm-tools is required for --strict mode", file=sys.stderr)
        sys.exit(1)

    print("host-deny: Building ts2wasm-cli...", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print("host-deny: error: cargo build failed", file=sys.stderr)
        sys.exit(1)

    ts2wasm_bin = REPO_ROOT / "target/debug/ts2wasm"
    if not ts2wasm_bin.exists():
        print("host-deny: error: ts2wasm binary not found", file=sys.stderr)
        sys.exit(1)

    compilable_cats = get_compilable_categories(catalog)
    non_compilable = {"negative", "parser", "test-infrastructure"}

    if fixtures:
        # Use explicitly listed fixtures
        all_fixtures = [(f, None, None, None) for f in fixtures]
    else:
        all_fixtures = list(discover_fixtures(catalog))
        # Filter by compilable category
        all_fixtures = [f for f in all_fixtures if f[2] in compilable_cats]

    if limit and limit > 0:
        all_fixtures = all_fixtures[:limit]

    total = len(all_fixtures)
    passed = 0
    failed = 0
    failures = []

    print(f"host-deny: Strict mode checking {total} fixture(s)...", file=sys.stderr)

    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        for idx, (rel, dir_name, cat, status) in enumerate(all_fixtures):
            fixture_path = REPO_ROOT / rel
            if not fixture_path.exists():
                print(f"host-deny: SKIP (not found): {rel}", file=sys.stderr)
                total -= 1
                continue

            wasm_path = tmpd / f"strict_{idx}.wasm"
            manifest_path = tmpd / f"strict_{idx}.json"

            ok, log = build_fixture_with_manifest(ts2wasm_bin, fixture_path, wasm_path, manifest_path)
            if not ok:
                failed += 1
                failures.append((rel, f"build_error: {log}"))
                if verbose:
                    print(f"  BUILD FAIL: {rel}", file=sys.stderr)
                continue

            ok, violations = validate_strict_fixture(manifest_path, wasm_path, rel)
            if ok:
                passed += 1
                if verbose:
                    print(f"  PASS: {rel}", file=sys.stderr)
            else:
                failed += 1
                failures.append((rel, violations))
                if verbose:
                    print(f"  FAIL: {rel}  [{'; '.join(violations)}]", file=sys.stderr)

    print(file=sys.stderr)
    print("=" * 70, file=sys.stderr)
    print("HOST-DENY STRICT MODE RESULTS", file=sys.stderr)
    print("=" * 70, file=sys.stderr)
    print(f"  Total: {total}", file=sys.stderr)
    print(f"  Pass:  {passed}", file=sys.stderr)
    print(f"  Fail:  {failed}", file=sys.stderr)
    print(file=sys.stderr)

    if failures:
        for rel, detail in failures:
            if isinstance(detail, list):
                for v in detail:
                    print(f"  FAIL: {rel}  [{v}]", file=sys.stderr)
            else:
                print(f"  FAIL: {rel}  [{detail}]", file=sys.stderr)
        print(file=sys.stderr)
        print("host-deny: FAIL (strict mode violations found)", file=sys.stderr)
        sys.exit(1)

    print("host-deny: OK (all fixtures pass strict mode)", file=sys.stderr)
    sys.exit(0)


def _make_minimal_wasm(imports):
    """Create a minimal valid wasm binary with given imports.

    imports: list of (module, name) tuples.
    Returns bytes of valid wasm module.
    """
    # Build type section content
    type_content = bytearray()
    num_types = len(imports)
    type_content.append(num_types)
    for _ in imports:
        # functype: (func (param i32))
        type_content.extend([0x60, 0x01, 0x7F, 0x00])

    # Build import section content
    import_content = bytearray()
    import_content.append(len(imports))
    for type_idx, (mod_name, fn_name) in enumerate(imports):
        mod_bytes = mod_name.encode("utf-8")
        fn_bytes = fn_name.encode("utf-8")
        import_content.append(len(mod_bytes))
        import_content.extend(mod_bytes)
        import_content.append(len(fn_bytes))
        import_content.extend(fn_bytes)
        import_content.append(0x00)  # import kind: func
        # Encode type index as LEB128
        val = type_idx
        while val > 0x7F:
            import_content.append((val & 0x7F) | 0x80)
            val >>= 7
        import_content.append(val & 0x7F)

    # Encode section sizes with LEB128
    def encode_leb128(val):
        result = bytearray()
        while True:
            byte_val = val & 0x7F
            val >>= 7
            if val != 0:
                byte_val |= 0x80
            result.append(byte_val)
            if val == 0:
                break
        return bytes(result)

    # Build sections
    type_sec_size = encode_leb128(len(type_content))
    import_sec_size = encode_leb128(len(import_content))
    func_sec = bytes([0x03]) + encode_leb128(1) + bytes([0x00])  # 0 functions

    wasm_parts = [
        bytes([0x00, 0x61, 0x73, 0x6D]),  # magic
        bytes([0x01, 0x00, 0x00, 0x00]),  # version
        bytes([0x01]), type_sec_size, bytes(type_content),   # type section
        bytes([0x02]), import_sec_size, bytes(import_content),  # import section
        func_sec,  # function section (0)
        bytes([0x05]) + encode_leb128(4) + bytes([0x01, 0x01, 0x00, 0x01]),  # memory section
    ]
    return b"".join(wasm_parts)


def run_self_test():
    """Run self-test with fake manifest/wasm import pairs."""
    print("host-deny: Running self-test...", file=sys.stderr)
    temp_dir = Path(tempfile.mkdtemp(prefix="host-deny-self-test-"))

    try:
        # Test 1: fake standalone fixture - should pass
        test1_dir = temp_dir / "test1"
        test1_dir.mkdir()
        test1_wasm = test1_dir / "test.wasm"
        test1_manifest = test1_dir / "manifest.json"

        test1_wasm.write_bytes(_make_minimal_wasm([("wasi_snapshot_preview1", "proc_exit")]))

        manifest = {
            "schema_version": 1,
            "target": "wasm32-wasi",
            "standalone": True,
            "wasi": {"stdout": False, "stdin": False, "stderr": False,
                      "args": False, "env": False, "clock": {"realtime": False},
                      "filesystem": {"read": [], "write": [], "preopens": []},
                      "random": False},
            "node_host": {"required": False, "imports": []},
            "capability_reasons": {},
        }
        with open(test1_manifest, "w") as f:
            json.dump(manifest, f)

        ok, violations = validate_strict_fixture(test1_manifest, test1_wasm, "test1/test.ts")
        assert ok, f"test1 should pass: {violations}"
        print("  PASS: Test 1 (valid standalone fixture)", file=sys.stderr)

        # Test 2: fixture with host import - should fail
        test2_dir = temp_dir / "test2"
        test2_dir.mkdir()
        test2_wasm = test2_dir / "test.wasm"
        test2_manifest = test2_dir / "manifest.json"

        test2_wasm.write_bytes(_make_minimal_wasm([("host", "crypto.randomBytes")]))

        manifest2 = dict(manifest)
        manifest2["standalone"] = False
        manifest2["node_host"] = {"required": True, "imports": ["host.crypto.randomBytes"]}
        manifest2["capability_reasons"] = {"host.crypto.randomBytes": ["test"]}
        with open(test2_manifest, "w") as f:
            json.dump(manifest2, f)

        ok, violations = validate_strict_fixture(test2_manifest, test2_wasm, "test2/test.ts")
        assert not ok, "test2 should fail (has host import)"
        print("  PASS: Test 2 (host import fixture correctly rejected)", file=sys.stderr)

        # Test 3: fixture missing reasons - should fail
        test3_dir = temp_dir / "test3"
        test3_dir.mkdir()
        test3_wasm = test3_dir / "test.wasm"
        test3_manifest = test3_dir / "manifest.json"

        test3_wasm.write_bytes(_make_minimal_wasm([("wasi_snapshot_preview1", "proc_exit")]))

        manifest3 = dict(manifest)
        manifest3["wasi"] = {"stdout": True, "stdin": False, "stderr": False,
                             "args": False, "env": False, "clock": {"realtime": False},
                             "filesystem": {"read": [], "write": [], "preopens": []},
                             "random": False}
        # Missing reason for wasi.stdout!
        with open(test3_manifest, "w") as f:
            json.dump(manifest3, f)

        ok, violations = validate_strict_fixture(test3_manifest, test3_wasm, "test3/test.ts")
        assert not ok, "test3 should fail (missing reason)"
        print("  PASS: Test 3 (missing reason fixture correctly rejected)", file=sys.stderr)

        print("host-deny: Self-test PASSED (all 3 tests passed)", file=sys.stderr)
    except Exception as e:
        print(f"host-deny: Self-test FAILED: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
    finally:
        shutil.rmtree(temp_dir, ignore_errors=True)
    sys.exit(0)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__)
        sys.exit(0)

    update_catalog = "--update-catalog" in args
    force_compile = "--compile" in args
    strict_mode = "--strict" in args
    report_only = "--report-only" in args
    self_test = "--self-test" in args
    verbose = "-v" in args or "--verbose" in args

    # Extract --fixture and --limit
    fixture_path = None
    limit = None
    filtered_args = []
    i = 0
    while i < len(args):
        if args[i] == "--fixture" and i + 1 < len(args):
            fixture_path = args[i + 1]
            i += 2
        elif args[i] == "--limit" and i + 1 < len(args):
            try:
                limit = int(args[i + 1])
            except ValueError:
                print(f"host-deny: invalid --limit value: {args[i+1]}", file=sys.stderr)
                sys.exit(1)
            i += 2
        elif args[i] in ("--update-catalog", "--compile", "--strict", "--report-only",
                         "--self-test", "-v", "--verbose"):
            filtered_args.append(args[i])
            i += 1
        else:
            i += 1

    if self_test:
        run_self_test()

    if not CATALOG_PATH.exists():
        print(f"host-deny: catalog not found: {CATALOG_PATH}", file=sys.stderr)
        sys.exit(1)

    catalog = load_catalog(CATALOG_PATH)

    if strict_mode:
        # Validate catalog: every compilable fixture should have host policy
        compilable_cats = get_compilable_categories(catalog)
        missing_policy = []
        for rel, dir_name, cat, status in discover_fixtures(catalog):
            if cat not in compilable_cats:
                continue
            explicit = get_catalog_explicit_status(catalog, rel)
            dir_info = catalog.get("directories", {}).get(dir_name, {})
            dir_explicit = dir_info.get("host_imports")
            if explicit is None and dir_explicit is None:
                missing_policy.append(rel)

        if missing_policy and not fixture_path:
            print(f"host-deny: warning: {len(missing_policy)} fixture(s) without host_imports policy", file=sys.stderr)
            if verbose:
                for rel in missing_policy[:10]:
                    print(f"  MISSING POLICY: {rel}", file=sys.stderr)
                if len(missing_policy) > 10:
                    print(f"  ... and {len(missing_policy) - 10} more", file=sys.stderr)

        # Determine which fixtures to check
        strict_fixtures = []
        if fixture_path:
            strict_fixtures = [fixture_path]
        else:
            for rel, dir_name, cat, status in discover_fixtures(catalog):
                if cat in compilable_cats:
                    strict_fixtures.append(rel)

        if strict_fixtures:
            run_strict_mode(catalog, strict_fixtures, limit, verbose)
        else:
            print("host-deny: strict mode: no fixtures to check", file=sys.stderr)
            sys.exit(0)

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

    # --compile mode with hard gate (unless --report-only)
    has_violations = any(
        not v.get("allow") for v in compile_results.values()
    ) if compile_results else False

    if force_compile and has_violations and not report_only:
        print("host-deny: FAIL (--compile mode, host import violations found)", file=sys.stderr)
        sys.exit(1)

    print("host-deny: OK (exiting 0 -- info-only gate)", file=sys.stderr)
    sys.exit(0)


if __name__ == "__main__":
    main()
